error_chain! {
    foreign_links {
        Meetup(::meetup::Error);
        Rss(::rss::Error);
    }
}
