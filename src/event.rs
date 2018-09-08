use std;

pub struct PollOpt {}

impl PollOpt {
    pub const EDGE: Interest = Interest(0b0000001);
    pub const LEVEL: Interest = Interest(0b0000010);
    pub const ONESHOT: Interest = Interest(0b0000100);
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Interest(pub usize);

impl std::ops::BitOr for Interest {
    type Output = Interest;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Interest(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for Interest {
    type Output = Interest;

    #[inline]
    fn bitand(self, rhs: Self) -> Self {
        Interest(self.0 & rhs.0)
    }
}

impl Interest {
    pub const HUP: Interest = Interest(0b0001000);
    pub const READ: Interest = Interest(0b0010000);
    pub const WRITE: Interest = Interest(0b0100000);
    pub const ERROR: Interest = Interest(0b1000000);

    #[inline]
    pub fn contains(&self, other: Interest) -> bool {
        (*self & other) == other
    }
}

#[derive(Debug)]
pub struct Event {
    token: usize,
    filters: Interest,
}

impl Event {
    #[inline]
    pub fn new(token: usize, filters: Interest) -> Event {
        Event { token, filters }
    }

    #[inline]
    pub fn token(&self) -> usize {
        self.token
    }

    pub fn is_hup(&self) -> bool {
        self.filters.contains(Interest::HUP)
    }

    #[inline]
    pub fn is_error(&self) -> bool {
        self.filters.contains(Interest::ERROR)
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        self.filters.contains(Interest::READ)
    }

    #[inline]
    pub fn is_writable(&self) -> bool {
        self.filters.contains(Interest::WRITE)
    }
}
