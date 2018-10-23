use types::*;
extern "C" {
    /*
     ** $Id: lua.h,v 1.332.1.2 2018/06/13 16:58:17 roberto Exp $
     ** Lua - A Scripting Language
     ** Lua.org, PUC-Rio, Brazil (http://www.lua.org)
     ** See Copyright Notice at the end of this file
     */
    /* mark for precompiled code ('<esc>Lua') */
    /* option for multiple returns in 'lua_pcall' and 'lua_call' */
    /*
     ** Pseudo-indices
     ** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
     ** space after that to help overflow detection)
     */
    /* thread status */
    pub type lua_State;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: lua_int,
        extramsg: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State, arg: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: lua_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
}
pub type size_t = lua_ulong;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = lua_double;
/* type for integer functions */
pub type lua_Integer = lua_longlong;
/* unsigned integer type */
pub type lua_Unsigned = lua_ulonglong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> lua_int>;

/*
** $Id: lauxlib.h,v 1.131.1.1 2017/04/19 17:20:42 roberto Exp $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/

/* extra error code for 'luaL_loadfilex' */
/* key, in the registry, for table of loaded modules */
/* key, in the registry, for table of preloaded loaders */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Reg {
    pub name: *const lua_char,
    pub func: lua_CFunction,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_bit32(mut L: *mut lua_State) -> lua_int {
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(16i32 as lua_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as lua_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg; 13]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, bitlib.as_ptr(), 0i32);
    return 1i32;
}
static mut bitlib: [luaL_Reg; 13] = [
    lua_reg!(b"arshift\x00", b_arshift),
    lua_reg!(b"band\x00", b_and),
    lua_reg!(b"bnot\x00", b_not),
    lua_reg!(b"bor\x00", b_or),
    lua_reg!(b"bxor\x00", b_xor),
    lua_reg!(b"btest\x00", b_test),
    lua_reg!(b"extract\x00", b_extract),
    lua_reg!(b"lrotate\x00", b_lrot),
    lua_reg!(b"lshift\x00", b_lshift),
    lua_reg!(b"replace\x00", b_replace),
    lua_reg!(b"rrotate\x00", b_rrot),
    lua_reg!(b"rshift\x00", b_rshift),
    lua_reg_none!(0),
];
unsafe extern "C" fn b_rshift(mut L: *mut lua_State) -> lua_int {
    return b_shift(
        L,
        luaL_checkinteger(L, 1i32) as lua_Unsigned,
        -luaL_checkinteger(L, 2i32),
    );
}
unsafe extern "C" fn b_shift(
    mut L: *mut lua_State,
    mut r: lua_Unsigned,
    mut i: lua_Integer,
) -> lua_int {
    if i < 0i32 as lua_longlong {
        /* shift right? */
        i = -i;
        r = r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
        if i >= 32i32 as lua_longlong {
            r = 0i32 as lua_Unsigned
        } else {
            r >>= i
        }
    } else {
        /* shift left */
        if i >= 32i32 as lua_longlong {
            r = 0i32 as lua_Unsigned
        } else {
            r <<= i
        }
        r = r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)
    }
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_rrot(mut L: *mut lua_State) -> lua_int {
    return b_rot(L, -luaL_checkinteger(L, 2i32));
}
unsafe extern "C" fn b_rot(mut L: *mut lua_State, mut d: lua_Integer) -> lua_int {
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned;
    /* i = d % NBITS */
    let mut i: lua_int = (d & (32i32 - 1i32) as lua_longlong) as lua_int;
    r = r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    /* avoid undefined shift of LUA_NBITS when i == 0 */
    if i != 0i32 {
        r = r << i | r >> 32i32 - i
    }
    lua_pushinteger(
        L,
        (r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)) as lua_Integer,
    );
    return 1i32;
}
unsafe extern "C" fn b_replace(mut L: *mut lua_State) -> lua_int {
    let mut w: lua_int = 0;
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut v: lua_Unsigned = luaL_checkinteger(L, 2i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut f: lua_int = fieldargs(L, 3i32, &mut w);
    let mut m: lua_Unsigned =
        !(!((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32) << 1i32 << w - 1i32);
    r = r & !(m << f) | (v & m) << f;
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
/*
** get field and width arguments for field-manipulation functions,
** checking whether they are valid.
** ('luaL_error' called without 'return' to avoid later warnings about
** 'width' being used uninitialized.)
*/
unsafe extern "C" fn fieldargs(
    mut L: *mut lua_State,
    mut farg: lua_int,
    mut width: *mut lua_int,
) -> lua_int {
    let mut f: lua_Integer = luaL_checkinteger(L, farg);
    let mut w: lua_Integer = luaL_optinteger(L, farg + 1i32, 1i32 as lua_Integer);
    (0i32 as lua_longlong <= f
        || 0 != luaL_argerror(L, farg, s!(b"field cannot be negative\x00"))) as lua_int;
    ((0i32 as lua_longlong) < w
        || 0 != luaL_argerror(L, farg + 1i32, s!(b"width must be positive\x00")))
        as lua_int;
    if f + w > 32i32 as lua_longlong {
        luaL_error(L, s!(b"trying to access non-existent bits\x00"));
    }
    *width = w as lua_int;
    return f as lua_int;
}
unsafe extern "C" fn b_lshift(mut L: *mut lua_State) -> lua_int {
    return b_shift(
        L,
        luaL_checkinteger(L, 1i32) as lua_Unsigned,
        luaL_checkinteger(L, 2i32),
    );
}
unsafe extern "C" fn b_lrot(mut L: *mut lua_State) -> lua_int {
    return b_rot(L, luaL_checkinteger(L, 2i32));
}
unsafe extern "C" fn b_extract(mut L: *mut lua_State) -> lua_int {
    let mut w: lua_int = 0;
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut f: lua_int = fieldargs(L, 2i32, &mut w);
    r = r >> f & !(!((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32) << 1i32 << w - 1i32);
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_test(mut L: *mut lua_State) -> lua_int {
    let mut r: lua_Unsigned = andaux(L);
    lua_pushboolean(L, (r != 0i32 as lua_ulonglong) as lua_int);
    return 1i32;
}
/*
** $Id: lbitlib.c,v 1.30.1.1 2017/04/19 17:20:42 roberto Exp $
** Standard library for bitwise operations
** See Copyright Notice in lua.h
*/
/* { */
/* number of bits to consider in a number */
/*
** a lua_Unsigned with its first LUA_NBITS bits equal to 1. (Shift must
** be made in two parts to avoid problems when LUA_NBITS is equal to the
** number of bits in a lua_Unsigned.)
*/
/* macro to trim extra bits */
/* builds a number with 'n' ones (1 <= n <= LUA_NBITS) */
unsafe extern "C" fn andaux(mut L: *mut lua_State) -> lua_Unsigned {
    let mut i: lua_int = 0;
    let mut n: lua_int = lua_gettop(L);
    let mut r: lua_Unsigned = !(0i32 as lua_Unsigned);
    i = 1i32;
    while i <= n {
        r &= luaL_checkinteger(L, i) as lua_Unsigned;
        i += 1
    }
    return r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
}
unsafe extern "C" fn b_xor(mut L: *mut lua_State) -> lua_int {
    let mut i: lua_int = 0;
    let mut n: lua_int = lua_gettop(L);
    let mut r: lua_Unsigned = 0i32 as lua_Unsigned;
    i = 1i32;
    while i <= n {
        r ^= luaL_checkinteger(L, i) as lua_Unsigned;
        i += 1
    }
    lua_pushinteger(
        L,
        (r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)) as lua_Integer,
    );
    return 1i32;
}
unsafe extern "C" fn b_or(mut L: *mut lua_State) -> lua_int {
    let mut i: lua_int = 0;
    let mut n: lua_int = lua_gettop(L);
    let mut r: lua_Unsigned = 0i32 as lua_Unsigned;
    i = 1i32;
    while i <= n {
        r |= luaL_checkinteger(L, i) as lua_Unsigned;
        i += 1
    }
    lua_pushinteger(
        L,
        (r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)) as lua_Integer,
    );
    return 1i32;
}
unsafe extern "C" fn b_not(mut L: *mut lua_State) -> lua_int {
    let mut r: lua_Unsigned = !(luaL_checkinteger(L, 1i32) as lua_Unsigned);
    lua_pushinteger(
        L,
        (r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)) as lua_Integer,
    );
    return 1i32;
}
unsafe extern "C" fn b_and(mut L: *mut lua_State) -> lua_int {
    let mut r: lua_Unsigned = andaux(L);
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_arshift(mut L: *mut lua_State) -> lua_int {
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned;
    let mut i: lua_Integer = luaL_checkinteger(L, 2i32);
    if i < 0i32 as lua_longlong || 0 == r & (1i32 as lua_Unsigned) << 32i32 - 1i32 {
        return b_shift(L, r, -i);
    } else {
        /* arithmetic shift for 'negative' number */
        if i >= 32i32 as lua_longlong {
            r = !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)
        } else {
            r = (r >> i | !((!(0i32 as lua_Unsigned)
                & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32))
                >> i))
                & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)
        }
        lua_pushinteger(L, r as lua_Integer);
        return 1i32;
    };
}
