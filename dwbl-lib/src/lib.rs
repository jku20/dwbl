use std::ffi::c_char;

use cxx::CxxString;

// TODO: This namespace is not hardcoded in yosys itself. This should probably read from the
// makefile.
#[cxx::bridge(namespace = "Yosys")]
mod ffi {
    unsafe extern "C++" {
        include!("kernel/yosys.h");
        fn log_str(msg: &CxxString) -> *const c_char;
    }

    extern "Rust" {
        fn log_from_rust(msg: &CxxString) -> *const c_char;
    }
}

pub fn log_from_rust(msg: &CxxString) -> *const c_char {
    ffi::log_str(msg)
}
