// enable sys params temporary for tests
pub mod sys;

#[cfg(target_os = "linux")]
mod linux_loop;

#[cfg(target_os = "windows")]
mod windows_loop;

pub mod event_loop {
    #[cfg(target_os = "linux")]
    pub use linux_loop::*;

    #[cfg(target_os = "windows")]
    pub use windows_loop::*;
}
