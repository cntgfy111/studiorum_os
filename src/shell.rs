use alloc::string::String;
use alloc::vec::Vec;

use conquer_once::spin::OnceCell;
use hashbrown::{hash_map, HashMap};

use crate::{print, println};
use crate::task::stdin::{read_std_in, StdInStream};

static PROGRAMS: OnceCell<HashMap<String, Entry>> = OnceCell::uninit();

struct Entry {
    function: dyn FnMut(),
    number_of_arguments: u64,
    args_parsers: Vec<dyn FnMut()>,
}

// Based on lsh: https://brennan.io/2015/01/16/write-a-shell-in-c/
pub async fn lsh() {
    let mut status = true;
    let mut line: String;
    init_table();
    while status {
        print!("> ");
        line = read_std_in().await;
        line.split_ascii_whitespace();
        print!("{}", line);
    }
}

// fn init_table() {
//     PROGRAMS.try_init_once(|| {
//         let map = HashMap::default();
//         map
//     })
// }
