// enable sys params temporary for tests
pub mod sys;

#[cfg(target_os = "linux")]
mod linux_loop;

// #[cfg(target_os = "windows")]
// mod windows_loop;

#[cfg(target_os = "macos")]
mod mac_loop;

pub mod event_loop {
    #[cfg(target_os = "linux")]
    pub use linux_loop::*;

    // #[cfg(target_os = "windows")]
    // pub use windows_loop::*;

    #[cfg(target_os = "macos")]
    pub use mac_loop::*;
}

pub extern crate libc;

use libc::{intptr_t, uintptr_t, EVFILT_READ, EV_ADD, EV_CLEAR};
use std::net::{SocketAddr, TcpListener, TcpStream};

// impl std::fmt::Debug for libc::kevent {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Hi: {}", self.id)
//     }
// }

pub fn run1() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;

    let mut kq = unsafe { libc::kqueue() };

    let mut changes: Vec<libc::kevent> = Vec::with_capacity(100);
    let mut events: Vec<libc::kevent> = Vec::with_capacity(100);

    changes.insert(
        0,
        libc::kevent {
            ident: listener.as_raw_fd() as uintptr_t,
            filter: EVFILT_READ,
            flags: EV_ADD | EV_CLEAR,
            fflags: 0,
            data: 0,
            udata: 0 as *mut libc::c_void,
        },
    );

    unsafe {
        libc::kevent(
            kq,
            changes.as_ptr() as *const libc::kevent,
            changes.len() as i32,
            std::ptr::null_mut(),
            0,
            std::ptr::null(),
        );
    };

    loop {
        unsafe {
            let count = libc::kevent(
                kq,
                std::ptr::null(),
                0,
                events.as_mut_ptr() as *mut libc::kevent,
                100,
                std::ptr::null(),
            );

            events.set_len(count as usize);

            // if (count > 0) {
            println!("{:#?}", events[0].udata);
            // }
        };
    }
}

/// test example for the queue
///
/// proves that eveything is working
pub extern crate nix;

use nix::sys::event::{
    ev_set, kevent, kevent_ts, kqueue, EventFilter, EventFlag, FilterFlag, KEvent,
};
// use amy::Poller;
// use amy::*;

// use libc::{intptr_t, uintptr_t};
use std::io::*;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;
    let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

    let mut eventlist: Vec<KEvent> = Vec::with_capacity(100);

    let mut kq = kqueue().unwrap();
    let mut items: Vec<KEvent> = Vec::with_capacity(100);

    items.insert(
        0,
        KEvent::new(
            listener.as_raw_fd() as uintptr_t,
            EventFilter::EVFILT_READ,
            EventFlag::EV_ADD | EventFlag::EV_CLEAR,
            FilterFlag::empty(),
            0,
            0,
        ),
    );

    // ev_set(
    //     &mut items[0],
    //     listener.as_raw_fd() as usize,
    //     EventFilter::EVFILT_READ,
    //     EventFlag::EV_ADD | EventFlag::EV_CLEAR,
    //     FilterFlag::empty(),
    //     0,
    // );

    kevent_ts(kq, &items, &mut [], None).unwrap();

    loop {
        unsafe {
            let dst = std::slice::from_raw_parts_mut(eventlist.as_mut_ptr(), eventlist.capacity());
            let count = kevent_ts(kq, &[], dst, None).unwrap();

            eventlist.set_len(count);
            println!("Executing",);

            if (count > 0) {
                println!("{:?}", eventlist.len());
            }
        }
    }
    // let new_kqueue = match Poller::new() {
    //     Ok(mut data) => {
    //         let reg: Registrar = data.get_registrar();
    //         println!("{:?}", reg.register(&listener, Event::Read));

    //         loop {
    //             // unsafe {
    //             let wait = data.wait(100000).unwrap();

    //             if wait.len() > 0 {
    //                 println!("{:?}", wait);
    //             }
    //             // }
    //             // println!("{:?}", wait);
    //         }
    //     }
    //     Err(_err) => {}
    // };
}
