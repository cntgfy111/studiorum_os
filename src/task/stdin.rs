use alloc::string::String;
use core::pin::Pin;
use core::sync::atomic::{AtomicBool, Ordering};
use core::sync::atomic::Ordering::SeqCst;

use conquer_once::spin::OnceCell;
use futures_util::future::OrElse;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use futures_util::task::{AtomicWaker, Context, Poll};
use lazy_static::lazy_static;
use spin::Mutex;

use crate::allocator::Locked;
use crate::println;

static WAKER: AtomicWaker = AtomicWaker::new();

lazy_static! {
    static ref STDIN: StdIn = StdIn::new();
}

pub struct StdIn {
    input: Mutex<String>,
    ready_flag: AtomicBool,
}

impl StdIn {
    fn new() -> Self {
        StdIn {
            input: Mutex::new(String::new()),
            ready_flag: AtomicBool::new(false),
        }
    }
}

// TODO Normal wake
pub fn std_in_ready() {
    STDIN.ready_flag.store(true, Ordering::SeqCst);
    WAKER.wake();
}

pub fn write_std_str(string: &str) {
    STDIN.input.lock().push_str(string);
}

pub fn write_std_char(c: char) {
    STDIN.input.lock().push(c);
}

pub fn erase_char_std() {
    STDIN.input.lock().pop();
}

pub struct StdInStream {
    _private: (),
}

impl StdInStream {
    fn new() -> Self {
        StdInStream {
            _private: ()
        }
    }
}

impl Stream for StdInStream {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        WAKER.register(&cx.waker());
        if STDIN.ready_flag.load(Ordering::SeqCst) {
            if let Some(mut locked_input) = STDIN.input.try_lock() {
                STDIN.ready_flag.store(false, Ordering::SeqCst);
                WAKER.take();
                let cloned = locked_input.clone();
                locked_input.clear();
                return Poll::Ready(Some(cloned.clone()));
            }
        }
        Poll::Pending
    }
}

pub async fn double_std_in() {
    let mut stdin_stream = StdInStream::new();
    while let Some(input) = stdin_stream.next().await {
        println!("{}", input);
    }
}

pub async fn read_std_in() -> String {
    let mut stdin_stream = StdInStream::new();
    return stdin_stream.next().await.unwrap();
}