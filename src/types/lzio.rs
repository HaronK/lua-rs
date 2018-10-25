use super::prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const lua_char,
    pub reader: lua_Reader,
    pub data: *mut lua_void,
    pub L: *mut lua_State,
}

pub type ZIO = Zio;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut lua_char,
    pub n: size_t,
    pub buffsize: size_t,
}
