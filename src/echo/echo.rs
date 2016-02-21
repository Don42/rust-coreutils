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

    let words = args.split_first().expect("Error parsing arguments").1;
    for argument in words {
        print!("{} ", argument);
    }
    if !matches.opt_present("n") {
        print!("\n");
    }
}
