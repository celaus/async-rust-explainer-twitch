//! This example builds on https://github.com/contextfreecode/async-exec

use std::{fs::OpenOptions, io::Read, time::Instant};
use {
    futures::future::Future,
    std::{
        pin::Pin,
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
    },
};

/// Generates 1GB of random seed and prints the elapsed time.
pub async fn random_seed() -> Arc<Vec<u8>> {
    let start = Instant::now();
    let result = RandomSeedFuture::new(1_000_000_000).await;
    println!(
        "{:?} generated a 1 GB random seed in {}s",
        std::thread::current().id(),
        start.elapsed().as_secs_f64()
    );
    result
}

/// Linux random data source
const SOURCE: &str = "/dev/random";

/// Data container for the future
struct RandomNumberSeed {
    completed: bool,
    data: Arc<Vec<u8>>,
    waker: Option<Waker>,
}

/// A future that reads a number of bytes from the Linux random device
pub struct RandomSeedFuture {
    shared_state: Arc<Mutex<RandomNumberSeed>>,
}

impl RandomSeedFuture {
    pub fn new(n_bytes: usize) -> Self {
        let shared_state = Arc::new(Mutex::new(RandomNumberSeed {
            completed: false,
            data: Arc::new(vec![]),
            waker: None,
        }));
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            let mut source = OpenOptions::new().read(true).open(SOURCE).unwrap();
            let mut buf = vec![0; n_bytes];
            let _ = source.read_exact(&mut buf);
            {
                let mut shared_state = thread_shared_state.lock().unwrap();
                shared_state.data = Arc::new(buf);
                shared_state.completed = true;
                if let Some(waker) = shared_state.waker.take() {
                    waker.wake()
                }
            }
        });
        RandomSeedFuture { shared_state }
    }
}

impl Future for RandomSeedFuture {
    type Output = Arc<Vec<u8>>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(shared_state.data.clone())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]

    use super::*;
    #[async_std::test]
    async fn RandomSeedFuture__is_awaitable() {
        let n_bytes = 1_000_000_000; // 1GB 
        let rsf = RandomSeedFuture::new(n_bytes);
        let data = rsf.await;
        assert!(data.len() == n_bytes);
    }
}
