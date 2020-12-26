use core::sync::atomic::Ordering;

use crate::library::time::TIME;

// GPL https://www.jstatsoft.org/article/view/v008i14
// Just xorshift algorithm
pub fn random() -> u64 {
    let mut x = TIME.load(Ordering::SeqCst);
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    x
}