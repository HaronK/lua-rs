use stdc::prelude::*;
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
    fn localeconv() -> *mut lconv;
    #[no_mangle]
    fn snprintf(_: *mut lua_char, _: lua_ulong, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn memcmp(_: *const lua_void, _: *const lua_void, _: lua_ulong) -> lua_int;
    #[no_mangle]
    fn memchr(_: *const lua_void, _: lua_int, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn strcpy(_: *mut lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    fn strchr(_: *const lua_char, _: lua_int) -> *mut lua_char;
    #[no_mangle]
    fn strpbrk(_: *const lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: lua_int, n: lua_int) -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, tp: lua_int) -> *const lua_char;
    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Number;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: lua_int) -> *mut lua_void;
    /*
     ** push functions (C -> stack)
     */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State, s: *const lua_char, len: size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();
    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut lua_void;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    /*
     ** 'load' and 'call' functions (load and run Lua code)
     */
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State,
        nargs: lua_int,
        nresults: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
    #[no_mangle]
    fn lua_dump(
        L: *mut lua_State,
        writer_0: lua_Writer,
        data: *mut lua_void,
        strip: lua_int,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, arg: lua_int, extramsg: *const lua_char) -> lua_int;
    #[no_mangle]
    fn luaL_checklstring(L: *mut lua_State, arg: lua_int, l: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: lua_int,
        def: *const lua_char,
        l: *mut size_t,
    ) -> *const lua_char;
    #[no_mangle]
    fn luaL_checknumber(L: *mut lua_State, arg: lua_int) -> lua_Number;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State, arg: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: lua_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, sz: lua_int, msg: *const lua_char) -> ();
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State, arg: lua_int, t: lua_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut lua_char;
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const lua_char, l: size_t) -> ();
    #[no_mangle]
    fn luaL_addstring(B: *mut luaL_Buffer, s: *const lua_char) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresultsize(B: *mut luaL_Buffer, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_buffinitsize(L: *mut lua_State, B: *mut luaL_Buffer, sz: size_t) -> *mut lua_char;
}

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
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> lua_int>;
/*
** Type for continuation functions
*/
pub type lua_KFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: lua_int, _: lua_KContext) -> lua_int>;
pub type lua_Writer = Option<
    unsafe extern "C" fn(_: *mut lua_State, _: *const lua_void, _: size_t, _: *mut lua_void)
        -> lua_int,
>;
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
/*
** ===============================================================
** some useful macros
** ===============================================================
*/
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
/* no-op (configuration or spaces) */
pub const Knop: KOption = 8;
/* padding */
pub const Kpadding: KOption = 6;
/* padding for alignment */
pub const Kpaddalign: KOption = 7;
/* zero-terminated strings */
pub const Kzstr: KOption = 5;
/*
** information to pack/unpack stuff
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Header {
    pub L: *mut lua_State,
    pub islittle: lua_int,
    pub maxalign: lua_int,
}
/* strings with prefixed length */
pub const Kstring: KOption = 4;
/* fixed-length strings */
pub const Kchar: KOption = 3;
/*
** Union for serializing floats
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union Ftypes {
    pub f: lua_float,
    pub d: lua_double,
    pub n: lua_Number,
    pub buff: [lua_char; 40],
}
/* }====================================================== */
/*
** {======================================================
** PACK/UNPACK
** =======================================================
*/
/* value used for padding */
/* maximum size for the binary representation of an integer */
/* number of bits in a character */
/* mask for one character (NB 1's) */
/* size of a lua_Integer */
/* dummy union to get native endianness */
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_0 {
    pub dummy: lua_int,
    pub little: lua_char,
}
/* floating-point numbers */
pub const Kfloat: KOption = 2;
/* signed integers */
pub const Kint: KOption = 0;
/*
** options for pack/unpack
*/
pub type KOption = lua_uint;
/* unsigned integers */
pub const Kuint: KOption = 1;
/*
** {======================================================
** PATTERN MATCHING
** =======================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MatchState {
    pub src_init: *const lua_char,
    pub src_end: *const lua_char,
    pub p_end: *const lua_char,
    pub L: *mut lua_State,
    pub matchdepth: lua_int,
    pub level: lua_uchar,
    pub capture: [unnamed_1; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
    pub init: *const lua_char,
    pub len: ptrdiff_t,
}
/* state for 'gmatch' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GMatchState {
    pub src: *const lua_char,
    pub p: *const lua_char,
    pub lastmatch: *const lua_char,
    pub ms: MatchState,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_string(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 18]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, strlib.as_ptr(), 0i32);
    createmetatable(L);
    return 1i32;
}
unsafe extern "C" fn createmetatable(mut L: *mut lua_State) -> () {
    /* table to be metatable for strings */
    lua_createtable(L, 0i32, 1i32);
    /* dummy string */
    lua_pushstring(L, s!(b"\x00"));
    /* copy table */
    lua_pushvalue(L, -2i32);
    /* set table as metatable for strings */
    lua_setmetatable(L, -2i32);
    /* pop dummy string */
    lua_settop(L, -1i32 - 1i32);
    /* get string library */
    lua_pushvalue(L, -2i32);
    /* metatable.__index = string */
    lua_setfield(L, -2i32, s!(b"__index\x00"));
    /* pop metatable */
    lua_settop(L, -1i32 - 1i32);
}
/* }====================================================== */
static mut strlib: [luaL_Reg; 18] = [
    lua_reg!(b"byte\x00", str_byte),
    lua_reg!(b"char\x00", str_char),
    lua_reg!(b"dump\x00", str_dump),
    lua_reg!(b"find\x00", str_find),
    lua_reg!(b"format\x00", str_format),
    lua_reg!(b"gmatch\x00", gmatch),
    lua_reg!(b"gsub\x00", str_gsub),
    lua_reg!(b"len\x00", str_len),
    lua_reg!(b"lower\x00", str_lower),
    lua_reg!(b"match\x00", str_match),
    lua_reg!(b"rep\x00", str_rep),
    lua_reg!(b"reverse\x00", str_reverse),
    lua_reg!(b"sub\x00", str_sub),
    lua_reg!(b"upper\x00", str_upper),
    lua_reg!(b"pack\x00", str_pack),
    lua_reg!(b"packsize\x00", str_packsize),
    lua_reg!(b"unpack\x00", str_unpack),
    lua_reg_none!(0),
];
unsafe extern "C" fn str_unpack(mut L: *mut lua_State) -> lua_int {
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    let mut fmt: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut ld: size_t = 0;
    let mut data: *const lua_char = luaL_checklstring(L, 2i32, &mut ld);
    let mut pos: size_t = (posrelat(luaL_optinteger(L, 3i32, 1i32 as lua_Integer), ld) as size_t)
        .wrapping_sub(1i32 as lua_ulong);
    /* number of results */
    let mut n: lua_int = 0i32;
    (pos <= ld || 0 != luaL_argerror(L, 3i32, s!(b"initial position out of string\x00")))
        as lua_int;
    initheader(L, &mut h);
    while *fmt as lua_int != '\u{0}' as i32 {
        let mut size: lua_int = 0;
        let mut ntoalign: lua_int = 0;
        let mut opt: KOption = getdetails(&mut h, pos, &mut fmt, &mut size, &mut ntoalign);
        if (ntoalign as size_t).wrapping_add(size as lua_ulong) > !pos
            || pos
                .wrapping_add(ntoalign as lua_ulong)
                .wrapping_add(size as lua_ulong)
                > ld
        {
            luaL_argerror(L, 2i32, s!(b"data string too short\x00"));
        }
        /* skip alignment */
        pos = (pos as lua_ulong).wrapping_add(ntoalign as lua_ulong) as size_t as size_t;
        /* stack space for item + next position */
        luaL_checkstack(L, 2i32, s!(b"too many results\x00"));
        n += 1;
        match opt as lua_uint {
            0 | 1 => {
                let mut res: lua_Integer = unpackint(
                    L,
                    data.offset(pos as isize),
                    h.islittle,
                    size,
                    (opt as lua_uint == Kint as lua_int as lua_uint) as lua_int,
                );
                lua_pushinteger(L, res);
            }
            2 => {
                let mut u: Ftypes = Ftypes { f: 0. };
                let mut num: lua_Number = 0.;
                copywithendian(
                    u.buff.as_mut_ptr(),
                    data.offset(pos as isize),
                    size,
                    h.islittle,
                );
                if size as lua_ulong == ::std::mem::size_of::<lua_float>() as lua_ulong {
                    num = u.f as lua_Number
                } else if size as lua_ulong == ::std::mem::size_of::<lua_double>() as lua_ulong {
                    num = u.d
                } else {
                    num = u.n
                }
                lua_pushnumber(L, num);
            }
            3 => {
                lua_pushlstring(L, data.offset(pos as isize), size as size_t);
            }
            4 => {
                let mut len: size_t =
                    unpackint(L, data.offset(pos as isize), h.islittle, size, 0i32) as size_t;
                (pos.wrapping_add(len).wrapping_add(size as lua_ulong) <= ld
                    || 0 != luaL_argerror(L, 2i32, s!(b"data string too short\x00")))
                    as lua_int;
                lua_pushlstring(L, data.offset(pos as isize).offset(size as isize), len);
                /* skip string */
                pos = (pos as lua_ulong).wrapping_add(len) as size_t as size_t
            }
            5 => {
                let mut len_0: size_t = strlen(data.offset(pos as isize)) as lua_int as size_t;
                lua_pushlstring(L, data.offset(pos as isize), len_0);
                /* skip string plus final '\0' */
                pos = (pos as lua_ulong).wrapping_add(len_0.wrapping_add(1i32 as lua_ulong))
                    as size_t as size_t
            }
            7 | 6 | 8 => {
                /* undo increment */
                n -= 1
            }
            _ => {}
        }
        pos = (pos as lua_ulong).wrapping_add(size as lua_ulong) as size_t as size_t
    }
    /* next position */
    lua_pushinteger(L, pos.wrapping_add(1i32 as lua_ulong) as lua_Integer);
    return n + 1i32;
}
/* translate a relative string position: negative means back from end */
unsafe extern "C" fn posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0i32 as lua_longlong {
        return pos;
    } else if (0u32 as lua_ulong).wrapping_sub(pos as size_t) > len {
        return 0i32 as lua_Integer;
    } else {
        return len as lua_Integer + pos + 1i32 as lua_longlong;
    };
}
/*
** Unpack an integer with 'size' bytes and 'islittle' endianness.
** If size is smaller than the size of a Lua integer and integer
** is signed, must do sign extension (propagating the sign to the
** higher bits); if size is larger than the size of a Lua integer,
** it must check the unread bytes to see whether they do not cause an
** overflow.
*/
unsafe extern "C" fn unpackint(
    mut L: *mut lua_State,
    mut str: *const lua_char,
    mut islittle: lua_int,
    mut size: lua_int,
    mut issigned: lua_int,
) -> lua_Integer {
    let mut res: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut i: lua_int = 0;
    let mut limit: lua_int = if size <= ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int
    {
        size
    } else {
        ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int
    };
    i = limit - 1i32;
    while i >= 0i32 {
        res <<= 8i32;
        res |= *str.offset((if 0 != islittle { i } else { size - 1i32 - i }) as isize) as lua_uchar
            as lua_Unsigned;
        i -= 1
    }
    if size < ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int {
        /* real size smaller than lua_Integer? */
        if 0 != issigned {
            /* needs sign extension? */
            let mut mask: lua_Unsigned = (1i32 as lua_Unsigned) << size * 8i32 - 1i32;
            /* do sign extension */
            res = (res ^ mask).wrapping_sub(mask)
        }
    } else if size > ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int {
        /* must check unread bytes */
        let mut mask_0: lua_int = if 0 == issigned || res as lua_Integer >= 0i32 as lua_longlong {
            0i32
        } else {
            (1i32 << 8i32) - 1i32
        };
        i = limit;
        while i < size {
            if *str.offset((if 0 != islittle { i } else { size - 1i32 - i }) as isize) as lua_uchar
                as lua_int
                != mask_0
            {
                luaL_error!(
                    L,
                    s!(b"%d-byte integer does not fit into Lua Integer\x00"),
                    size,
                );
            }
            i += 1
        }
    }
    return res as lua_Integer;
}
/*
** Copy 'size' bytes from 'src' to 'dest', correcting endianness if
** given 'islittle' is different from native endianness.
*/
unsafe extern "C" fn copywithendian(
    mut dest: *mut lua_char,
    mut src: *const lua_char,
    mut size: lua_int,
    mut islittle: lua_int,
) -> () {
    if islittle == nativeendian.little as lua_int {
        loop {
            let fresh0 = size;
            size = size - 1;
            if !(fresh0 != 0i32) {
                break;
            }
            let fresh2 = dest;
            dest = dest.offset(1);
            let fresh1 = src;
            src = src.offset(1);
            ::std::ptr::write_volatile(fresh2, *fresh1)
        }
    } else {
        dest = dest.offset((size - 1i32) as isize);
        loop {
            let fresh3 = size;
            size = size - 1;
            if !(fresh3 != 0i32) {
                break;
            }
            let fresh5 = dest;
            dest = dest.offset(-1);
            let fresh4 = src;
            src = src.offset(1);
            ::std::ptr::write_volatile(fresh5, *fresh4)
        }
    };
}
static mut nativeendian: unnamed_0 = unnamed_0 { dummy: 1i32 };
/*
** Read, classify, and fill other details about the next option.
** 'psize' is filled with option's size, 'notoalign' with its
** alignment requirements.
** Local variable 'size' gets the size to be aligned. (Kpadal option
** always gets its full alignment, other options are limited by
** the maximum alignment ('maxalign'). Kchar option needs no alignment
** despite its size.
*/
unsafe extern "C" fn getdetails(
    mut h: *mut Header,
    mut totalsize: size_t,
    mut fmt: *mut *const lua_char,
    mut psize: *mut lua_int,
    mut ntoalign: *mut lua_int,
) -> KOption {
    let mut opt: KOption = getoption(h, fmt, psize);
    /* usually, alignment follows size */
    let mut align: lua_int = *psize;
    if opt as lua_uint == Kpaddalign as lua_int as lua_uint {
        /* 'X' gets alignment from following option */
        if **fmt as lua_int == '\u{0}' as i32
            || getoption(h, fmt, &mut align) as lua_uint == Kchar as lua_int as lua_uint
            || align == 0i32
        {
            luaL_argerror(
                (*h).L,
                1i32,
                s!(b"invalid next option for option \'X\'\x00"),
            );
        }
    }
    /* need no alignment? */
    if align <= 1i32 || opt as lua_uint == Kchar as lua_int as lua_uint {
        *ntoalign = 0i32
    } else {
        /* enforce maximum alignment */
        if align > (*h).maxalign {
            align = (*h).maxalign
        }
        /* is 'align' not a power of 2? */
        if align & align - 1i32 != 0i32 {
            luaL_argerror(
                (*h).L,
                1i32,
                s!(b"format asks for alignment not power of 2\x00"),
            );
        }
        *ntoalign = align - (totalsize & (align - 1i32) as lua_ulong) as lua_int & align - 1i32
    }
    return opt;
}
/*
** Read and classify next option. 'size' is filled with option's size.
*/
unsafe extern "C" fn getoption(
    mut h: *mut Header,
    mut fmt: *mut *const lua_char,
    mut size: *mut lua_int,
) -> KOption {
    let fresh6 = *fmt;
    *fmt = (*fmt).offset(1);
    let mut opt: lua_int = *fresh6 as lua_int;
    /* default */
    *size = 0i32;
    match opt {
        98 => {
            *size = ::std::mem::size_of::<lua_char>() as lua_ulong as lua_int;
            return Kint;
        }
        66 => {
            *size = ::std::mem::size_of::<lua_char>() as lua_ulong as lua_int;
            return Kuint;
        }
        104 => {
            *size = ::std::mem::size_of::<lua_short>() as lua_ulong as lua_int;
            return Kint;
        }
        72 => {
            *size = ::std::mem::size_of::<lua_short>() as lua_ulong as lua_int;
            return Kuint;
        }
        108 => {
            *size = ::std::mem::size_of::<lua_long>() as lua_ulong as lua_int;
            return Kint;
        }
        76 => {
            *size = ::std::mem::size_of::<lua_long>() as lua_ulong as lua_int;
            return Kuint;
        }
        106 => {
            *size = ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int;
            return Kint;
        }
        74 => {
            *size = ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int;
            return Kuint;
        }
        84 => {
            *size = ::std::mem::size_of::<size_t>() as lua_ulong as lua_int;
            return Kuint;
        }
        102 => {
            *size = ::std::mem::size_of::<lua_float>() as lua_ulong as lua_int;
            return Kfloat;
        }
        100 => {
            *size = ::std::mem::size_of::<lua_double>() as lua_ulong as lua_int;
            return Kfloat;
        }
        110 => {
            *size = ::std::mem::size_of::<lua_Number>() as lua_ulong as lua_int;
            return Kfloat;
        }
        105 => {
            *size = getnumlimit(
                h,
                fmt,
                ::std::mem::size_of::<lua_int>() as lua_ulong as lua_int,
            );
            return Kint;
        }
        73 => {
            *size = getnumlimit(
                h,
                fmt,
                ::std::mem::size_of::<lua_int>() as lua_ulong as lua_int,
            );
            return Kuint;
        }
        115 => {
            *size = getnumlimit(
                h,
                fmt,
                ::std::mem::size_of::<size_t>() as lua_ulong as lua_int,
            );
            return Kstring;
        }
        99 => {
            *size = getnum(fmt, -1i32);
            if *size == -1i32 {
                luaL_error!((*h).L, s!(b"missing size for format option \'c\'\x00"));
            }
            return Kchar;
        }
        122 => return Kzstr,
        120 => {
            *size = 1i32;
            return Kpadding;
        }
        88 => return Kpaddalign,
        32 => {}
        60 => (*h).islittle = 1i32,
        62 => (*h).islittle = 0i32,
        61 => (*h).islittle = nativeendian.little as lua_int,
        33 => (*h).maxalign = getnumlimit(h, fmt, 8u64 as lua_int),
        _ => {
            luaL_error!((*h).L, s!(b"invalid format option \'%c\'\x00"), opt);
        }
    }
    return Knop;
}
/*
** Read an integer numeral and raises an error if it is larger
** than the maximum size for integers.
*/
unsafe extern "C" fn getnumlimit(
    mut h: *mut Header,
    mut fmt: *mut *const lua_char,
    mut df: lua_int,
) -> lua_int {
    let mut sz: lua_int = getnum(fmt, df);
    if sz > 16i32 || sz <= 0i32 {
        return luaL_error!(
            (*h).L,
            s!(b"integral size (%d) out of limits [1,%d]\x00"),
            sz,
            16i32,
        );
    } else {
        return sz;
    };
}
unsafe extern "C" fn getnum(mut fmt: *mut *const lua_char, mut df: lua_int) -> lua_int {
    /* no number? */
    if 0 == digit(**fmt as lua_int) {
        /* return default value */
        return df;
    } else {
        let mut a: lua_int = 0i32;
        loop {
            let fresh7 = *fmt;
            *fmt = (*fmt).offset(1);
            a = a * 10i32 + (*fresh7 as lua_int - '0' as i32);
            if !(0 != digit(**fmt as lua_int)
                && a <= ((if (::std::mem::size_of::<size_t>() as lua_ulong)
                    < ::std::mem::size_of::<lua_int>() as lua_ulong
                {
                    !(0i32 as size_t)
                } else {
                    2147483647i32 as size_t
                }) as lua_int
                    - 9i32)
                    / 10i32)
            {
                break;
            }
        }
        return a;
    };
}
/*
** Read an integer numeral from string 'fmt' or return 'df' if
** there is no numeral
*/
unsafe extern "C" fn digit(mut c: lua_int) -> lua_int {
    return ('0' as i32 <= c && c <= '9' as i32) as lua_int;
}
/*
** Initialize Header
*/
unsafe extern "C" fn initheader(mut L: *mut lua_State, mut h: *mut Header) -> () {
    (*h).L = L;
    (*h).islittle = nativeendian.little as lua_int;
    (*h).maxalign = 1i32;
}
unsafe extern "C" fn str_packsize(mut L: *mut lua_State) -> lua_int {
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    /* format string */
    let mut fmt: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    /* accumulate total size of result */
    let mut totalsize: size_t = 0i32 as size_t;
    initheader(L, &mut h);
    while *fmt as lua_int != '\u{0}' as i32 {
        let mut size: lua_int = 0;
        let mut ntoalign: lua_int = 0;
        let mut opt: KOption = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        /* total space used by option */
        size += ntoalign;
        (totalsize
            <= if (::std::mem::size_of::<size_t>() as lua_ulong)
                < ::std::mem::size_of::<lua_int>() as lua_ulong
            {
                !(0i32 as size_t)
            } else {
                2147483647i32 as size_t
            }
            .wrapping_sub(size as lua_ulong)
            || 0 != luaL_argerror(L, 1i32, s!(b"format result too large\x00"))) as lua_int;
        totalsize = (totalsize as lua_ulong).wrapping_add(size as lua_ulong) as size_t as size_t;
        match opt as lua_uint {
            4 => {}
            5 => {}
            _ => {
                /* zero-terminated string */
                continue;
            }
        }
        luaL_argerror(L, 1i32, s!(b"variable-length format\x00"));
    }
    /* call never return, but to avoid warnings: */
    /* FALLTHROUGH */
    lua_pushinteger(L, totalsize as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn str_pack(mut L: *mut lua_State) -> lua_int {
    let mut lim: lua_Integer = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut h: Header = Header {
        L: 0 as *mut lua_State,
        islittle: 0,
        maxalign: 0,
    };
    /* format string */
    let mut fmt: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    /* current argument to pack */
    let mut arg: lua_int = 1i32;
    /* accumulate total size of result */
    let mut totalsize: size_t = 0i32 as size_t;
    initheader(L, &mut h);
    /* mark to separate arguments from string buffer */
    lua_pushnil(L);
    luaL_buffinit(L, &mut b);
    's_11: while *fmt as lua_int != '\u{0}' as i32 {
        let mut size: lua_int = 0;
        let mut ntoalign: lua_int = 0;
        let mut opt: KOption = getdetails(&mut h, totalsize, &mut fmt, &mut size, &mut ntoalign);
        totalsize = (totalsize as lua_ulong).wrapping_add((ntoalign + size) as lua_ulong) as size_t
            as size_t;
        loop {
            let fresh8 = ntoalign;
            ntoalign = ntoalign - 1;
            if !(fresh8 > 0i32) {
                break;
            }
            /* fill alignment */
            (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
            let fresh9 = b.n;
            b.n = b.n.wrapping_add(1);
            *b.b.offset(fresh9 as isize) = 0i32 as lua_char
        }
        arg += 1;
        match opt as lua_uint {
            0 => {
                /* signed integers */
                let mut n: lua_Integer = luaL_checkinteger(L, arg);
                if size < ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int {
                    /* need overflow check? */
                    lim = (1i32 as lua_Integer) << size * 8i32 - 1i32;
                    (-lim <= n && n < lim
                        || 0 != luaL_argerror(L, arg, s!(b"integer overflow\x00")))
                        as lua_int;
                }
                packint(
                    &mut b,
                    n as lua_Unsigned,
                    h.islittle,
                    size,
                    (n < 0i32 as lua_longlong) as lua_int,
                );
                continue;
            }
            1 => {
                /* unsigned integers */
                let mut n_0: lua_Integer = luaL_checkinteger(L, arg);
                /* need overflow check? */
                if size < ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int {
                    ((n_0 as lua_Unsigned) < (1i32 as lua_Unsigned) << size * 8i32
                        || 0 != luaL_argerror(L, arg, s!(b"unsigned overflow\x00")))
                        as lua_int;
                }
                packint(&mut b, n_0 as lua_Unsigned, h.islittle, size, 0i32);
                continue;
            }
            2 => {
                /* floating-point options */
                let mut u: Ftypes = Ftypes { f: 0. };
                let mut buff: *mut lua_char = luaL_prepbuffsize(&mut b, size as size_t);
                /* get argument */
                let mut n_1: lua_Number = luaL_checknumber(L, arg);
                if size as lua_ulong == ::std::mem::size_of::<lua_float>() as lua_ulong {
                    /* copy it into 'u' */
                    ::std::ptr::write_volatile(&mut u.f as *mut lua_float, n_1 as lua_float)
                } else if size as lua_ulong == ::std::mem::size_of::<lua_double>() as lua_ulong {
                    ::std::ptr::write_volatile(&mut u.d as *mut lua_double, n_1)
                } else {
                    ::std::ptr::write_volatile(&mut u.n as *mut lua_Number, n_1)
                }
                /* move 'u' to final result, correcting endianness if needed */
                copywithendian(buff as *mut lua_char, u.buff.as_mut_ptr(), size, h.islittle);
                b.n = (b.n as lua_ulong).wrapping_add(size as lua_ulong) as size_t as size_t;
                continue;
            }
            3 => {
                /* fixed-size string */
                let mut len: size_t = 0;
                let mut s: *const lua_char = luaL_checklstring(L, arg, &mut len);
                (len <= size as size_t
                    || 0 != luaL_argerror(L, arg, s!(b"string longer than given size\x00")))
                    as lua_int;
                /* add string */
                luaL_addlstring(&mut b, s, len);
                /* pad extra space */
                loop {
                    let fresh10 = len;
                    len = len.wrapping_add(1);
                    if !(fresh10 < size as size_t) {
                        continue 's_11;
                    }
                    (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null())
                        as lua_int;
                    let fresh11 = b.n;
                    b.n = b.n.wrapping_add(1);
                    *b.b.offset(fresh11 as isize) = 0i32 as lua_char
                }
            }
            4 => {
                /* strings with length count */
                let mut len_0: size_t = 0;
                let mut s_0: *const lua_char = luaL_checklstring(L, arg, &mut len_0);
                (size >= ::std::mem::size_of::<size_t>() as lua_ulong as lua_int
                    || len_0 < (1i32 as size_t) << size * 8i32
                    || 0 != luaL_argerror(
                        L,
                        arg,
                        s!(b"string length does not fit in given size\x00"),
                    )) as lua_int;
                /* pack length */
                packint(&mut b, len_0 as lua_Unsigned, h.islittle, size, 0i32);
                luaL_addlstring(&mut b, s_0, len_0);
                totalsize = (totalsize as lua_ulong).wrapping_add(len_0) as size_t as size_t;
                continue;
            }
            5 => {
                /* zero-terminated string */
                let mut len_1: size_t = 0;
                let mut s_1: *const lua_char = luaL_checklstring(L, arg, &mut len_1);
                (strlen(s_1) == len_1
                    || 0 != luaL_argerror(L, arg, s!(b"string contains zeros\x00")))
                    as lua_int;
                luaL_addlstring(&mut b, s_1, len_1);
                /* add zero at the end */
                (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
                let fresh12 = b.n;
                b.n = b.n.wrapping_add(1);
                *b.b.offset(fresh12 as isize) = '\u{0}' as i32 as lua_char;
                totalsize = (totalsize as lua_ulong)
                    .wrapping_add(len_1.wrapping_add(1i32 as lua_ulong))
                    as size_t as size_t;
                continue;
            }
            6 => {
                /* FALLTHROUGH */
                (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
                let fresh13 = b.n;
                b.n = b.n.wrapping_add(1);
                *b.b.offset(fresh13 as isize) = 0i32 as lua_char
            }
            7 | 8 => {}
            _ => {
                continue;
            }
        }
        /* undo increment */
        arg -= 1
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
/*
** Pack integer 'n' with 'size' bytes and 'islittle' endianness.
** The final 'if' handles the case when 'size' is larger than
** the size of a Lua integer, correcting the extra sign-extension
** bytes if necessary (by default they would be zeros).
*/
unsafe extern "C" fn packint(
    mut b: *mut luaL_Buffer,
    mut n: lua_Unsigned,
    mut islittle: lua_int,
    mut size: lua_int,
    mut neg: lua_int,
) -> () {
    let mut buff: *mut lua_char = luaL_prepbuffsize(b, size as size_t);
    let mut i: lua_int = 0;
    /* first byte */
    *buff.offset((if 0 != islittle { 0i32 } else { size - 1i32 }) as isize) =
        (n & ((1i32 << 8i32) - 1i32) as lua_ulonglong) as lua_char;
    i = 1i32;
    while i < size {
        n >>= 8i32;
        *buff.offset((if 0 != islittle { i } else { size - 1i32 - i }) as isize) =
            (n & ((1i32 << 8i32) - 1i32) as lua_ulonglong) as lua_char;
        i += 1
    }
    if 0 != neg && size > ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int {
        /* negative number need sign extension? */
        /* correct extra bytes */
        i = ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int;
        while i < size {
            *buff.offset((if 0 != islittle { i } else { size - 1i32 - i }) as isize) =
                ((1i32 << 8i32) - 1i32) as lua_char;
            i += 1
        }
    }
    /* add result to buffer */
    (*b).n = ((*b).n as lua_ulong).wrapping_add(size as lua_ulong) as size_t as size_t;
}
unsafe extern "C" fn str_upper(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut lua_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) = {
            let mut __c: lua_int = 0;
            let mut __res: lua_int = 0;
            if ::std::mem::size_of::<lua_uchar>() as lua_ulong > 1i32 as lua_ulong {
                if 0 != 0 {
                    __c = *s.offset(i as isize) as lua_uchar as lua_int;
                    __res = if __c < -128i32 || __c > 255i32 {
                        __c
                    } else {
                        toupper(__c)
                    }
                } else {
                    __res = toupper(*s.offset(i as isize) as lua_uchar as lua_int)
                }
            } else {
                __res = toupper(*s.offset(i as isize) as lua_uchar as lua_int)
            }
            __res
        } as lua_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
unsafe extern "C" fn str_sub(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut start: lua_Integer = posrelat(luaL_checkinteger(L, 2i32), l);
    let mut end: lua_Integer = posrelat(luaL_optinteger(L, 3i32, -1i32 as lua_Integer), l);
    if start < 1i32 as lua_longlong {
        start = 1i32 as lua_Integer
    }
    if end > l as lua_Integer {
        end = l as lua_Integer
    }
    if start <= end {
        lua_pushlstring(
            L,
            s.offset(start as isize).offset(-1isize),
            ((end - start) as size_t).wrapping_add(1i32 as lua_ulong),
        );
    } else {
        lua_pushstring(L, s!(b"\x00"));
    }
    return 1i32;
}
unsafe extern "C" fn str_reverse(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut lua_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) =
            *s.offset(l.wrapping_sub(i).wrapping_sub(1i32 as lua_ulong) as isize);
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
unsafe extern "C" fn str_rep(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut lsep: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut n: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut sep: *const lua_char = luaL_optlstring(L, 3i32, s!(b"\x00"), &mut lsep);
    if n <= 0i32 as lua_longlong {
        lua_pushstring(L, s!(b"\x00"));
    } else if l.wrapping_add(lsep) < l
        || l.wrapping_add(lsep) as lua_ulonglong
            > ((if (::std::mem::size_of::<size_t>() as lua_ulong)
                < ::std::mem::size_of::<lua_int>() as lua_ulong
            {
                !(0i32 as size_t)
            } else {
                2147483647i32 as size_t
            }) as lua_ulonglong)
                .wrapping_div(n as lua_ulonglong)
    {
        return luaL_error!(L, s!(b"resulting string too large\x00"));
    } else {
        let mut totallen: size_t = (n as size_t)
            .wrapping_mul(l)
            .wrapping_add(((n - 1i32 as lua_longlong) as size_t).wrapping_mul(lsep));
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut lua_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        let mut p: *mut lua_char = luaL_buffinitsize(L, &mut b, totallen);
        loop {
            let fresh14 = n;
            n = n - 1;
            if !(fresh14 > 1i32 as lua_longlong) {
                break;
            }
            /* first n-1 copies (followed by separator) */
            memcpy(
                p as *mut lua_void,
                s as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            p = p.offset(l as isize);
            if !(lsep > 0i32 as lua_ulong) {
                continue;
            }
            /* empty 'memcpy' is not that cheap */
            memcpy(
                p as *mut lua_void,
                sep as *const lua_void,
                lsep.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            p = p.offset(lsep as isize)
        }
        /* last copy (not followed by separator) */
        memcpy(
            p as *mut lua_void,
            s as *const lua_void,
            l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
        luaL_pushresultsize(&mut b, totallen);
    }
    return 1i32;
}
unsafe extern "C" fn str_match(mut L: *mut lua_State) -> lua_int {
    return str_find_aux(L, 0i32);
}
unsafe extern "C" fn str_find_aux(mut L: *mut lua_State, mut find: lua_int) -> lua_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut ls);
    let mut p: *const lua_char = luaL_checklstring(L, 2i32, &mut lp);
    let mut init: lua_Integer = posrelat(luaL_optinteger(L, 3i32, 1i32 as lua_Integer), ls);
    if init < 1i32 as lua_longlong {
        init = 1i32 as lua_Integer
    } else if init > ls as lua_Integer + 1i32 as lua_longlong {
        /* start after string's end? */
        /* cannot find anything */
        lua_pushnil(L);
        return 1i32;
    }
    /* explicit request or no special characters? */
    if 0 != find && (0 != lua_toboolean(L, 4i32) || 0 != nospecials(p, lp)) {
        /* do a plain search */
        let mut s2: *const lua_char = lmemfind(
            s.offset(init as isize).offset(-1isize),
            ls.wrapping_sub(init as size_t)
                .wrapping_add(1i32 as lua_ulong),
            p,
            lp,
        );
        if !s2.is_null() {
            lua_pushinteger(
                L,
                (s2.wrapping_offset_from(s) as lua_long + 1i32 as lua_long) as lua_Integer,
            );
            lua_pushinteger(
                L,
                (s2.wrapping_offset_from(s) as lua_long as lua_ulong).wrapping_add(lp)
                    as lua_Integer,
            );
            return 2i32;
        }
    } else {
        let mut ms: MatchState = MatchState {
            src_init: 0 as *const lua_char,
            src_end: 0 as *const lua_char,
            p_end: 0 as *const lua_char,
            L: 0 as *mut lua_State,
            matchdepth: 0,
            level: 0,
            capture: [unnamed_1 {
                init: 0 as *const lua_char,
                len: 0,
            }; 32],
        };
        let mut s1: *const lua_char = s.offset(init as isize).offset(-1isize);
        let mut anchor: lua_int = (*p as lua_int == '^' as i32) as lua_int;
        if 0 != anchor {
            p = p.offset(1isize);
            /* skip anchor character */
            lp = lp.wrapping_sub(1)
        }
        prepstate(&mut ms, L, s, ls, p, lp);
        loop {
            let mut res: *const lua_char = 0 as *const lua_char;
            reprepstate(&mut ms);
            res = match_0(&mut ms, s1, p);
            if !res.is_null() {
                if 0 != find {
                    /* start */
                    lua_pushinteger(
                        L,
                        (s1.wrapping_offset_from(s) as lua_long + 1i32 as lua_long) as lua_Integer,
                    );
                    /* end */
                    lua_pushinteger(L, res.wrapping_offset_from(s) as lua_long as lua_Integer);
                    return push_captures(&mut ms, 0 as *const lua_char, 0 as *const lua_char)
                        + 2i32;
                } else {
                    return push_captures(&mut ms, s1, res);
                }
            } else {
                let fresh15 = s1;
                s1 = s1.offset(1);
                if !(fresh15 < ms.src_end && 0 == anchor) {
                    break;
                }
            }
        }
    }
    /* not found */
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn push_captures(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut e: *const lua_char,
) -> lua_int {
    let mut i: lua_int = 0;
    let mut nlevels: lua_int = if (*ms).level as lua_int == 0i32 && !s.is_null() {
        1i32
    } else {
        (*ms).level as lua_int
    };
    luaL_checkstack((*ms).L, nlevels, s!(b"too many captures\x00"));
    i = 0i32;
    while i < nlevels {
        push_onecapture(ms, i, s, e);
        i += 1
    }
    /* number of strings pushed */
    return nlevels;
}
unsafe extern "C" fn push_onecapture(
    mut ms: *mut MatchState,
    mut i: lua_int,
    mut s: *const lua_char,
    mut e: *const lua_char,
) -> () {
    if i >= (*ms).level as lua_int {
        /* ms->level == 0, too */
        if i == 0i32 {
            /* add whole match */
            lua_pushlstring((*ms).L, s, e.wrapping_offset_from(s) as lua_long as size_t);
        } else {
            luaL_error!((*ms).L, s!(b"invalid capture index %%%d\x00"), i + 1i32);
        }
    } else {
        let mut l: ptrdiff_t = (*ms).capture[i as usize].len;
        if l == -1i32 as lua_long {
            luaL_error!((*ms).L, s!(b"unfinished capture\x00"));
        }
        if l == -2i32 as lua_long {
            lua_pushinteger(
                (*ms).L,
                ((*ms).capture[i as usize]
                    .init
                    .wrapping_offset_from((*ms).src_init) as lua_long
                    + 1i32 as lua_long) as lua_Integer,
            );
        } else {
            lua_pushlstring((*ms).L, (*ms).capture[i as usize].init, l as size_t);
        }
    };
}
/* recursive function */
unsafe extern "C" fn match_0(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
) -> *const lua_char {
    let mut ep_0: *const lua_char = 0 as *const lua_char;
    let mut current_block: u64;
    let fresh16 = (*ms).matchdepth;
    (*ms).matchdepth = (*ms).matchdepth - 1;
    if fresh16 == 0i32 {
        luaL_error!((*ms).L, s!(b"pattern too complex\x00"));
    }
    /* using goto's to optimize tail recursion */
    loop {
        if !(p != (*ms).p_end) {
            current_block = 11174649648027449784;
            break;
        }
        /* end of pattern? */
        match *p as lua_int {
            40 => {
                /* start capture */
                /* position capture? */
                if *p.offset(1isize) as lua_int == ')' as i32 {
                    current_block = 12675440807659640239;
                    break;
                } else {
                    current_block = 16658872821858055392;
                    break;
                }
            }
            41 => {
                /* end capture */
                s = end_capture(ms, s, p.offset(1isize));
                current_block = 11174649648027449784;
                break;
            }
            36 => {
                /* is the '$' the last char in pattern? */
                if !(p.offset(1isize) != (*ms).p_end) {
                    /* no; go to default */
                    /* check end of string */
                    s = if s == (*ms).src_end {
                        s
                    } else {
                        0 as *const lua_char
                    };
                    current_block = 11174649648027449784;
                    break;
                }
            }
            37 => {
                /* escaped sequences not in the format class[*+?-]? */
                match *p.offset(1isize) as lua_int {
                    98 => {
                        current_block = 16769867817448547787;
                        match current_block {
                            16769867817448547787 => {
                                /* balanced string? */
                                s = matchbalance(ms, s, p.offset(2isize));
                                if !s.is_null() {
                                    p = p.offset(4isize);
                                    /* return match(ms, s, p + 4); */
                                    continue;
                                } else {
                                    /* else fail (s == NULL) */
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            7458319281600708569 => {
                                /* frontier? */
                                let mut ep: *const lua_char = 0 as *const lua_char;
                                let mut previous: lua_char = 0;
                                p = p.offset(2isize);
                                if *p as lua_int != '[' as i32 {
                                    luaL_error!(
                                        (*ms).L,
                                        s!(b"missing \'[\' after \'%%f\' in pattern\x00"),
                                    );
                                }
                                /* points to what is next */
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\u{0}' as i32
                                } else {
                                    *s.offset(-1isize) as lua_int
                                }) as lua_char;
                                if 0 == matchbracketclass(
                                    previous as lua_uchar as lua_int,
                                    p,
                                    ep.offset(-1isize),
                                ) && 0
                                    != matchbracketclass(
                                        *s as lua_uchar as lua_int,
                                        p,
                                        ep.offset(-1isize),
                                    ) {
                                    p = ep;
                                    /* return match(ms, s, ep); */
                                    continue;
                                } else {
                                    /* match failed */
                                    s = 0 as *const lua_char;
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            _ => {
                                /* capture results (%0-%9)? */
                                s = match_capture(ms, s, *p.offset(1isize) as lua_uchar as lua_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break;
                                }
                                p = p.offset(2isize);
                                /* return match(ms, s, p + 2) */
                                continue;
                            }
                        }
                    }
                    102 => {
                        current_block = 7458319281600708569;
                        match current_block {
                            16769867817448547787 => {
                                /* balanced string? */
                                s = matchbalance(ms, s, p.offset(2isize));
                                if !s.is_null() {
                                    p = p.offset(4isize);
                                    /* return match(ms, s, p + 4); */
                                    continue;
                                } else {
                                    /* else fail (s == NULL) */
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            7458319281600708569 => {
                                /* frontier? */
                                let mut ep: *const lua_char = 0 as *const lua_char;
                                let mut previous: lua_char = 0;
                                p = p.offset(2isize);
                                if *p as lua_int != '[' as i32 {
                                    luaL_error!(
                                        (*ms).L,
                                        s!(b"missing \'[\' after \'%%f\' in pattern\x00"),
                                    );
                                }
                                /* points to what is next */
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\u{0}' as i32
                                } else {
                                    *s.offset(-1isize) as lua_int
                                }) as lua_char;
                                if 0 == matchbracketclass(
                                    previous as lua_uchar as lua_int,
                                    p,
                                    ep.offset(-1isize),
                                ) && 0
                                    != matchbracketclass(
                                        *s as lua_uchar as lua_int,
                                        p,
                                        ep.offset(-1isize),
                                    ) {
                                    p = ep;
                                    /* return match(ms, s, ep); */
                                    continue;
                                } else {
                                    /* match failed */
                                    s = 0 as *const lua_char;
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            _ => {
                                /* capture results (%0-%9)? */
                                s = match_capture(ms, s, *p.offset(1isize) as lua_uchar as lua_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break;
                                }
                                p = p.offset(2isize);
                                /* return match(ms, s, p + 2) */
                                continue;
                            }
                        }
                    }
                    48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                        current_block = 12405266282655138779;
                        match current_block {
                            16769867817448547787 => {
                                /* balanced string? */
                                s = matchbalance(ms, s, p.offset(2isize));
                                if !s.is_null() {
                                    p = p.offset(4isize);
                                    /* return match(ms, s, p + 4); */
                                    continue;
                                } else {
                                    /* else fail (s == NULL) */
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            7458319281600708569 => {
                                /* frontier? */
                                let mut ep: *const lua_char = 0 as *const lua_char;
                                let mut previous: lua_char = 0;
                                p = p.offset(2isize);
                                if *p as lua_int != '[' as i32 {
                                    luaL_error!(
                                        (*ms).L,
                                        s!(b"missing \'[\' after \'%%f\' in pattern\x00"),
                                    );
                                }
                                /* points to what is next */
                                ep = classend(ms, p);
                                previous = (if s == (*ms).src_init {
                                    '\u{0}' as i32
                                } else {
                                    *s.offset(-1isize) as lua_int
                                }) as lua_char;
                                if 0 == matchbracketclass(
                                    previous as lua_uchar as lua_int,
                                    p,
                                    ep.offset(-1isize),
                                ) && 0
                                    != matchbracketclass(
                                        *s as lua_uchar as lua_int,
                                        p,
                                        ep.offset(-1isize),
                                    ) {
                                    p = ep;
                                    /* return match(ms, s, ep); */
                                    continue;
                                } else {
                                    /* match failed */
                                    s = 0 as *const lua_char;
                                    current_block = 11174649648027449784;
                                    break;
                                }
                            }
                            _ => {
                                /* capture results (%0-%9)? */
                                s = match_capture(ms, s, *p.offset(1isize) as lua_uchar as lua_int);
                                if s.is_null() {
                                    current_block = 11174649648027449784;
                                    break;
                                }
                                p = p.offset(2isize);
                                /* return match(ms, s, p + 2) */
                                continue;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        /* pattern class plus optional suffix */
        /* points to optional suffix */
        ep_0 = classend(ms, p);
        /* does not match at least once? */
        if 0 == singlematch(ms, s, p, ep_0) {
            if *ep_0 as lua_int == '*' as i32
                || *ep_0 as lua_int == '?' as i32
                || *ep_0 as lua_int == '-' as i32
            {
                /* accept empty? */
                p = ep_0.offset(1isize)
            } else {
                /* return match(ms, s, ep + 1); */
                /* '+' or no suffix */
                /* fail */
                s = 0 as *const lua_char;
                current_block = 11174649648027449784;
                break;
            }
        } else {
            match *ep_0 as lua_int {
                63 => {
                    /* optional */
                    let mut res: *const lua_char = 0 as *const lua_char;
                    res = match_0(ms, s.offset(1isize), ep_0.offset(1isize));
                    if !res.is_null() {
                        s = res;
                        current_block = 11174649648027449784;
                        break;
                    } else {
                        p = ep_0.offset(1isize)
                    }
                }
                43 => {
                    /* else return match(ms, s, ep + 1); */
                    /* 1 match already done */
                    s = s.offset(1isize);
                    /* FALLTHROUGH */
                    /* 0 or more repetitions */
                    current_block = 3794797436593513832;
                    break;
                }
                42 => {
                    current_block = 3794797436593513832;
                    break;
                }
                45 => {
                    s = min_expand(ms, s, p, ep_0);
                    current_block = 11174649648027449784;
                    break;
                }
                _ => {
                    s = s.offset(1isize);
                    p = ep_0
                }
            }
        }
    }
    /* return match(ms, s + 1, ep); */
    match current_block {
        3794797436593513832 => s = max_expand(ms, s, p, ep_0),
        16658872821858055392 => s = start_capture(ms, s, p.offset(1isize), -1i32),
        12675440807659640239 => s = start_capture(ms, s, p.offset(2isize), -2i32),
        _ => {}
    }
    (*ms).matchdepth += 1;
    return s;
}
unsafe extern "C" fn classend(mut ms: *mut MatchState, mut p: *const lua_char) -> *const lua_char {
    let fresh17 = p;
    p = p.offset(1);
    match *fresh17 as lua_int {
        37 => {
            if p == (*ms).p_end {
                luaL_error!((*ms).L, s!(b"malformed pattern (ends with \'%%\')\x00"));
            }
            return p.offset(1isize);
        }
        91 => {
            if *p as lua_int == '^' as i32 {
                p = p.offset(1isize)
            }
            loop {
                /* look for a ']' */
                if p == (*ms).p_end {
                    luaL_error!((*ms).L, s!(b"malformed pattern (missing \']\')\x00"));
                }
                let fresh18 = p;
                p = p.offset(1);
                if *fresh18 as lua_int == '%' as i32 && p < (*ms).p_end {
                    /* skip escapes (e.g. '%]') */
                    p = p.offset(1isize)
                }
                if !(*p as lua_int != ']' as i32) {
                    break;
                }
            }
            return p.offset(1isize);
        }
        _ => return p,
    };
}
unsafe extern "C" fn min_expand(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
    mut ep: *const lua_char,
) -> *const lua_char {
    loop {
        let mut res: *const lua_char = match_0(ms, s, ep.offset(1isize));
        if !res.is_null() {
            return res;
        } else if 0 != singlematch(ms, s, p, ep) {
            /* try with one more repetition */
            s = s.offset(1isize)
        } else {
            return 0 as *const lua_char;
        }
    }
}
unsafe extern "C" fn singlematch(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
    mut ep: *const lua_char,
) -> lua_int {
    if s >= (*ms).src_end {
        return 0i32;
    } else {
        let mut c: lua_int = *s as lua_uchar as lua_int;
        match *p as lua_int {
            46 => {
                /* matches any char */
                return 1i32;
            }
            37 => return match_class(c, *p.offset(1isize) as lua_uchar as lua_int),
            91 => return matchbracketclass(c, p, ep.offset(-1isize)),
            _ => return (*p as lua_uchar as lua_int == c) as lua_int,
        }
    };
}
unsafe extern "C" fn matchbracketclass(
    mut c: lua_int,
    mut p: *const lua_char,
    mut ec: *const lua_char,
) -> lua_int {
    let mut sig: lua_int = 1i32;
    if *p.offset(1isize) as lua_int == '^' as i32 {
        sig = 0i32;
        /* skip the '^' */
        p = p.offset(1isize)
    }
    loop {
        p = p.offset(1isize);
        if !(p < ec) {
            break;
        }
        if *p as lua_int == '%' as i32 {
            p = p.offset(1isize);
            if !(0 != match_class(c, *p as lua_uchar as lua_int)) {
                continue;
            }
            return sig;
        } else if *p.offset(1isize) as lua_int == '-' as i32 && p.offset(2isize) < ec {
            p = p.offset(2isize);
            if !(*p.offset(-2isize) as lua_uchar as lua_int <= c && c <= *p as lua_uchar as lua_int)
            {
                continue;
            }
            return sig;
        } else {
            if !(*p as lua_uchar as lua_int == c) {
                continue;
            }
            return sig;
        }
    }
    return (0 == sig) as lua_int;
}
unsafe extern "C" fn match_class(mut c: lua_int, mut cl: lua_int) -> lua_int {
    let mut res: lua_int = 0;
    match {
        let mut __c: lua_int = 0;
        let mut __res: lua_int = 0;
        if ::std::mem::size_of::<lua_int>() as lua_ulong > 1i32 as lua_ulong {
            if 0 != 0 {
                __c = cl;
                __res = if __c < -128i32 || __c > 255i32 {
                    __c
                } else {
                    tolower(__c)
                }
            } else {
                __res = tolower(cl)
            }
        } else {
            __res = tolower(cl)
        }
        __res
    } {
        97 => res = isalpha(c),
        99 => res = iscntrl(c),
        100 => res = isdigit(c),
        103 => res = isgraph(c),
        108 => res = islower(c),
        112 => res = ispunct(c),
        115 => res = isspace(c),
        117 => res = isupper(c),
        119 => res = isalnum(c),
        120 => res = isxdigit(c),
        122 => res = (c == 0i32) as lua_int,
        _ => {
            /* deprecated option */
            return (cl == c) as lua_int;
        }
    }
    return if 0 != islower(cl) {
        res
    } else {
        (0 == res) as lua_int
    };
}
unsafe extern "C" fn max_expand(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
    mut ep: *const lua_char,
) -> *const lua_char {
    /* counts maximum expand for item */
    let mut i: ptrdiff_t = 0i32 as ptrdiff_t;
    while 0 != singlematch(ms, s.offset(i as isize), p, ep) {
        i += 1
    }
    /* keeps trying to match with the maximum repetitions */
    while i >= 0i32 as lua_long {
        let mut res: *const lua_char = match_0(ms, s.offset(i as isize), ep.offset(1isize));
        if !res.is_null() {
            return res;
        } else {
            i -= 1
        }
    }
    return 0 as *const lua_char;
}
unsafe extern "C" fn match_capture(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut l: lua_int,
) -> *const lua_char {
    let mut len: size_t = 0;
    l = check_capture(ms, l);
    len = (*ms).capture[l as usize].len as size_t;
    if (*ms).src_end.wrapping_offset_from(s) as lua_long as size_t >= len
        && memcmp(
            (*ms).capture[l as usize].init as *const lua_void,
            s as *const lua_void,
            len,
        ) == 0i32
    {
        return s.offset(len as isize);
    } else {
        return 0 as *const lua_char;
    };
}
/* maximum recursion depth for 'match' */
unsafe extern "C" fn check_capture(mut ms: *mut MatchState, mut l: lua_int) -> lua_int {
    l -= '1' as i32;
    if l < 0i32 || l >= (*ms).level as lua_int || (*ms).capture[l as usize].len == -1i32 as lua_long
    {
        return luaL_error!((*ms).L, s!(b"invalid capture index %%%d\x00"), l + 1i32);
    } else {
        return l;
    };
}
unsafe extern "C" fn matchbalance(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
) -> *const lua_char {
    if p >= (*ms).p_end.offset(-1isize) {
        luaL_error!(
            (*ms).L,
            s!(b"malformed pattern (missing arguments to \'%%b\')\x00"),
        );
    }
    if *s as lua_int != *p as lua_int {
        return 0 as *const lua_char;
    } else {
        let mut b: lua_int = *p as lua_int;
        let mut e: lua_int = *p.offset(1isize) as lua_int;
        let mut cont: lua_int = 1i32;
        loop {
            s = s.offset(1isize);
            if !(s < (*ms).src_end) {
                break;
            }
            if *s as lua_int == e {
                cont -= 1;
                if !(cont == 0i32) {
                    continue;
                }
                return s.offset(1isize);
            } else {
                if !(*s as lua_int == b) {
                    continue;
                }
                cont += 1
            }
        }
        /* string ends out of balance */
        return 0 as *const lua_char;
    };
}
unsafe extern "C" fn end_capture(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
) -> *const lua_char {
    let mut l: lua_int = capture_to_close(ms);
    let mut res: *const lua_char = 0 as *const lua_char;
    /* close capture */
    (*ms).capture[l as usize].len =
        s.wrapping_offset_from((*ms).capture[l as usize].init) as lua_long;
    /* match failed? */
    res = match_0(ms, s, p);
    if res.is_null() {
        /* undo capture */
        (*ms).capture[l as usize].len = -1i32 as ptrdiff_t
    }
    return res;
}
unsafe extern "C" fn capture_to_close(mut ms: *mut MatchState) -> lua_int {
    let mut level: lua_int = (*ms).level as lua_int;
    level -= 1;
    while level >= 0i32 {
        if (*ms).capture[level as usize].len == -1i32 as lua_long {
            return level;
        } else {
            level -= 1
        }
    }
    return luaL_error!((*ms).L, s!(b"invalid pattern capture\x00"));
}
unsafe extern "C" fn start_capture(
    mut ms: *mut MatchState,
    mut s: *const lua_char,
    mut p: *const lua_char,
    mut what: lua_int,
) -> *const lua_char {
    let mut res: *const lua_char = 0 as *const lua_char;
    let mut level: lua_int = (*ms).level as lua_int;
    if level >= 32i32 {
        luaL_error!((*ms).L, s!(b"too many captures\x00"));
    }
    (*ms).capture[level as usize].init = s;
    (*ms).capture[level as usize].len = what as ptrdiff_t;
    (*ms).level = (level + 1i32) as lua_uchar;
    /* match failed? */
    res = match_0(ms, s, p);
    if res.is_null() {
        /* undo capture */
        (*ms).level = (*ms).level.wrapping_sub(1)
    }
    return res;
}
unsafe extern "C" fn reprepstate(mut ms: *mut MatchState) -> () {
    (*ms).level = 0i32 as lua_uchar;
}
unsafe extern "C" fn prepstate(
    mut ms: *mut MatchState,
    mut L: *mut lua_State,
    mut s: *const lua_char,
    mut ls: size_t,
    mut p: *const lua_char,
    mut lp: size_t,
) -> () {
    (*ms).L = L;
    (*ms).matchdepth = 200i32;
    (*ms).src_init = s;
    (*ms).src_end = s.offset(ls as isize);
    (*ms).p_end = p.offset(lp as isize);
}
unsafe extern "C" fn lmemfind(
    mut s1: *const lua_char,
    mut l1: size_t,
    mut s2: *const lua_char,
    mut l2: size_t,
) -> *const lua_char {
    if l2 == 0i32 as lua_ulong {
        /* empty strings are everywhere */
        return s1;
    } else if l2 > l1 {
        /* avoids a negative 'l1' */
        return 0 as *const lua_char;
    } else {
        /* to search for a '*s2' inside 's1' */
        let mut init: *const lua_char = 0 as *const lua_char;
        /* 1st char will be checked by 'memchr' */
        l2 = l2.wrapping_sub(1);
        /* 's2' cannot be found after that */
        l1 = l1.wrapping_sub(l2);
        while l1 > 0i32 as lua_ulong && {
            init = memchr(s1 as *const lua_void, *s2 as lua_int, l1) as *const lua_char;
            !init.is_null()
        } {
            /* 1st char is already checked */
            init = init.offset(1isize);
            if memcmp(
                init as *const lua_void,
                s2.offset(1isize) as *const lua_void,
                l2,
            ) == 0i32
            {
                return init.offset(-1isize);
            } else {
                /* correct 'l1' and 's1' to try again */
                l1 = (l1 as lua_ulong)
                    .wrapping_sub(init.wrapping_offset_from(s1) as lua_long as lua_ulong)
                    as size_t as size_t;
                s1 = init
            }
        }
        /* not found */
        return 0 as *const lua_char;
    };
}
/* check whether pattern has no special characters */
unsafe extern "C" fn nospecials(mut p: *const lua_char, mut l: size_t) -> lua_int {
    let mut upto: size_t = 0i32 as size_t;
    loop {
        if !strpbrk(p.offset(upto as isize), s!(b"^$*+?.([%-\x00")).is_null() {
            /* pattern has a special character */
            return 0i32;
        } else {
            /* may have more after \0 */
            upto = (upto as lua_ulong)
                .wrapping_add(strlen(p.offset(upto as isize)).wrapping_add(1i32 as lua_ulong))
                as size_t as size_t;
            if !(upto <= l) {
                break;
            }
        }
    }
    /* no special chars found */
    return 1i32;
}
unsafe extern "C" fn str_lower(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut p: *mut lua_char = luaL_buffinitsize(L, &mut b, l);
    i = 0i32 as size_t;
    while i < l {
        *p.offset(i as isize) = {
            let mut __c: lua_int = 0;
            let mut __res: lua_int = 0;
            if ::std::mem::size_of::<lua_uchar>() as lua_ulong > 1i32 as lua_ulong {
                if 0 != 0 {
                    __c = *s.offset(i as isize) as lua_uchar as lua_int;
                    __res = if __c < -128i32 || __c > 255i32 {
                        __c
                    } else {
                        tolower(__c)
                    }
                } else {
                    __res = tolower(*s.offset(i as isize) as lua_uchar as lua_int)
                }
            } else {
                __res = tolower(*s.offset(i as isize) as lua_uchar as lua_int)
            }
            __res
        } as lua_char;
        i = i.wrapping_add(1)
    }
    luaL_pushresultsize(&mut b, l);
    return 1i32;
}
/*
** $Id: lstrlib.c,v 1.254.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard library for string operations and pattern-matching
** See Copyright Notice in lua.h
*/
/*
** maximum number of captures that a pattern can do during
** pattern-matching. This limit is arbitrary, but must fit in
** an unsigned char.
*/
/* macro to 'unsign' a character */
/*
** Some sizes are better limited to fit in 'int', but must also fit in
** 'size_t'. (We assume that 'lua_Integer' cannot be smaller than 'int'.)
*/
unsafe extern "C" fn str_len(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    luaL_checklstring(L, 1i32, &mut l);
    lua_pushinteger(L, l as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn str_gsub(mut L: *mut lua_State) -> lua_int {
    let mut srcl: size_t = 0;
    let mut lp: size_t = 0;
    /* subject */
    let mut src: *const lua_char = luaL_checklstring(L, 1i32, &mut srcl);
    /* pattern */
    let mut p: *const lua_char = luaL_checklstring(L, 2i32, &mut lp);
    /* end of last match */
    let mut lastmatch: *const lua_char = 0 as *const lua_char;
    /* replacement type */
    let mut tr: lua_int = lua_type(L, 3i32);
    /* max replacements */
    let mut max_s: lua_Integer =
        luaL_optinteger(L, 4i32, srcl.wrapping_add(1i32 as lua_ulong) as lua_Integer);
    let mut anchor: lua_int = (*p as lua_int == '^' as i32) as lua_int;
    /* replacement count */
    let mut n: lua_Integer = 0i32 as lua_Integer;
    let mut ms: MatchState = MatchState {
        src_init: 0 as *const lua_char,
        src_end: 0 as *const lua_char,
        p_end: 0 as *const lua_char,
        L: 0 as *mut lua_State,
        matchdepth: 0,
        level: 0,
        capture: [unnamed_1 {
            init: 0 as *const lua_char,
            len: 0,
        }; 32],
    };
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    (tr == 3i32
        || tr == 4i32
        || tr == 6i32
        || tr == 5i32
        || 0 != luaL_argerror(L, 3i32, s!(b"string/function/table expected\x00"))) as lua_int;
    luaL_buffinit(L, &mut b);
    if 0 != anchor {
        p = p.offset(1isize);
        /* skip anchor character */
        lp = lp.wrapping_sub(1)
    }
    prepstate(&mut ms, L, src, srcl, p, lp);
    while n < max_s {
        let mut e: *const lua_char = 0 as *const lua_char;
        /* (re)prepare state for new match */
        reprepstate(&mut ms);
        e = match_0(&mut ms, src, p);
        if !e.is_null() && e != lastmatch {
            /* match? */
            n += 1;
            /* add replacement to buffer */
            add_value(&mut ms, &mut b, src, e, tr);
            lastmatch = e;
            src = lastmatch
        } else if src < ms.src_end {
            (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
            let fresh20 = b.n;
            b.n = b.n.wrapping_add(1);
            let fresh19 = src;
            src = src.offset(1);
            *b.b.offset(fresh20 as isize) = *fresh19
        } else {
            /* end of subject */
            break;
        }
        if 0 != anchor {
            break;
        }
    }
    luaL_addlstring(
        &mut b,
        src,
        ms.src_end.wrapping_offset_from(src) as lua_long as size_t,
    );
    luaL_pushresult(&mut b);
    /* number of substitutions */
    lua_pushinteger(L, n);
    return 2i32;
}
unsafe extern "C" fn add_value(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const lua_char,
    mut e: *const lua_char,
    mut tr: lua_int,
) -> () {
    let mut L: *mut lua_State = (*ms).L;
    match tr {
        6 => {
            let mut n: lua_int = 0;
            lua_pushvalue(L, 3i32);
            n = push_captures(ms, s, e);
            lua_callk(L, n, 1i32, 0i32 as lua_KContext, None);
        }
        5 => {
            push_onecapture(ms, 0i32, s, e);
            lua_gettable(L, 3i32);
        }
        _ => {
            /* LUA_TNUMBER or LUA_TSTRING */
            add_s(ms, b, s, e);
            return;
        }
    }
    if 0 == lua_toboolean(L, -1i32) {
        /* nil or false? */
        lua_settop(L, -1i32 - 1i32);
        /* keep original text */
        lua_pushlstring(L, s, e.wrapping_offset_from(s) as lua_long as size_t);
    } else if 0 == lua_isstring(L, -1i32) {
        luaL_error!(
            L,
            s!(b"invalid replacement value (a %s)\x00"),
            lua_typename(L, lua_type(L, -1i32)),
        );
    }
    /* add result to accumulator */
    luaL_addvalue(b);
}
unsafe extern "C" fn add_s(
    mut ms: *mut MatchState,
    mut b: *mut luaL_Buffer,
    mut s: *const lua_char,
    mut e: *const lua_char,
) -> () {
    let mut l: size_t = 0;
    let mut i: size_t = 0;
    let mut L: *mut lua_State = (*ms).L;
    let mut news: *const lua_char = lua_tolstring(L, 3i32, &mut l);
    i = 0i32 as size_t;
    while i < l {
        if *news.offset(i as isize) as lua_int != '%' as i32 {
            ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
            let fresh21 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh21 as isize) = *news.offset(i as isize)
        } else {
            /* skip ESC */
            i = i.wrapping_add(1);
            if 0 == isdigit(*news.offset(i as isize) as lua_uchar as lua_int) {
                if *news.offset(i as isize) as lua_int != '%' as i32 {
                    luaL_error!(
                        L,
                        s!(b"invalid use of \'%c\' in replacement string\x00"),
                        '%' as i32,
                    );
                }
                ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
                let fresh22 = (*b).n;
                (*b).n = (*b).n.wrapping_add(1);
                *(*b).b.offset(fresh22 as isize) = *news.offset(i as isize)
            } else if *news.offset(i as isize) as lua_int == '0' as i32 {
                luaL_addlstring(b, s, e.wrapping_offset_from(s) as lua_long as size_t);
            } else {
                push_onecapture(ms, *news.offset(i as isize) as lua_int - '1' as i32, s, e);
                /* if number, convert it to string */
                luaL_tolstring(L, -1i32, 0 as *mut size_t);
                /* remove original value */
                lua_rotate(L, -2i32, -1i32);
                lua_settop(L, -1i32 - 1i32);
                /* add capture to accumulated result */
                luaL_addvalue(b);
            }
        }
        i = i.wrapping_add(1)
    }
}
unsafe extern "C" fn gmatch(mut L: *mut lua_State) -> lua_int {
    let mut ls: size_t = 0;
    let mut lp: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut ls);
    let mut p: *const lua_char = luaL_checklstring(L, 2i32, &mut lp);
    let mut gm: *mut GMatchState = 0 as *mut GMatchState;
    /* keep them on closure to avoid being collected */
    lua_settop(L, 2i32);
    gm = lua_newuserdata(L, ::std::mem::size_of::<GMatchState>() as lua_ulong) as *mut GMatchState;
    prepstate(&mut (*gm).ms, L, s, ls, p, lp);
    (*gm).src = s;
    (*gm).p = p;
    (*gm).lastmatch = 0 as *const lua_char;
    lua_pushcclosure(L, Some(gmatch_aux), 3i32);
    return 1i32;
}
unsafe extern "C" fn gmatch_aux(mut L: *mut lua_State) -> lua_int {
    let mut gm: *mut GMatchState =
        lua_touserdata(L, -1000000i32 - 1000i32 - 3i32) as *mut GMatchState;
    let mut src: *const lua_char = 0 as *const lua_char;
    (*gm).ms.L = L;
    src = (*gm).src;
    while src <= (*gm).ms.src_end {
        let mut e: *const lua_char = 0 as *const lua_char;
        reprepstate(&mut (*gm).ms);
        e = match_0(&mut (*gm).ms, src, (*gm).p);
        if !e.is_null() && e != (*gm).lastmatch {
            (*gm).lastmatch = e;
            (*gm).src = (*gm).lastmatch;
            return push_captures(&mut (*gm).ms, src, e);
        } else {
            src = src.offset(1isize)
        }
    }
    /* not found */
    return 0i32;
}
unsafe extern "C" fn str_format(mut L: *mut lua_State) -> lua_int {
    let mut top: lua_int = lua_gettop(L);
    let mut arg: lua_int = 1i32;
    let mut sfl: size_t = 0;
    let mut strfrmt: *const lua_char = luaL_checklstring(L, arg, &mut sfl);
    let mut strfrmt_end: *const lua_char = strfrmt.offset(sfl as isize);
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    while strfrmt < strfrmt_end {
        if *strfrmt as lua_int != '%' as i32 {
            (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
            let fresh24 = b.n;
            b.n = b.n.wrapping_add(1);
            let fresh23 = strfrmt;
            strfrmt = strfrmt.offset(1);
            *b.b.offset(fresh24 as isize) = *fresh23
        } else {
            strfrmt = strfrmt.offset(1isize);
            if *strfrmt as lua_int == '%' as i32 {
                /* %% */
                (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
                let fresh26 = b.n;
                b.n = b.n.wrapping_add(1);
                let fresh25 = strfrmt;
                strfrmt = strfrmt.offset(1);
                *b.b.offset(fresh26 as isize) = *fresh25
            } else {
                /* format item */
                /* to store the format ('%...') */
                let mut form: [lua_char; 32] = [0; 32];
                /* to put formatted item */
                let mut buff: *mut lua_char =
                    luaL_prepbuffsize(&mut b, (120i32 + 308i32) as size_t);
                /* number of bytes in added item */
                let mut nb: lua_int = 0i32;
                arg += 1;
                if arg > top {
                    luaL_argerror(L, arg, s!(b"no value\x00"));
                }
                strfrmt = scanformat(L, strfrmt, form.as_mut_ptr());
                let fresh27 = strfrmt;
                strfrmt = strfrmt.offset(1);
                match *fresh27 as lua_int {
                    99 => {
                        nb = snprintf(
                            buff,
                            (120i32 + 308i32) as lua_ulong,
                            form.as_mut_ptr(),
                            luaL_checkinteger(L, arg) as lua_int,
                        )
                    }
                    100 | 105 | 111 | 117 | 120 | 88 => {
                        let mut n: lua_Integer = luaL_checkinteger(L, arg);
                        addlenmod(form.as_mut_ptr(), s!(b"ll\x00"));
                        nb = snprintf(buff, (120i32 + 308i32) as lua_ulong, form.as_mut_ptr(), n)
                    }
                    97 | 65 => {
                        addlenmod(form.as_mut_ptr(), s!(b"\x00"));
                        nb = snprintf(
                            buff,
                            (120i32 + 308i32) as lua_ulong,
                            form.as_mut_ptr(),
                            luaL_checknumber(L, arg),
                        )
                    }
                    101 | 69 | 102 | 103 | 71 => {
                        let mut n_0: lua_Number = luaL_checknumber(L, arg);
                        addlenmod(form.as_mut_ptr(), s!(b"\x00"));
                        nb = snprintf(buff, (120i32 + 308i32) as lua_ulong, form.as_mut_ptr(), n_0)
                    }
                    113 => {
                        addliteral(L, &mut b, arg);
                    }
                    115 => {
                        let mut l: size_t = 0;
                        let mut s: *const lua_char = luaL_tolstring(L, arg, &mut l);
                        /* no modifiers? */
                        if form[2usize] as lua_int == '\u{0}' as i32 {
                            /* keep entire string */
                            luaL_addvalue(&mut b);
                        } else {
                            (l == strlen(s)
                                || 0 != luaL_argerror(L, arg, s!(b"string contains zeros\x00")))
                                as lua_int;
                            if strchr(form.as_mut_ptr(), '.' as i32).is_null()
                                && l >= 100i32 as lua_ulong
                            {
                                /* no precision and string is too long to be formatted */
                                /* keep entire string */
                                luaL_addvalue(&mut b);
                            } else {
                                /* format the string into 'buff' */
                                nb = snprintf(
                                    buff,
                                    (120i32 + 308i32) as lua_ulong,
                                    form.as_mut_ptr(),
                                    s,
                                );
                                /* remove result from 'luaL_tolstring' */
                                lua_settop(L, -1i32 - 1i32);
                            }
                        }
                    }
                    _ => {
                        /* also treat cases 'pnLlh' */
                        return luaL_error!(
                            L,
                            s!(b"invalid option \'%%%c\' to \'format\'\x00"),
                            *strfrmt.offset(-1isize) as lua_int,
                        );
                    }
                }
                b.n = (b.n as lua_ulong).wrapping_add(nb as lua_ulong) as size_t as size_t
            }
        }
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn addliteral(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut arg: lua_int,
) -> () {
    let mut n: lua_Number = 0.;
    let mut n_0: lua_Integer = 0;
    let mut format: *const lua_char = 0 as *const lua_char;
    match lua_type(L, arg) {
        4 => {
            let mut len: size_t = 0;
            let mut s: *const lua_char = lua_tolstring(L, arg, &mut len);
            addquoted(b, s, len);
        }
        3 => {
            let mut buff: *mut lua_char = luaL_prepbuffsize(b, (120i32 + 308i32) as size_t);
            let mut nb: lua_int = 0;
            if 0 == lua_isinteger(L, arg) {
                /* float? */
                /* write as hexa ('%a') */
                n = lua_tonumberx(L, arg, 0 as *mut lua_int);
                nb = snprintf(buff, (120i32 + 308i32) as lua_ulong, s!(b"%a\x00"), n);
                /* ensure it uses a dot */
                checkdp(buff, nb);
            } else {
                /* integers */
                n_0 = lua_tointegerx(L, arg, 0 as *mut lua_int);
                /* corner case? */
                format = if n_0 == -9223372036854775807i64 - 1i64 {
                    s!(b"0x%llx\x00")
                } else {
                    s!(b"%lld\x00")
                };
                /* use hexa */
                /* else use default format */
                nb = snprintf(buff, (120i32 + 308i32) as lua_ulong, format, n_0)
            }
            (*b).n = ((*b).n as lua_ulong).wrapping_add(nb as lua_ulong) as size_t as size_t
        }
        0 | 1 => {
            luaL_tolstring(L, arg, 0 as *mut size_t);
            luaL_addvalue(b);
        }
        _ => {
            luaL_argerror(L, arg, s!(b"value has no literal form\x00"));
        }
    };
}
/*
** Ensures the 'buff' string uses a dot as the radix character.
*/
unsafe extern "C" fn checkdp(mut buff: *mut lua_char, mut nb: lua_int) -> () {
    if memchr(buff as *const lua_void, '.' as i32, nb as lua_ulong).is_null() {
        /* no dot? */
        /* try locale point */
        let mut point: lua_char = *(*localeconv()).decimal_point.offset(0isize);
        let mut ppoint: *mut lua_char =
            memchr(buff as *const lua_void, point as lua_int, nb as lua_ulong) as *mut lua_char;
        if !ppoint.is_null() {
            /* change it to a dot */
            *ppoint = '.' as i32 as lua_char
        }
    };
}
/* }====================================================== */
/*
** {======================================================
** STRING FORMAT
** =======================================================
*/
/* { */
/* } */
/*
** Maximum size of each formatted item. This maximum size is produced
** by format('%.99f', -maxfloat), and is equal to 99 + 3 ('-', '.',
** and '\0') + number of decimal digits to represent maxfloat (which
** is maximum exponent + 1). (99+3+1 then rounded to 120 for "extra
** expenses", such as locale-dependent stuff)
*/
/* valid flags in a format specification */
/*
** maximum size of each format specification (such as "%-099.99d")
*/
unsafe extern "C" fn addquoted(
    mut b: *mut luaL_Buffer,
    mut s: *const lua_char,
    mut len: size_t,
) -> () {
    ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
    let fresh28 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).b.offset(fresh28 as isize) = '\"' as i32 as lua_char;
    loop {
        let fresh29 = len;
        len = len.wrapping_sub(1);
        if !(0 != fresh29) {
            break;
        }
        if *s as lua_int == '\"' as i32
            || *s as lua_int == '\\' as i32
            || *s as lua_int == '\n' as i32
        {
            ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
            let fresh30 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh30 as isize) = '\\' as i32 as lua_char;
            ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
            let fresh31 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh31 as isize) = *s
        } else if 0 != iscntrl(*s as lua_uchar as lua_int) {
            let mut buff: [lua_char; 10] = [0; 10];
            if 0 == isdigit(*s.offset(1isize) as lua_uchar as lua_int) {
                snprintf(
                    buff.as_mut_ptr(),
                    ::std::mem::size_of::<[lua_char; 10]>() as lua_ulong,
                    s!(b"\\%d\x00"),
                    *s as lua_uchar as lua_int,
                );
            } else {
                snprintf(
                    buff.as_mut_ptr(),
                    ::std::mem::size_of::<[lua_char; 10]>() as lua_ulong,
                    s!(b"\\%03d\x00"),
                    *s as lua_uchar as lua_int,
                );
            }
            luaL_addstring(b, buff.as_mut_ptr());
        } else {
            ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
            let fresh32 = (*b).n;
            (*b).n = (*b).n.wrapping_add(1);
            *(*b).b.offset(fresh32 as isize) = *s
        }
        s = s.offset(1isize)
    }
    ((*b).n < (*b).size || !luaL_prepbuffsize(b, 1i32 as size_t).is_null()) as lua_int;
    let fresh33 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).b.offset(fresh33 as isize) = '\"' as i32 as lua_char;
}
/*
** add length modifier into formats
*/
unsafe extern "C" fn addlenmod(mut form: *mut lua_char, mut lenmod: *const lua_char) -> () {
    let mut l: size_t = strlen(form);
    let mut lm: size_t = strlen(lenmod);
    let mut spec: lua_char = *form.offset(l.wrapping_sub(1i32 as lua_ulong) as isize);
    strcpy(form.offset(l as isize).offset(-1isize), lenmod);
    *form.offset(l.wrapping_add(lm).wrapping_sub(1i32 as lua_ulong) as isize) = spec;
    *form.offset(l.wrapping_add(lm) as isize) = '\u{0}' as i32 as lua_char;
}
unsafe extern "C" fn scanformat(
    mut _L: *mut lua_State,
    mut strfrmt: *const lua_char,
    mut form: *mut lua_char,
) -> *const lua_char {
    let mut p: *const lua_char = strfrmt;
    while *p as lua_int != '\u{0}' as i32 && !strchr(s!(b"-+ #0\x00"), *p as lua_int).is_null() {
        /* skip flags */
        p = p.offset(1isize)
    }
    if p.wrapping_offset_from(strfrmt) as lua_long as size_t
        >= (::std::mem::size_of::<[lua_char; 6]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
    {
        luaL_error!(L, s!(b"invalid format (repeated flags)\x00"));
    }
    if 0 != isdigit(*p as lua_uchar as lua_int) {
        /* skip width */
        p = p.offset(1isize)
    }
    if 0 != isdigit(*p as lua_uchar as lua_int) {
        /* (2 digits at most) */
        p = p.offset(1isize)
    }
    if *p as lua_int == '.' as i32 {
        p = p.offset(1isize);
        if 0 != isdigit(*p as lua_uchar as lua_int) {
            /* skip precision */
            p = p.offset(1isize)
        }
        if 0 != isdigit(*p as lua_uchar as lua_int) {
            /* (2 digits at most) */
            p = p.offset(1isize)
        }
    }
    if 0 != isdigit(*p as lua_uchar as lua_int) {
        luaL_error!(L, s!(b"invalid format (width or precision too long)\x00"));
    }
    let fresh34 = form;
    form = form.offset(1);
    *fresh34 = '%' as i32 as lua_char;
    memcpy(
        form as *mut lua_void,
        strfrmt as *const lua_void,
        ((p.wrapping_offset_from(strfrmt) as lua_long + 1i32 as lua_long) as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    form = form.offset((p.wrapping_offset_from(strfrmt) as lua_long + 1i32 as lua_long) as isize);
    *form = '\u{0}' as i32 as lua_char;
    return p;
}
unsafe extern "C" fn str_find(mut L: *mut lua_State) -> lua_int {
    return str_find_aux(L, 1i32);
}
unsafe extern "C" fn str_dump(mut L: *mut lua_State) -> lua_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut strip: lua_int = lua_toboolean(L, 2i32);
    luaL_checktype(L, 1i32, 6i32);
    lua_settop(L, 1i32);
    luaL_buffinit(L, &mut b);
    if lua_dump(
        L,
        Some(writer),
        &mut b as *mut luaL_Buffer as *mut lua_void,
        strip,
    ) != 0i32
    {
        return luaL_error!(L, s!(b"unable to dump given function\x00"));
    } else {
        luaL_pushresult(&mut b);
        return 1i32;
    };
}
unsafe extern "C" fn writer(
    mut _L: *mut lua_State,
    mut b: *const lua_void,
    mut size: size_t,
    mut B: *mut lua_void,
) -> lua_int {
    luaL_addlstring(B as *mut luaL_Buffer, b as *const lua_char, size);
    return 0i32;
}
unsafe extern "C" fn str_char(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    let mut i: lua_int = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut p: *mut lua_char = luaL_buffinitsize(L, &mut b, n as size_t);
    i = 1i32;
    while i <= n {
        let mut c: lua_Integer = luaL_checkinteger(L, i);
        (c as lua_uchar as lua_longlong == c
            || 0 != luaL_argerror(L, i, s!(b"value out of range\x00"))) as lua_int;
        *p.offset((i - 1i32) as isize) = c as lua_uchar as lua_char;
        i += 1
    }
    luaL_pushresultsize(&mut b, n as size_t);
    return 1i32;
}
unsafe extern "C" fn str_byte(mut L: *mut lua_State) -> lua_int {
    let mut l: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut l);
    let mut posi: lua_Integer = posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), l);
    let mut pose: lua_Integer = posrelat(luaL_optinteger(L, 3i32, posi), l);
    let mut n: lua_int = 0;
    let mut i: lua_int = 0;
    if posi < 1i32 as lua_longlong {
        posi = 1i32 as lua_Integer
    }
    if pose > l as lua_Integer {
        pose = l as lua_Integer
    }
    if posi > pose {
        /* empty interval; return no values */
        return 0i32;
    } else if pose - posi >= 2147483647i32 as lua_longlong {
        return luaL_error!(L, s!(b"string slice too long\x00"));
    } else {
        n = (pose - posi) as lua_int + 1i32;
        luaL_checkstack(L, n, s!(b"string slice too long\x00"));
        i = 0i32;
        while i < n {
            lua_pushinteger(
                L,
                *s.offset((posi + i as lua_longlong - 1i32 as lua_longlong) as isize) as lua_uchar
                    as lua_Integer,
            );
            i += 1
        }
        return n;
    };
}
