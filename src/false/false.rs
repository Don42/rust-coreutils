use std::env;

static VERSION: &'static str = "false (RUST implementation of GNU coreutils) 0.1
Copyright (C) 2015 Marco Kaulea
License GPLv2: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>.
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.

Written by Marco 'don' Kaulea.
";

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
        match args[1].as_ref() {
            "--help" => println!("{}", HELP_MESSAGE),
            "--version" => println!("{}", VERSION),
            _ => (),
        }
    }
    std::process::exit(1);
}
