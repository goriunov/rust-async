use std::os::unix::io::RawFd;
use std::ptr;

pub use self::ffi::kevent as KEvent;

#[derive(Debug)]
pub struct timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

mod ffi {
    use super::timespec;
    use super::{EventFilter, EventFlag, FilterFlag};

    #[derive(Debug)]
    #[repr(C)]
    pub struct kevent {
        pub ident: usize,        // 8
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

#[repr(i16)]
#[derive(PartialEq, Debug)]
pub enum EventFilter {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,
    EVFILT_TIMER = -7,
    EVFILT_MACHPORT = -8,
    EVFILT_FS = -9,
    EVFILT_USER = -10,
    // -11: unused
    EVFILT_VM = -12,
    EVFILT_SYSCOUNT = 13,
}

#[derive(Debug)]
pub struct EventFlag {}
impl EventFlag {
    pub const EV_ADD: u16 = 0x0001;
    pub const EV_DELETE: u16 = 0x0002;
    pub const EV_ENABLE: u16 = 0x0004;
    pub const EV_DISABLE: u16 = 0x0008;
    pub const EV_RECEIPT: u16 = 0x0040;
    pub const EV_ONESHOT: u16 = 0x0010;
    pub const EV_CLEAR: u16 = 0x0020;
    pub const EV_DISPATCH: u16 = 0x0080;
    pub const EV_SYSFLAGS: u16 = 0xF000;
    pub const EV_FLAG0: u16 = 0x1000;
    pub const EV_FLAG1: u16 = 0x2000;
    pub const EV_EOF: u16 = 0x8000;
    pub const EV_ERROR: u16 = 0x4000;
}

#[derive(Debug)]
pub struct FilterFlag {}
impl FilterFlag {
    pub const NOTE_TRIGGER: u32 = 0x01000000;
    pub const NOTE_FFNOP: u32 = 0x00000000;
    pub const NOTE_FFAND: u32 = 0x40000000;
    pub const NOTE_FFOR: u32 = 0x80000000;
    pub const NOTE_FFCOPY: u32 = 0xc0000000;
    pub const NOTE_FFCTRLMASK: u32 = 0xc0000000;
    pub const NOTE_FFLAGSMASK: u32 = 0x00ffffff;
    pub const NOTE_LOWAT: u32 = 0x00000001;
    pub const NOTE_DELETE: u32 = 0x00000001;
    pub const NOTE_WRITE: u32 = 0x00000002;
    pub const NOTE_EXTEND: u32 = 0x00000004;
    pub const NOTE_ATTRIB: u32 = 0x00000008;
    pub const NOTE_LINK: u32 = 0x00000010;
    pub const NOTE_RENAME: u32 = 0x00000020;
    pub const NOTE_REVOKE: u32 = 0x00000040;
    pub const NOTE_NONE: u32 = 0x00000080;
    pub const NOTE_EXIT: u32 = 0x80000000;
    pub const NOTE_FORK: u32 = 0x40000000;
    pub const NOTE_EXEC: u32 = 0x20000000;
    pub const NOTE_REAP: u32 = 0x10000000;
    pub const NOTE_SIGNAL: u32 = 0x08000000;
    pub const NOTE_EXITSTATUS: u32 = 0x04000000;
    pub const NOTE_RESOURCEEND: u32 = 0x02000000;
    pub const NOTE_APPACTIVE: u32 = 0x00800000;
    pub const NOTE_APPBACKGROUND: u32 = 0x00400000;
    pub const NOTE_APPNONUI: u32 = 0x00200000;
    pub const NOTE_APPINACTIVE: u32 = 0x00100000;
    pub const NOTE_APPALLSTATES: u32 = 0x00f00000;
    pub const NOTE_PDATAMASK: u32 = 0x000fffff;
    pub const NOTE_PCTRLMASK: u32 = 0xfff00000;
    pub const NOTE_EXIT_REPARENTED: u32 = 0x00080000;
    pub const NOTE_VM_PRESSURE: u32 = 0x80000000;
    pub const NOTE_VM_PRESSURE_TERMINATE: u32 = 0x40000000;
    pub const NOTE_VM_PRESSURE_SUDDEN_TERMINATE: u32 = 0x20000000;
    pub const NOTE_VM_ERROR: u32 = 0x10000000;
    pub const NOTE_SECONDS: u32 = 0x00000001;
    pub const NOTE_USECONDS: u32 = 0x00000002;
    pub const NOTE_NSECONDS: u32 = 0x00000004;
    pub const NOTE_ABSOLUTE: u32 = 0x00000008;
    pub const NOTE_TRACK: u32 = 0x00000001;
    pub const NOTE_TRACKERR: u32 = 0x00000002;
    pub const NOTE_CHILD: u32 = 0x00000004;
}

pub fn kqueue() -> RawFd {
    unsafe { ffi::kqueue() }
}

pub fn kevent(
    kq: RawFd,
    changelist: &[KEvent],
    eventlist: &mut [KEvent],
    timeout_ms: usize,
) -> usize {
    // Convert ms to timespec
    let timeout = timespec {
        tv_sec: (timeout_ms / 1000) as i64,
        tv_nsec: ((timeout_ms % 1000) * 1_000_000) as i64,
    };

    let res = unsafe {
        ffi::kevent(
            kq,
            changelist.as_ptr(),
            changelist.len() as i32,
            eventlist.as_mut_ptr(),
            eventlist.len() as i32,
            ptr::null()
        )
    };

    return res as usize;
}

#[inline]
pub fn ev_set(
    ev: &mut KEvent,
    ident: usize,
    filter: EventFilter,
    flags: u16,  // EventFlag,
    fflags: u32, // FilterFlag,
    udata: usize,
) {
    ev.ident = ident as usize;
    ev.filter = filter;
    ev.flags = flags;
    ev.fflags = fflags;
    ev.data = 0;
    ev.udata = udata;
}
