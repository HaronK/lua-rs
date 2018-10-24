pub mod common;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

pub mod prelude {
    pub use super::common::*;

    #[cfg(target_os = "linux")]
    pub use super::linux::*;
    #[cfg(target_os = "windows")]
    pub use super::windows::*;
}
