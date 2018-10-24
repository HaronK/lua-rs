use super::prelude::*;

/* type of protected functions, to be ran by 'runprotected' */
pub type Pfunc = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_void) -> ()>;

/* chain list of long jump buffers */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: lua_int,
}
