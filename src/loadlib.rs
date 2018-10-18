use libc;
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: libc::c_int, len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    /*
    ** push functions (C -> stack)
    */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    /*
    ** 'load' and 'call' functions (load and run Lua code)
    */
    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: libc::c_int,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_optlstring(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        l: *mut size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_gsub(
        L: *mut lua_State,
        s: *const libc::c_char,
        p: *const libc::c_char,
        r: *const libc::c_char,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_getsubtable(
        L: *mut lua_State,
        idx: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn dlsym(__handle: *mut libc::c_void, __name: *const libc::c_char) -> *mut libc::c_void;
    #[no_mangle]
    fn dlerror() -> *mut libc::c_char;
    #[no_mangle]
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
    pub b: *mut libc::c_char,
    pub size: size_t,
    pub n: size_t,
    pub L: *mut lua_State,
    pub initb: [libc::c_char; 8192],
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State) -> libc::c_int {
    createclibstable(L);
    /* create 'package' table */
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
    luaL_setfuncs(L, pk_funcs.as_ptr(), 0i32);
    createsearcherstable(L);
    /* set paths */
    setpath(L, b"path\x00" as *const u8 as *const libc::c_char,
            b"LUA_PATH\x00" as *const u8 as *const libc::c_char,
            b"/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua\x00"
                as *const u8 as *const libc::c_char);
    setpath(
        L,
        b"cpath\x00" as *const u8 as *const libc::c_char,
        b"LUA_CPATH\x00" as *const u8 as *const libc::c_char,
        b"/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so\x00" as *const u8
            as *const libc::c_char,
    );
    /* store config information */
    lua_pushstring(
        L,
        b"/\n;\n?\n!\n-\n\x00" as *const u8 as *const libc::c_char,
    );
    lua_setfield(L, -2i32, b"config\x00" as *const u8 as *const libc::c_char);
    /* set field 'loaded' */
    luaL_getsubtable(
        L,
        -1000000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    lua_setfield(L, -2i32, b"loaded\x00" as *const u8 as *const libc::c_char);
    /* set field 'preload' */
    luaL_getsubtable(
        L,
        -1000000i32 - 1000i32,
        b"_PRELOAD\x00" as *const u8 as *const libc::c_char,
    );
    lua_setfield(L, -2i32, b"preload\x00" as *const u8 as *const libc::c_char);
    lua_rawgeti(L, -1000000i32 - 1000i32, 2i32 as lua_Integer);
    /* set 'package' as upvalue for next lib */
    lua_pushvalue(L, -2i32);
    /* open lib into global table */
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1i32);
    /* pop global table */
    lua_settop(L, -1i32 - 1i32);
    /* return 'package' table */
    return 1i32;
}
/* placeholders */
static mut ll_funcs: [luaL_Reg; 2] = unsafe {
    [
        luaL_Reg {
            name: b"require\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_require),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn ll_require(mut L: *mut lua_State) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    /* LOADED table will be at index 2 */
    lua_settop(L, 1i32);
    lua_getfield(
        L,
        -1000000i32 - 1000i32,
        b"_LOADED\x00" as *const u8 as *const libc::c_char,
    );
    /* LOADED[name] */
    lua_getfield(L, 2i32, name);
    /* is it there? */
    if 0 != lua_toboolean(L, -1i32) {
        /* package is already loaded */
        return 1i32;
    } else {
        /* else must load package */
        /* remove 'getfield' result */
        lua_settop(L, -1i32 - 1i32);
        findloader(L, name);
        /* pass name as argument to module loader */
        lua_pushstring(L, name);
        /* name is 1st argument (before search data) */
        lua_rotate(L, -2i32, 1i32);
        /* run loader to load module */
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        /* non-nil return? */
        if !(lua_type(L, -1i32) == 0i32) {
            /* LOADED[name] = returned value */
            lua_setfield(L, 2i32, name);
        }
        if lua_getfield(L, 2i32, name) == 0i32 {
            /* module set no value? */
            /* use true as result */
            lua_pushboolean(L, 1i32);
            /* extra copy to be returned */
            lua_pushvalue(L, -1i32);
            /* LOADED[name] = true */
            lua_setfield(L, 2i32, name);
        }
        return 1i32;
    };
}
unsafe extern "C" fn findloader(mut L: *mut lua_State, mut name: *const libc::c_char) -> () {
    let mut i: libc::c_int = 0;
    /* to build error message */
    let mut msg: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    /* push 'package.searchers' to index 3 in the stack */
    if lua_getfield(
        L,
        -1000000i32 - 1000i32 - 1i32,
        b"searchers\x00" as *const u8 as *const libc::c_char,
    ) != 5i32
    {
        luaL_error(
            L,
            b"\'package.searchers\' must be a table\x00" as *const u8 as *const libc::c_char,
        );
    }
    /*  iterate over available searchers to find a loader */
    i = 1i32;
    loop {
        if lua_rawgeti(L, 3i32, i as lua_Integer) == 0i32 {
            /* no more searchers? */
            /* remove nil */
            lua_settop(L, -1i32 - 1i32);
            /* create error message */
            luaL_pushresult(&mut msg);
            luaL_error(
                L,
                b"module \'%s\' not found:%s\x00" as *const u8 as *const libc::c_char,
                name,
                lua_tolstring(L, -1i32, 0 as *mut size_t),
            );
        }
        lua_pushstring(L, name);
        /* call it */
        lua_callk(L, 1i32, 2i32, 0i32 as lua_KContext, None);
        /* did it find a loader? */
        if lua_type(L, -2i32) == 6i32 {
            /* module loader found */
            return;
        } else {
            if 0 != lua_isstring(L, -2i32) {
                /* searcher returned error message? */
                /* remove extra return */
                lua_settop(L, -1i32 - 1i32);
                /* concatenate error message */
                luaL_addvalue(&mut msg);
            } else {
                /* remove both returns */
                lua_settop(L, -2i32 - 1i32);
            }
            i += 1
        }
    }
}
/*
** Set a path
*/
unsafe extern "C" fn setpath(
    mut L: *mut lua_State,
    mut fieldname: *const libc::c_char,
    mut envname: *const libc::c_char,
    mut dft: *const libc::c_char,
) -> () {
    let mut nver: *const libc::c_char = lua_pushfstring(
        L,
        b"%s%s\x00" as *const u8 as *const libc::c_char,
        envname,
        b"_5_3\x00" as *const u8 as *const libc::c_char,
    );
    /* use versioned name */
    let mut path: *const libc::c_char = getenv(nver);
    /* no environment variable? */
    if path.is_null() {
        /* try unversioned name */
        path = getenv(envname)
    }
    /* no environment variable? */
    if path.is_null() || 0 != noenv(L) {
        /* use default */
        lua_pushstring(L, dft);
    } else {
        /* replace ";;" by ";AUXMARK;" and then AUXMARK by default path */
        path = luaL_gsub(
            L,
            path,
            b";;\x00" as *const u8 as *const libc::c_char,
            b";\x01;\x00" as *const u8 as *const libc::c_char,
        );
        luaL_gsub(
            L,
            path,
            b"\x01\x00" as *const u8 as *const libc::c_char,
            dft,
        );
        /* remove result from 1st 'gsub' */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* package[fieldname] = path value */
    lua_setfield(L, -3i32, fieldname);
    /* pop versioned variable name */
    lua_settop(L, -1i32 - 1i32);
}
/* }====================================================== */
/* }{ */
/* } */
/*
** {==================================================================
** Set Paths
** ===================================================================
*/
/*
** LUA_PATH_VAR and LUA_CPATH_VAR are the names of the environment
** variables that Lua check to set its paths.
*/
/* auxiliary mark */
/*
** return registry.LUA_NOENV as a boolean
*/
unsafe extern "C" fn noenv(mut L: *mut lua_State) -> libc::c_int {
    let mut b: libc::c_int = 0;
    lua_getfield(
        L,
        -1000000i32 - 1000i32,
        b"LUA_NOENV\x00" as *const u8 as *const libc::c_char,
    );
    b = lua_toboolean(L, -1i32);
    /* remove value */
    lua_settop(L, -1i32 - 1i32);
    return b;
}
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State) -> () {
    static mut searchers: [lua_CFunction; 5] = unsafe {
        [
            Some(searcher_preload),
            Some(searcher_Lua),
            Some(searcher_C),
            Some(searcher_Croot),
            None,
        ]
    };
    let mut i: libc::c_int = 0;
    /* create 'searchers' table */
    lua_createtable(
        L,
        (::std::mem::size_of::<[lua_CFunction; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<lua_CFunction>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
        0i32,
    );
    /* fill it with predefined searchers */
    i = 0i32;
    while searchers[i as usize].is_some() {
        /* set 'package' as upvalue for all searchers */
        lua_pushvalue(L, -2i32);
        lua_pushcclosure(L, searchers[i as usize], 1i32);
        lua_rawseti(L, -2i32, (i + 1i32) as lua_Integer);
        i += 1
    }
    /* put it in field 'searchers' */
    lua_setfield(
        L,
        -2i32,
        b"searchers\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut p: *const libc::c_char = strchr(name, '.' as i32);
    let mut stat: libc::c_int = 0;
    if p.is_null() {
        /* is root */
        return 0i32;
    } else {
        lua_pushlstring(
            L,
            name,
            p.wrapping_offset_from(name) as libc::c_long as size_t,
        );
        filename = findfile(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            b"cpath\x00" as *const u8 as *const libc::c_char,
            b"/\x00" as *const u8 as *const libc::c_char,
        );
        if filename.is_null() {
            /* root not found */
            return 1i32;
        } else {
            stat = loadfunc(L, filename, name);
            if stat != 0i32 {
                if stat != 2i32 {
                    /* real error */
                    return checkload(L, 0i32, filename);
                } else {
                    /* open function not found */
                    lua_pushfstring(
                        L,
                        b"\n\tno module \'%s\' in file \'%s\'\x00" as *const u8
                            as *const libc::c_char,
                        name,
                        filename,
                    );
                    return 1i32;
                }
            } else {
                /* will be 2nd argument to module */
                lua_pushstring(L, filename);
                return 2i32;
            }
        }
    };
}
unsafe extern "C" fn checkload(
    mut L: *mut lua_State,
    mut stat: libc::c_int,
    mut filename: *const libc::c_char,
) -> libc::c_int {
    if 0 != stat {
        /* module loaded successfully? */
        /* will be 2nd argument to module */
        lua_pushstring(L, filename);
        /* return open function and file name */
        return 2i32;
    } else {
        return luaL_error(
            L,
            b"error loading module \'%s\' from file \'%s\':\n\t%s\x00" as *const u8
                as *const libc::c_char,
            lua_tolstring(L, 1i32, 0 as *mut size_t),
            filename,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
        );
    };
}
/*
** Try to find a load function for module 'modname' at file 'filename'.
** First, change '.' to '_' in 'modname'; then, if 'modname' has
** the form X-Y (that is, it has an "ignore mark"), build a function
** name "luaopen_X" and look for it. (For compatibility, if that
** fails, it also tries "luaopen_Y".) If there is no ignore mark,
** look for a function named "luaopen_modname".
*/
unsafe extern "C" fn loadfunc(
    mut L: *mut lua_State,
    mut filename: *const libc::c_char,
    mut modname: *const libc::c_char,
) -> libc::c_int {
    let mut openfunc: *const libc::c_char = 0 as *const libc::c_char;
    let mut mark: *const libc::c_char = 0 as *const libc::c_char;
    modname = luaL_gsub(
        L,
        modname,
        b".\x00" as *const u8 as *const libc::c_char,
        b"_\x00" as *const u8 as *const libc::c_char,
    );
    mark = strchr(
        modname,
        *(b"-\x00" as *const u8 as *const libc::c_char) as libc::c_int,
    );
    if !mark.is_null() {
        let mut stat: libc::c_int = 0;
        openfunc = lua_pushlstring(
            L,
            modname,
            mark.wrapping_offset_from(modname) as libc::c_long as size_t,
        );
        openfunc = lua_pushfstring(
            L,
            b"luaopen_%s\x00" as *const u8 as *const libc::c_char,
            openfunc,
        );
        stat = lookforfunc(L, filename, openfunc);
        if stat != 2i32 {
            return stat;
        } else {
            modname = mark.offset(1isize)
        }
    }
    openfunc = lua_pushfstring(
        L,
        b"luaopen_%s\x00" as *const u8 as *const libc::c_char,
        modname,
    );
    return lookforfunc(L, filename, openfunc);
}
/* error codes for 'lookforfunc' */
/*
** Look for a C function named 'sym' in a dynamically loaded library
** 'path'.
** First, check whether the library is already loaded; if not, try
** to load it.
** Then, if 'sym' is '*', return true (as library has been loaded).
** Otherwise, look for symbol 'sym' in the library and push a
** C function with that symbol.
** Return 0 and 'true' or a function in the stack; in case of
** errors, return an error code and an error message in the stack.
*/
unsafe extern "C" fn lookforfunc(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut sym: *const libc::c_char,
) -> libc::c_int {
    /* check loaded C libraries */
    let mut reg: *mut libc::c_void = checkclib(L, path);
    if reg.is_null() {
        /* must load library? */
        /* global symbols if 'sym'=='*' */
        reg = lsys_load(L, path, (*sym as libc::c_int == '*' as i32) as libc::c_int);
        if reg.is_null() {
            /* unable to load library */
            return 1i32;
        } else {
            addtoclib(L, path, reg);
        }
    }
    if *sym as libc::c_int == '*' as i32 {
        /* loading only library (no function)? */
        /* return 'true' */
        lua_pushboolean(L, 1i32);
        /* no errors */
        return 0i32;
    } else {
        let mut f: lua_CFunction = lsys_sym(L, reg, sym);
        if f.is_none() {
            /* unable to find function */
            return 2i32;
        } else {
            /* else create new function */
            lua_pushcclosure(L, f, 0i32);
            /* no errors */
            return 0i32;
        }
    };
}
/* }================================================================== */
/*
** return registry.CLIBS[path]
*/
unsafe extern "C" fn checkclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) -> *mut libc::c_void {
    let mut plib: *mut libc::c_void = 0 as *mut libc::c_void;
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_getfield(L, -1i32, path);
    /* plib = CLIBS[path] */
    plib = lua_touserdata(L, -1i32);
    /* pop CLIBS table and 'plib' */
    lua_settop(L, -2i32 - 1i32);
    return plib;
}
/*
** $Id: loadlib.c,v 1.130.1.1 2017/04/19 17:20:42 roberto Exp $
** Dynamic library loader for Lua
** See Copyright Notice in lua.h
**
** This module contains an implementation of loadlib for Unix systems
** that have dlfcn, an implementation for Windows, and a stub for other
** systems.
*/
/*
** LUA_IGMARK is a mark to ignore all before it when building the
** luaopen_ function name.
*/
/*
** LUA_CSUBSEP is the character that replaces dots in submodule names
** when searching for a C loader.
** LUA_LSUBSEP is the character that replaces dots in submodule names
** when searching for a Lua loader.
*/
/* prefix for open functions in C libraries */
/* separator for open functions in C libraries */
/*
** unique key for table in the registry that keeps handles
** for all loaded C libraries
*/
static mut CLIBS: libc::c_int = unsafe { 0i32 };
/*
** Try to find a function named 'sym' in library 'lib'.
** Returns the function; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_sym(
    mut L: *mut lua_State,
    mut lib: *mut libc::c_void,
    mut sym: *const libc::c_char,
) -> lua_CFunction {
    let mut f: lua_CFunction =
        ::std::mem::transmute::<*mut libc::c_void, lua_CFunction>(dlsym(lib, sym));
    if f.is_none() {
        lua_pushstring(L, dlerror());
    }
    return f;
}
/*
** registry.CLIBS[path] = plib        -- for queries
** registry.CLIBS[#CLIBS + 1] = plib  -- also keep a list of all libraries
*/
unsafe extern "C" fn addtoclib(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut plib: *mut libc::c_void,
) -> () {
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -1i32);
    /* CLIBS[path] = plib */
    lua_setfield(L, -3i32, path);
    /* CLIBS[#CLIBS + 1] = plib */
    lua_rawseti(L, -2i32, luaL_len(L, -2i32) + 1i32 as libc::c_longlong);
    /* pop CLIBS table */
    lua_settop(L, -1i32 - 1i32);
}
/*
** load C library in file 'path'. If 'seeglb', load with all names in
** the library global.
** Returns the library; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_load(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
    mut seeglb: libc::c_int,
) -> *mut libc::c_void {
    let mut lib: *mut libc::c_void =
        dlopen(path, 0x2i32 | if 0 != seeglb { 0x100i32 } else { 0i32 });
    if lib.is_null() {
        lua_pushstring(L, dlerror());
    }
    return lib;
}
unsafe extern "C" fn findfile(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut pname: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    lua_getfield(L, -1000000i32 - 1000i32 - 1i32, pname);
    path = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if path.is_null() {
        luaL_error(
            L,
            b"\'package.%s\' must be a string\x00" as *const u8 as *const libc::c_char,
            pname,
        );
    }
    return searchpath(
        L,
        name,
        path,
        b".\x00" as *const u8 as *const libc::c_char,
        dirsep,
    );
}
unsafe extern "C" fn searchpath(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
    mut path: *const libc::c_char,
    mut sep: *const libc::c_char,
    mut dirsep: *const libc::c_char,
) -> *const libc::c_char {
    /* to build error message */
    let mut msg: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    /* non-empty separator? */
    if *sep as libc::c_int != '\u{0}' as i32 {
        /* replace it by 'dirsep' */
        name = luaL_gsub(L, name, sep, dirsep)
    }
    loop {
        path = pushnexttemplate(L, path);
        if path.is_null() {
            break;
        }
        let mut filename: *const libc::c_char = luaL_gsub(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            b"?\x00" as *const u8 as *const libc::c_char,
            name,
        );
        /* remove path template */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* does file exist and is readable? */
        if 0 != readable(filename) {
            /* return that file name */
            return filename;
        } else {
            lua_pushfstring(
                L,
                b"\n\tno file \'%s\'\x00" as *const u8 as *const libc::c_char,
                filename,
            );
            /* remove file name */
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
            /* concatenate error msg. entry */
            luaL_addvalue(&mut msg);
        }
    }
    /* create error message */
    luaL_pushresult(&mut msg);
    /* not found */
    return 0 as *const libc::c_char;
}
/*
** {======================================================
** 'require' function
** =======================================================
*/
unsafe extern "C" fn readable(mut filename: *const libc::c_char) -> libc::c_int {
    /* try to open file */
    let mut f: *mut FILE = fopen(filename, b"r\x00" as *const u8 as *const libc::c_char);
    if f.is_null() {
        /* open failed */
        return 0i32;
    } else {
        fclose(f);
        return 1i32;
    };
}
unsafe extern "C" fn pushnexttemplate(
    mut L: *mut lua_State,
    mut path: *const libc::c_char,
) -> *const libc::c_char {
    let mut l: *const libc::c_char = 0 as *const libc::c_char;
    while *path as libc::c_int == *(b";\x00" as *const u8 as *const libc::c_char) as libc::c_int {
        /* skip separators */
        path = path.offset(1isize)
    }
    if *path as libc::c_int == '\u{0}' as i32 {
        /* no more templates */
        return 0 as *const libc::c_char;
    } else {
        /* find next separator */
        l = strchr(
            path,
            *(b";\x00" as *const u8 as *const libc::c_char) as libc::c_int,
        );
        if l.is_null() {
            l = path.offset(strlen(path) as isize)
        }
        /* template */
        lua_pushlstring(
            L,
            path,
            l.wrapping_offset_from(path) as libc::c_long as size_t,
        );
        return l;
    };
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut filename: *const libc::c_char = findfile(
        L,
        name,
        b"cpath\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (loadfunc(L, filename, name) == 0i32) as libc::c_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    filename = findfile(
        L,
        name,
        b"path\x00" as *const u8 as *const libc::c_char,
        b"/\x00" as *const u8 as *const libc::c_char,
    );
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (luaL_loadfilex(L, filename, 0 as *const libc::c_char) == 0i32) as libc::c_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State) -> libc::c_int {
    let mut name: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_getfield(
        L,
        -1000000i32 - 1000i32,
        b"_PRELOAD\x00" as *const u8 as *const libc::c_char,
    );
    /* not found? */
    if lua_getfield(L, -1i32, name) == 0i32 {
        lua_pushfstring(
            L,
            b"\n\tno field package.preload[\'%s\']\x00" as *const u8 as *const libc::c_char,
            name,
        );
    }
    return 1i32;
}
/* }====================================================== */
/*
** {======================================================
** 'module' function
** =======================================================
*/
/* }====================================================== */
static mut pk_funcs: [luaL_Reg; 8] = unsafe {
    [
        luaL_Reg {
            name: b"loadlib\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_loadlib),
        },
        luaL_Reg {
            name: b"searchpath\x00" as *const u8 as *const libc::c_char,
            func: Some(ll_searchpath),
        },
        luaL_Reg {
            name: b"preload\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"cpath\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"path\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"searchers\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: b"loaded\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State) -> libc::c_int {
    let mut f: *const libc::c_char = searchpath(
        L,
        luaL_checklstring(L, 1i32, 0 as *mut size_t),
        luaL_checklstring(L, 2i32, 0 as *mut size_t),
        luaL_optlstring(
            L,
            3i32,
            b".\x00" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
        luaL_optlstring(
            L,
            4i32,
            b"/\x00" as *const u8 as *const libc::c_char,
            0 as *mut size_t,
        ),
    );
    if !f.is_null() {
        return 1i32;
    } else {
        /* error message is on top of the stack */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        /* return nil + error message */
        return 2i32;
    };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State) -> libc::c_int {
    let mut path: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut init: *const libc::c_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
    let mut stat: libc::c_int = lookforfunc(L, path, init);
    /* no errors? */
    if stat == 0i32 {
        /* return the loaded function */
        return 1i32;
    } else {
        /* error; error message is on stack top */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        lua_pushstring(
            L,
            if stat == 1i32 {
                b"open\x00" as *const u8 as *const libc::c_char
            } else {
                b"init\x00" as *const u8 as *const libc::c_char
            },
        );
        /* return nil, error message, and where */
        return 3i32;
    };
}
/*
** create table CLIBS to keep track of loaded C libraries,
** setting a finalizer to close all libraries when closing state.
*/
unsafe extern "C" fn createclibstable(mut L: *mut lua_State) -> () {
    /* create CLIBS table */
    lua_createtable(L, 0i32, 0i32);
    /* create metatable for CLIBS */
    lua_createtable(L, 0i32, 1i32);
    lua_pushcclosure(L, Some(gctm), 0i32);
    /* set finalizer for CLIBS table */
    lua_setfield(L, -2i32, b"__gc\x00" as *const u8 as *const libc::c_char);
    lua_setmetatable(L, -2i32);
    /* set CLIBS table in registry */
    lua_rawsetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const libc::c_int as *const libc::c_void,
    );
}
/*
** __gc tag method for CLIBS table: calls 'lsys_unloadlib' for all lib
** handles in list CLIBS
*/
unsafe extern "C" fn gctm(mut L: *mut lua_State) -> libc::c_int {
    let mut n: lua_Integer = luaL_len(L, 1i32);
    while n >= 1i32 as libc::c_longlong {
        /* for each handle, in reverse order */
        /* get handle CLIBS[n] */
        lua_rawgeti(L, 1i32, n);
        lsys_unloadlib(lua_touserdata(L, -1i32));
        /* pop handle */
        lua_settop(L, -1i32 - 1i32);
        n -= 1
    }
    return 0i32;
}
/*
** system-dependent functions
*/
/*
** unload library 'lib'
*/
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut libc::c_void) -> () {
    dlclose(lib);
}
