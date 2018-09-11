pub extern crate libc;

mod event;

pub use event::Event;
pub use event::Interest;
pub use event::PollOpt;

#[cfg(target_os = "linux")]
mod epoll;
#[cfg(target_os = "windows")]
mod iocp;
#[cfg(target_os = "macos")]
mod kqueue;

#[cfg(target_os = "linux")]
pub use epoll::*;
#[cfg(target_os = "macos")]
pub use kqueue::*;
#[cfg(target_os = "windows")]
pub use windows::*;
