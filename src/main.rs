extern crate getopts;

extern crate meetup_find_events_rss;

use getopts::Options;

use std::env;
use std::process::exit;

use meetup_find_events_rss::build_rss;

fn print_usage(opts: Options) {
    let brief = "usage: meetup-find-events-rss";
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.reqopt(
        "",
        "meetup-api-token",
        "Meetup.com API token (required)",
        "TOKEN"
    );
    opts.optflag("h", "help", "print this help menu");

    let opt_matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("meetup-find-events-rss: error: {}", f.to_string());
            exit(1);
        },
    };

    if opt_matches.opt_present("h") {
        print_usage(opts);
        return;
    }

    let meetup_token = opt_matches.opt_str("meetup-api-token").unwrap();

    build_rss::write_feed().unwrap();
}
