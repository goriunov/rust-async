use event::Event;
use libc;
use std;
use std::os::unix::io::{AsRawFd, RawFd};

pub struct EventLoop {
    events: Vec<libc::kevent>,
    event_loop: RawFd,
}

fn ev_set() {
    // need to add functionality
}

impl EventLoop {
    pub fn new(capacity: usize) -> EventLoop {
        EventLoop {
            events: Vec::with_capacity(capacity),
            event_loop: unsafe { libc::kqueue() },
        }
    }

    pub fn add_event<T: AsRawFd>(&self, event: &T, id: usize) {
        let changes = [libc::kevent {
            ident: event.as_raw_fd() as libc::uintptr_t,
            filter: libc::EVFILT_READ,
            flags: libc::EV_ADD | libc::EV_CLEAR,
            fflags: 0,
            data: 0,
            udata: id as *mut libc::c_void,
        }];

        unsafe {
            libc::kevent(
                self.event_loop,
                changes.as_ptr() as *const libc::kevent,
                changes.len() as i32,
                std::ptr::null_mut(),
                0,
                std::ptr::null(),
            );
        }
    }

    // need to fix
    pub fn poll(&mut self) {
        unsafe {
            let call_events = libc::kevent(
                self.event_loop,
                std::ptr::null(),
                0,
                self.events.as_mut_ptr() as *mut libc::kevent,
                self.events.capacity() as i32,
                std::ptr::null(),
            );

            self.events.set_len(call_events as usize);

            // let mut ready_events = Vec::with_capacity(call_events);
            // for (i, event) in self.events {
            //     ready_events.insert(i, Event::new(event.udata))
            // }

            // ready_events
        };
    }

    pub fn get_events(&self) -> &Vec<libc::kevent> {
        &self.events
    }
}
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
