use crate::args::Args;
use ahash::HashMap;
use parking_lot::Mutex;
use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tokio::time::Instant;

#[derive(Debug)]
pub struct State {
    pub start: Instant,
    pub duration: Duration,
    pub deadline: Instant,
    pub errors: Mutex<ErrorMap>,
    pub request_total: Counter,
    pub request_second: Counter,
    pub transfer_total: Counter,
}

impl State {
    pub fn new(args: &Args) -> Self {
        let start = Instant::now();
        let duration = Duration::from_secs(args.duration);
        let deadline = start + duration;

        Self {
            start,
            duration,
            deadline,
            errors: Default::default(),
            request_total: Default::default(),
            request_second: Default::default(),
            transfer_total: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct Counter {
    inner: AtomicU64,
}

impl Counter {
    pub fn add(&self, n: u64) -> u64 {
        self.inner.fetch_add(n, Ordering::Relaxed)
    }

    pub fn set(&self, n: u64) {
        self.inner.store(n, Ordering::Relaxed);
    }

    pub fn load(&self) -> u64 {
        self.inner.load(Ordering::Relaxed)
    }
}

#[derive(Debug, Default)]
pub struct ErrorMap {
    pub map: HashMap<String, u32>,
}

impl ErrorMap {
    pub fn add_error(&mut self, e: impl ToString) {
        *self.map.entry(e.to_string()).or_default() += 1;
    }
}
