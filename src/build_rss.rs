// Copyright © 2018 Teddy Wing
//
// This file is part of Meetup Find Events RSS.
//
// Meetup Find Events RSS is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// Meetup Find Events RSS is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied warranty
// of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Meetup Find Events RSS. If not, see
// <http://www.gnu.org/licenses/>.

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
