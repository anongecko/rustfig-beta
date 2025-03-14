use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::thread;

/// A flag that can be set to cancel operations
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
    
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }
    
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
    
    pub fn clone_token(&self) -> Self {
        Self {
            cancelled: Arc::clone(&self.cancelled),
        }
    }
}

/// Execute a function with a timeout
pub fn with_timeout<F, R>(f: F, timeout: Duration) -> Option<R>
where
    F: FnOnce() -> R,
    R: Send + 'static,
{
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = thread::spawn(move || {
        let result = f();
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(timeout) {
        Ok(result) => Some(result),
        Err(_) => {
            // Timed out, thread will continue but we don't wait for it
            None
        }
    }
}

/// A helper for periodic tasks
pub struct PeriodicTask {
    last_run: Instant,
    interval: Duration,
}

impl PeriodicTask {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            last_run: Instant::now(),
            interval: Duration::from_millis(interval_ms),
        }
    }
    
    pub fn should_run(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.last_run) >= self.interval {
            self.last_run = now;
            true
        } else {
            false
        }
    }
    
    pub fn run_if_needed<F>(&mut self, f: F)
    where
        F: FnOnce(),
    {
        if self.should_run() {
            f();
        }
    }
}
