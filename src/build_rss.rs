use std::error::Error;

use meetup::client;
use meetup_rss;

/// Requests the Meetup API, generates an RSS feed from the resulting events,
/// and writes the XML to standard output.
pub fn write_feed(
    token: String,
    latitude: String,
    longitude: String,
    end_date_range: String,
    radius: Option<String>,
    page: Option<String>,
) -> Result<(), Box<Error>> {
    let client = client::Client::new(token);
    let events = client.find_upcoming_events(
        latitude,
        longitude,
        end_date_range,
        radius,
        page
    )?;
    let channel = meetup_rss::generate(&events)?;
    meetup_rss::write(channel)?;

    Ok(())
}
