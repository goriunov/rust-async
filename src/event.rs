use std::os::unix::io::{AsRawFd, RawFd};

pub struct Token(pub usize);

pub enum Filter {
    Read,
    Write,
}

pub struct Event {
    token: Token,
    filters: Filter,
}

impl Event {
    pub fn new(token: Token, filters: Filter) -> Event {
        Event { token, filters }
    }

    pub fn get_token_value(&self) -> usize {
        self.token.0
    }

    // pub fn get_filters(&self) -> Filter {
    //     self.filters
    // }
}
