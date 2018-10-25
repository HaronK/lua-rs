use super::prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const lua_char,
    pub func: lua_CFunction,
}

/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/
/* userdata to box arbitrary data */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UBox {
    pub box_0: *mut lua_void,
    pub bsize: size_t,
}

/*
** {======================================================
** Generic Buffer manipulation
** =======================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Buffer {
    pub b: *mut lua_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub initb: [lua_char; 8192],
}
/* }====================================================== */
/*
** {======================================================
** File handles for IO library
** =======================================================
*/
/*
** A file handle is a userdata with metatable 'LUA_FILEHANDLE' and
** initial structure 'luaL_Stream' (it may contain other fields
** after that initial structure).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Stream {
    pub f: *mut FILE,
    pub closef: lua_CFunction,
}
/* }====================================================== */
pub type LStream = luaL_Stream;
