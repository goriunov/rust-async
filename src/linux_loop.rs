use libc;
use std::os::unix::io::AsRawFd;

// linux epoll event loop
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

    pub fn add_event<T: AsRawFd>(&self, event: &T, conf: &mut libc::epoll_event) {
        unsafe {
            libc::epoll_ctl(
                self.event_loop,
                libc::EPOLL_CTL_ADD,
                event.as_raw_fd(),
                conf,
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

    pub fn poll(&mut self) {
        unsafe {
            let call_events = libc::epoll_wait(self.event_loop, self.events.as_mut_ptr(), 32, -1);
            self.events.set_len(call_events as usize)
        };
    }

    pub fn get_events(&self) -> &Vec<libc::epoll_event> {
        &self.events
    }
}
