// need to modify  events (too annoing)
#[derive(Debug)]
#[repr(C)]
pub enum EventFilter {
    EVFILT_READ = -1,
    EVFILT_WRITE = -2,
    EVFILT_AIO = -3,
    EVFILT_VNODE = -4,
    EVFILT_PROC = -5,
    EVFILT_SIGNAL = -6,
    EVFILT_TIMER = -7,
    EVFILT_PROCDESC = -8,
    EVFILT_FS = -9,
    EVFILT_LIO = -10,
    EVFILT_USER = -11,
    EVFILT_SENDFILE = -12,
    EVFILT_SYSCOUNT = 12,
}

#[derive(Debug)]
#[repr(C)]
pub enum EventFlag {
    EV_ADD = 0x0001,
    EV_DELETE = 0x0002,
    EV_ENABLE = 0x0004,
    EV_DISABLE = 0x0008,
    EV_FORCEONESHOT = 0x0100,
    EV_ONESHOT = 0x0010,
    EV_CLEAR = 0x0020,
    EV_RECEIPT = 0x0040,
    EV_DISPATCH = 0x0080,
    EV_SYSFLAGS = 0xF000,
    EV_DROP = 0x1000,
    EV_FLAG1 = 0x2000,
    // EV_FLAG2 = 0x4000,
    EV_EOF = 0x8000,
    EV_ERROR = 0x4000,
}

#[derive(Debug)]
#[repr(C)]
pub enum FilterFlag {
    NOTE_FFNOP = 0x00000000,
    // NOTE_FFAND = 0x40000000,
    // NOTE_FFOR = 0x80000000,
    NOTE_FFCOPY = 0xc0000000,
    // NOTE_FFCTRLMASK = 0xc0000000,
    NOTE_FFLAGSMASK = 0x00ffffff,
    NOTE_TRIGGER = 0x01000000,
    // NOTE_LOWAT = 0x00000001,
    // NOTE_FILE_POLL = 0x00000002,
    NOTE_DELETE = 0x00000001,
    NOTE_WRITE = 0x00000002,
    NOTE_EXTEND = 0x00000004,
    NOTE_ATTRIB = 0x00000008,
    NOTE_LINK = 0x00000010,
    NOTE_RENAME = 0x00000020,
    NOTE_REVOKE = 0x00000040,
    NOTE_OPEN = 0x00000080,
    NOTE_CLOSE = 0x00000100,
    NOTE_CLOSE_WRITE = 0x00000200,
    NOTE_READ = 0x00000400,
    NOTE_EXIT = 0x80000000,
    NOTE_FORK = 0x40000000,
    NOTE_EXEC = 0x20000000,
    NOTE_PCTRLMASK = 0xf0000000,
    NOTE_PDATAMASK = 0x000fffff,
    // NOTE_TRACK = 0x00000001,
    // NOTE_TRACKERR = 0x00000002,
    // NOTE_CHILD = 0x00000004,
    // NOTE_SECONDS = 0x00000001,
    // NOTE_MSECONDS = 0x00000002,
    // NOTE_USECONDS = 0x00000004,
    // NOTE_NSECONDS = 0x00000008,
}

// pub struct k_event {
//     pub events: u32,
//     pub data: u64,
// }

#[derive(Debug)]
#[repr(C)]
pub struct KEvent {
    pub ident: i32,
    pub filter: EventFilter,
    pub flags: EventFlag,
    pub fflags: FilterFlag,
    pub data: u64,
    // pub udata: *mut c_void,
}

mod __glibc {
    use sys::kqueue::*;

    extern "C" {
        pub fn kqueue() -> i32;

        pub fn kevent(
            kq: i32,
            changelist: *mut KEvent,
            nchanges: u32,
            eventlist: *mut KEvent,
            nevents: u32,
            timeout: i32,
        ) -> i32;

        pub fn kqueue1(flags: i32) -> i32;
    }
}

pub fn kqueue() -> i32 {
    unsafe { __glibc::kqueue() }
}

pub fn kqueue1(flags: i32) -> i32 {
    unsafe { __glibc::kqueue1(flags) }
}

pub fn kevent(
    kq: i32,
    changelist: &mut [KEvent],
    nchanges: u32,
    eventlist: &mut [KEvent],
    nevents: u32,
    timeout: i32,
) -> i32 {
    unsafe {
        __glibc::kevent(
            kq,
            changelist.as_mut_ptr(),
            nchanges,
            eventlist.as_mut_ptr(),
            nevents,
            timeout,
        )
    }
}

// pub fn ev_set(items: &mut Vec<kevent>) {}
