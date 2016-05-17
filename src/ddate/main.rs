// Crates
extern crate chrono;
extern crate docopt;
extern crate rustc_serialize;
extern crate time;
extern crate libddate;

// Standard library imports

//Crate imports
use chrono::{NaiveDate, Datelike};
use docopt::Docopt;
use libddate::ddate;

const YEAR_OFFSET: i32 = 1900;

static VERSION: &'static str = "ddate (RUST implementaion of gnucoreutils) 0.1
Copyright (C) 2016 Marco Kaulea
License GPLv2: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Marco 'don' Kaulea.
";

const USAGE: &'static str = "
ddate

USAGE:
    ddate [options] [<timestamp>]
    ddate [options] date <date>

Options:
    --help          Dispaly this help message and exit
    --version       Output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_timestamp: Option<i64>,
    arg_date: Option<String>,
    flag_help: bool,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION);
        return;
        }
    let date = if args.arg_date.is_some() {
        let input_date = NaiveDate::parse_from_str(args.arg_date.unwrap().as_str(),
                                                   "%Y-%m-%d").unwrap();
        ddate::convert(input_date.ordinal0() as u16,
                       input_date.year() as i32).unwrap()
    } else {
        let greg_date = match args.arg_timestamp {
            Some(t) => time::at(time::Timespec {sec: t, nsec: 0}),
            None => time::now(),
        };
        ddate::convert(greg_date.tm_yday as u16,
                                  greg_date.tm_year + YEAR_OFFSET).unwrap()
    };
    println!("{:?}, ", date);
}

