extern crate rustc_serialize;
extern crate docopt;

use std::io::{self, Read, Write};
use std::error::Error;
use std::fs::File;
use std::path::Path;

use docopt::Docopt;
use rustc_serialize::base64::{STANDARD, ToBase64, FromBase64};

static VERSION: &'static str = "base64 (RUST implementation of GNU coreutils) 0.1
Copyright (C) 2015 Marco Kaulea
License GPLv2: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Marco 'don' Kaulea.
";


static USAGE: &'static str = "
Usage:
    base64 [options] <file>...
    base64 --help
    base64 --version

Options:
    -d --decode         Decode data
    --help              Display this help message and exit
    --version           Output version information and exit
";

#[derive(RustcDecodable, Debug)]
struct Args {
	arg_file: Vec<String>,
    flag_decode: bool,
	flag_help: bool,
	flag_version: bool,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
							 .and_then(|d| d.decode())
							 .unwrap_or_else(|e| e.exit());
	if args.flag_help {
        println!("{}", USAGE);
        return;
        }
    if args.flag_version {
        println!("{}", VERSION);
        return;
        }
    if !args.flag_decode {
        for name in args.arg_file {
            encode_base64(name);
        }
    } else {
        for name in args.arg_file {
            decode_base64(name);
        }
    }
}

fn encode_base64(file_name: String) {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = Vec::new();
    match file.read_to_end(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {
            let base64_string = s.to_base64(STANDARD);
            io::stdout().write(base64_string.as_bytes()).unwrap();
            // print!("{}", s.to_base64(STANDARD)),
        }
    }
    print!("\n")
}

fn decode_base64(file_name: String) {
    let path = Path::new(&file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   Error::description(&why)),
        Ok(_) => {
            let byte_array = s.from_base64().unwrap();
            io::stdout().write(&byte_array).unwrap();
        }
    }
}
