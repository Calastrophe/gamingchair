// We need the offset for the entity list, we should also differentiate between team and enemy.
//
//
// Lastly, we should have an offset for determining the current map.
#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "windows")]
pub use windows::*;