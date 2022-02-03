#[allow(unused_unsafe)]
use std::sync::Once;
use winapi::{
    ctypes::c_void,
    shared::{
        minwindef::BOOL,
        ntdef::HRESULT,
        windef::{
            DPI_AWARENESS_CONTEXT, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE,
            DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2,
        },
    },
    um::{
        libloaderapi::{GetProcAddress, LoadLibraryA},
        shellscalingapi::{PROCESS_DPI_AWARENESS, PROCESS_PER_MONITOR_DPI_AWARE},
        winnt::LPCSTR,
    },
};

/// Copied from winit::platform_impl::util
// Helper function to dynamically load function pointer.
// `library` and `function` must be zero-terminated.
fn get_function_impl(library: &str, function: &str) -> Option<*const c_void> {
    assert_eq!(library.chars().last(), Some('\0'));
    assert_eq!(function.chars().last(), Some('\0'));

    // Library names we will use are ASCII so we can use the A version to avoid string conversion.
    let module = unsafe { LoadLibraryA(library.as_ptr() as LPCSTR) };
    if module.is_null() {
        return None;
    }

    let function_ptr = unsafe { GetProcAddress(module, function.as_ptr() as LPCSTR) };
    if function_ptr.is_null() {
        return None;
    }

    Some(function_ptr as _)
}

macro_rules! get_function {
    ($lib:expr, $func:ident) => {
        crate::platform::dpi::get_function_impl(
            concat!($lib, '\0'),
            concat!(stringify!($func), '\0'),
        )
        .map(|f| unsafe { std::mem::transmute::<*const _, $func>(f) })
    };
}

type SetProcessDPIAware = unsafe extern "system" fn() -> BOOL;
type SetProcessDpiAwareness = unsafe extern "system" fn(value: PROCESS_DPI_AWARENESS) -> HRESULT;
type SetProcessDpiAwarenessContext = unsafe extern "system" fn(value: DPI_AWARENESS_CONTEXT) -> BOOL;

pub(crate) fn set_dpi_aware() {
    static ENABLE_DPI_AWARENESS: Once = Once::new();
    ENABLE_DPI_AWARENESS.call_once(|| {
        unsafe {
            if let Some(handle) = get_function!("user32.dll", SetProcessDpiAwarenessContext) {
                // We are on Windows 10 Anniversary Update (1607) or later.
                if handle(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2) == 0 {
                    // V2 only works with Windows 10 Creators Update (1703). Try using the older
                    // V1 if we can't set V2.
                    handle(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);
                }
            } else if let Some(handle) = get_function!("shcore.dll", SetProcessDpiAwareness) {
                // We are on Windows 8.1 or later.
                handle(PROCESS_PER_MONITOR_DPI_AWARE);
            } else if let Some(handle) = get_function!("user32.dll", SetProcessDPIAware) {
                // We are on Vista or later.
                handle();
            }
        }
    });
}
