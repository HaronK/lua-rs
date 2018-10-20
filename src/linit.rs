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
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    /*
     ** $Id: lualib.h,v 1.45.1.1 2017/04/19 17:20:42 roberto Exp $
     ** Lua standard libraries
     ** See Copyright Notice in lua.h
     */
    /* version suffix for environment variable names */
    #[no_mangle]
    fn luaopen_base(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_coroutine(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_table(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_io(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_os(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_string(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_utf8(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_bit32(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_math(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_debug(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaopen_package(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaL_requiref(
        L: *mut lua_State,
        modname: *const libc::c_char,
        openf: lua_CFunction,
        glb: libc::c_int,
    ) -> ();
}
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
/* open all previous libraries */
#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State) -> () {
    let mut lib: *const luaL_Reg = 0 as *const luaL_Reg;
    /* "require" functions from 'loadedlibs' and set results to global table */
    lib = loadedlibs.as_ptr();
    while (*lib).func.is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1i32);
        /* remove lib */
        lua_settop(L, -1i32 - 1i32);
        lib = lib.offset(1isize)
    }
}
/*
** $Id: linit.c,v 1.39.1.1 2017/04/19 17:20:42 roberto Exp $
** Initialization of libraries for lua.c and other clients
** See Copyright Notice in lua.h
*/
/*
** If you embed Lua in your program and need to open the standard
** libraries, call luaL_openlibs in your program. If you need a
** different set of libraries, copy this file to your project and edit
** it to suit your needs.
**
** You can also *preload* libraries, so that a later 'require' can
** open the library, which is already linked to the application.
** For that, do the following code:
**
**  luaL_getsubtable(L, LUA_REGISTRYINDEX, LUA_PRELOAD_TABLE);
**  lua_pushcfunction(L, luaopen_modname);
**  lua_setfield(L, -2, modname);
**  lua_pop(L, 1);  // remove PRELOAD table
*/
/*
** these libs are loaded by lua.c and are readily available to any Lua
** program
*/
static mut loadedlibs: [luaL_Reg; 12] = [
    luaL_Reg {
        name: s!(b"_G\x00"),
        func: Some(luaopen_base),
    },
    luaL_Reg {
        name: s!(b"package\x00"),
        func: Some(luaopen_package),
    },
    luaL_Reg {
        name: s!(b"coroutine\x00"),
        func: Some(luaopen_coroutine),
    },
    luaL_Reg {
        name: s!(b"table\x00"),
        func: Some(luaopen_table),
    },
    luaL_Reg {
        name: s!(b"io\x00"),
        func: Some(luaopen_io),
    },
    luaL_Reg {
        name: s!(b"os\x00"),
        func: Some(luaopen_os),
    },
    luaL_Reg {
        name: s!(b"string\x00"),
        func: Some(luaopen_string),
    },
    luaL_Reg {
        name: s!(b"math\x00"),
        func: Some(luaopen_math),
    },
    luaL_Reg {
        name: s!(b"utf8\x00"),
        func: Some(luaopen_utf8),
    },
    luaL_Reg {
        name: s!(b"debug\x00"),
        func: Some(luaopen_debug),
    },
    luaL_Reg {
        name: s!(b"bit32\x00"),
        func: Some(luaopen_bit32),
    },
    luaL_Reg {
        name: 0 as *const libc::c_char,
        func: None,
    },
];
