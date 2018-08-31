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
            event_loop: kqueue(),
        }
    }

    pub fn add_event(&mut self, ident: RawFd, id: usize) {
        let mut event = KEvent {
            ident: 0,                         // 8
            filter: EventFilter::EVFILT_READ, // 2
            flags: EventFlag::EV_ADD,         // 2
            fflags: FilterFlag::NOTE_FFCOPY,  // 4
            data: 0,                          // 8
            udata: 0,                         // 8
        };
        ev_set(
            &mut event,
            ident as usize,
            EventFilter::EVFILT_READ,
            EventFlag::EV_ADD,
            0,
            0,
        );

        self.change_events.insert(id, event);
    }

    //     // pub fn remove_event(&self, event: RawFd) {
    //     //     epoll_ctl(self.event_loop, EPOLL_CTL_DEL, event, &conf);
    //     // }

    //     // pub fn modify_event(&self, event: RawFd) {
    //     //     epoll_ctl(self.event_loop, EPOLL_CTL_MOD, event, &conf);
    //     // }

    pub fn fetch_events(&mut self) {
        let call_events = kevent(self.event_loop, &[], &mut self.events, 0);
        unsafe { self.events.set_len(call_events as usize) };
    }
}
