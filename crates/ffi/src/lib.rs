//! LibrePods FFI Bridge - C-compatible interface

use librepods_core::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

static ENGINE: Mutex<Option<Engine>> = Mutex::new(None);

#[no_mangle]
pub extern "C" fn librepods_init() -> i32 {
    if let Ok(mut engine) = ENGINE.lock() {
        *engine = Some(Engine::new());
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn librepods_cleanup() {
    if let Ok(mut engine) = ENGINE.lock() {
        *engine = None;
    }
}
