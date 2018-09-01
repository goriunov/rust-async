use std::os::unix::io::RawFd;
use std::ptr;
use sys::kqueue::*;

use self::ffi::kevent as KEvent;

#[derive(Debug)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

mod ffi {
    use super::timespec;
    use super::EventFilter;
    use std::os::unix::io::RawFd;

    #[derive(Debug)]
    #[repr(C)]
    pub struct kevent {
        pub ident: RawFd,        // 8
        pub filter: EventFilter, // 2
        pub flags: u16,          // EventFlag,    // 2
        pub fflags: u32,         // FilterFlag,  // 4
        pub data: isize,         // 8
        pub udata: usize,        // 8
    }

    // impl kevent {
    //     pub fn ev_set(){

    //     }
    // }

    // Bug in rustc, cannot determine that kevent is #[repr(C)]
    #[allow(improper_ctypes)]
    extern "C" {
        pub fn kqueue() -> i32;

        pub fn kevent(
            kq: i32,
            changelist: *const kevent,
            nchanges: i32,
            eventlist: *mut kevent,
            nevents: i32,
            timeout: *const timespec,
        ) -> i32;
    }
}

// actuall event loop
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
            event_loop: unsafe { ffi::kqueue() },
        }
    }

    pub fn add_event(&mut self, ident: RawFd, id: usize) {
        let mut changes = [KEvent {
            ident: ident,                                                           // 8
            filter: EventFilter::EVFILT_READ,                                       // 2
            flags: EventFlag::EV_ADD | EventFlag::EV_CLEAR | EventFlag::EV_RECEIPT, // 2
            fflags: 0,                                                              // 4
            data: 0,                                                                // 8
            udata: id + 1,                                                          // 8
        }];
        // need to create array of events

        // ev_set(
        //     &mut event,
        //     ident,
        //     EventFilter::EVFILT_READ,
        //     EventFlag::EV_ADD,
        //     0,
        //     0,
        // );

        // self.change_events.insert(id, event);

        unsafe {
            println!(
                "{}",
                ffi::kevent(
                    self.event_loop,
                    changes.as_ptr(),
                    changes.len() as i32,
                    changes.as_mut_ptr(),
                    changes.len() as i32,
                    ptr::null(),
                )
            )
        };
    }

    //     // pub fn remove_event(&self, event: RawFd) {
    //     //     epoll_ctl(self.event_loop, EPOLL_CTL_DEL, event, &conf);
    //     // }

    //     // pub fn modify_event(&self, event: RawFd) {
    //     //     epoll_ctl(self.event_loop, EPOLL_CTL_MOD, event, &conf);
    //     // }

    pub fn fetch_events(&mut self) {
        unsafe {
            let cnt = ffi::kevent(
                self.event_loop,
                ptr::null(),
                0,
                self.events.as_mut_ptr(),
                self.events.capacity() as i32,
                ptr::null_mut(),
            );

            println!("{}", cnt);
            // let call_events = ffi::kevent(
            //     self.event_loop,
            //     self.change_events.as_ptr(),
            //     self.change_events.len() as i32,
            //     self.events.as_mut_ptr(),
            //     32,
            //     ptr::null_mut(),
            // );

            // println!("{}", call_events);

            // self.events.set_len(call_events as usize);
        };
    }
}
