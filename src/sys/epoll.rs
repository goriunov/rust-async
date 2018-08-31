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

#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub data: u64,
}

// external
mod __glibc {
    // use epoll event
    use sys::epoll::epoll_event;

    extern "C" {
        pub fn epoll_create1(flags: u32) -> i32;
        pub fn epoll_ctl(epfd: i32, op: u32, fd: i32, event: *const epoll_event) -> i32;
        pub fn epoll_wait(epfd: i32, events: *mut epoll_event, maxevents: i32, timeout: i32)
            -> i32;
    }
}

pub fn epoll_create1(flags: u32) -> i32 {
    unsafe { __glibc::epoll_create1(flags) }
}

pub fn epoll_ctl(epfd: i32, op: u32, fd: i32, event: &epoll_event) -> i32 {
    unsafe { __glibc::epoll_ctl(epfd, op, fd, event) }
}

pub fn epoll_wait(epfd: i32, events: &mut [epoll_event], maxevents: i32, timeout: i32) -> i32 {
    unsafe { __glibc::epoll_wait(epfd, events.as_mut_ptr(), maxevents, timeout) }
}

// May need some modifications

// pub fn epoll_create() -> i32 {
//     unsafe { __glibc::epoll_create1(0) }
// }

// not sure if need to repreent this data
// #[cfg(target_arch = "x86_64")]
// #[repr(C, packed)]
// pub struct epoll_event {
//     pub events: u32,
//     pub data: u64,
// }
// #[cfg(not(target_arch = "x86_64"))]
