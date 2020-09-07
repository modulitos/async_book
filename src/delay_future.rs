use std::time::{Duration, Instant};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

#[derive(Debug)]
/// Inspired by: https://tokio.rs/tokio/tutorial/async#implementing-future
pub struct Delay {
    when: Instant,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
            -> Poll<&'static str>
    {
        if Instant::now() >= self.when {
            println!("Hello world");
            Poll::Ready("done")
        } else {

            // When a future returns Poll::Pending, it must ensure that the waker is signalled at
            // some point. Forgetting to do this results in the task hanging indefinitely.

            cx.waker().wake_by_ref();

            // Notice that you are allowed to signal the waker more often than necessary. In this
            // particular case, we signal the waker even though we are not ready to continue the
            // operation at all. There is nothing wrong with this besides some wasted CPU cycles,
            // however, this particular implementation will result in a busy loop.

            Poll::Pending
        }
    }
}

impl Delay {
    pub fn new(duration: Duration) -> Self {
        Delay {
            when: Instant::now() + duration
        }
    }
}
