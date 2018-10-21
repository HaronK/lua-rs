use libc;
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
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
}
pub type size_t = libc::c_ulong;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;


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
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_bit32(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg; 13]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, bitlib.as_ptr(), 0i32);
    return 1i32;
}
static mut bitlib: [luaL_Reg; 13] = [
    lua_reg!(b"arshift\x00", b_arshift),
    lua_reg!(b"band\x00", b_and),
    lua_reg!(b"bnot\x00", b_not),
    luaL_Reg {
        name: s!(b"bor\x00"),
        func: Some(b_or),
    },
    luaL_Reg {
        name: s!(b"bxor\x00"),
        func: Some(b_xor),
    },
    luaL_Reg {
        name: s!(b"btest\x00"),
        func: Some(b_test),
    },
    luaL_Reg {
        name: s!(b"extract\x00"),
        func: Some(b_extract),
    },
    luaL_Reg {
        name: s!(b"lrotate\x00"),
        func: Some(b_lrot),
    },
    luaL_Reg {
        name: s!(b"lshift\x00"),
        func: Some(b_lshift),
    },
    luaL_Reg {
        name: s!(b"replace\x00"),
        func: Some(b_replace),
    },
    luaL_Reg {
        name: s!(b"rrotate\x00"),
        func: Some(b_rrot),
    },
    luaL_Reg {
        name: s!(b"rshift\x00"),
        func: Some(b_rshift),
    },
    luaL_Reg {
        name: 0 as *const libc::c_char,
        func: None,
    },
];
unsafe extern "C" fn b_rshift(mut L: *mut lua_State) -> libc::c_int {
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
) -> libc::c_int {
    if i < 0i32 as libc::c_longlong {
        /* shift right? */
        i = -i;
        r = r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
        if i >= 32i32 as libc::c_longlong {
            r = 0i32 as lua_Unsigned
        } else {
            r >>= i
        }
    } else {
        /* shift left */
        if i >= 32i32 as libc::c_longlong {
            r = 0i32 as lua_Unsigned
        } else {
            r <<= i
        }
        r = r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)
    }
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_rrot(mut L: *mut lua_State) -> libc::c_int {
    return b_rot(L, -luaL_checkinteger(L, 2i32));
}
unsafe extern "C" fn b_rot(mut L: *mut lua_State, mut d: lua_Integer) -> libc::c_int {
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned;
    /* i = d % NBITS */
    let mut i: libc::c_int = (d & (32i32 - 1i32) as libc::c_longlong) as libc::c_int;
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
unsafe extern "C" fn b_replace(mut L: *mut lua_State) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut v: lua_Unsigned = luaL_checkinteger(L, 2i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut f: libc::c_int = fieldargs(L, 3i32, &mut w);
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
    mut farg: libc::c_int,
    mut width: *mut libc::c_int,
) -> libc::c_int {
    let mut f: lua_Integer = luaL_checkinteger(L, farg);
    let mut w: lua_Integer = luaL_optinteger(L, farg + 1i32, 1i32 as lua_Integer);
    (0i32 as libc::c_longlong <= f
        || 0 != luaL_argerror(L, farg, s!(b"field cannot be negative\x00"))) as libc::c_int;
    ((0i32 as libc::c_longlong) < w
        || 0 != luaL_argerror(L, farg + 1i32, s!(b"width must be positive\x00")))
        as libc::c_int;
    if f + w > 32i32 as libc::c_longlong {
        luaL_error(L, s!(b"trying to access non-existent bits\x00"));
    }
    *width = w as libc::c_int;
    return f as libc::c_int;
}
unsafe extern "C" fn b_lshift(mut L: *mut lua_State) -> libc::c_int {
    return b_shift(
        L,
        luaL_checkinteger(L, 1i32) as lua_Unsigned,
        luaL_checkinteger(L, 2i32),
    );
}
unsafe extern "C" fn b_lrot(mut L: *mut lua_State) -> libc::c_int {
    return b_rot(L, luaL_checkinteger(L, 2i32));
}
unsafe extern "C" fn b_extract(mut L: *mut lua_State) -> libc::c_int {
    let mut w: libc::c_int = 0;
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned
        & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
    let mut f: libc::c_int = fieldargs(L, 2i32, &mut w);
    r = r >> f & !(!((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32) << 1i32 << w - 1i32);
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_test(mut L: *mut lua_State) -> libc::c_int {
    let mut r: lua_Unsigned = andaux(L);
    lua_pushboolean(L, (r != 0i32 as libc::c_ulonglong) as libc::c_int);
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
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
    let mut r: lua_Unsigned = !(0i32 as lua_Unsigned);
    i = 1i32;
    while i <= n {
        r &= luaL_checkinteger(L, i) as lua_Unsigned;
        i += 1
    }
    return r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32);
}
unsafe extern "C" fn b_xor(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
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
unsafe extern "C" fn b_or(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = lua_gettop(L);
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
unsafe extern "C" fn b_not(mut L: *mut lua_State) -> libc::c_int {
    let mut r: lua_Unsigned = !(luaL_checkinteger(L, 1i32) as lua_Unsigned);
    lua_pushinteger(
        L,
        (r & !((!(0i32 as lua_Unsigned)) << 32i32 - 1i32 << 1i32)) as lua_Integer,
    );
    return 1i32;
}
unsafe extern "C" fn b_and(mut L: *mut lua_State) -> libc::c_int {
    let mut r: lua_Unsigned = andaux(L);
    lua_pushinteger(L, r as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn b_arshift(mut L: *mut lua_State) -> libc::c_int {
    let mut r: lua_Unsigned = luaL_checkinteger(L, 1i32) as lua_Unsigned;
    let mut i: lua_Integer = luaL_checkinteger(L, 2i32);
    if i < 0i32 as libc::c_longlong || 0 == r & (1i32 as lua_Unsigned) << 32i32 - 1i32 {
        return b_shift(L, r, -i);
    } else {
        /* arithmetic shift for 'negative' number */
        if i >= 32i32 as libc::c_longlong {
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
