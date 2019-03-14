//! Lazy future-driven queue processing.

#![deny(missing_docs)]

use futures::{try_ready, Async, Future, IntoFuture, Poll, Sink, StartSend, Stream};
use std::{fmt, mem};
use tokio_sync::mpsc;

/// A sink-like queue.
pub struct LazyQueue<Item> {
    sender: mpsc::Sender<Item>,
}

impl<Item> Sink for LazyQueue<Item> {
    type SinkItem = Item;
    type SinkError = mpsc::error::SendError;

    fn start_send(&mut self, item: Item) -> StartSend<Item, Self::SinkError> {
        self.sender.start_send(item)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        self.sender.poll_complete()
    }
}

impl<Item> LazyQueue<Item> {
    /// Creates a new lazy queue using given processor and maximum queue length.
    pub fn new<F, R>(processor: F, max_len: usize) -> (Self, QueueProcessor<Item, F, R>)
    where
        F: FnMut(Item) -> R,
        R: IntoFuture,
    {
        let (sender, receiver) = mpsc::channel(max_len);
        (
            LazyQueue { sender },
            QueueProcessor {
                receiver,
                processor,
                state: State::WaitingForItem,
            },
        )
    }
}

/// Lazy queue processor.
#[must_use = "futures do nothing unless polled"]
pub struct QueueProcessor<Item, F, R>
where
    F: FnMut(Item) -> R,
    R: IntoFuture,
{
    receiver: mpsc::Receiver<Item>,
    processor: F,
    state: State<R>,
}

enum State<R: IntoFuture> {
    WaitingForItem,
    RunningProcessor(R::Future),
}

/// An error that might happen during processing of a queue.
#[derive(Debug)]
pub enum ProcessingError<E> {
    /// Channel has closed.
    ReceiverError(mpsc::error::RecvError),
    /// Error returned by a processor.
    FutureError(E),
}

impl<E: fmt::Display> fmt::Display for ProcessingError<E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessingError::ReceiverError(e) => write!(fmt, "{}", e),
            ProcessingError::FutureError(e) => write!(fmt, "{}", e),
        }
    }
}

impl<E: std::error::Error> std::error::Error for ProcessingError<E> {
    fn description(&self) -> &str {
        match self {
            ProcessingError::ReceiverError(e) => e.description(),
            ProcessingError::FutureError(e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            ProcessingError::ReceiverError(e) => Some(e),
            ProcessingError::FutureError(e) => Some(e),
        }
    }
}

impl<Item, F, R> Future for QueueProcessor<Item, F, R>
where
    F: FnMut(Item) -> R,
    R: IntoFuture,
{
    type Item = ();
    type Error = ProcessingError<R::Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let next = match &mut self.state {
                State::WaitingForItem => {
                    if let Some(item) =
                        try_ready!(self.receiver.poll().map_err(ProcessingError::ReceiverError))
                    {
                        let fut = (self.processor)(item);
                        State::RunningProcessor(fut.into_future())
                    } else {
                        return Ok(Async::Ready(()));
                    }
                }
                State::RunningProcessor(fut) => {
                    try_ready!(fut.poll().map_err(ProcessingError::FutureError));
                    State::WaitingForItem
                }
            };
            mem::replace(&mut self.state, next);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::lazy;
    use std::{cell::RefCell, rc::Rc};
    use tokio::runtime::Runtime;

    fn try_send<Item>(queue: &mut LazyQueue<Item>, item: Item) -> Result<(), &'static str> {
        if !queue
            .start_send(item)
            .map(|ok| ok.is_ready())
            .unwrap_or(false)
        {
            return Err("Not ready");
        }
        if !queue.poll_complete().unwrap().is_ready() {
            Err("Not ready")
        } else {
            Ok(())
        }
    }

    #[test]
    fn test_queue_length() {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(lazy(move || {
            const QUEUE_LEN: usize = 4;
            let (mut queue, mut driver) = LazyQueue::new(|_: u8| Ok::<_, ()>(()), QUEUE_LEN);
            assert!(driver.poll().unwrap().is_not_ready());
            for _ in 0..QUEUE_LEN {
                try_send(&mut queue, 0).unwrap();
            }
            if try_send(&mut queue, 0).is_ok() {
                panic!("Ready but shouldn't be");
            }
            drop(queue);
            assert!(driver.poll().unwrap().is_ready());
            Ok::<_, ()>(())
        }))
        .unwrap();
    }

    #[test]
    fn test_running() {
        let mut rt = Runtime::new().unwrap();
        rt.block_on(lazy(move || {
            let received = Rc::new(RefCell::new(vec![]));
            let received_clone = Rc::clone(&received);
            let processor = move |item: u8| {
                received_clone.borrow_mut().push(item);
                Ok::<_, ()>(())
            };
            let (mut queue, mut driver) = LazyQueue::new(processor, 10);
            try_send(&mut queue, 0).unwrap();
            try_send(&mut queue, 2).unwrap();
            try_send(&mut queue, 1).unwrap();
            drop(queue);
            assert!(driver.poll().unwrap().is_ready());
            assert_eq!(&[0, 2, 1], received.borrow().as_slice());
            Ok::<_, ()>(())
        }))
        .unwrap();
    }
}
