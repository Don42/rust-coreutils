#![feature(exit_status)]
#![feature(convert)]
use std::env;

    static HELP_MESSAGE: &'static str =
    "Usage:
    /bin/false [ignored command line arguments]
    /bin/false OPTION

    Exit with a status code indicating failure.

        --help     display this help and exit
        --version  output version information and exit";

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() == 2 {
        match args[1].as_str() {
            "--help" => println!("{}", HELP_MESSAGE),
            "--version" => println!("This is an early version."),
            _ => print!(""),
        }
    }
    std::env::set_exit_status(1);
}
