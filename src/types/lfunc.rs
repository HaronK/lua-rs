use super::prelude::*;

/*
** Upvalues for Lua closures
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,   /* points to stack or to its own value */
    pub refcount: lu_mem, /* reference counter */
    pub u: unnamed_2,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    pub open: unnamed_3, /* (when open) */
    pub value: TValue,   /* the value (when closed) */
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal, /* linked list */
    pub touched: lua_int, /* mark to avoid cycles with dead threads */
}
