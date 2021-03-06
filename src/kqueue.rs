use libc;
use std;

use event::Event;
use event::Interest;
use event::PollOpt;

use std::os::unix::io::{AsRawFd, RawFd};
// Need to add more platforms

pub struct Poll {
  events: Vec<libc::kevent>,
  event_loop: RawFd,
}

macro_rules! ev_set {
  ($id:expr, $filter:expr, $flags:expr, $data:expr) => {
    libc::kevent {
      ident: $id as ::libc::uintptr_t,
      filter: $filter as i16,
      flags: $flags,
      fflags: 0,
      data: 0,
      udata: $data as *mut libc::c_void,
    }
  };
}

// this function need some fixes :)
fn parse_to_interests(kind: Interest, event: libc::kevent) -> Interest {
  let mut kind = kind;

  if (event.flags & libc::EV_ERROR) != 0 {
    kind = kind | Interest::ERROR;
  }

  if event.filter == libc::EVFILT_READ {
    kind = kind | Interest::READ;
  }

  if event.filter == libc::EVFILT_WRITE {
    kind = kind | Interest::WRITE;
  }

  if (event.flags & libc::EV_EOF) != 0 {
    kind = kind | Interest::HUP;

    // need to handle error need to check if it is fine to add vent twice
    if event.fflags != 0 {
      kind = kind | Interest::ERROR;
    }
  }

  kind
}

impl Poll {
  pub fn new(capacity: usize) -> Poll {
    Poll {
      events: Vec::with_capacity(capacity),
      event_loop: unsafe { libc::kqueue() },
    }
  }

  pub fn add<T: AsRawFd>(
    &self,
    registrar: &T,
    token: usize,
    interests: Interest,
    poll_op: Interest,
  ) {
    let mut flags = libc::EV_RECEIPT;

    if poll_op.contains(PollOpt::EDGE) {
      flags |= libc::EV_CLEAR
    }

    if poll_op.contains(PollOpt::ONESHOT) {
      flags |= libc::EV_ONESHOT
    }

    let r = match interests.contains(Interest::READ) {
      true => libc::EV_ADD,
      false => libc::EV_DELETE,
    };

    let w = match interests.contains(Interest::WRITE) {
      true => libc::EV_ADD,
      false => libc::EV_DELETE,
    };

    let changes = [
      ev_set!(registrar.as_raw_fd(), libc::EVFILT_READ, flags | r, token),
      ev_set!(registrar.as_raw_fd(), libc::EVFILT_WRITE, flags | w, token),
    ];

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

  pub fn remove<T: AsRawFd>(&self, registrar: &T) {
    // EV_DELETE
    let changes = [
      ev_set!(
        registrar.as_raw_fd(),
        libc::EVFILT_READ,
        libc::EV_DELETE | libc::EV_RECEIPT,
        0
      ),
      ev_set!(
        registrar.as_raw_fd(),
        libc::EVFILT_WRITE,
        libc::EV_DELETE | libc::EV_RECEIPT,
        0
      ),
    ];

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

  pub fn modify<T: AsRawFd>(
    &self,
    registrar: &T,
    token: usize,
    interests: Interest,
    poll_op: Interest,
  ) {
    self.add(registrar, token, interests, poll_op)
  }

  // need to fix
  pub fn poll(&mut self, events_vec: &mut Vec<Event>) {
    unsafe {
      let call_events = libc::kevent(
        self.event_loop,
        std::ptr::null(),
        0,
        self.events.as_mut_ptr() as *mut libc::kevent,
        self.events.capacity() as i32,
        std::ptr::null(),
      ) as usize;

      events_vec.clear();
      self.events.set_len(call_events);

      // need to coalize events in to one
      for i in 0..call_events {
        events_vec.insert(
          i,
          Event::new(
            self.events[i].udata as usize,
            parse_to_interests(Interest(0), self.events[i]),
          ),
        )
      }
    };
  }
}
