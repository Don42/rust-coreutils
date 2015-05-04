#![feature(collections)]
extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("n", "", "do not output the trailing newline");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    for argument in args.tail() {
        print!("{} ", argument);
    }
    if !matches.opt_present("n") {
        print!("\n");
    }
}
