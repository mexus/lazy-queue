use crate::ProcessingError;
use futures::{try_ready, Async, Future, IntoFuture, Poll, Stream};
use std::mem;

/// A `ForEach`-like future.
#[must_use = "futures do nothing unless polled"]
pub struct StreamProcessor<S, P, I>
where
    I: IntoFuture,
{
    receiver: S,
    processor: P,
    state: State<I>,
}

impl<S, P, I> StreamProcessor<S, P, I>
where
    I: IntoFuture,
{
    pub fn new(receiver: S, processor: P) -> Self {
        StreamProcessor {
            receiver,
            processor,
            state: State::WaitingForItem,
        }
    }
}

enum State<I: IntoFuture> {
    WaitingForItem,
    RunningProcessor(I::Future),
}

impl<S, P, I> Future for StreamProcessor<S, P, I>
where
    S: Stream,
    P: FnMut(S::Item) -> I,
    I: IntoFuture,
{
    type Item = ();
    type Error = ProcessingError<S::Error, I::Error>;

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
