#![feature(collections)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for argument in args.tail() {
        print!("{} ", argument);
    }
    print!("\n");
}
