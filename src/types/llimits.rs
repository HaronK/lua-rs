use super::prelude::*;

/*
** 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
** the total memory used by Lua (in bytes). Usually, 'size_t' and
** 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
*/
pub type lu_mem = size_t;
pub type l_mem = ptrdiff_t;

/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = lua_uchar;

/* type to ensure maximum alignment */
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub n: lua_Number,
    pub u: lua_double,
    pub s: *mut lua_void,
    pub i: lua_Integer,
    pub l: lua_long,
}

/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = lua_uint;
