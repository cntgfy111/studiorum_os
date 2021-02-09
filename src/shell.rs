#![feature(iterator_fold_self)]

use alloc::string::String;
use alloc::vec::Vec;
use core::str::SplitAsciiWhitespace;

use conquer_once::spin::OnceCell;

use crate::{print, println};
use crate::task::stdin::{read_std_in, StdInStream};

static MESSAGE: &str = "List of available programs:
help for this message
echo to print arguments";

// Based on lsh: https://brennan.io/2015/01/16/write-a-shell-in-c/
pub async fn lsh() {
    let mut status = true;
    let mut line: String;
    while status {
        print!("> ");
        line = read_std_in().await;
        let command = line.split_ascii_whitespace();
        run_command(command);
    }
}

fn run_command(mut args: SplitAsciiWhitespace) {
    if let Some(program_name) = args.next() {
        match program_name {
            "echo" => echo(args),
            _ => println!("{}", MESSAGE),
        }
    }
}

fn echo(args: SplitAsciiWhitespace) {
    println!("{}", args.fold(String::new(), |mut acc, x| {
        acc.push_str(x);
        acc.push(' ');
        acc
    }))
}

