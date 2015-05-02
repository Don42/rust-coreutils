#![feature(exit_status)]
use std::env;

    static HELP_MESSAGE: &'static str =
    "Usage:
    /bin/false [ignored command line arguments]
    /bin/false OPTION

    Exit with a status code indicating failure.

        --help     display this help and exit
        --version  output version information and exit";

fn main(){

    let args: Vec<_> = env::args().collect();
    if args.len() == 2 && args[1] == "--version" {
        println!("The first argument is {}", args[1]);
    }else if args.len() == 2 && args[1] == "--help" {
        println!("{}", HELP_MESSAGE)
    }
    std::env::set_exit_status(1);
}
