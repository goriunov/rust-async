extern crate asyncio;

use asyncio::event_loop::EventLoop;

use std::io::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;
    // let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

    let mut event_loop = EventLoop::new(100);

    event_loop.add_event(listener.as_raw_fd(), count);

    loop {
        event_loop.poll();
        println!("{:?}", event_loop.get_events()[0].data);
        // for event in event_loop.get_events() {
        //     if event.data == 0 {
        //         let socket = listener.accept().unwrap().0;
        //         socket.set_nonblocking(true).unwrap();
        //         socket.set_nodelay(true).unwrap();
        //         count += 1;

        //         // let new_even = epoll_event {
        //         //     events: EPOLLIN | EPOLLET | EPOLLHUP,
        //         //     data: count,
        //         // };

        //         event_loop.add_event(socket.as_raw_fd(), count);
        //         existing_events.insert(count as usize, socket);
        //     } else {
        //         let mut socket = existing_events.get_mut(event.data as usize).unwrap();
        //         let mut buf = [0; 1938];

        //         match socket.read(&mut buf) {
        //             Ok(0) => {
        //                 continue;
        //             }
        //                 // write back
        //             }
        //             Err(_e) => continue,
        //         };
        //     }
        // }
    }
}
