//! Thread safe queue implementation.

use crate::{inner::StreamProcessor, ProcessingError};
use tokio_sync::mpsc;

pub mod bounded {
    //! Bounded queue.
    use super::*;

    implement!(
        "A clonable thread safe sink-like queue.",
        mpsc::Sender<Item>,
        mpsc::Receiver<Item>,
        (queue_len: usize),
        mpsc::channel(queue_len),
        mpsc::error::SendError,
        mpsc::error::RecvError,
    );
}

pub mod unbounded {
    //! Unbounded queue.
    use super::*;

    implement!(
        "A clonable thread safe sink-like queue.",
        mpsc::UnboundedSender<Item>,
        mpsc::UnboundedReceiver<Item>,
        (),
        mpsc::unbounded_channel(),
        mpsc::error::UnboundedSendError,
        mpsc::error::UnboundedRecvError,
    );
}

#[cfg(test)]
mod tests {
    use super::bounded::*;
    use crate::test::try_once;
    use futures::{stream::iter_ok, Future, Sink, Stream};
    use std::sync::mpsc::channel;
    use tokio::runtime::Runtime;
    use tokio_sync::watch;

    #[test]
    fn multithread() {
        let (mut unpause, rx) = watch::channel(false);

        let (events_sender, events_receiver) = channel();

        // This "processor" will simply store all the incoming items.
        // But it won't complete until `unpause.broadcast(true)` is run.
        let rx_clone = rx.clone();
        let processor = move |item: u8| {
            let events_sender = events_sender.clone();
            rx_clone
                .clone()
                .filter(|&val| val)
                .into_future()
                .then(move |_| events_sender.send(item))
        };

        const CAPACITY: usize = 3;
        let (mut queue, driver) = LazyQueue::new(processor, CAPACITY);

        let mut rt = Runtime::new().unwrap();
        rt.spawn(driver.map_err(|e| panic!("Error while processing a queue: {}", e)));

        // Fill up the queue by sending into it a CAPACITY number of items.
        let items = vec![0, 2, 1];
        assert_eq!(CAPACITY, items.len()); // just a sanity check.
        queue = rt
            .block_on(
                queue
                    .send_all(iter_ok(items.clone()))
                    .map(|(queue, _)| queue),
            )
            .unwrap();

        // Now if we try to send anything else the call should block since the queue is already
        // filled up. We are sending 2 elements since the real queue's capacity might be
        // CAPACITY + 1.
        let maybe_queue = rt
            .block_on(try_once(queue.send_all(iter_ok(vec![9, 10]))))
            .unwrap();
        assert!(maybe_queue.is_none());
        unpause.broadcast(true).unwrap();

        rt.shutdown_on_idle().wait().unwrap();

        // Check that the processor has received the same items we have sent to it.
        assert_eq!(items, events_receiver.iter().take(3).collect::<Vec<_>>());
    }
}
