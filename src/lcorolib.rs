use libc;
use lua::*;
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
    fn lua_newthread(L: *mut lua_State) -> *mut lua_State;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State, idx: libc::c_int) -> *mut lua_State;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    /*
     ** coroutine functions
     */
    #[no_mangle]
    fn lua_yieldk(
        L: *mut lua_State,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_status(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_isyieldable(L: *mut lua_State) -> libc::c_int;
    /*
     ** miscellaneous functions
     */
    #[no_mangle]
    fn lua_error(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_where(L: *mut lua_State, lvl: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
}
pub type size_t = libc::c_ulong;
pub type intptr_t = libc::c_long;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
/*
** Type for continuation functions
*/
pub type lua_KFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: libc::c_int, _: lua_KContext) -> libc::c_int>;
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
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; 60],
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
    pub name: *const libc::c_char,
    pub func: lua_CFunction,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_coroutine(mut L: *mut lua_State) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg; 8]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, co_funcs.as_ptr(), 0i32);
    return 1i32;
}
static mut co_funcs: [luaL_Reg; 8] = [
    lua_reg!(b"create\x00", luaB_cocreate),
    lua_reg!(b"resume\x00", luaB_coresume),
    lua_reg!(b"running\x00", luaB_corunning),
    lua_reg!(b"status\x00", luaB_costatus),
    lua_reg!(b"wrap\x00", luaB_cowrap),
    lua_reg!(b"yield\x00", luaB_yield),
    lua_reg!(b"isyieldable\x00", luaB_yieldable),
    lua_reg_none!(0),
];
unsafe extern "C" fn luaB_yieldable(mut L: *mut lua_State) -> libc::c_int {
    lua_pushboolean(L, lua_isyieldable(L));
    return 1i32;
}
unsafe extern "C" fn luaB_yield(mut L: *mut lua_State) -> libc::c_int {
    return lua_yieldk(L, lua_gettop(L), 0i32 as lua_KContext, None);
}
unsafe extern "C" fn luaB_cowrap(mut L: *mut lua_State) -> libc::c_int {
    luaB_cocreate(L);
    lua_pushcclosure(L, Some(luaB_auxwrap), 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_auxwrap(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = lua_tothread(L, -1000000i32 - 1000i32 - 1i32);
    let mut r: libc::c_int = auxresume(L, co, lua_gettop(L));
    if r < 0i32 {
        if lua_type(L, -1i32) == 4i32 {
            /* error object is a string? */
            /* add extra info */
            luaL_where(L, 1i32);
            lua_rotate(L, -2i32, 1i32);
            lua_concat(L, 2i32);
        }
        /* propagate error */
        return lua_error(L);
    } else {
        return r;
    };
}
unsafe extern "C" fn auxresume(
    mut L: *mut lua_State,
    mut co: *mut lua_State,
    mut narg: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if 0 == lua_checkstack(co, narg) {
        lua_pushstring(L, s!(b"too many arguments to resume\x00"));
        /* error flag */
        return -1i32;
    } else if lua_status(co) == LUA_OK && lua_gettop(co) == 0i32 {
        lua_pushstring(L, s!(b"cannot resume dead coroutine\x00"));
        /* error flag */
        return -1i32;
    } else {
        lua_xmove(L, co, narg);
        status = lua_resume(co, L, narg);
        if status == LUA_OK || status == LUA_YIELD {
            let mut nres: libc::c_int = lua_gettop(co);
            if 0 == lua_checkstack(L, nres + 1i32) {
                /* remove results anyway */
                lua_settop(co, -nres - 1i32);
                lua_pushstring(L, s!(b"too many results to resume\x00"));
                /* error flag */
                return -1i32;
            } else {
                /* move yielded values */
                lua_xmove(co, L, nres);
                return nres;
            }
        } else {
            /* move error message */
            lua_xmove(co, L, 1i32);
            /* error flag */
            return -1i32;
        }
    };
}
unsafe extern "C" fn luaB_cocreate(mut L: *mut lua_State) -> libc::c_int {
    let mut NL: *mut lua_State = 0 as *mut lua_State;
    luaL_checktype(L, 1i32, 6i32);
    NL = lua_newthread(L);
    /* move function to top */
    lua_pushvalue(L, 1i32);
    /* move function from L to NL */
    lua_xmove(L, NL, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_costatus(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = getco(L);
    if L == co {
        lua_pushstring(L, s!(b"running\x00"));
    } else {
        match lua_status(co) {
            LUA_YIELD => {
                lua_pushstring(L, s!(b"suspended\x00"));
            }
            LUA_OK => {
                let mut ar: lua_Debug = lua_Debug {
                    event: 0,
                    name: 0 as *const libc::c_char,
                    namewhat: 0 as *const libc::c_char,
                    what: 0 as *const libc::c_char,
                    source: 0 as *const libc::c_char,
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
                /* does it have frames? */
                if lua_getstack(co, 0i32, &mut ar) > 0i32 {
                    /* it is running */
                    lua_pushstring(L, s!(b"normal\x00"));
                } else if lua_gettop(co) == 0i32 {
                    lua_pushstring(L, s!(b"dead\x00"));
                } else {
                    /* initial state */
                    lua_pushstring(L, s!(b"suspended\x00"));
                }
            }
            _ => {
                lua_pushstring(L, s!(b"dead\x00"));
            }
        }
    }
    return 1i32;
}
/*
** $Id: lcorolib.c,v 1.10.1.1 2017/04/19 17:20:42 roberto Exp $
** Coroutine Library
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn getco(mut L: *mut lua_State) -> *mut lua_State {
    let mut co: *mut lua_State = lua_tothread(L, 1i32);
    (!co.is_null() || 0 != luaL_argerror(L, 1i32, s!(b"thread expected\x00"))) as libc::c_int;
    return co;
}
unsafe extern "C" fn luaB_corunning(mut L: *mut lua_State) -> libc::c_int {
    let mut ismain: libc::c_int = lua_pushthread(L);
    lua_pushboolean(L, ismain);
    return 2i32;
}
unsafe extern "C" fn luaB_coresume(mut L: *mut lua_State) -> libc::c_int {
    let mut co: *mut lua_State = getco(L);
    let mut r: libc::c_int = 0;
    r = auxresume(L, co, lua_gettop(L) - 1i32);
    if r < 0i32 {
        lua_pushboolean(L, 0i32);
        lua_rotate(L, -2i32, 1i32);
        /* return false + error message */
        return 2i32;
    } else {
        lua_pushboolean(L, 1i32);
        lua_rotate(L, -(r + 1i32), 1i32);
        /* return true + 'resume' returns */
        return r + 1i32;
    };
}
