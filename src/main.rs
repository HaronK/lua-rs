//TODO: remove all not necessary allows and features
#![feature(extern_types)]
#![feature(const_slice_as_ptr)]
#![feature(ptr_wrapping_offset_from)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![allow(unused_macros)]

#[macro_use]
pub mod macros;

pub mod ffi;
pub mod stdc;
pub mod types;

pub mod lapi;
pub mod lauxlib;
pub mod lbaselib;
pub mod lbitlib;
pub mod lcode;
pub mod lcorolib;
pub mod lctype;
pub mod ldblib;
pub mod ldebug;
pub mod ldo;
pub mod ldump;
pub mod lfunc;
pub mod lgc;
pub mod linit;
pub mod liolib;
pub mod llex;
pub mod llimits;
pub mod lmathlib;
pub mod lmem;
pub mod loadlib;
pub mod lobject;
pub mod lopcodes;
pub mod loslib;
pub mod lparser;
pub mod lstate;
pub mod lstring;
pub mod lstrlib;
pub mod ltable;
pub mod ltablib;
pub mod ltm;
pub mod lua;
pub mod luac;
pub mod luaconf;
pub mod lundump;
pub mod lutf8lib;
pub mod lvm;
pub mod lzio;

pub mod prelude {
    // pub use types::prelude::*;
    // pub use stdc::prelude::*;
    pub use super::{types::prelude::*, stdc::prelude::*, ffi::prelude::*,
        lapi::*, lauxlib::*, lbaselib::*, lbitlib::*, lcode::*, lcorolib::*, lctype::*, ldblib::*,
        ldebug::*, ldo::*, ldump::*, lfunc::*, lgc::*, linit::*, liolib::*, llex::*, llimits::*,
        lmathlib::*, lmem::*, loadlib::*, lobject::*, lopcodes::*, loslib::*, lparser::*,
        lstate::*, lstring::*, lstrlib::*, ltable::*, ltablib::*, ltm::*, lua::*, luac::*,
        luaconf::*, lundump::*, lutf8lib::*, lvm::*, lzio::*, *,
    };
}

use luac::main_0;
use types::prelude::*;

fn main() -> () {
    let mut args: Vec<*mut lua_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as lua_int,
            args.as_mut_ptr() as *mut *mut lua_char,
        ) as i32)
    }
}
