#![feature(fs_time)]
extern crate rustc_serialize;
extern crate docopt;
extern crate time;

use docopt::Docopt;
use std::fs::OpenOptions;
use std::fs::set_file_times;

static USAGE: &'static str = "
Usage: touch [options] <file>...

Options:
    -c, --no-create     do not create any files
    -f                  (ignored)
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Vec<String>,
    flag_no_create: bool,
    flag_f: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    let now = (time::get_time().sec * 1000) as u64;
    for name in args.arg_file {
        if !args.flag_no_create {
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
        }
        match std::fs::set_file_times(std::path::Path::new(&name), now, now) {
            Err(e) => {
                if !args.flag_no_create {
                    println!("Couldn't write time for {}: {}", &name, e);
                }
            },
            Ok(_) => (),
        };
    }
}
