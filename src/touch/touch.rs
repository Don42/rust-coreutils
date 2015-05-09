#![feature(fs_time,metadata_ext)]
extern crate rustc_serialize;
extern crate docopt;
extern crate time;

use docopt::Docopt;
use std::fs::OpenOptions;
use std::fs::set_file_times;
use std::os::unix::fs::MetadataExt;

static USAGE: &'static str = "
Usage: touch [options] <file>...

Options:
    -a                  Change access time only
    -c, --no-create     Do not create any files
    -f                  (ignored)
    -m                  Change modification time only
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_file: Vec<String>,
    flag_no_create: bool,
    flag_f: bool,
    flag_a : bool,
    flag_m: bool,
}

fn get_metadata(file_name: &String)
    -> Result<(i64, i64), std::io::Error> {
    let f = match OpenOptions::new()
                             .read(true)
                             .open(&file_name) {
        Err(e) => {
            println!("Couldn't open {}: {}", &file_name, e);
            return Err(e);
        },
        Ok(f) => f,
    };
    match f.metadata() {
        Err(e) => {
            return Err(e)
        },
        Ok(m) => {
            let meta = m.as_raw();
            return Ok((meta.atime() * 1000 + (meta.atime_nsec() / 1_000_000),
                       meta.mtime() * 1000 + (meta.mtime_nsec() / 1_000_000)));
        },
    };
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    for name in args.arg_file {
        if !args.flag_no_create {
            match OpenOptions::new()
                                     .read(true)
                                     .create(true)
                                     .open(&name) {
                Err(e) => {
                    println!("Couldn't open {}: {}", &name, e);
                    continue;
                },
                Ok(_) => (),
            };
        }


        let now = {
            let time_spec = time::get_time();
            time_spec.sec * 1000 + (time_spec.nsec / 1_000_000) as i64
            };
        let file_meta = get_metadata(&name);
        let atime = if args.flag_a || !args.flag_m {now}
                    else {
            match file_meta {
                Err(e) => {
                    if !args.flag_no_create {
                        println!("Couldn't access metadata for {}: {}", &name, e);
                    }
                    continue;
                },
                Ok((a, _)) => a,
           }
        } as u64;
        let mtime = if args.flag_m || !args.flag_a {now}
                    else {
            match file_meta {
                Err(e) => {
                    if !args.flag_no_create {
                        println!("Couldn't access metadata for {}: {}", &name, e);
                    }
                    continue;
                },
                Ok((_, m)) => m,
           }
        } as u64;

        match std::fs::set_file_times(std::path::Path::new(&name), atime, mtime) {
            Err(e) => {
                if !args.flag_no_create {
                    println!("Couldn't write time for {}: {}", &name, e);
                }
            },
            Ok(_) => (),
        };
    }
}
