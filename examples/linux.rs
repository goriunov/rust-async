extern crate asyncio;

use asyncio::event_loop::*;
use asyncio::libc;

use std::io::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;
    let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

    let mut new_even = libc::epoll_event {
        events: libc::EPOLLIN as u32 | libc::EPOLLOUT as u32 | libc::EPOLLET as u32,
        u64: count,
    };

    let mut event_loop = EventLoop::new(100);

    event_loop.add_event(&listener, &mut new_even);

    loop {
        let events = event_loop.poll();
        // println!("New connection");
        for event in events {
            let token = event.get_token_value();
            if token == 0 {
                let socket = listener.accept().unwrap().0;
                socket.set_nonblocking(true).unwrap();
                socket.set_nodelay(true).unwrap();
                count += 1;

                let mut new_even = libc::epoll_event {
                    events: libc::EPOLLIN as u32 | libc::EPOLLET as u32 | libc::EPOLLHUP as u32,
                    u64: count,
                };

                event_loop.add_event(&socket, &mut new_even);
                existing_events.insert((count - 1) as usize, socket);
            } else {
                let mut socket = existing_events.get_mut(token - 1).unwrap();
                let mut buf = [0; 1938];

                // println!("Got buff");
                match socket.read(&mut buf) {
                    Ok(0) => {
                        continue;
                    }
                    Ok(_n) => {
                        socket.write(&buf).expect("Could not write");
                        // write back
                    }
                    Err(_e) => continue,
                };
            }
        }
    }
}
