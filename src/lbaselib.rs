use types::*;
use lua::*;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __ctype_b_loc() -> *mut *const lua_ushort;
    #[no_mangle]
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fwrite(__ptr: *const lua_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn strspn(_: *const lua_char, _: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: lua_int, n: lua_int) -> ();
    #[no_mangle]
    fn lua_copy(L: *mut lua_State, fromidx: lua_int, toidx: lua_int) -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, tp: lua_int) -> *const lua_char;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State, idx: lua_int) -> size_t;
    #[no_mangle]
    fn lua_rawequal(L: *mut lua_State, idx1: lua_int, idx2: lua_int) -> lua_int;
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
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();
    /*
     ** get functions (Lua -> stack)
     */
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const lua_char) -> lua_int;
    #[no_mangle]
    fn lua_geti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, idx: lua_int) -> ();
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
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: lua_int,
        nresults: lua_int,
        errfunc: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> lua_int;
    #[no_mangle]
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut lua_void,
        chunkname: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
    /*
     ** garbage-collection function and options
     */
    #[no_mangle]
    fn lua_gc(L: *mut lua_State, what: lua_int, data: lua_int) -> lua_int;
    /*
     ** miscellaneous functions
     */
    #[no_mangle]
    fn lua_error(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_next(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: lua_int) -> ();
    #[no_mangle]
    fn lua_stringtonumber(L: *mut lua_State, s: *const lua_char) -> size_t;
    #[no_mangle]
    fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: lua_int,
        n: lua_int,
    ) -> *const lua_char;
    #[no_mangle]
    fn luaL_getmetafield(
        L: *mut lua_State,
        obj: lua_int,
        e: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t)
        -> *const lua_char;
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: lua_int,
        extramsg: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: lua_int,
        def: *const lua_char,
        l: *mut size_t,
    ) -> *const lua_char;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State, arg: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: lua_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, sz: lua_int, msg: *const lua_char) -> ();
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State, arg: lua_int, t: lua_int) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, arg: lua_int) -> ();
    #[no_mangle]
    fn luaL_where(L: *mut lua_State, lvl: lua_int) -> ();
    #[no_mangle]
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: lua_int,
        def: *const lua_char,
        lst: *const *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const lua_char,
        sz: size_t,
        name: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
}
pub type __int32_t = lua_int;
pub type __off_t = lua_long;
pub type __off64_t = lua_long;
pub type unnamed = lua_uint;
pub const _ISalnum: unnamed = 8;
pub const _ISpunct: unnamed = 4;
pub const _IScntrl: unnamed = 2;
pub const _ISblank: unnamed = 1;
pub const _ISgraph: unnamed = 32768;
pub const _ISprint: unnamed = 16384;
pub const _ISspace: unnamed = 8192;
pub const _ISxdigit: unnamed = 4096;
pub const _ISdigit: unnamed = 2048;
pub const _ISalpha: unnamed = 1024;
pub const _ISlower: unnamed = 512;
pub const _ISupper: unnamed = 256;
pub type size_t = lua_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: lua_int,
    pub _IO_read_ptr: *mut lua_char,
    pub _IO_read_end: *mut lua_char,
    pub _IO_read_base: *mut lua_char,
    pub _IO_write_base: *mut lua_char,
    pub _IO_write_ptr: *mut lua_char,
    pub _IO_write_end: *mut lua_char,
    pub _IO_buf_base: *mut lua_char,
    pub _IO_buf_end: *mut lua_char,
    pub _IO_save_base: *mut lua_char,
    pub _IO_backup_base: *mut lua_char,
    pub _IO_save_end: *mut lua_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: lua_int,
    pub _flags2: lua_int,
    pub _old_offset: __off_t,
    pub _cur_column: lua_ushort,
    pub _vtable_offset: lua_schar,
    pub _shortbuf: [lua_char; 1],
    pub _lock: *mut lua_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut lua_void,
    pub __pad5: size_t,
    pub _mode: lua_int,
    pub _unused2: [lua_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type intptr_t = lua_long;
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
/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_void, _: *mut size_t)
        -> *const lua_char,
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
unsafe extern "C" fn toupper(mut __c: lua_int) -> lua_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
/*
** $Id: lualib.h,v 1.45.1.1 2017/04/19 17:20:42 roberto Exp $
** Lua standard libraries
** See Copyright Notice in lua.h
*/
/* version suffix for environment variable names */
#[no_mangle]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> lua_int {
    /* open lib into global table */
    lua_rawgeti(L, -1000000i32 - 1000i32, 2i32 as lua_Integer);
    luaL_setfuncs(L, base_funcs.as_ptr(), 0i32);
    /* set global _G */
    lua_pushvalue(L, -1i32);
    lua_setfield(L, -2i32, s!(b"_G\x00"));
    /* set global _VERSION */
    lua_pushstring(L, s!(b"Lua 5.3\x00"));
    lua_setfield(L, -2i32, s!(b"_VERSION\x00"));
    return 1i32;
}
static mut base_funcs: [luaL_Reg; 25] = [
    lua_reg!(b"assert\x00", luaB_assert),
    lua_reg!(b"collectgarbage\x00", luaB_collectgarbage),
    lua_reg!(b"dofile\x00", luaB_dofile),
    lua_reg!(b"error\x00", luaB_error),
    lua_reg!(b"getmetatable\x00", luaB_getmetatable),
    lua_reg!(b"ipairs\x00", luaB_ipairs),
    lua_reg!(b"loadfile\x00", luaB_loadfile),
    lua_reg!(b"load\x00", luaB_load),
    lua_reg!(b"next\x00", luaB_next),
    lua_reg!(b"pairs\x00", luaB_pairs),
    lua_reg!(b"pcall\x00", luaB_pcall),
    lua_reg!(b"print\x00", luaB_print),
    lua_reg!(b"rawequal\x00", luaB_rawequal),
    lua_reg!(b"rawlen\x00", luaB_rawlen),
    lua_reg!(b"rawget\x00", luaB_rawget),
    lua_reg!(b"rawset\x00", luaB_rawset),
    lua_reg!(b"select\x00", luaB_select),
    lua_reg!(b"setmetatable\x00", luaB_setmetatable),
    lua_reg!(b"tonumber\x00", luaB_tonumber),
    lua_reg!(b"tostring\x00", luaB_tostring),
    lua_reg!(b"type\x00", luaB_type),
    lua_reg!(b"xpcall\x00", luaB_xpcall),
    lua_reg_none!(b"_G\x00"),
    lua_reg_none!(b"_VERSION\x00"),
    lua_reg_none!(0),
];
/*
** Do a protected call with error handling. After 'lua_rotate', the
** stack will have <f, err, true, f, [args...]>; so, the function passes
** 2 to 'finishpcall' to skip the 2 first values when returning results.
*/
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> lua_int {
    let mut status: lua_int = 0;
    let mut n: lua_int = lua_gettop(L);
    /* check error function */
    luaL_checktype(L, 2i32, 6i32);
    /* first result */
    lua_pushboolean(L, 1i32);
    /* function */
    lua_pushvalue(L, 1i32);
    /* move them below function's arguments */
    lua_rotate(L, 3i32, 2i32);
    status = lua_pcallk(
        L,
        n - 2i32,
        -1i32,
        2i32,
        2i32 as lua_KContext,
        Some(finishpcall),
    );
    return finishpcall(L, status, 2i32 as lua_KContext);
}
/*
** Continuation function for 'pcall' and 'xpcall'. Both functions
** already pushed a 'true' before doing the call, so in case of success
** 'finishpcall' only has to return everything in the stack minus
** 'extra' values (where 'extra' is exactly the number of items to be
** ignored).
*/
unsafe extern "C" fn finishpcall(
    mut L: *mut lua_State,
    mut status: lua_int,
    mut extra: lua_KContext,
) -> lua_int {
    if status != LUA_OK && status != LUA_YIELD {
        /* error? */
        /* first result (false) */
        lua_pushboolean(L, 0i32);
        /* error message */
        lua_pushvalue(L, -2i32);
        /* return false, msg */
        return 2i32;
    } else {
        return lua_gettop(L) - extra as lua_int;
    };
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 1i32);
    (t != -1i32 || 0 != luaL_argerror(L, 1i32, s!(b"value expected\x00"))) as lua_int;
    lua_pushstring(L, lua_typename(L, t));
    return 1i32;
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    luaL_tolstring(L, 1i32, 0 as *mut size_t);
    return 1i32;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> lua_int {
    if lua_type(L, 2i32) <= 0i32 {
        /* standard conversion? */
        luaL_checkany(L, 1i32);
        if lua_type(L, 1i32) == 3i32 {
            /* already a number? */
            /* yes; return it */
            lua_settop(L, 1i32);
            return 1i32;
        } else {
            let mut l: size_t = 0;
            let mut s: *const lua_char = lua_tolstring(L, 1i32, &mut l);
            if !s.is_null() && lua_stringtonumber(L, s) == l.wrapping_add(1i32 as lua_ulong) {
                /* successful conversion to number */
                return 1i32;
            }
        }
    } else {
        /* else not a number */
        let mut l_0: size_t = 0;
        let mut s_0: *const lua_char = 0 as *const lua_char;
        /* to avoid warnings */
        let mut n: lua_Integer = 0i32 as lua_Integer;
        let mut base: lua_Integer = luaL_checkinteger(L, 2i32);
        /* no numbers as strings */
        luaL_checktype(L, 1i32, 4i32);
        s_0 = lua_tolstring(L, 1i32, &mut l_0);
        (2i32 as lua_longlong <= base && base <= 36i32 as lua_longlong
            || 0 != luaL_argerror(L, 2i32, s!(b"base out of range\x00"))) as lua_int;
        if b_str2int(s_0, base as lua_int, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1i32;
        }
    }
    /* else not a number */
    /* else not a number */
    /* not a number */
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn b_str2int(
    mut s: *const lua_char,
    mut base: lua_int,
    mut pn: *mut lua_Integer,
) -> *const lua_char {
    let mut n: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut neg: lua_int = 0i32;
    /* skip initial spaces */
    s = s.offset(strspn(s, s!(b" \x0c\n\r\t\x0b\x00")) as isize);
    if *s as lua_int == '-' as i32 {
        s = s.offset(1isize);
        /* handle signal */
        neg = 1i32
    } else if *s as lua_int == '+' as i32 {
        s = s.offset(1isize)
    }
    /* no digit? */
    if 0 == *(*__ctype_b_loc()).offset(*s as lua_uchar as lua_int as isize) as lua_int
        & _ISalnum as lua_int as lua_ushort as lua_int
    {
        return 0 as *const lua_char;
    } else {
        loop {
            let mut digit: lua_int = if 0
                != *(*__ctype_b_loc()).offset(*s as lua_uchar as lua_int as isize)
                    as lua_int
                    & _ISdigit as lua_int as lua_ushort as lua_int
            {
                *s as lua_int - ('0' as i32)
            } else {
                {
                    let mut __c: lua_int = 0;
                    let mut __res: lua_int = 0;
                    if ::std::mem::size_of::<lua_uchar>() as lua_ulong
                        > 1i32 as lua_ulong
                    {
                        if 0 != 0 {
                            __c = *s as lua_uchar as lua_int;
                            __res = if __c < -128i32 || __c > 255i32 {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            }
                        } else {
                            __res = toupper(*s as lua_uchar as lua_int)
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(*s as lua_uchar as lua_int as isize)
                    }
                    //__res // TODO: What is this?
                }
                -('A' as i32) + 10i32
            };
            if digit >= base {
                /* invalid numeral */
                return 0 as *const lua_char;
            } else {
                n = n
                    .wrapping_mul(base as lua_ulonglong)
                    .wrapping_add(digit as lua_ulonglong);
                s = s.offset(1isize);
                if !(0
                    != *(*__ctype_b_loc()).offset(*s as lua_uchar as lua_int as isize)
                        as lua_int
                        & _ISalnum as lua_int as lua_ushort as lua_int)
                {
                    break;
                }
            }
        }
        /* skip trailing spaces */
        s = s.offset(strspn(s, s!(b" \x0c\n\r\t\x0b\x00")) as isize);
        *pn = (if 0 != neg {
            (0u32 as lua_ulonglong).wrapping_sub(n)
        } else {
            n
        }) as lua_Integer;
        return s;
    };
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 2i32);
    luaL_checktype(L, 1i32, 5i32);
    (t == 0i32 || t == 5i32 || 0 != luaL_argerror(L, 2i32, s!(b"nil or table expected\x00")))
        as lua_int;
    if luaL_getmetafield(L, 1i32, s!(b"__metatable\x00")) != 0i32 {
        return luaL_error!(L, s!(b"cannot change a protected metatable\x00"));
    } else {
        lua_settop(L, 2i32);
        lua_setmetatable(L, 1i32);
        return 1i32;
    };
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_int = lua_gettop(L);
    if lua_type(L, 1i32) == 4i32
        && *lua_tolstring(L, 1i32, 0 as *mut size_t) as lua_int == '#' as i32
    {
        lua_pushinteger(L, (n - 1i32) as lua_Integer);
        return 1i32;
    } else {
        let mut i: lua_Integer = luaL_checkinteger(L, 1i32);
        if i < 0i32 as lua_longlong {
            i = n as lua_longlong + i
        } else if i > n as lua_longlong {
            i = n as lua_Integer
        }
        (1i32 as lua_longlong <= i
            || 0 != luaL_argerror(L, 1i32, s!(b"index out of range\x00"))) as lua_int;
        return n - i as lua_int;
    };
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    luaL_checkany(L, 3i32);
    lua_settop(L, 3i32);
    lua_rawset(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_rawget(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawlen(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 1i32);
    (t == 5i32 || t == 4i32 || 0 != luaL_argerror(L, 1i32, s!(b"table or string expected\x00")))
        as lua_int;
    lua_pushinteger(L, lua_rawlen(L, 1i32) as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    luaL_checkany(L, 2i32);
    lua_pushboolean(L, lua_rawequal(L, 1i32, 2i32));
    return 1i32;
}
/*
** $Id: lbaselib.c,v 1.314.1.1 2017/04/19 17:39:34 roberto Exp $
** Basic library
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    let mut i: lua_int = 0;
    lua_getglobal(L, s!(b"tostring\x00"));
    i = 1i32;
    while i <= n {
        let mut s: *const lua_char = 0 as *const lua_char;
        let mut l: size_t = 0;
        /* function to be called */
        lua_pushvalue(L, -1i32);
        /* value to print */
        lua_pushvalue(L, i);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        /* get result */
        s = lua_tolstring(L, -1i32, &mut l);
        if s.is_null() {
            return luaL_error!(L, s!(b"\'tostring\' must return a string to \'print\'\x00"));
        } else {
            if i > 1i32 {
                fwrite(
                    s!(b"\t\x00") as *const lua_void,
                    ::std::mem::size_of::<lua_char>() as lua_ulong,
                    1i32 as size_t,
                    stdout,
                );
            }
            fwrite(
                s as *const lua_void,
                ::std::mem::size_of::<lua_char>() as lua_ulong,
                l,
                stdout,
            );
            /* pop result */
            lua_settop(L, -1i32 - 1i32);
            i += 1
        }
    }
    fwrite(
        s!(b"\n\x00") as *const lua_void,
        ::std::mem::size_of::<lua_char>() as lua_ulong,
        1i32 as size_t,
        stdout,
    );
    fflush(stdout);
    return 0i32;
}
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> lua_int {
    let mut status: lua_int = 0;
    luaL_checkany(L, 1i32);
    /* first result if no errors */
    lua_pushboolean(L, 1i32);
    /* put it in place */
    lua_rotate(L, 1i32, 1i32);
    status = lua_pcallk(
        L,
        lua_gettop(L) - 2i32,
        -1i32,
        0i32,
        0i32 as lua_KContext,
        Some(finishpcall),
    );
    return finishpcall(L, status, 0i32 as lua_KContext);
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> lua_int {
    return pairsmeta(L, s!(b"__pairs\x00"), 0i32, Some(luaB_next));
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    /* create a 2nd argument if there isn't one */
    lua_settop(L, 2i32);
    if 0 != lua_next(L, 1i32) {
        return 2i32;
    } else {
        lua_pushnil(L);
        return 1i32;
    };
}
unsafe extern "C" fn pairsmeta(
    mut L: *mut lua_State,
    mut method: *const lua_char,
    mut iszero: lua_int,
    mut iter: lua_CFunction,
) -> lua_int {
    luaL_checkany(L, 1i32);
    if luaL_getmetafield(L, 1i32, method) == 0i32 {
        /* no metamethod? */
        /* will return generator, */
        lua_pushcclosure(L, iter, 0i32);
        /* state, */
        lua_pushvalue(L, 1i32);
        if 0 != iszero {
            /* and initial value */
            lua_pushinteger(L, 0i32 as lua_Integer);
        } else {
            lua_pushnil(L);
        }
    } else {
        /* argument 'self' to metamethod */
        lua_pushvalue(L, 1i32);
        /* get 3 values from metamethod */
        lua_callk(L, 1i32, 3i32, 0i32 as lua_KContext, None);
    }
    return 3i32;
}
unsafe extern "C" fn luaB_load(mut L: *mut lua_State) -> lua_int {
    let mut chunkname: *const lua_char = 0 as *const lua_char;
    let mut chunkname_0: *const lua_char = 0 as *const lua_char;
    let mut status: lua_int = 0;
    let mut l: size_t = 0;
    let mut s: *const lua_char = lua_tolstring(L, 1i32, &mut l);
    let mut mode: *const lua_char = luaL_optlstring(L, 3i32, s!(b"bt\x00"), 0 as *mut size_t);
    /* 'env' index or 0 if no 'env' */
    let mut env: lua_int = if !(lua_type(L, 4i32) == -1i32) {
        4i32
    } else {
        0i32
    };
    if !s.is_null() {
        /* loading a string? */
        chunkname = luaL_optlstring(L, 2i32, s, 0 as *mut size_t);
        status = luaL_loadbufferx(L, s, l, chunkname, mode)
    } else {
        /* loading from a reader function */
        chunkname_0 = luaL_optlstring(L, 2i32, s!(b"=(load)\x00"), 0 as *mut size_t);
        luaL_checktype(L, 1i32, 6i32);
        /* create reserved slot */
        lua_settop(L, 5i32);
        status = lua_load(
            L,
            Some(generic_reader),
            0 as *mut lua_void,
            chunkname_0,
            mode,
        )
    }
    return load_aux(L, status, env);
}
unsafe extern "C" fn load_aux(
    mut L: *mut lua_State,
    mut status: lua_int,
    mut envidx: lua_int,
) -> lua_int {
    if status == LUA_OK {
        if envidx != 0i32 {
            /* 'env' parameter? */
            /* environment for loaded function */
            lua_pushvalue(L, envidx);
            /* set it as 1st upvalue */
            if lua_setupvalue(L, -2i32, 1i32).is_null() {
                /* remove 'env' if not used by previous call */
                lua_settop(L, -1i32 - 1i32);
            }
        }
        return 1i32;
    } else {
        /* error (message is on top of the stack) */
        lua_pushnil(L);
        /* put before error message */
        lua_rotate(L, -2i32, 1i32);
        /* return nil plus error message */
        return 2i32;
    };
}
/*
** {======================================================
** Generic Read function
** =======================================================
*/
/*
** reserved slot, above all arguments, to hold a copy of the returned
** string to avoid it being collected while parsed. 'load' has four
** optional arguments (chunk, source name, mode, and environment).
*/
/*
** Reader for generic 'load' function: 'lua_load' uses the
** stack for internal stuff, so the reader cannot change the
** stack top. Instead, it keeps its resulting string in a
** reserved slot inside the stack.
*/
unsafe extern "C" fn generic_reader(
    mut L: *mut lua_State,
    mut _ud: *mut lua_void,
    mut size: *mut size_t,
) -> *const lua_char {
    /* not used */
    luaL_checkstack(L, 2i32, s!(b"too many nested functions\x00"));
    /* get function */
    lua_pushvalue(L, 1i32);
    /* call it */
    lua_callk(L, 0i32, 1i32, 0i32 as lua_KContext, None);
    if lua_type(L, -1i32) == 0i32 {
        /* pop result */
        lua_settop(L, -1i32 - 1i32);
        *size = 0i32 as size_t;
        return 0 as *const lua_char;
    } else {
        if 0 == lua_isstring(L, -1i32) {
            luaL_error!(L, s!(b"reader function must return a string\x00"));
        }
        /* save string in reserved slot */
        lua_copy(L, -1i32, 5i32);
        lua_settop(L, -1i32 - 1i32);
        return lua_tolstring(L, 5i32, size);
    };
}
unsafe extern "C" fn luaB_loadfile(mut L: *mut lua_State) -> lua_int {
    let mut fname: *const lua_char =
        luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    let mut mode: *const lua_char =
        luaL_optlstring(L, 2i32, 0 as *const lua_char, 0 as *mut size_t);
    /* 'env' index or 0 if no 'env' */
    let mut env: lua_int = if !(lua_type(L, 3i32) == -1i32) {
        3i32
    } else {
        0i32
    };
    let mut status: lua_int = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}
/*
** 'ipairs' function. Returns 'ipairsaux', given "table", 0.
** (The given "table" may not be a table.)
*/
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> lua_int {
    return pairsmeta(L, s!(b"__ipairs\x00"), 1i32, Some(ipairsaux));
}
/*
** Traversal function for 'ipairs'
*/
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> lua_int {
    let mut i: lua_Integer = luaL_checkinteger(L, 2i32) + 1i32 as lua_longlong;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1i32, i) == 0i32 {
        1i32
    } else {
        2i32
    };
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        lua_pushnil(L);
        /* no metatable */
        return 1i32;
    } else {
        luaL_getmetafield(L, 1i32, s!(b"__metatable\x00"));
        /* returns either __metatable field (if present) or metatable */
        return 1i32;
    };
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> lua_int {
    let mut level: lua_int = luaL_optinteger(L, 2i32, 1i32 as lua_Integer) as lua_int;
    lua_settop(L, 1i32);
    if lua_type(L, 1i32) == 4i32 && level > 0i32 {
        /* add extra information */
        luaL_where(L, level);
        lua_pushvalue(L, 1i32);
        lua_concat(L, 2i32);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> lua_int {
    let mut fname: *const lua_char =
        luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    lua_settop(L, 1i32);
    if luaL_loadfilex(L, fname, 0 as *const lua_char) != LUA_OK {
        return lua_error(L);
    } else {
        lua_callk(L, 0i32, LUA_MULTRET, 0i32 as lua_KContext, Some(dofilecont));
        return dofilecont(L, 0i32, 0i32 as lua_KContext);
    };
}
/* }====================================================== */
unsafe extern "C" fn dofilecont(
    mut L: *mut lua_State,
    mut _d1: lua_int,
    mut _d2: lua_KContext,
) -> lua_int {
    /* only to match 'lua_Kfunction' prototype */
    return lua_gettop(L) - 1i32;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> lua_int {
    static mut opts: [*const lua_char; 9] = unsafe {
        [
            s!(b"stop\x00"),
            s!(b"restart\x00"),
            s!(b"collect\x00"),
            s!(b"count\x00"),
            s!(b"step\x00"),
            s!(b"setpause\x00"),
            s!(b"setstepmul\x00"),
            s!(b"isrunning\x00"),
            0 as *const lua_char,
        ]
    };
    static mut optsnum: [lua_int; 8] =
        unsafe { [0i32, 1i32, 2i32, 3i32, 5i32, 6i32, 7i32, 9i32] };
    let mut o: lua_int =
        optsnum[luaL_checkoption(L, 1i32, s!(b"collect\x00"), opts.as_ptr()) as usize];
    let mut ex: lua_int = luaL_optinteger(L, 2i32, 0i32 as lua_Integer) as lua_int;
    let mut res: lua_int = lua_gc(L, o, ex);
    match o {
        3 => {
            let mut b: lua_int = lua_gc(L, 4i32, 0i32);
            lua_pushnumber(
                L,
                res as lua_Number + b as lua_Number / 1024i32 as lua_double,
            );
            return 1i32;
        }
        5 | 9 => {
            lua_pushboolean(L, res);
            return 1i32;
        }
        _ => {
            lua_pushinteger(L, res as lua_Integer);
            return 1i32;
        }
    };
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> lua_int {
    /* condition is true? */
    if 0 != lua_toboolean(L, 1i32) {
        /* return all arguments */
        return lua_gettop(L);
    } else {
        /* error */
        /* there must be a condition */
        luaL_checkany(L, 1i32);
        /* remove it */
        lua_rotate(L, 1i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* default message */
        lua_pushstring(L, s!(b"assertion failed!\x00"));
        /* leave only message (default if no other one) */
        lua_settop(L, 1i32);
        /* call 'error' */
        return luaB_error(L);
    };
}
