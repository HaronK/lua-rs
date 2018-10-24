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

/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = lua_uint;
