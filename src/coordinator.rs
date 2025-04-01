use std::sync::atomic::{AtomicUsize, Ordering};

static COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn next_task() -> u64 {
    COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}