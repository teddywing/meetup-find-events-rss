use std::error::Error;

use meetup::event;
use meetup_rss;

/// Requests the Meetup API, generates an RSS feed from the resulting events,
/// and writes the XML to standard output.
pub fn write_feed() -> Result<(), Box<Error>> {
    let events = event::find_upcoming_events()?;
    let channel = meetup_rss::generate(&events)?;
    meetup_rss::write(channel);

    Ok(())
}
