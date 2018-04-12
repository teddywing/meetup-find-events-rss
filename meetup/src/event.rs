#[derive(Debug, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: String,
    pub link: String,
    pub local_date: String,
    pub local_time: String,
    // venue struct
}


// pub fn find_upcoming_events() -> Vec<Event> {
// }


fn parse_json(json: String) -> Vec<Event> {
    Vec::new()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_json_parses_event_json() {
        let events = parse_json(
            include_str!("../testdata/meetup--find-upcoming_events.json")
                .to_owned()
        );

        assert_eq!(
            events,
            vec![
                Event {
                    name: "Hackspace Project Night - Open to Everyone!".to_owned(),
                    description: "<p>Spend some time on your project by bringing it along to our weekly project night, an evening designed around you and what you're working on.</p> <p>This weekly event is about finding time to tinker, being creative, and meeting other members of the community. You'll get the chance to see what others are working on, share your projects, and to get inspiration and feedback from fellow tinkerers.</p> <p>On any given Tuesday night you have people working on electronics, knitting, writing code, wood-working, or just their plans for world domination. Beer is also consumed.</p> <p>We’ll provide: space to work, basic tools, power, a room full of like minded people.</p> <p>You’ll provide: your project, ideas and beer (optional). A laptop would probably be useful too.</p> <p>Sometimes we work on group projects together, which are usually set up as separate meetup events. Keep an eye on our events page to see what we’re working on next.</p> <p>Note: We usually have around 10 or 20 people at these events, many of our members come along but aren't counted as they don't respond on meetup.</p> <p><img src=\"http://photos3.meetupstatic.com/photos/event/b/3/b/2/600_459406002.jpeg\" /></p> ".to_owned(),
                    link: "https://www.meetup.com/Cambridge-Hackspace/events/249620800/".to_owned(),
                    local_date: "2018-04-17".to_owned(),
                    local_time: "18:30".to_owned(),
                },
            ]
        );
    }
}
