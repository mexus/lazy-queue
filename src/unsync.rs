//! Single-threaded queue implementation.

use crate::{inner::StreamProcessor, ProcessingError};
use futures::unsync::mpsc;

pub mod bounded {
    //! Bounded queue.
    use super::*;

    implement!(
        "A clonable single-threaded sink-like queue.",
        mpsc::Sender<Item>,
        mpsc::Receiver<Item>,
        (queue_len: usize),
        mpsc::channel(queue_len),
        mpsc::SendError<Item>,
        (),
    );
}

pub mod unbounded {
    //! Unbounded queue.
    use super::*;

    implement!(
        "A clonable single-threaded sink-like queue.",
        mpsc::UnboundedSender<Item>,
        mpsc::UnboundedReceiver<Item>,
        (),
        mpsc::unbounded(),
        mpsc::SendError<Item>,
        (),
    );
}

#[cfg(test)]
mod tests {
    use super::bounded::*;
    use crate::test::try_once;
    use futures::{stream::iter_ok, Future, Sink};
    use std::sync::mpsc::channel;
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn singlethread() {
        let (events_sender, events_receiver) = channel();

        // This "processor" will simply store all the incoming items.
        let processor = move |item: u8| events_sender.send(item);

        const CAPACITY: usize = 3;
        let (mut queue, driver) = LazyQueue::new(processor, CAPACITY);

        let mut rt = Runtime::new().unwrap();
        rt.spawn(driver.map_err(|e| panic!("Error while processing a queue: {:?}", e)));

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

        rt.run().unwrap();

        // Check that the processor has received the same items we have sent to it.
        assert_eq!(items, events_receiver.iter().take(3).collect::<Vec<_>>());
    }
}
