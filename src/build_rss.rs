use errors::*;

use meetup;
use meetup_rss;

/// Requests the Meetup API, generates an RSS feed from the resulting events,
/// and writes the XML to standard output.
pub fn write_feed(
    token: String,
    latitude: String,
    longitude: String,
    end_date: String,
    radius: Option<String>,
    page: Option<String>,
) -> Result<()> {
    let client = meetup::Client::new(token);
    let events = client.find_upcoming_events(
        latitude,
        longitude,
        format!("{}T23:59:59", end_date),
        radius,
        page
    )?;
    let channel = meetup_rss::generate(&events)?;
    meetup_rss::write(channel)?;

    Ok(())
}
