mod epoll;

use epoll::*;

use std::io::*;
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};

pub fn callme() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;
    let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

    let new_even = epoll_event {
        events: EPOLLIN | EPOLLOUT | EPOLLET,
        data: count,
    };

    let mut event_loop = EventLoop::new(1000);

    event_loop.add_listener_event(&listener, new_even);

    loop {
        event_loop.get_events();

        for event in &event_loop.events {
            if event.data == 0 {
                let socket = listener.accept().unwrap().0;
                socket.set_nonblocking(true).unwrap();
                count += 1;

                let new_even = epoll_event {
                    events: EPOLLIN | EPOLLET | EPOLLHUP,
                    data: count,
                };

                event_loop.add_socket_event(&socket, new_even);
                existing_events.insert((count - 1) as usize, socket);
            } else {
                let mut socket = existing_events.get_mut((event.data - 1) as usize).unwrap();
                let mut buf = [0; 1938];

                match socket.read(&mut buf) {
                    Ok(0) => {
                        continue;
                    }
                    Ok(n) => {
                        socket.write(&buf).expect("Could not write");
                        // write back
                    }
                    Err(_e) => continue,
                };
            }
        }
        // println!("{:?}", events);

        // for value in (0..call_events) {
        //     println!("{:?}, {:?}", value, events[value as usize]);
        // }
    }
}
