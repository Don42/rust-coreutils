#![feature(fs_time)]
extern crate rustc_serialize;
extern crate docopt;
extern crate time;

use docopt::Docopt;
use std::fs::OpenOptions;
use std::fs::set_file_times;

static USAGE: &'static str = "
Usage: touch <file>...
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());
    for name in args.arg_file {
        match OpenOptions::new()
                                     .read(true)
                                     .create(true)
                                     .open(&name) {
            Err(e) => {
                println!("Couldn't open {}: {}", &name, e);
                return;
            },
            Ok(_) => (),
        };
        let now = (time::get_time().sec * 1000) as u64;
        std::fs::set_file_times(std::path::Path::new(&name), now, now)
         .unwrap_or_else(|e| println!("Couldn't write {}: {}", &name, e));
    }
}
