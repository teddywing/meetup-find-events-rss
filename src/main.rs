#[macro_use]
extern crate error_chain;
extern crate getopts;

extern crate meetup_find_events_rss;

use getopts::Options;

use std::env;
use std::process::exit;

use meetup_find_events_rss::build_rss;

mod errors {
    error_chain! {}
}

use errors::*;

fn main() {
    if let Err(ref e) = run() {
        use error_chain::ChainedError;

        eprintln!("{}", e.display_chain());
        exit(1);
    }
}

fn print_usage(opts: Options) {
    let brief = "usage: meetup-find-events-rss";
    print!("{}", opts.usage(&brief));
}

fn run() -> Result<()> {
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

        return Ok(());
    }

    let meetup_token = opt_matches.opt_str("meetup-api-token").unwrap();

    build_rss::write_feed(
        meetup_token,
        "48".to_owned(),
        "2".to_owned(),
        "2018-04-17T00:00:00".to_owned(),
        None,
        None,
    ).chain_err(|| "could not write RSS feed.")?;

    Ok(())
}
