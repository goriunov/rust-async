use libc;
use std::os::unix::io::AsRawFd;

use event::Event;
use event::Interest;

pub struct EventLoop {
    pub events: Vec<libc::epoll_event>,
    event_loop: i32,
}

#[inline]
fn parse_from_interests(filters: Interest) -> u32 {
    let mut kind = libc::EPOLLET;

    if filters.contains(Interest::read()) {
        kind |= libc::EPOLLIN;
    }

    if filters.contains(Interest::write()) {
        kind |= libc::EPOLLOUT;
    }

    kind as u32
}

fn parse_to_interests(epoll: u32) -> Interest {
    let mut kind = 0;
    // need to implement bak transform
    // if epoll.contains(libc::EPOLLIN) {
    //     kind |= 0b00001
    // }

    Interest(kind)
}

impl EventLoop {
    #[inline]
    pub fn new(capacity: usize) -> EventLoop {
        EventLoop {
            events: Vec::with_capacity(capacity),
            event_loop: unsafe { libc::epoll_create1(0) },
        }
    }

    #[inline]
    pub fn add_event<T: AsRawFd>(&self, registrar: &T, token: usize, interests: Interest) {
        // need to fiz get filters
        let mut epoll_event = libc::epoll_event {
            events: parse_from_interests(interests), // need to create anothe function to pars events
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
    pub fn remove_event<T: AsRawFd>(&self, event: &T, conf: &mut libc::epoll_event) {
        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_DEL,
                event.as_raw_fd(),
                conf,
            )
        };
    }

    #[inline]
    pub fn modify_event<T: AsRawFd>(&self, event: &T, conf: &mut libc::epoll_event) {
        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_MOD,
                event.as_raw_fd(),
                conf,
            )
        };
    }

    #[inline]
    pub fn poll(&mut self) -> Vec<Event> {
        unsafe {
            let call_events = libc::epoll_wait(
                self.event_loop,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                -1,
            ) as usize;

            self.events.set_len(call_events);

            // create return event properly
            let mut ready_events = Vec::with_capacity(call_events);

            for i in 0..call_events {
                ready_events.insert(i, Event::new(self.events[i].u64 as usize, Interest::read()))
            }

            ready_events
        }
    }
}
