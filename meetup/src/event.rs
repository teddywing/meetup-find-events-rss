#[derive(Debug, Deserialize, PartialEq)]
pub struct Event {
    pub name: String,
    pub description: Option<String>,
    pub link: String,
    pub local_date: Option<String>,
    pub local_time: Option<String>,
    // venue struct
}



// lat
// lon
// order 'time'
// [radius]
// end_date_range
// [page]

