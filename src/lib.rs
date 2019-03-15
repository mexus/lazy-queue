//! [![travis](https://img.shields.io/travis/mexus/lazy-queue.svg)](https://travis-ci.org/mexus/lazy-queue)
//! [![crates.io](https://img.shields.io/crates/v/lazy-queue.svg)](https://crates.io/crates/lazy-queue)
//! [![docs.rs](https://docs.rs/futures-retry/badge.svg)](https://docs.rs/lazy-queue)
//!
//! [Master docs](https://mexus.github.io/lazy-queue/lazy_queue/index.html)
//!
//! Lazy future-driven queue processing.
//!
//! Some typical use cases would be:
//!
//! * Offloading work from multiple threads into a single-threaded processor.
//! * Posponing computation.
//! * Processing multiple tasks in a batch.
//! * ...
//!
//! ## License
//!
//! Licensed under either of
//!
//!  * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//!
//! at your option.
//!
//! ### Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
//! additional terms or conditions.

#![deny(missing_docs)]

use std::fmt;

/// This macro creates `LazyQueue<Item>` and `QueueProcessor<Item, P, I>` (where `I: IntoFuture`)
/// and implementations for them.
macro_rules! implement {
    (
        $doc:literal,
        $sender:ty,
        $receiver:ty,
        ($($new_args:ident: $new_args_ty:ty),*),
        $make_chan:expr,
        $send_error:ty,
        $rcv_error:ty $(,)?
    ) => {
        #[doc = $doc]
        pub struct LazyQueue<Item> {
            sender: $sender,
        }

        impl<Item> Clone for LazyQueue<Item> {
            fn clone(&self) -> Self {
                LazyQueue {
                    sender: self.sender.clone(),
                }
            }
        }

        /// Lazy queue processor.
        #[must_use = "futures do nothing unless polled"]
        pub struct QueueProcessor<Item, P, I>
        where
            I: ::futures::IntoFuture,
        {
            inner: crate::inner::StreamProcessor<$receiver, P, I>,
        }

        impl<Item> ::futures::Sink for LazyQueue<Item> {
            type SinkItem = Item;
            type SinkError = $send_error;

            fn start_send(&mut self, item: Item) -> ::futures::StartSend<Item, $send_error> {
                self.sender.start_send(item)
            }

            fn poll_complete(&mut self) -> ::futures::Poll<(), $send_error> {
                self.sender.poll_complete()
            }
        }

        impl<Item> LazyQueue<Item> {
            /// Creates a new lazy queue using given processor.
            pub fn new<F, I>(processor: F, $($new_args: $new_args_ty),*) -> (Self, QueueProcessor<Item, F, I>)
            where
                F: FnMut(Item) -> I,
                I: ::futures::IntoFuture,
            {
                let (sender, receiver) = $make_chan;
                (
                    LazyQueue { sender },
                    QueueProcessor {
                        inner: StreamProcessor::new(receiver, processor),
                    },
                )
            }
        }

        impl<Item, P, I> ::futures::Future for QueueProcessor<Item, P, I>
        where
            P: FnMut(Item) -> I,
            I: ::futures::IntoFuture,
        {
            type Item = ();
            type Error = ProcessingError<$rcv_error, I::Error>;

            fn poll(&mut self) -> ::futures::Poll<Self::Item, Self::Error> {
                self.inner.poll()
            }
        }
    };
}

mod inner;
pub mod sync;
pub mod unsync;

/// An error that might happen during processing of a queue.
#[derive(Debug)]
pub enum ProcessingError<R, E> {
    /// Channel has closed.
    ReceiverError(R),
    /// Error returned by a processor.
    FutureError(E),
}

impl<R: fmt::Display, E: fmt::Display> fmt::Display for ProcessingError<R, E> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProcessingError::ReceiverError(e) => write!(fmt, "{}", e),
            ProcessingError::FutureError(e) => write!(fmt, "{}", e),
        }
    }
}

impl<R: std::error::Error, E: std::error::Error> std::error::Error for ProcessingError<R, E> {
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

#[cfg(test)]
mod test {
    use futures::{future::poll_fn, Async, Future};

    /// Tries to resolve the future on the first try.
    pub fn try_once<F: Future>(mut f: F) -> impl Future<Item = Option<F::Item>, Error = F::Error> {
        poll_fn(move || {
            if let Async::Ready(item) = f.poll()? {
                Ok(Async::Ready(Some(item)))
            } else {
                Ok(Async::Ready(None))
            }
        })
    }
}
