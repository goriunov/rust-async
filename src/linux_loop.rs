use std::os::unix::io::RawFd;

// use sys::epoll::*;

// move all configurations to ffi
pub const EPOLL_CLOEXEC: u32 = 0x80000;

pub const EPOLL_CTL_ADD: u32 = 1;
pub const EPOLL_CTL_DEL: u32 = 2;
pub const EPOLL_CTL_MOD: u32 = 3;

pub const EPOLLIN: u32 = 0x01;
pub const EPOLLET: u32 = 0x80000000;
pub const EPOLLPRI: u32 = 0x02;
pub const EPOLLOUT: u32 = 0x04;
pub const EPOLLERR: u32 = 0x08;
pub const EPOLLHUP: u32 = 0x10;
pub const EPOLLONESHOT: u32 = 0x40000000;

#[cfg(target_arch = "x86_64")]
#[repr(C, packed)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

// external
mod ffi {
    // use epoll event
    use super::epoll_event;

    extern "C" {
        pub fn epoll_create1(flags: u32) -> i32;
        pub fn epoll_ctl(epfd: i32, op: u32, fd: i32, event: *const epoll_event) -> i32;
        pub fn epoll_wait(epfd: i32, events: *mut epoll_event, maxevents: i32, timeout: i32)
            -> i32;
    }
}

// linux epoll event loop
pub struct EventLoop {
    pub events: Vec<epoll_event>,
    event_loop: i32,
}

impl EventLoop {
    pub fn new(capacity: usize) -> EventLoop {
        // needs better event managing system
        EventLoop {
            events: Vec::with_capacity(capacity),
            event_loop: unsafe { ffi::epoll_create1(0) },
        }
    }

    pub fn add_event(&self, event: RawFd, conf: epoll_event) {
        unsafe { ffi::epoll_ctl(self.event_loop, EPOLL_CTL_ADD, event, &conf) };
    }

    pub fn remove_event(&self, event: RawFd, conf: epoll_event) {
        unsafe { ffi::epoll_ctl(self.event_loop, EPOLL_CTL_DEL, event, &conf) };
    }

    pub fn modify_event(&self, event: RawFd, conf: epoll_event) {
        unsafe { ffi::epoll_ctl(self.event_loop, EPOLL_CTL_MOD, event, &conf) };
    }

    pub fn fetch_events(&mut self) {
        unsafe {
            let call_events = ffi::epoll_wait(self.event_loop, self.events.as_mut_ptr(), 32, -1);
            self.events.set_len(call_events as usize)
        };
    }
}
