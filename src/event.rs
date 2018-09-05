use std;

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
    #[inline]
    pub fn read() -> Interest {
        Interest(0b00001)
    }

    #[inline]
    pub fn write() -> Interest {
        Interest(0b00010)
    }

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
    pub fn get_token(&self) -> usize {
        self.token
    }

    #[inline]
    pub fn is_readable(&self) -> bool {
        self.filters.contains(Interest::read())
    }

    #[inline]
    pub fn is_writable(&self) -> bool {
        self.filters.contains(Interest::write())
    }
}
