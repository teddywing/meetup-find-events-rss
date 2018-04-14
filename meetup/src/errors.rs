error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
        SerdeJson(::serde_json::Error);
    }
}
