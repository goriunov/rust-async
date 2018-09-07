extern crate asyncio;

use asyncio::event_loop::*;

use std::io::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut count = 0;
    let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

    let mut event_loop = EventLoop::new(100);

    event_loop.add(&listener, count, Interest::READ, PollOpt::EDGE);
    // event_loop.remove(&listener);
    let mut events = Vec::with_capacity(100);
    loop {
        event_loop.poll(&mut events);
        // println!("Connection",);

        for event in &events {
            let token = event.token();

            if token == 0 {
                let socket = listener.accept().unwrap().0;
                socket.set_nonblocking(true).unwrap();
                socket.set_nodelay(true).unwrap();
                count += 1;

                event_loop.add(
                    &socket,
                    count,
                    Interest::READ | Interest::WRITE,
                    PollOpt::EDGE,
                );

                existing_events.insert(count - 1, socket);
            } else {
                if event.is_hup() {
                    println!("Got hup");
                }

                // if event.is_readable() {
                let mut socket = existing_events.get_mut(token - 1).unwrap();
                let mut buf = [0; 1938];
                match socket.read(&mut buf) {
                    Ok(0) => {
                        println!("Socket closed");
                        // println!("{:?}", existing_events.);
                        event_loop.remove(socket);
                        continue;
                    }
                    Ok(_n) => socket.write(&buf).expect("Could not write"),
                    Err(e) => {
                        // println!("{:?}", e);
                        continue;
                    }
                };
                // }
            }
        }
    }
}
