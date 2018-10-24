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
    /* private part */
    pub type CallInfo;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fopen(__filename: *const lua_char, __modes: *const lua_char) -> *mut FILE;
    #[no_mangle]
    fn freopen(
        __filename: *const lua_char,
        __modes: *const lua_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fread(__ptr: *mut lua_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn realloc(_: *mut lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn free(__ptr: *mut lua_void) -> ();
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn strcmp(_: *const lua_char, _: *const lua_char) -> lua_int;
    #[no_mangle]
    fn strncmp(_: *const lua_char, _: *const lua_char, _: lua_ulong) -> lua_int;
    #[no_mangle]
    fn strstr(_: *const lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn strerror(_: lua_int) -> *mut lua_char;
    /*
     ** state manipulation
     */
    #[no_mangle]
    fn lua_newstate(f: lua_Alloc, ud: *mut lua_void) -> *mut lua_State;
    #[no_mangle]
    fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;
    #[no_mangle]
    fn lua_version(L: *mut lua_State) -> *const lua_Number;
    /*
     ** basic stack manipulation
     */
    #[no_mangle]
    fn lua_absindex(L: *mut lua_State, idx: lua_int) -> lua_int;
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
    fn lua_checkstack(L: *mut lua_State, n: lua_int) -> lua_int;
    /*
     ** access functions (stack -> C)
     */
    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, idx: lua_int) -> lua_int;
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
    fn lua_rawlen(L: *mut lua_State, idx: lua_int) -> size_t;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: lua_int) -> *mut lua_void;
    #[no_mangle]
    fn lua_topointer(L: *mut lua_State, idx: lua_int) -> *const lua_void;
    #[no_mangle]
    fn lua_rawequal(L: *mut lua_State, idx1: lua_int, idx2: lua_int) -> lua_int;
    /*
     ** push functions (C -> stack)
     */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State, s: *const lua_char, len: size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;
    #[no_mangle]
    fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const lua_char,
        argp: *mut __va_list_tag,
    ) -> *const lua_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> lua_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut lua_void;
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    /*
     ** set functions (stack -> Lua)
     */
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const lua_char) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> ();
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
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        dt: *mut lua_void,
        chunkname: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
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
    fn lua_len(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_getallocf(L: *mut lua_State, ud: *mut *mut lua_void) -> lua_Alloc;
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: lua_int, ar: *mut lua_Debug) -> lua_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const lua_char, ar: *mut lua_Debug) -> lua_int;
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
    unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_void, _: *mut size_t) -> *const lua_char,
>;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut lua_void, _: *mut lua_void, _: size_t, _: size_t) -> *mut lua_void,
>;
/*
** {==============================================================
** some useful macros
** ===============================================================
*/
/* }============================================================== */
/*
** {==============================================================
** compatibility macros for unsigned conversions
** ===============================================================
*/
/* }============================================================== */
/*
** {======================================================================
** Debug API
** =======================================================================
*/
/*
** Event codes
*/
/*
** Event masks
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: lua_int,
    pub name: *const lua_char,
    pub namewhat: *const lua_char,
    pub what: *const lua_char,
    pub source: *const lua_char,
    pub currentline: lua_int,
    pub linedefined: lua_int,
    pub lastlinedefined: lua_int,
    pub nups: lua_uchar,
    pub nparams: lua_uchar,
    pub isvararg: lua_char,
    pub istailcall: lua_char,
    pub short_src: [lua_char; 60],
    pub i_ci: *mut CallInfo,
}
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
/* }====================================================== */
/*
** {======================================================
** Load functions
** =======================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadF {
    pub n: lua_int,
    pub f: *mut FILE,
    pub buff: [lua_char; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadS {
    pub s: *const lua_char,
    pub size: size_t,
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
/* }====================================================== */
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
#[no_mangle]
pub unsafe extern "C" fn luaL_checkversion_(
    mut L: *mut lua_State,
    mut ver: lua_Number,
    mut sz: size_t,
) -> () {
    let mut v: *const lua_Number = lua_version(L);
    /* check numeric types */
    if sz
        != (::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(16i32 as lua_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as lua_ulong)
    {
        luaL_error!(
            L,
            s!(b"core and library have incompatible numeric types\x00"),
        );
    }
    if v != lua_version(0 as *mut lua_State) {
        luaL_error!(L, s!(b"multiple Lua VMs detected\x00"),);
    } else if *v != ver {
        luaL_error!(
            L,
            s!(b"version mismatch: app. needs %f, Lua core provides %f\x00"),
            ver,
            *v,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_where(mut L: *mut lua_State, mut level: lua_int) -> () {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    if 0 != lua_getstack(L, level, &mut ar) {
        /* check function at level */
        /* get info about it */
        lua_getinfo(L, s!(b"Sl\x00"), &mut ar);
        if ar.currentline > 0i32 {
            /* is there info? */
            lua_pushfstring!(
                L,
                s!(b"%s:%d: \x00"),
                ar.short_src.as_mut_ptr(),
                ar.currentline,
            );
            return;
        }
    }
    /* else, no information available... */
    lua_pushfstring!(L, s!(b"\x00"));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getmetafield(
    mut L: *mut lua_State,
    mut obj: lua_int,
    mut event: *const lua_char,
) -> lua_int {
    /* no metatable? */
    if 0 == lua_getmetatable(L, obj) {
        return 0i32;
    } else {
        let mut tt: lua_int = 0;
        lua_pushstring(L, event);
        tt = lua_rawget(L, -2i32);
        /* is metafield nil? */
        if tt == 0i32 {
            /* remove metatable and metafield */
            lua_settop(L, -2i32 - 1i32);
        } else {
            /* remove only metatable */
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
        }
        /* return metafield type */
        return tt;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_callmeta(
    mut L: *mut lua_State,
    mut obj: lua_int,
    mut event: *const lua_char,
) -> lua_int {
    obj = lua_absindex(L, obj);
    /* no metafield? */
    if luaL_getmetafield(L, obj, event) == 0i32 {
        return 0i32;
    } else {
        lua_pushvalue(L, obj);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_tolstring(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut len: *mut size_t,
) -> *const lua_char {
    if 0 != luaL_callmeta(L, idx, s!(b"__tostring\x00")) {
        /* metafield? */
        if 0 == lua_isstring(L, -1i32) {
            luaL_error!(L, s!(b"\'__tostring\' must return a string\x00"),);
        }
    } else {
        match lua_type(L, idx) {
            3 => {
                if 0 != lua_isinteger(L, idx) {
                    lua_pushfstring!(L, s!(b"%I\x00"), lua_tointegerx(L, idx, 0 as *mut lua_int),);
                } else {
                    lua_pushfstring!(L, s!(b"%f\x00"), lua_tonumberx(L, idx, 0 as *mut lua_int),);
                }
            }
            4 => {
                lua_pushvalue(L, idx);
            }
            1 => {
                lua_pushstring(
                    L,
                    if 0 != lua_toboolean(L, idx) {
                        s!(b"true\x00")
                    } else {
                        s!(b"false\x00")
                    },
                );
            }
            0 => {
                lua_pushstring(L, s!(b"nil\x00"));
            }
            _ => {
                /* try name */
                let mut tt: lua_int = luaL_getmetafield(L, idx, s!(b"__name\x00"));
                let mut kind: *const lua_char = if tt == 4i32 {
                    lua_tolstring(L, -1i32, 0 as *mut size_t)
                } else {
                    lua_typename(L, lua_type(L, idx))
                };
                lua_pushfstring!(L, s!(b"%s: %p\x00"), kind, lua_topointer(L, idx),);
                if tt != 0i32 {
                    /* remove '__name' */
                    lua_rotate(L, -2i32, -1i32);
                    lua_settop(L, -1i32 - 1i32);
                }
            }
        }
    }
    return lua_tolstring(L, -1i32, len);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_argerror(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut _extramsg: *const lua_char,
) -> lua_int {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    /* no stack frame? */
    if 0 == lua_getstack(L, 0i32, &mut ar) {
        return luaL_error!(L, s!(b"bad argument #%d (%s)\x00"), arg, extramsg,);
    } else {
        lua_getinfo(L, s!(b"n\x00"), &mut ar);
        if strcmp(ar.namewhat, s!(b"method\x00")) == 0i32 {
            /* do not count 'self' */
            arg -= 1;
            /* error is in the self argument itself? */
            if arg == 0i32 {
                return luaL_error!(
                    L,
                    s!(b"calling \'%s\' on bad self (%s)\x00"),
                    ar.name,
                    extramsg,
                );
            }
        }
        if ar.name.is_null() {
            ar.name = if 0 != pushglobalfuncname(L, &mut ar) {
                lua_tolstring(L, -1i32, 0 as *mut size_t)
            } else {
                s!(b"?\x00")
            }
        }
        return luaL_error!(
            L,
            s!(b"bad argument #%d to \'%s\' (%s)\x00"),
            arg,
            ar.name,
            extramsg,
        );
    };
}
/*
** Search for a name for a function in all loaded modules
*/
unsafe extern "C" fn pushglobalfuncname(mut L: *mut lua_State, mut ar: *mut lua_Debug) -> lua_int {
    let mut top: lua_int = lua_gettop(L);
    /* push function */
    lua_getinfo(L, s!(b"f\x00"), ar);
    lua_getfield(L, -1000000i32 - 1000i32, s!(b"_LOADED\x00"));
    if 0 != findfield(L, top + 1i32, 2i32) {
        let mut name: *const lua_char = lua_tolstring(L, -1i32, 0 as *mut size_t);
        if strncmp(name, s!(b"_G.\x00"), 3i32 as lua_ulong) == 0i32 {
            /* name start with '_G.'? */
            /* push name without prefix */
            lua_pushstring(L, name.offset(3isize));
            /* remove original name */
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
        }
        /* move name to proper place */
        lua_copy(L, -1i32, top + 1i32);
        /* remove pushed values */
        lua_settop(L, -2i32 - 1i32);
        return 1i32;
    } else {
        /* remove function and global table */
        lua_settop(L, top);
        return 0i32;
    };
}
/*
** $Id: lauxlib.c,v 1.289.1.1 2017/04/19 17:20:42 roberto Exp $
** Auxiliary functions for building Lua libraries
** See Copyright Notice in lua.h
*/
/*
** This file uses only the official API of Lua.
** Any function declared here could be written as an application function.
*/
/*
** {======================================================
** Traceback
** =======================================================
*/
/* size of the first part of the stack */
/* size of the second part of the stack */
/*
** search for 'objidx' in table at index -1.
** return 1 + string at top if find a good name.
*/
unsafe extern "C" fn findfield(
    mut L: *mut lua_State,
    mut objidx: lua_int,
    mut level: lua_int,
) -> lua_int {
    if level == 0i32 || !(lua_type(L, -1i32) == 5i32) {
        /* not found */
        return 0i32;
    } else {
        /* start 'next' loop */
        lua_pushnil(L);
        while 0 != lua_next(L, -2i32) {
            /* for each pair in table */
            if lua_type(L, -2i32) == 4i32 {
                /* ignore non-string keys */
                if 0 != lua_rawequal(L, objidx, -1i32) {
                    /* found object? */
                    /* remove value (but keep name) */
                    lua_settop(L, -1i32 - 1i32);
                    return 1i32;
                } else if 0 != findfield(L, objidx, level - 1i32) {
                    /* try recursively */
                    /* remove table (but keep name) */
                    lua_rotate(L, -2i32, -1i32);
                    lua_settop(L, -1i32 - 1i32);
                    lua_pushstring(L, s!(b".\x00"));
                    /* place '.' between the two names */
                    lua_rotate(L, -2i32, 1i32);
                    lua_concat(L, 3i32);
                    return 1i32;
                }
            }
            /* remove value */
            lua_settop(L, -1i32 - 1i32);
        }
        /* not found */
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checklstring(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut len: *mut size_t,
) -> *const lua_char {
    let mut s: *const lua_char = lua_tolstring(L, arg, len);
    if s.is_null() {
        tag_error(L, arg, 4i32);
    }
    return s;
}
unsafe extern "C" fn tag_error(mut L: *mut lua_State, mut arg: lua_int, mut tag: lua_int) -> () {
    typeerror(L, arg, lua_typename(L, tag));
}
unsafe extern "C" fn typeerror(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut tname: *const lua_char,
) -> lua_int {
    let mut msg: *const lua_char = 0 as *const lua_char;
    /* name for the type of the actual argument */
    let mut typearg: *const lua_char = 0 as *const lua_char;
    if luaL_getmetafield(L, arg, s!(b"__name\x00")) == 4i32 {
        /* use the given type name */
        typearg = lua_tolstring(L, -1i32, 0 as *mut size_t)
    } else if lua_type(L, arg) == 2i32 {
        /* special name for messages */
        typearg = s!(b"light userdata\x00")
    } else {
        typearg = lua_typename(L, lua_type(L, arg))
    }
    msg = lua_pushfstring!(L, s!(b"%s expected, got %s\x00"), tname, typearg,);
    return luaL_argerror(L, arg, msg);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optlstring(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut def: *const lua_char,
    mut len: *mut size_t,
) -> *const lua_char {
    if lua_type(L, arg) <= 0i32 {
        if !len.is_null() {
            *len = if !def.is_null() {
                strlen(def)
            } else {
                0i32 as lua_ulong
            }
        }
        return def;
    } else {
        return luaL_checklstring(L, arg, len);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checknumber(mut L: *mut lua_State, mut arg: lua_int) -> lua_Number {
    let mut isnum: lua_int = 0;
    let mut d: lua_Number = lua_tonumberx(L, arg, &mut isnum);
    if 0 == isnum {
        tag_error(L, arg, 3i32);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optnumber(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut def: lua_Number,
) -> lua_Number {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checknumber(L, arg)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkinteger(mut L: *mut lua_State, mut arg: lua_int) -> lua_Integer {
    let mut isnum: lua_int = 0;
    let mut d: lua_Integer = lua_tointegerx(L, arg, &mut isnum);
    if 0 == isnum {
        interror(L, arg);
    }
    return d;
}
unsafe extern "C" fn interror(mut L: *mut lua_State, mut arg: lua_int) -> () {
    if 0 != lua_isnumber(L, arg) {
        luaL_argerror(L, arg, s!(b"number has no integer representation\x00"));
    } else {
        tag_error(L, arg, 3i32);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_optinteger(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut def: lua_Integer,
) -> lua_Integer {
    return if lua_type(L, arg) <= 0i32 {
        def
    } else {
        luaL_checkinteger(L, arg)
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkstack(
    mut L: *mut lua_State,
    mut space: lua_int,
    mut msg: *const lua_char,
) -> () {
    if 0 == lua_checkstack(L, space) {
        if !msg.is_null() {
            luaL_error!(L, s!(b"stack overflow (%s)\x00"), msg,);
        } else {
            luaL_error!(L, s!(b"stack overflow\x00"),);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checktype(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut t: lua_int,
) -> () {
    if lua_type(L, arg) != t {
        tag_error(L, arg, t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkany(mut L: *mut lua_State, mut arg: lua_int) -> () {
    if lua_type(L, arg) == -1i32 {
        luaL_argerror(L, arg, s!(b"value expected\x00"));
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newmetatable(
    mut L: *mut lua_State,
    mut tname: *const lua_char,
) -> lua_int {
    /* name already in use? */
    if lua_getfield(L, -1000000i32 - 1000i32, tname) != 0i32 {
        /* leave previous value on top, but return 0 */
        return 0i32;
    } else {
        lua_settop(L, -1i32 - 1i32);
        /* create metatable */
        lua_createtable(L, 0i32, 2i32);
        lua_pushstring(L, tname);
        /* metatable.__name = tname */
        lua_setfield(L, -2i32, s!(b"__name\x00"));
        lua_pushvalue(L, -1i32);
        /* registry.name = metatable */
        lua_setfield(L, -1000000i32 - 1000i32, tname);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setmetatable(
    mut L: *mut lua_State,
    mut tname: *const lua_char,
) -> () {
    lua_getfield(L, -1000000i32 - 1000i32, tname);
    lua_setmetatable(L, -2i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_testudata(
    mut L: *mut lua_State,
    mut ud: lua_int,
    mut tname: *const lua_char,
) -> *mut lua_void {
    let mut p: *mut lua_void = lua_touserdata(L, ud);
    if !p.is_null() {
        /* value is a userdata? */
        if 0 != lua_getmetatable(L, ud) {
            /* does it have a metatable? */
            /* get correct metatable */
            lua_getfield(L, -1000000i32 - 1000i32, tname);
            /* not the same? */
            if 0 == lua_rawequal(L, -1i32, -2i32) {
                /* value is a userdata with wrong metatable */
                p = 0 as *mut lua_void
            }
            /* remove both metatables */
            lua_settop(L, -2i32 - 1i32);
            return p;
        }
    }
    /* value is not a userdata with a metatable */
    return 0 as *mut lua_void;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkudata(
    mut L: *mut lua_State,
    mut ud: lua_int,
    mut tname: *const lua_char,
) -> *mut lua_void {
    let mut p: *mut lua_void = luaL_testudata(L, ud, tname);
    if p.is_null() {
        typeerror(L, ud, tname);
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_checkoption(
    mut L: *mut lua_State,
    mut arg: lua_int,
    mut def: *const lua_char,
    mut lst: *const *const lua_char,
) -> lua_int {
    let mut name: *const lua_char = if !def.is_null() {
        luaL_optlstring(L, arg, def, 0 as *mut size_t)
    } else {
        luaL_checklstring(L, arg, 0 as *mut size_t)
    };
    let mut i: lua_int = 0;
    i = 0i32;
    while !(*lst.offset(i as isize)).is_null() {
        if strcmp(*lst.offset(i as isize), name) == 0i32 {
            return i;
        } else {
            i += 1
        }
    }
    return luaL_argerror(
        L,
        arg,
        lua_pushfstring!(L, s!(b"invalid option \'%s\'\x00"), name,),
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaL_fileresult(
    mut L: *mut lua_State,
    mut stat: lua_int,
    mut fname: *const lua_char,
) -> lua_int {
    /* calls to Lua API may change this value */
    let mut en: lua_int = errno();
    if 0 != stat {
        lua_pushboolean(L, 1i32);
        return 1i32;
    } else {
        lua_pushnil(L);
        if !fname.is_null() {
            lua_pushfstring!(L, s!(b"%s: %s\x00"), fname, strerror(en),);
        } else {
            lua_pushstring(L, strerror(en));
        }
        lua_pushinteger(L, en as lua_Integer);
        return 3i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_execresult(mut L: *mut lua_State, mut stat: lua_int) -> lua_int {
    /* type of termination */
    let mut what: *const lua_char = s!(b"exit\x00");
    /* error? */
    if stat == -1i32 {
        return luaL_fileresult(L, 0i32, 0 as *const lua_char);
    } else {
        if stat & 0x7fi32 == 0i32 {
            stat = (stat & 0xff00i32) >> 8i32
        } else if ((stat & 0x7fi32) + 1i32) as lua_schar as lua_int >> 1i32 > 0i32 {
            stat = stat & 0x7fi32;
            what = s!(b"signal\x00")
        }
        /* interpret result */
        /* successful termination? */
        if *what as lua_int == 'e' as i32 && stat == 0i32 {
            lua_pushboolean(L, 1i32);
        } else {
            lua_pushnil(L);
        }
        lua_pushstring(L, what);
        lua_pushinteger(L, stat as lua_Integer);
        /* return true/nil,what,code */
        return 3i32;
    };
}
/* predefined references */
#[no_mangle]
pub unsafe extern "C" fn luaL_ref(mut L: *mut lua_State, mut t: lua_int) -> lua_int {
    let mut ref_0: lua_int = 0;
    if lua_type(L, -1i32) == 0i32 {
        /* remove from stack */
        lua_settop(L, -1i32 - 1i32);
        /* 'nil' has a unique fixed reference */
        return -1i32;
    } else {
        t = lua_absindex(L, t);
        /* get first free element */
        lua_rawgeti(L, t, 0i32 as lua_Integer);
        /* ref = t[freelist] */
        ref_0 = lua_tointegerx(L, -1i32, 0 as *mut lua_int) as lua_int;
        /* remove it from stack */
        lua_settop(L, -1i32 - 1i32);
        if ref_0 != 0i32 {
            /* any free element? */
            /* remove it from list */
            lua_rawgeti(L, t, ref_0 as lua_Integer);
            /* (t[freelist] = t[ref]) */
            lua_rawseti(L, t, 0i32 as lua_Integer);
        } else {
            ref_0 = lua_rawlen(L, t) as lua_int + 1i32
        }
        lua_rawseti(L, t, ref_0 as lua_Integer);
        return ref_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_unref(
    mut L: *mut lua_State,
    mut t: lua_int,
    mut ref_0: lua_int,
) -> () {
    if ref_0 >= 0i32 {
        t = lua_absindex(L, t);
        lua_rawgeti(L, t, 0i32 as lua_Integer);
        /* t[ref] = t[freelist] */
        lua_rawseti(L, t, ref_0 as lua_Integer);
        lua_pushinteger(L, ref_0 as lua_Integer);
        /* t[freelist] = ref */
        lua_rawseti(L, t, 0i32 as lua_Integer);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadfilex(
    mut L: *mut lua_State,
    mut filename: *const lua_char,
    mut mode: *const lua_char,
) -> lua_int {
    let mut lf: LoadF = LoadF {
        n: 0,
        f: 0 as *mut FILE,
        buff: [0; 8192],
    };
    let mut status: lua_int = 0;
    let mut readstatus: lua_int = 0;
    let mut c: lua_int = 0;
    /* index of filename on the stack */
    let mut fnameindex: lua_int = lua_gettop(L) + 1i32;
    if filename.is_null() {
        lua_pushstring(L, s!(b"=stdin\x00"));
        lf.f = stdin
    } else {
        lua_pushfstring!(L, s!(b"@%s\x00"), filename);
        lf.f = fopen(filename, s!(b"r\x00"));
        if lf.f.is_null() {
            return errfile(L, s!(b"open\x00"), fnameindex);
        }
    }
    /* read initial portion */
    if 0 != skipcomment(&mut lf, &mut c) {
        /* add line to correct line numbers */
        let fresh0 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh0 as usize] = '\n' as i32 as lua_char
    }
    if c == (*::std::mem::transmute::<&[u8; 5], &[lua_char; 5]>(b"\x1bLua\x00"))[0usize] as lua_int
        && !filename.is_null()
    {
        /* binary file? */
        /* reopen in binary mode */
        lf.f = freopen(filename, s!(b"rb\x00"), lf.f);
        if lf.f.is_null() {
            return errfile(L, s!(b"reopen\x00"), fnameindex);
        } else {
            /* re-read initial portion */
            skipcomment(&mut lf, &mut c);
        }
    }
    if c != -1i32 {
        /* 'c' is the first character of the stream */
        let fresh1 = lf.n;
        lf.n = lf.n + 1;
        lf.buff[fresh1 as usize] = c as lua_char
    }
    status = lua_load(
        L,
        Some(getF),
        &mut lf as *mut LoadF as *mut lua_void,
        lua_tolstring(L, -1i32, 0 as *mut size_t),
        mode,
    );
    readstatus = ferror(lf.f);
    if !filename.is_null() {
        /* close file (even in case of errors) */
        fclose(lf.f);
    }
    if 0 != readstatus {
        /* ignore results from 'lua_load' */
        lua_settop(L, fnameindex);
        return errfile(L, s!(b"read\x00"), fnameindex);
    } else {
        lua_rotate(L, fnameindex, -1i32);
        lua_settop(L, -1i32 - 1i32);
        return status;
    };
}
unsafe extern "C" fn errfile(
    mut L: *mut lua_State,
    mut what: *const lua_char,
    mut fnameindex: lua_int,
) -> lua_int {
    let mut serr: *const lua_char = strerror(errno());
    let mut filename: *const lua_char =
        lua_tolstring(L, fnameindex, 0 as *mut size_t).offset(1isize);
    lua_pushfstring!(L, s!(b"cannot %s %s: %s\x00"), what, filename, serr,);
    lua_rotate(L, fnameindex, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return 6i32 + 1i32;
}
unsafe extern "C" fn getF(
    mut _L: *mut lua_State,
    mut ud: *mut lua_void,
    mut size: *mut size_t,
) -> *const lua_char {
    let mut lf: *mut LoadF = ud as *mut LoadF;
    /* not used */
    if (*lf).n > 0i32 {
        /* are there pre-read characters to be read? */
        /* return them (chars already in buffer) */
        *size = (*lf).n as size_t;
        /* no more pre-read characters */
        (*lf).n = 0i32
    } else if 0 != feof((*lf).f) {
        return 0 as *const lua_char;
    } else {
        *size = fread(
            (*lf).buff.as_mut_ptr() as *mut lua_void,
            1i32 as size_t,
            ::std::mem::size_of::<[lua_char; 8192]>() as lua_ulong,
            (*lf).f,
        )
    }
    return (*lf).buff.as_mut_ptr();
}
/*
** reads the first character of file 'f' and skips an optional BOM mark
** in its beginning plus its first line if it starts with '#'. Returns
** true if it skipped the first line.  In any case, '*cp' has the
** first "valid" character of the file (after the optional BOM and
** a first-line comment).
*/
unsafe extern "C" fn skipcomment(mut lf: *mut LoadF, mut cp: *mut lua_int) -> lua_int {
    *cp = skipBOM(lf);
    let mut c: lua_int = *cp;
    if c == '#' as i32 {
        /* first line is a comment (Unix exec. file)? */
        loop {
            /* skip first line */
            c = getc((*lf).f);
            if !(c != -1i32 && c != '\n' as i32) {
                break;
            }
        }
        /* skip end-of-line, if present */
        *cp = getc((*lf).f);
        /* there was a comment */
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn skipBOM(mut lf: *mut LoadF) -> lua_int {
    /* UTF-8 BOM mark */
    let mut p: *const lua_char = s!(b"\xef\xbb\xbf\x00");
    let mut c: lua_int = 0;
    (*lf).n = 0i32;
    loop {
        c = getc((*lf).f);
        if c == -1i32 || {
            let fresh2 = p;
            p = p.offset(1);
            c != *(fresh2 as *const lua_uchar) as lua_int
        } {
            return c;
        } else {
            /* to be read by the parser */
            let fresh3 = (*lf).n;
            (*lf).n = (*lf).n + 1;
            (*lf).buff[fresh3 as usize] = c as lua_char;
            if !(*p as lua_int != '\u{0}' as i32) {
                break;
            }
        }
    }
    /* prefix matched; discard it */
    (*lf).n = 0i32;
    /* return next character */
    return getc((*lf).f);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadbufferx(
    mut L: *mut lua_State,
    mut buff: *const lua_char,
    mut size: size_t,
    mut name: *const lua_char,
    mut mode: *const lua_char,
) -> lua_int {
    let mut ls: LoadS = LoadS {
        s: 0 as *const lua_char,
        size: 0,
    };
    ls.s = buff;
    ls.size = size;
    return lua_load(
        L,
        Some(getS),
        &mut ls as *mut LoadS as *mut lua_void,
        name,
        mode,
    );
}
unsafe extern "C" fn getS(
    mut _L: *mut lua_State,
    mut ud: *mut lua_void,
    mut size: *mut size_t,
) -> *const lua_char {
    let mut ls: *mut LoadS = ud as *mut LoadS;
    /* not used */
    if (*ls).size == 0i32 as lua_ulong {
        return 0 as *const lua_char;
    } else {
        *size = (*ls).size;
        (*ls).size = 0i32 as size_t;
        return (*ls).s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_loadstring(mut L: *mut lua_State, mut s: *const lua_char) -> lua_int {
    return luaL_loadbufferx(L, s, strlen(s), s, 0 as *const lua_char);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_newstate() -> *mut lua_State {
    let mut L: *mut lua_State = lua_newstate(Some(l_alloc), 0 as *mut lua_void);
    if !L.is_null() {
        lua_atpanic(L, Some(panic));
    }
    return L;
}
unsafe extern "C" fn l_alloc(
    mut _ud: *mut lua_void,
    mut ptr: *mut lua_void,
    mut _osize: size_t,
    mut nsize: size_t,
) -> *mut lua_void {
    /* not used */
    if nsize == 0i32 as lua_ulong {
        free(ptr);
        return 0 as *mut lua_void;
    } else {
        return realloc(ptr, nsize);
    };
}
unsafe extern "C" fn panic(mut L: *mut lua_State) -> lua_int {
    fprintf(
        stderr,
        s!(b"PANIC: unprotected error in call to Lua API (%s)\n\x00"),
        lua_tolstring(L, -1i32, 0 as *mut size_t),
    );
    fflush(stderr);
    /* return to Lua to abort */
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_len(mut L: *mut lua_State, mut idx: lua_int) -> lua_Integer {
    let mut l: lua_Integer = 0;
    let mut isnum: lua_int = 0;
    lua_len(L, idx);
    l = lua_tointegerx(L, -1i32, &mut isnum);
    if 0 == isnum {
        luaL_error!(L, s!(b"object length is not an integer\x00"),);
    }
    /* remove object */
    lua_settop(L, -1i32 - 1i32);
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_gsub(
    mut L: *mut lua_State,
    mut s: *const lua_char,
    mut p: *const lua_char,
    mut r: *const lua_char,
) -> *const lua_char {
    let mut wild: *const lua_char = 0 as *const lua_char;
    let mut l: size_t = strlen(p);
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        wild = strstr(s, p);
        if wild.is_null() {
            break;
        }
        /* push prefix */
        luaL_addlstring(
            &mut b,
            s,
            wild.wrapping_offset_from(s) as lua_long as size_t,
        );
        /* push replacement in place of pattern */
        luaL_addstring(&mut b, r);
        /* continue after 'p' */
        s = wild.offset(l as isize)
    }
    /* push last suffix */
    luaL_addstring(&mut b, s);
    luaL_pushresult(&mut b);
    return lua_tolstring(L, -1i32, 0 as *mut size_t);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresult(mut B: *mut luaL_Buffer) -> () {
    let mut L: *mut lua_State = (*B).L;
    lua_pushlstring(L, (*B).b, (*B).n);
    if (*B).b != (*B).initb.as_mut_ptr() {
        /* delete old buffer */
        resizebox(L, -2i32, 0i32 as size_t);
        /* remove its header from the stack */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    };
}
unsafe extern "C" fn resizebox(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut newsize: size_t,
) -> *mut lua_void {
    let mut ud: *mut lua_void = 0 as *mut lua_void;
    let mut allocf: lua_Alloc = lua_getallocf(L, &mut ud);
    let mut box_0: *mut UBox = lua_touserdata(L, idx) as *mut UBox;
    let mut temp: *mut lua_void =
        allocf.expect("non-null function pointer")(ud, (*box_0).box_0, (*box_0).bsize, newsize);
    if temp.is_null() && newsize > 0i32 as lua_ulong {
        /* allocation error? */
        /* free buffer */
        resizebox(L, idx, 0i32 as size_t);
        luaL_error!(L, s!(b"not enough memory for buffer allocation\x00"),);
    }
    (*box_0).box_0 = temp;
    (*box_0).bsize = newsize;
    return temp;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addstring(mut B: *mut luaL_Buffer, mut s: *const lua_char) -> () {
    luaL_addlstring(B, s, strlen(s));
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addlstring(
    mut B: *mut luaL_Buffer,
    mut s: *const lua_char,
    mut l: size_t,
) -> () {
    if l > 0i32 as lua_ulong {
        /* avoid 'memcpy' when 's' can be NULL */
        let mut b: *mut lua_char = luaL_prepbuffsize(B, l);
        memcpy(
            b as *mut lua_void,
            s as *const lua_void,
            l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
        (*B).n = ((*B).n as lua_ulong).wrapping_add(l) as size_t as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_prepbuffsize(
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut lua_char {
    let mut L: *mut lua_State = (*B).L;
    if (*B).size.wrapping_sub((*B).n) < sz {
        /* not enough space? */
        let mut newbuff: *mut lua_char = 0 as *mut lua_char;
        /* double buffer size */
        let mut newsize: size_t = (*B).size.wrapping_mul(2i32 as lua_ulong);
        /* not big enough? */
        if newsize.wrapping_sub((*B).n) < sz {
            newsize = (*B).n.wrapping_add(sz)
        }
        if newsize < (*B).n || newsize.wrapping_sub((*B).n) < sz {
            luaL_error!(L, s!(b"buffer too large\x00"),);
        }
        /* create larger buffer */
        if (*B).b != (*B).initb.as_mut_ptr() {
            newbuff = resizebox(L, -1i32, newsize) as *mut lua_char
        } else {
            /* no buffer yet */
            newbuff = newbox(L, newsize) as *mut lua_char;
            /* copy original content */
            memcpy(
                newbuff as *mut lua_void,
                (*B).b as *const lua_void,
                (*B).n
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        }
        (*B).b = newbuff;
        (*B).size = newsize
    }
    return &mut *(*B).b.offset((*B).n as isize) as *mut lua_char;
}
unsafe extern "C" fn newbox(mut L: *mut lua_State, mut newsize: size_t) -> *mut lua_void {
    let mut box_0: *mut UBox =
        lua_newuserdata(L, ::std::mem::size_of::<UBox>() as lua_ulong) as *mut UBox;
    (*box_0).box_0 = 0 as *mut lua_void;
    (*box_0).bsize = 0i32 as size_t;
    if 0 != luaL_newmetatable(L, s!(b"LUABOX\x00")) {
        /* creating metatable? */
        lua_pushcclosure(L, Some(boxgc), 0i32);
        /* metatable.__gc = boxgc */
        lua_setfield(L, -2i32, s!(b"__gc\x00"));
    }
    lua_setmetatable(L, -2i32);
    return resizebox(L, -1i32, newsize);
}
unsafe extern "C" fn boxgc(mut L: *mut lua_State) -> lua_int {
    resizebox(L, 1i32, 0i32 as size_t);
    return 0i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinit(mut L: *mut lua_State, mut B: *mut luaL_Buffer) -> () {
    (*B).L = L;
    (*B).b = (*B).initb.as_mut_ptr();
    (*B).n = 0i32 as size_t;
    (*B).size = (0x80i32 as lua_ulong)
        .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
        .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong) as lua_int
        as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_setfuncs(
    mut L: *mut lua_State,
    mut l: *const luaL_Reg,
    mut nup: lua_int,
) -> () {
    luaL_checkstack(L, nup, s!(b"too many upvalues\x00"));
    while !(*l).name.is_null() {
        /* fill the table with given functions */
        let mut i: lua_int = 0;
        /* copy upvalues to the top */
        i = 0i32;
        while i < nup {
            lua_pushvalue(L, -nup);
            i += 1
        }
        /* closure with those upvalues */
        lua_pushcclosure(L, (*l).func, nup);
        lua_setfield(L, -(nup + 2i32), (*l).name);
        l = l.offset(1isize)
    }
    /* remove upvalues */
    lua_settop(L, -nup - 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_getsubtable(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut fname: *const lua_char,
) -> lua_int {
    if lua_getfield(L, idx, fname) == 5i32 {
        /* table already there */
        return 1i32;
    } else {
        /* remove previous result */
        lua_settop(L, -1i32 - 1i32);
        idx = lua_absindex(L, idx);
        lua_createtable(L, 0i32, 0i32);
        /* copy to be left at top */
        lua_pushvalue(L, -1i32);
        /* assign new table to field */
        lua_setfield(L, idx, fname);
        /* false, because did not find table there */
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_traceback(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut msg: *const lua_char,
    mut level: lua_int,
) -> () {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut top: lua_int = lua_gettop(L);
    let mut last: lua_int = lastlevel(L1);
    let mut n1: lua_int = if last - level > 10i32 + 11i32 {
        10i32
    } else {
        -1i32
    };
    if !msg.is_null() {
        lua_pushfstring!(L, s!(b"%s\n\x00"), msg);
    }
    luaL_checkstack(L, 10i32, 0 as *const lua_char);
    lua_pushstring(L, s!(b"stack traceback:\x00"));
    loop {
        let fresh4 = level;
        level = level + 1;
        if !(0 != lua_getstack(L1, fresh4, &mut ar)) {
            break;
        }
        let fresh5 = n1;
        n1 = n1 - 1;
        if fresh5 == 0i32 {
            /* too many levels? */
            /* add a '...' */
            lua_pushstring(L, s!(b"\n\t...\x00"));
            /* and skip to last ones */
            level = last - 11i32 + 1i32
        } else {
            lua_getinfo(L1, s!(b"Slnt\x00"), &mut ar);
            lua_pushfstring!(L, s!(b"\n\t%s:\x00"), ar.short_src.as_mut_ptr(),);
            if ar.currentline > 0i32 {
                lua_pushfstring!(L, s!(b"%d:\x00"), ar.currentline,);
            }
            lua_pushstring(L, s!(b" in \x00"));
            pushfuncname(L, &mut ar);
            if 0 != ar.istailcall {
                lua_pushstring(L, s!(b"\n\t(...tail calls...)\x00"));
            }
            lua_concat(L, lua_gettop(L) - top);
        }
    }
    lua_concat(L, lua_gettop(L) - top);
}
unsafe extern "C" fn pushfuncname(mut L: *mut lua_State, mut ar: *mut lua_Debug) -> () {
    if 0 != pushglobalfuncname(L, ar) {
        /* try first a global name */
        lua_pushfstring!(
            L,
            s!(b"function \'%s\'\x00"),
            lua_tolstring(L, -1i32, 0 as *mut size_t),
        );
        /* remove name */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    } else if *(*ar).namewhat as lua_int != '\u{0}' as i32 {
        /* use it */
        lua_pushfstring!(L, s!(b"%s \'%s\'\x00"), (*ar).namewhat, (*ar).name,);
    } else if *(*ar).what as lua_int == 'm' as i32 {
        lua_pushstring(L, s!(b"main chunk\x00"));
    } else if *(*ar).what as lua_int != 'C' as i32 {
        lua_pushfstring!(
            L,
            s!(b"function <%s:%d>\x00"),
            (*ar).short_src.as_mut_ptr(),
            (*ar).linedefined,
        );
    } else {
        /* nothing left... */
        lua_pushstring(L, s!(b"?\x00"));
    };
}
unsafe extern "C" fn lastlevel(mut L: *mut lua_State) -> lua_int {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut li: lua_int = 1i32;
    let mut le: lua_int = 1i32;
    /* find an upper bound */
    while 0 != lua_getstack(L, le, &mut ar) {
        li = le;
        le *= 2i32
    }
    /* do a binary search */
    while li < le {
        let mut m: lua_int = (li + le) / 2i32;
        if 0 != lua_getstack(L, m, &mut ar) {
            li = m + 1i32
        } else {
            le = m
        }
    }
    return le - 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaL_requiref(
    mut L: *mut lua_State,
    mut modname: *const lua_char,
    mut openf: lua_CFunction,
    mut glb: lua_int,
) -> () {
    luaL_getsubtable(L, -1000000i32 - 1000i32, s!(b"_LOADED\x00"));
    /* LOADED[modname] */
    lua_getfield(L, -1i32, modname);
    if 0 == lua_toboolean(L, -1i32) {
        /* package not already loaded? */
        /* remove field */
        lua_settop(L, -1i32 - 1i32);
        lua_pushcclosure(L, openf, 0i32);
        /* argument to open function */
        lua_pushstring(L, modname);
        /* call 'openf' to open module */
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        /* make copy of module (call result) */
        lua_pushvalue(L, -1i32);
        /* LOADED[modname] = module */
        lua_setfield(L, -3i32, modname);
    }
    /* remove LOADED table */
    lua_rotate(L, -2i32, -1i32);
    lua_settop(L, -1i32 - 1i32);
    if 0 != glb {
        /* copy of module */
        lua_pushvalue(L, -1i32);
        /* _G[modname] = module */
        lua_setglobal(L, modname);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaL_addvalue(mut B: *mut luaL_Buffer) -> () {
    let mut L: *mut lua_State = (*B).L;
    let mut l: size_t = 0;
    let mut s: *const lua_char = lua_tolstring(L, -1i32, &mut l);
    if (*B).b != (*B).initb.as_mut_ptr() {
        /* put value below buffer */
        lua_rotate(L, -2i32, 1i32);
    }
    luaL_addlstring(B, s, l);
    /* remove value */
    lua_rotate(
        L,
        if (*B).b != (*B).initb.as_mut_ptr() {
            -2i32
        } else {
            -1i32
        },
        -1i32,
    );
    lua_settop(L, -1i32 - 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_pushresultsize(mut B: *mut luaL_Buffer, mut sz: size_t) -> () {
    (*B).n = ((*B).n as lua_ulong).wrapping_add(sz) as size_t as size_t;
    luaL_pushresult(B);
}
#[no_mangle]
pub unsafe extern "C" fn luaL_buffinitsize(
    mut L: *mut lua_State,
    mut B: *mut luaL_Buffer,
    mut sz: size_t,
) -> *mut lua_char {
    luaL_buffinit(L, B);
    return luaL_prepbuffsize(B, sz);
}
