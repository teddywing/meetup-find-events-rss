use rss::{Channel, ChannelBuilder, Item};

use std::io;

use meetup::event::Event;

pub fn generate(events: &Vec<Event>) -> Result<Channel, String> {
    let items: Vec<Item> = events.into_iter().map(|event| {
        let mut item = Item::default();
        item.set_title(event.name.clone());
        item.set_link(event.link.clone());
        item.set_description(event.description.clone());

        item
    }).collect();

    Ok(
        ChannelBuilder::default()
            .title("Test")
            .description("Description")
            .items(items)
            .build()?
    )
}

/// Writes the channel to standard output.
pub fn write(channel: Channel) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    channel.write_to(handle);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_builds_a_channel_of_events() {
        let events = vec![
            Event {
                name: "Summer Sun Celebration".to_owned(),
                description: "Description".to_owned(),
                link: "http://example.com".to_owned(),
                local_date: "2018-04-13".to_owned(),
                local_time: "18:30".to_owned(),
            }
        ];
        let event = &events[0];

        let channel = generate(&events).unwrap();

        let item = channel.items().first().unwrap();

        assert_eq!(event.name, item.title().unwrap());
        assert_eq!(event.link, item.link().unwrap());
        assert_eq!(event.description, item.description().unwrap());
    }
}
