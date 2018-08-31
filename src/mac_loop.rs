use std::os::unix::io::RawFd;

use sys::kqueue::*;

pub struct EventLoop {
    pub events: Vec<KEvent>,
    change_events: Vec<KEvent>,
    event_loop: i32,
}

impl EventLoop {
    pub fn new(capacity: usize) -> EventLoop {
        // needs better event managing system
        EventLoop {
            events: Vec::with_capacity(capacity),
            change_events: Vec::with_capacity(capacity),
            event_loop: kqueue1(0),
        }
    }

    pub fn add_event(&mut self, ident: RawFd, id: usize) {
        self.change_events.insert(
            id,
            KEvent {
                ident,
                filter: EventFilter::EVFILT_READ,
                flags: EventFlag::EV_ADD,
                fflags: FilterFlag::NOTE_FFCOPY,
                data: id as u64,
            },
        )
        // epoll_ctl(self.event_loop, EPOLL_CTL_ADD, event, &conf);
    }

    // pub fn remove_event(&self, event: RawFd) {
    //     epoll_ctl(self.event_loop, EPOLL_CTL_DEL, event, &conf);
    // }

    // pub fn modify_event(&self, event: RawFd) {
    //     epoll_ctl(self.event_loop, EPOLL_CTL_MOD, event, &conf);
    // }

    pub fn fetch_events(&mut self) {
        let call_events = kevent(
            self.event_loop,
            &mut self.change_events,
            32,
            &mut self.events,
            32,
            -1,
        );

        unsafe { self.events.set_len(call_events as usize) };
    }
}
