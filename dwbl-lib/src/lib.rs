use std::ffi::c_char;
use std::pin::Pin;

use cxx::{let_cxx_string, CxxString};
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

// TODO: This namespace is not hardcoded in yosys itself. This should probably read from the
// makefile.
#[cxx::bridge(namespace = "Yosys::RTLIL")]
pub mod rtlil {
    unsafe extern "C++" {
        include!("lib.h");
        include!("kernel/yosys.h");

        type IdString;
        type Cell;

        type SigSpec;
        unsafe fn makeSigSpec(wire: *mut Wire, offset: i32, width: i32) -> UniquePtr<SigSpec>;

        type Design;
        fn has(self: &Design, id: &IdString) -> bool;
        fn addModule(design: Pin<&mut Design>, name: &IdString) -> *mut Module;
        fn addModuleStr(design: Pin<&mut Design>, name: &CxxString) -> *mut Module;

        type Module;
        unsafe fn fixup_ports(module: *mut Module);
        unsafe fn addWire(module: *mut Module, name: &IdString, width: i32) -> *mut Wire;
        unsafe fn addWireStr(module: *mut Module, name: &CxxString, width: i32) -> *mut Wire;
        unsafe fn addNeg(
            module: *mut Module,
            name: &IdString,
            sig_a: UniquePtr<SigSpec>,
            sig_y: UniquePtr<SigSpec>,
            is_signed: bool,
            src: &CxxString,
        ) -> *mut Cell;
        unsafe fn addMux(
            module: *mut Module,
            name: &IdString,
            sig_a: UniquePtr<SigSpec>,
            sig_b: UniquePtr<SigSpec>,
            sig_s: UniquePtr<SigSpec>,
            sig_y: UniquePtr<SigSpec>,
            src: &CxxString,
        ) -> *mut Cell;
        unsafe fn addNegStr(
            module: *mut Module,
            name: &CxxString,
            sig_a: UniquePtr<SigSpec>,
            sig_y: UniquePtr<SigSpec>,
            is_signed: bool,
            src: &CxxString,
        ) -> *mut Cell;
        unsafe fn addMuxStr(
            module: *mut Module,
            name: &CxxString,
            sig_a: UniquePtr<SigSpec>,
            sig_b: UniquePtr<SigSpec>,
            sig_s: UniquePtr<SigSpec>,
            sig_y: UniquePtr<SigSpec>,
            src: &CxxString,
        ) -> *mut Cell;

        type Wire;
        unsafe fn setPortInput(wire: *mut Wire, b: bool);
        unsafe fn setPortOutput(wire: *mut Wire, b: bool);
        unsafe fn setPortId(wire: *mut Wire, id: i32);
    }

    extern "Rust" {
        unsafe fn build_module(design: Pin<&mut Design>);
    }
}

pub fn log_from_rust(msg: &CxxString) -> *const c_char {
    ffi::log_str(msg)
}

pub unsafe fn build_module(design: Pin<&mut rtlil::Design>) {
    let_cxx_string!(name = "\\absval");
    let module = rtlil::addModuleStr(design, &name);

    let_cxx_string!(a_name = "\\a");
    let a = rtlil::addWireStr(module, &a_name, 4);
    rtlil::setPortInput(a, true);
    rtlil::setPortId(a, 1);

    let_cxx_string!(y_name = "\\y");
    let y = rtlil::addWireStr(module, &y_name, 4);
    rtlil::setPortOutput(y, true);
    rtlil::setPortId(y, 2);

    let_cxx_string!(a_inv_name = "\\id1");
    let a_inv = rtlil::addWireStr(module, &a_inv_name, 4);
    let_cxx_string!(empty = "");
    let_cxx_string!(neg_name = "\\id2");
    let spec_a = rtlil::makeSigSpec(a, 0, 0);
    rtlil::addNegStr(
        module,
        &neg_name,
        spec_a,
        rtlil::makeSigSpec(a_inv, 0, 0),
        true,
        &empty,
    );
    let_cxx_string!(mux_name = "\\id3");
    rtlil::addMuxStr(
        module,
        &mux_name,
        rtlil::makeSigSpec(a, 0, 0),
        rtlil::makeSigSpec(a_inv, 0, 0),
        rtlil::makeSigSpec(a, 3, 1),
        rtlil::makeSigSpec(y, 0, 0),
        &empty,
    );

    rtlil::fixup_ports(module);
}
