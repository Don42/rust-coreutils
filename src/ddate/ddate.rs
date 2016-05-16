// Crates
extern crate docopt;
extern crate rustc_serialize;
extern crate time;

// Standard library imports

//Crate imports
use docopt::Docopt;

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

Options:
    --help          Dispaly this help message and exit
    --version       Output version information and exit
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_timestamp: Option<i64>,
    flag_help: bool,
    flag_version: bool,
}

#[derive(Debug)]
enum Day {
    Sweetmorn,
    Boomtime,
    Pungenday,
    PricklePrickle,
    SettingOrange,
    StTibsDay,
}

#[derive(Debug)]
enum Season {
    Chaos,
    Discord,
    Confusion,
    Bureaucracy,
    TheAftermath,
    StTibsDay,
}

#[derive(Debug)]
struct DiscordianTime {
    season: Season,
    day: u8,
    year_day: u16,
    year: i32,
    week_day: Day,
    week: Option<u8>,
}

fn week_day(nday: u16) -> Day{
    match nday % 5 {
        0 => Day::Sweetmorn,
        1 => Day::Boomtime,
        2 => Day::Pungenday,
        3 => Day::PricklePrickle,
        4 => Day::SettingOrange,
        _ => panic!("Weekday out of range: {}", nday % 5)
    }
}


fn convert(nday: u16, nyear: i32) -> Option<DiscordianTime> {
    let year = nyear + 1166;
    let year_day = nday + 1;  // Switch to one-based

    if !is_leap_year(nyear) {
        let season = match nday {
            0 ... 72 => Season::Chaos,
            73 ... 145 => Season::Discord,
            146 ... 218 => Season::Confusion,
            219 ... 291 => Season::Bureaucracy,
            292 ... 364 => Season::TheAftermath,
            _ => panic!("Day out of range: {}", nday)
        };

        let week_day = week_day(nday);
        let day = ((nday % 73) + 1) as u8;
        let week = Some((nday / 5) as u8);
        return Some(DiscordianTime {season: season, day: day,
                             year_day: year_day, year: year,
                             week: week, week_day: week_day})
    } else {
        let season = match nday {
            59 => Season::StTibsDay,
            0 ... 73 => Season::Chaos,
            74 ... 146 => Season::Discord,
            147 ... 219 => Season::Confusion,
            220 ... 292 => Season::Bureaucracy,
            293 ... 365 => Season::TheAftermath,
            _ => panic!("Day out of range: {}", nday)
        };

        let week_day = match nday {
                0 ... 58 => week_day(nday),
                59 => Day::StTibsDay,
                60 ... 365 => week_day(nday - 1),
                _ => panic!("Day out of range: {}", nday)
        };

        let day = match nday {
                0 ... 58 => nday,
                59 => 1,
                60 ... 365 => (nday - 1) % 73 + 1,
                _ => panic!("Day out of range: {}", nday)
        } as u8;

        let week = match nday {
                0 ... 58 => Some((nday / 5) as u8),
                59 => None,
                60 ... 365 => Some(((nday - 1) / 5) as u8),
                _ => panic!("Day out of range: {}", nday)
        };

        return Some(DiscordianTime {season: season, day: day,
                             year_day: year_day, year: year,
                             week: week, week_day: week_day})

    }

}


fn is_leap_year(year_ce: i32) -> bool{
    let has_factor = |n| year_ce % n == 0;
    return has_factor(4) && !has_factor(100) || has_factor(400)
}


fn main() {
    let args: Args = Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());
    if args.flag_version {
        println!("{}", VERSION);
        return;
        }
    let greg_date = match args.arg_timestamp {
        Some(t) => time::at(time::Timespec {sec: t, nsec: 0}),
        None => time::now(),
    };
    println!("{:?}, ", greg_date);
    let date = convert(greg_date.tm_yday as u16, greg_date.tm_year + 1900).unwrap();
    println!("{:?}, ", date);
}
