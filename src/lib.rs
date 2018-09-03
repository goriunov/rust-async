// enable sys params temporary for tests
pub mod sys;
pub extern crate libc;

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

// use libc::{intptr_t, uintptr_t, EVFILT_READ, EV_ADD, EV_CLEAR};
// use std::net::{SocketAddr, TcpListener, TcpStream};

// // impl std::fmt::Debug for libc::kevent {
// //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
// //         write!(f, "Hi: {}", self.id)
// //     }
// // }

// pub fn run1() {
//     let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
//     listener
//         .set_nonblocking(true)
//         .expect("Cannot set non-blocking");

//     let mut count = 0;

//     let mut kq = unsafe { libc::kqueue() };

//     let mut changes: Vec<libc::kevent> = Vec::with_capacity(100);
//     let mut events: Vec<libc::kevent> = Vec::with_capacity(100);

//     changes.insert(
//         0,
//         libc::kevent {
//             ident: listener.as_raw_fd() as uintptr_t,
//             filter: EVFILT_READ,
//             flags: EV_ADD | EV_CLEAR,
//             fflags: 0,
//             data: 0,
//             udata: 0 as *mut libc::c_void,
//         },
//     );

//     unsafe {
//         libc::kevent(
//             kq,
//             changes.as_ptr() as *const libc::kevent,
//             changes.len() as i32,
//             std::ptr::null_mut(),
//             0,
//             std::ptr::null(),
//         );
//     };

//     loop {
//         unsafe {
//             let count = libc::kevent(
//                 kq,
//                 std::ptr::null(),
//                 0,
//                 events.as_mut_ptr() as *mut libc::kevent,
//                 100,
//                 std::ptr::null(),
//             );

//             events.set_len(count as usize);

//             // if (count > 0) {
//             println!("{:#?}", events[0].udata);
//             // }
//         };
//     }
// }
