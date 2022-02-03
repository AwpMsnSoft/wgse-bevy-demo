pub(crate) use self::platform::*;

#[cfg(target_os = "windows")]
#[path = "windows/mod.rs"]
mod platform;

#[cfg(not(target_os = "windows"))]
compile_error!("Current OS not supported yet.");
