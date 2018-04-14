pub struct Client {
    pub token: String,
}

impl Client {
    pub fn new(token: String) -> Client {
        Client { token: token }
    }
}
