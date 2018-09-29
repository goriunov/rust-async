extern crate asyncio;

use asyncio::*;

use std::io::*;
use std::net::{TcpListener, TcpStream};

fn main() {
  let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

  listener
    .set_nonblocking(true)
    .expect("Cannot set non-blocking");

  let mut count = 0;
  let mut existing_events: Vec<TcpStream> = Vec::with_capacity(32);

  let mut poll = Poll::new(100);

  poll.add(&listener, count, Interest::READ, PollOpt::EDGE);

  let mut events = Vec::with_capacity(100);
  loop {
    poll.poll(&mut events);
    // println!("Connection, {:?}, {}", events, events[0].is_readable());

    for event in &events {
      let mut token = event.token();

      if token == 0 {
        let socket = listener.accept().unwrap().0;

        // make socket async
        socket.set_nonblocking(true).unwrap();
        socket.set_nodelay(true).unwrap();

        count += 1;

        poll.add(
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

        // let mut another_token = token.clone();
        // if another_token > 1 {
        //   another_token = 1;
        // }

        // if event.is_readable() {
        let mut socket = existing_events.get_mut(token - 1).unwrap();
        let mut buf = [0; 1938];
        match socket.read(&mut buf) {
          Ok(0) => {
            println!("Socket closed");
            // println!("{:?}", existing_events.);
            poll.remove(socket);
            continue;
          }
          Ok(_n) => {
            // poll.modify(
            //     socket,
            //     token + 1,
            //     Interest::READ | Interest::WRITE,
            //     PollOpt::EDGE,
            // );
            // existing_events.insert(token, socket);
            socket.write(&buf).expect("Could not write");
          }
          Err(_e) => {
            // println!(extern crate asyncio;  // "{:?}", e);
            continue;
          }
        };
      }
    }
  }
}
