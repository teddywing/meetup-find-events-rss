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

use rss::{Channel, ChannelBuilder, Item};

use std::io;

use errors::*;
use meetup::event::Event;

pub fn generate(events: &Vec<Event>) -> Result<Channel> {
    let items: Vec<Item> = events.into_iter().map(|event| {
        let mut item = Item::default();
        item.set_title(event.name.clone());
        item.set_link(event.link.clone());
        item.set_description(
            format!(
                "<p>{}</p>{}",
                description_header(&event),
                event.description.clone().unwrap_or("".to_owned()),
            )
        );

        item
    }).collect();

    Ok(
        ChannelBuilder::default()
            .title("Meetup Events")
            .description("Upcoming meetups")
            .items(items)
            .build()?
    )
}

/// Writes the channel to standard output.
pub fn write(channel: Channel) -> Result<()> {
    let stdout = io::stdout();
    let handle = stdout.lock();

    channel.write_to(handle)?;

    Ok(())
}


/// Generates a string containing the date and venue of an `Event` for inclusion
/// in the description field of an `rss::Item`.
///
/// Looks like:
/// ``` text
/// When: 2018-04-15 19:00
/// Where: Passage, 99 Passage des Panoramas, Paris, France
/// ```
fn description_header(event: &Event) -> String {
    let when = if event.local_date.is_some() &&
            event.local_time.is_some() {
        format!(
            "When: {} {}",
            event.local_date.clone().unwrap_or("".to_owned()),
            event.local_time.clone().unwrap_or("".to_owned()),
        )
    } else {
        "".to_owned()
    };

    let place = if let Some(venue) = event.venue.clone() {
        format!(
            "Where: {}, {}, {}, {}",
            venue.name,
            venue.address_1,
            venue.city,
            venue.localized_country_name,
        )
    } else {
        "".to_owned()
    };

    join_nonempty(&[when, place], "<br />")
}

/// Joins a slice of `String`s with `separator`, filtering out empty strings.
fn join_nonempty(strings: &[String], separator: &str) -> String {
    strings
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(separator)
}


#[cfg(test)]
mod tests {
    use super::*;
    use meetup::event::Venue;

    #[test]
    fn test_generate_builds_a_channel_of_events() {
        let events = vec![
            Event {
                name: "Summer Sun Celebration".to_owned(),
                description: Some("Description".to_owned()),
                link: "http://example.com".to_owned(),
                local_date: Some("2018-04-13".to_owned()),
                local_time: Some("18:30".to_owned()),
                venue: None,
            }
        ];
        let event = &events[0];

        let channel = generate(&events).unwrap();

        let item = channel.items().first().unwrap();

        assert_eq!(event.name, item.title().unwrap());
        assert_eq!(event.link, item.link().unwrap());
        assert_eq!(
            format!(
                "<p>When: {} {}</p>{}",
                event.local_date.clone().unwrap(),
                event.local_time.clone().unwrap(),
                event.description.clone().unwrap(),
            ),
            item.description().unwrap()
        );
    }

    #[test]
    fn description_header_makes_a_string_of_time_and_venue() {
        let event = Event {
            name: "Fairies Story 3 Release Party".to_owned(),
            description: Some("Empty".to_owned()),
            link: "http://example.com".to_owned(),
            local_date: Some("2018-04-15".to_owned()),
            local_time: Some("19:00".to_owned()),
            venue: Some(Venue {
                name: "Passage".to_owned(),
                address_1: "99 Passage des Panoramas".to_owned(),
                city: "Paris".to_owned(),
                localized_country_name: "France".to_owned(),
            }),
        };

        let header = description_header(&event);

        assert_eq!(
            "When: 2018-04-15 19:00<br />Where: Passage, 99 Passage des Panoramas, Paris, France",
            header
        );
    }

    #[test]
    fn description_header_excludes_when_if_no_date_present() {
        let header = description_header(&Event {
            name: "Fairies Story 3 Release Party".to_owned(),
            description: Some("Empty".to_owned()),
            link: "http://example.com".to_owned(),
            local_date: None,
            local_time: Some("19:00".to_owned()),
            venue: Some(Venue {
                name: "Passage".to_owned(),
                address_1: "99 Passage des Panoramas".to_owned(),
                city: "Paris".to_owned(),
                localized_country_name: "France".to_owned(),
            }),
        });

        assert_eq!(
            "Where: Passage, 99 Passage des Panoramas, Paris, France",
            header
        );
    }

    #[test]
    fn description_header_excludes_when_if_no_time_present() {
        let header = description_header(&Event {
            name: "Fairies Story 3 Release Party".to_owned(),
            description: Some("Empty".to_owned()),
            link: "http://example.com".to_owned(),
            local_date: Some("2018-04-15".to_owned()),
            local_time: None,
            venue: Some(Venue {
                name: "Passage".to_owned(),
                address_1: "99 Passage des Panoramas".to_owned(),
                city: "Paris".to_owned(),
                localized_country_name: "France".to_owned(),
            }),
        });

        assert_eq!(
            "Where: Passage, 99 Passage des Panoramas, Paris, France",
            header
        );
    }

    #[test]
    fn description_header_excludes_where_if_no_venue_present() {
        let header = description_header(&Event {
            name: "Fairies Story 3 Release Party".to_owned(),
            description: Some("Empty".to_owned()),
            link: "http://example.com".to_owned(),
            local_date: Some("2018-04-15".to_owned()),
            local_time: Some("19:00".to_owned()),
            venue: None,
        });

        assert_eq!(
            "When: 2018-04-15 19:00",
            header
        );
    }
}
