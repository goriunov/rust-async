use libc;
use std::os::unix::io::AsRawFd;

use event::Event;
use event::Token;

pub struct EventLoop {
    pub events: Vec<libc::epoll_event>,
    event_loop: i32,
}

impl EventLoop {
    pub fn new(capacity: usize) -> EventLoop {
        EventLoop {
            events: Vec::with_capacity(capacity),
            event_loop: unsafe { libc::epoll_create1(0) },
        }
    }

    pub fn add_event<T: AsRawFd>(&self, item: T, event: Event) {
        // need to fiz get filters
        let mut register_event = libc::epoll_event {
            events: event.get_filters() as u32,
            u64: event.get_token_value() as u64,
        };

        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_ADD,
                item.as_raw_fd(),
                &mut register_event,
            )
        };
    }

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

    pub fn poll(&mut self) -> Vec<Event> {
        unsafe {
            let call_events = libc::epoll_wait(
                self.event_loop,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                -1,
            ) as usize;

            self.events.set_len(call_events);

            // create return event
            let mut ready_events = Vec::with_capacity(call_events);

            for i in 0..call_events {
                ready_events.insert(i, Event::new(Token(self.events[i].u64 as usize)))
            }

            ready_events
        }
    }
}
