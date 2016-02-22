extern crate rustc_serialize;
extern crate docopt;
extern crate time;

use docopt::Docopt;
use std::fs::OpenOptions;
//use std::fs::set_file_times;
use std::os::unix::fs::MetadataExt;
use std::error::Error;
use std::io::ErrorKind;


static VERSION: &'static str = "touch (RUST implementation of GNU coreutils) 0.1
Copyright (C) 2015 Marco Kaulea
License GPLv2: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Marco 'don' Kaulea.
";


static USAGE: &'static str = "
Usage:
    touch [options] <file>...
    touch --help
    touch --version

Options:
    -a                  Change access time only
    -c, --no-create     Do not create any files
    -f                  (ignored)
    -m                  Change modification time only
    --time=<word>       change the specified time:
                            <word> is access, atime, or use: equivalent to -a
                            <word> is modify or mtime: equivalent to -m
    --help              Display this help message and exit
    --version           Output version information and exit
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Vec<String>,
    flag_no_create: bool,
    flag_f: bool,
    flag_a : bool,
    flag_m: bool,
    flag_help: bool,
    flag_time: String,
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

    let mut atime_flag: bool = args.flag_a;
    let mut mtime_flag: bool = args.flag_m;

    if args.flag_time == "" {
        ();
    } else if args.flag_time == "access" {
        atime_flag = true;
    }else if args.flag_time == "atime" {
        atime_flag = true;
    }else if args.flag_time == "use" {
        atime_flag = true;
    }else if args.flag_time == "modify" {
        mtime_flag = true;
    }else if args.flag_time == "mtime" {
        mtime_flag = true;
    }else {
        println!("Error, {} not recognized", args.flag_time);
    }

    for name in args.arg_file {
        touch_file(name, args.flag_no_create, atime_flag, mtime_flag);
    }
}


fn touch_file(name: String, no_create: bool, flag_a: bool, flag_m: bool) {
    if !no_create {
        match OpenOptions::new()
                                 .read(true)
                                 .create(true)
                                 .open(&name) {
            Err(e) => {
                print_io_error(e.kind(), name, no_create);
                return;
            },
            Ok(_) => (),
        };
    }

    let (atime, mtime) = match decide_times(&name, flag_a, flag_m) {
        Err(e) => {
            print_io_error(e.kind(), name, no_create);
            return;
        },
        Ok((a, m)) => (a, m),
    };

    //match std::fs::set_file_times(std::path::Path::new(&name), atime, mtime) {
    //    Err(e) => print_io_error(e.kind(), name, no_create),
    //    Ok(_) => (),
    //};
}


fn get_metadata(file_name: &String)
    -> Result<(i64, i64), std::io::Error> {
    let f = match OpenOptions::new()
                             .read(true)
                             .open(&file_name) {
        Err(e) => {
            return Err(e);
        },
        Ok(f) => f,
    };
    match f.metadata() {
        Err(e) => {
            return Err(e)
        },
        Ok(m) => {
            return Ok((m.atime() * 1000 + (m.atime_nsec() / 1_000_000),
                       m.mtime() * 1000 + (m.mtime_nsec() / 1_000_000)));
        },
    };
}




fn decide_times(name: &String, flag_a: bool, flag_m: bool)
    -> Result<(u64, u64), std::io::Error> {
    let now = {
        let time_spec = time::get_time();
        time_spec.sec * 1000 + (time_spec.nsec / 1_000_000) as i64
        };
    let file_meta = get_metadata(&name);
    let atime = if flag_a || !flag_m {now}
                else {
        match file_meta {
            Err(e) => return Err(e),
            Ok((a, _)) => a,
       }
    } as u64;
    let mtime = if flag_m || !flag_a {now}
                else {
        match file_meta {
            Err(e) => return Err(e),
            Ok((_, m)) => m,
       }
    } as u64;
    return Ok((atime, mtime));
}


fn print_io_error(kind: ErrorKind, name: String, no_create: bool) {
    match kind {
        ErrorKind::NotFound => {
            if !no_create {
                    println!("IO Error on file {}: {:?}", &name, kind);
            }
        },
        _ => println!("IO Error {}: {:?}", &name, kind),
    }
}
