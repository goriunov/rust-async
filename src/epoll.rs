use libc;
use std::os::unix::io::AsRawFd;

use event::Event;
use event::Interest;
use event::PollOpt;

pub struct Poll {
    pub events: Vec<libc::epoll_event>,
    event_loop: i32,
}

#[inline]
fn parse_from_interests(filters: Interest) -> u32 {
    let mut kind = 0;

    if filters.contains(Interest::READ) {
        kind |= libc::EPOLLIN;
    }

    if filters.contains(Interest::WRITE) {
        kind |= libc::EPOLLOUT;
    }

    if filters.contains(Interest::HUP) {
        kind |= libc::EPOLLRDHUP;
    }

    if filters.contains(PollOpt::EDGE) {
        kind |= libc::EPOLLET;
    }

    if filters.contains(PollOpt::ONESHOT) {
        kind |= libc::EPOLLONESHOT;
    }

    if filters.contains(PollOpt::LEVEL) {
        kind &= !libc::EPOLLET;
    }

    kind as u32
}

#[inline]
fn parse_to_interests(epoll: i32) -> Interest {
    let mut kind = Interest(0);

    if (epoll & libc::EPOLLIN) != 0 {
        kind = kind | Interest::READ;
    }

    if (epoll & libc::EPOLLOUT) != 0 {
        kind = kind | Interest::WRITE;
    }

    if (epoll & libc::EPOLLERR) != 0 {
        kind = kind | Interest::ERROR;
    }

    if (epoll & libc::EPOLLRDHUP) != 0 || (epoll & libc::EPOLLHUP) != 0 {
        kind = kind | Interest::HUP;
    }

    kind
}

impl Poll {
    #[inline]
    pub fn new(capacity: usize) -> Poll {
        Poll {
            events: Vec::with_capacity(capacity),
            event_loop: unsafe { libc::epoll_create1(0) },
        }
    }

    #[inline]
    pub fn add<T: AsRawFd>(
        &self,
        registrar: &T,
        token: usize,
        interests: Interest,
        poll_op: Interest,
    ) {
        let mut epoll_event = libc::epoll_event {
            events: parse_from_interests(interests | poll_op),
            u64: token as u64,
        };

        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_ADD,
                registrar.as_raw_fd(),
                &mut epoll_event,
            )
        };
    }

    #[inline]
    pub fn remove<T: AsRawFd>(&self, registrar: &T) {
        let mut epoll_event = libc::epoll_event { events: 0, u64: 0 };

        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_DEL,
                registrar.as_raw_fd(),
                &mut epoll_event,
            )
        };
    }

    #[inline]
    pub fn modify<T: AsRawFd>(
        &self,
        registrar: &T,
        token: usize,
        interests: Interest,
        poll_op: Interest,
    ) {
        let mut epoll_event = libc::epoll_event {
            events: parse_from_interests(interests | poll_op),
            u64: token as u64,
        };

        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_MOD,
                registrar.as_raw_fd(),
                &mut epoll_event,
            )
        };
    }

    #[inline]
    pub fn poll(&mut self, events_vec: &mut Vec<Event>) {
        unsafe {
            let call_events = libc::epoll_wait(
                self.event_loop,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                -1,
            ) as usize;

            events_vec.clear();
            self.events.set_len(call_events);

            for i in 0..call_events {
                events_vec.insert(
                    i,
                    Event::new(
                        self.events[i].u64 as usize,
                        parse_to_interests(self.events[i].events as i32),
                    ),
                )
            }
        }
    }
}
