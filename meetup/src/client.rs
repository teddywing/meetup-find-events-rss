use reqwest;
use serde_json;

use errors::*;
use event::Event;

const MEETUP_BASE_URL: &'static str = "https://api.meetup.com";

pub struct Client {
    pub token: String,
}

impl Client {
    pub fn new(token: String) -> Client {
        Client { token: token }
    }

    pub fn find_upcoming_events(
        &self,
        latitude: String,
        longitude: String,
        end_date_range: String,
        radius: Option<String>,
        page: Option<String>,
    ) -> Result<Vec<Event>> {
        let mut params = vec![
            ("key", self.token.clone()),
            ("lat", latitude),
            ("lon", longitude),
            ("end_date_range", end_date_range),
            ("order", "time".to_owned()),
        ];

        if let Some(r) = radius {
            params.push(("radius", r))
        }

        if let Some(p) = page {
            params.push(("page", p))
        }

        let client = reqwest::Client::new();
        let response_text = client
            .get(&format!("{}{}", MEETUP_BASE_URL, "/find/upcoming_events"))
            .query(&params)
            .send()?
            .text()?;

        Ok(parse_json(response_text)?)
    }
}


fn parse_json(json: String) -> Result<Vec<Event>> {
    let parsed: serde_json::Value = serde_json::from_str(json.as_ref())?;
    let events: Vec<Event> = serde_json::from_value(parsed["events"].clone())?;

    Ok(events)
}


#[cfg(test)]
mod tests {
    use super::*;
    use event::Venue;

    #[test]
    fn test_parse_json_parses_event_json() {
        let events = parse_json(
            include_str!("../testdata/meetup--find-upcoming_events.json")
                .to_owned()
        ).unwrap();

        assert_eq!(
            events,
            vec![
                Event {
                    name: "Hackspace Project Night - Open to Everyone!".to_owned(),
                    description: Some("<p>Spend some time on your project by bringing it along to our weekly project night, an evening designed around you and what you're working on.</p> <p>This weekly event is about finding time to tinker, being creative, and meeting other members of the community. You'll get the chance to see what others are working on, share your projects, and to get inspiration and feedback from fellow tinkerers.</p> <p>On any given Tuesday night you have people working on electronics, knitting, writing code, wood-working, or just their plans for world domination. Beer is also consumed.</p> <p>We’ll provide: space to work, basic tools, power, a room full of like minded people.</p> <p>You’ll provide: your project, ideas and beer (optional). A laptop would probably be useful too.</p> <p>Sometimes we work on group projects together, which are usually set up as separate meetup events. Keep an eye on our events page to see what we’re working on next.</p> <p>Note: We usually have around 10 or 20 people at these events, many of our members come along but aren't counted as they don't respond on meetup.</p> <p><img src=\"http://photos3.meetupstatic.com/photos/event/b/3/b/2/600_459406002.jpeg\" /></p> ".to_owned()),
                    link: "https://www.meetup.com/Cambridge-Hackspace/events/249620800/".to_owned(),
                    local_date: Some("2018-04-17".to_owned()),
                    local_time: Some("18:30".to_owned()),
                    venue: Some(Venue {
                        name: "Cambridge Hackspace".to_owned(),
                        address_1: "438 Somerville Avenue".to_owned(),
                        city: "Somerville".to_owned(),
                        localized_country_name: "USA".to_owned(),
                    }),
                },
                Event {
                    name: "PyCon Rehearsal Night #1".to_owned(),
                    description: Some("<p>.</p> ".to_owned()),
                    link: "https://www.meetup.com/bostonpython/events/247552529/".to_owned(),
                    local_date: Some("2018-04-25".to_owned()),
                    local_time: Some("19:00".to_owned()),
                    venue: Some(Venue {
                        name: "VMWare".to_owned(),
                        address_1: "2 Ave de Lafayette".to_owned(),
                        city: "Boston".to_owned(),
                        localized_country_name: "USA".to_owned(),
                    }),
                },
            ]
        );
    }
}
