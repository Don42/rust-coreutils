extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use std::fs::OpenOptions;
use std::io::Write;

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
        let mut f = match OpenOptions::new()
                                     .read(true)
                                     .create(true)
                                     .open(&name) {
            Err(e) => {
                println!("Couldn't open {}: {}", &name, e);
                return;
            },
            Ok(f) => f,
        };
        match f.write_all(b"") {
            Err(e) => {
                println!("Couldn't write {}: {}", &name, e);
                continue;
            },
            Ok(_) => (),
        };
    }
}
