// extern crate asyncio;

// use asyncio::event_loop::EventLoop;
// use asyncio::sys::kqueue::*;

// use std::io::*;
// use std::net::{SocketAddr, TcpListener, TcpStream};
// use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

// fn main() {
//     let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
//     listener
//         .set_nonblocking(true)
//         .expect("Cannot set non-blocking");

//     let mut count = 0;
//     let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

//     // let new_even = epoll_event {
//     //     events: EPOLLIN | EPOLLOUT | EPOLLET,
//     //     data: count,
//     // };

//     let mut event_loop = EventLoop::new(100);

//     event_loop.add_event(listener.as_raw_fd(), count);

//     loop {
//         event_loop.fetch_events();
//         // println!("{:?}", event_loop.events);
//         // for event in &event_loop.events {
//         //     if event.data == 0 {
//         //         let socket = listener.accept().unwrap().0;
//         //         socket.set_nonblocking(true).unwrap();
//         //         socket.set_nodelay(true).unwrap();
//         //         count += 1;

//         //         // let new_even = epoll_event {
//         //         //     events: EPOLLIN | EPOLLET | EPOLLHUP,
//         //         //     data: count,
//         //         // };

//         //         event_loop.add_event(socket.as_raw_fd(), count);
//         //         existing_events.insert(count as usize, socket);
//         //     } else {
//         //         let mut socket = existing_events.get_mut(event.data as usize).unwrap();
//         //         let mut buf = [0; 1938];

//         //         match socket.read(&mut buf) {
//         //             Ok(0) => {
//         //                 continue;
//         //             }
//         //             Ok(n) => {
//         //                 socket.write(&buf).expect("Could not write");
//         //                 // write back
//         //             }
//         //             Err(_e) => continue,
//         //         };
//         //     }
//         // }
//     }
// }

extern crate nix;

use nix::sys::event::{kevent, kqueue, EventFilter, FilterFlag, KEvent};
use nix::sys::event::{EV_ADD, EV_ENABLE};

use std::io::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

fn event(id: usize) -> KEvent {
    KEvent {
        ident: id,
        filter: EventFilter::EVFILT_READ,
        flags: EV_ADD | EV_ENABLE,
        fflags: FilterFlag::empty(),
        data: 0,
        udata: 0,
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;

    let kq = kqueue().expect("Could not get kqueue");

    let mut changes: [KEvent; 1] = vec![event(listener.as_raw_fd() as usize)];

    kevent(kq, changes.as_slice(), &mut [], 0).unwrap();

    loop {
        match kevent(kq, &[], changes.as_mut_slice(), 0) {
        Ok(v) if v > 0 => {
        println!("---");
        for i in 0..v {
            println!("Event with ID {:?} triggered", changes.get(i).unwrap().ident);
        }
        }
        Err(e) => panic!("{:?}", e), // Panic on Errors
        _ => () // Ignore Ok(0),
    }
    }
}
