pub extern crate libc;

mod event;

#[cfg(target_os = "linux")]
mod epoll_loop;

#[cfg(target_os = "windows")]
mod windows_loop;

#[cfg(target_os = "macos")]
mod kqueue_loop;

pub mod event_loop {
    pub use event::Event;
    pub use event::Interest;
    pub use event::PollOpt;

    #[cfg(target_os = "linux")]
    pub use epoll_loop::*;

    #[cfg(target_os = "windows")]
    pub use windows_loop::*;

    #[cfg(target_os = "macos")]
    pub use kqueue_loop::*;
}
