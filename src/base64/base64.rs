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
    base64 [options] [<file>]
    base64 --help
    base64 --version

Options:
    -d --decode         Decode data
    --help              Display this help message and exit
    --version           Output version information and exit
";

#[derive(RustcDecodable, Debug)]
struct Args {
	arg_file: Option<String>,
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

    let file = args.arg_file.unwrap_or(String::from("-"));
    let output = match args.flag_decode {
        true => decode_base64(file),
        false => encode_base64(file),
    };
    io::stdout().write(&output).unwrap();
}

// TODO Exit code on error, formatting strings from errors

fn read_binary_from_stdin() -> io::Result<Vec<u8>> {
    let mut buf = Vec::new();
    match io::stdin().read_to_end(&mut buf) {
        Ok(_) => return Ok(buf),
        Err(why) => return Err(why),
    }
}

fn read_binary_from_file(file_name: &String) -> io::Result<Vec<u8>> {
    let path = Path::new(&file_name);

    let mut file = match File::open(&path) {
        Err(why) => return Err(why),
        Ok(file) => file,
    };
    let mut buf = Vec::new();
    match file.read_to_end(&mut buf) {
        Err(why) => return Err(why),
        Ok(_) => return Ok(buf),
    }
}

fn read_binary(file_name: &String) -> Option<Vec<u8>> {
    let path = Path::new(&file_name);
    let display = path.display();

    if file_name == "-" {
        match read_binary_from_stdin() {
            Ok(buf) => return Some(buf),
            Err(why) => panic!("Error reading from stdin: {}", Error::description(&why)),
        }
    } else {
        match read_binary_from_file(&file_name) {
            Ok(buf) => return Some(buf),
            Err(why) => panic!("Error reading from file {}: {}", display,
                                                                 Error::description(&why)),
        }
    }
}


fn encode_base64(file_name: String) -> Vec<u8> {
	let mut base64_string = read_binary(&file_name).unwrap().to_base64(STANDARD);
	base64_string.push('\n');
	return base64_string.into_bytes();
}

fn decode_base64(file_name: String) -> Vec<u8> {
    let base64_string = read_binary(&file_name).unwrap()
                            .from_base64().unwrap();
    return base64_string
}
