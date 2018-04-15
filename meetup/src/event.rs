#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub link: String,
    pub local_date: Option<String>,
    pub local_time: Option<String>,
    pub venue: Option<Venue>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Venue {
    pub name: String,
    pub address_1: String,
    pub city: String,
    pub localized_country_name: String,
}
