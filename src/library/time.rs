use core::sync::atomic::{AtomicU64, Ordering};

pub static TIME: AtomicU64 = AtomicU64::new(0);

pub fn get_time() -> u64 {
    TIME.load(Ordering::SeqCst)
}