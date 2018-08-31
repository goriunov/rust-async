use std::os::unix::io::RawFd;

use sys::epoll::*;

pub struct EventLoop {
    pub events: Vec<epoll_event>,
    event_loop: i32,
}

impl EventLoop {
    pub fn new(capacity: usize) -> EventLoop {
        // needs better event managing system
        EventLoop {
            events: Vec::with_capacity(capacity),
            event_loop: epoll_create1(0),
        }
    }

    pub fn add_event(&self, event: RawFd, conf: epoll_event) {
        epoll_ctl(self.event_loop, EPOLL_CTL_ADD, event, &conf);
    }

    pub fn remove_event(&self, event: RawFd, conf: epoll_event) {
        epoll_ctl(self.event_loop, EPOLL_CTL_DEL, event, &conf);
    }

    pub fn modify_event(&self, event: RawFd, conf: epoll_event) {
        epoll_ctl(self.event_loop, EPOLL_CTL_MOD, event, &conf);
    }

    pub fn fetch_events(&mut self) {
        let call_events = epoll_wait(self.event_loop, &mut self.events, 32, -1);
        unsafe { self.events.set_len(call_events as usize) };
    }
}
