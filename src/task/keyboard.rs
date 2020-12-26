use alloc::string::String;
use core::pin::Pin;
use core::sync::atomic::AtomicU8;

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::stream::Stream;
use futures_util::StreamExt;
use futures_util::task::{AtomicWaker, Context, Poll, Waker};
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyCode, layouts, ScancodeSet1};
use spin::Mutex;

use crate::{print, println, serial_println};
use crate::task::stdin::{erase_char_std, std_in_ready, write_std_char};
use crate::vga_buffer::{color, Color, erase, erase_line};

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

// pub async fn print_keypresses() {
//     let mut scancode_stream = ScancodeStream::new();
//     let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
//                                      HandleControl::Ignore);
//
//
//     while let Some(scancode) = scancode_stream.next().await {
//         if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
//             if let Some(key) = keyboard.process_keyevent(key_event) {
//                 match key {
//                     DecodedKey::Unicode(char) => {
//                         print!("{}", char);
//                     },
//                     DecodedKey::RawKey(key) => match key {
//                         KeyCode::ArrowLeft => erase(),
//                         _ => print!("{:?}", key)
//                     }
//                 }
//             }
//         }
//     }
// }

pub async fn print_keypresses() {
    let mut scancode_stream = ScancodeStream::new();
    let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1,
                                     HandleControl::Ignore);


    while let Some(scancode) = scancode_stream.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(char) => {
                        print!("{}", char);
                        write_std_char(char);
                        if char == '\n' {
                            std_in_ready();
                        }
                    },
                    DecodedKey::RawKey(key) => match key {
                        KeyCode::ArrowLeft => {
                            erase_char_std();
                            erase();
                        },
                        _ => print!("{:?}", key)
                    }
                }
            }
        }
    }
}


/// Called by the keyboard interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        color(Color::Yellow, Color::Black);
        print!("WARNING: scancode queue uninitialized");
        color(Color::LightGreen, Color::Black);
        println!();
    }
}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("ScancodeStream::new should only be called once");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("scancode queue not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}