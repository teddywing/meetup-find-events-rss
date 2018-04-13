use rss::{Channel, ChannelBuilder, Item};

use meetup::event::Event;

pub fn generate(events: Vec<Event>) -> Result<Channel, String> {
    let items: Vec<Item> = events.into_iter().map(|event| {
        let mut item = Item::default();
        item.set_title(event.name);
        item.set_link(event.link);
        item.set_description(event.description);

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_builds_a_channel() {
        let x = generate(
            vec![
                Event {
                    name: "Summer Sun Celebration".to_owned(),
                    description: "Description".to_owned(),
                    link: "http://example.com".to_owned(),
                    local_date: "2018-04-13".to_owned(),
                    local_time: "18:30".to_owned(),
                }
            ]
        );
        println!("{:?}", x);
    }
}
