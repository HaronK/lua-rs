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
    /* private part */
    pub type CallInfo;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
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
    fn lua_checkstack(L: *mut lua_State, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: libc::c_int, len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State, idx: libc::c_int) -> *mut lua_State;
    /*
    ** push functions (C -> stack)
    */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_getuservalue(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State, idx: libc::c_int, p: *const libc::c_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_setuservalue(L: *mut lua_State, idx: libc::c_int) -> ();
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
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: libc::c_int, ar: *mut lua_Debug) -> libc::c_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const libc::c_char, ar: *mut lua_Debug)
        -> libc::c_int;
    #[no_mangle]
    fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: libc::c_int)
        -> *const libc::c_char;
    #[no_mangle]
    fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: libc::c_int)
        -> *const libc::c_char;
    #[no_mangle]
    fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: libc::c_int,
        n: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_upvalueid(L: *mut lua_State, fidx: libc::c_int, n: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: libc::c_int,
        n1: libc::c_int,
        fidx2: libc::c_int,
        n2: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: libc::c_int, count: libc::c_int) -> ();
    #[no_mangle]
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    #[no_mangle]
    fn lua_gethookmask(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_gethookcount(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: libc::c_int,
        extramsg: *const libc::c_char,
    ) -> libc::c_int;
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
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checktype(L: *mut lua_State, arg: libc::c_int, t: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: libc::c_int,
    ) -> ();
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
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
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
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg; 17]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0i32);
    return 1i32;
}
static mut dblib: [luaL_Reg; 17] = unsafe {
    [
        luaL_Reg {
            name: b"debug\x00" as *const u8 as *const libc::c_char,
            func: Some(db_debug),
        },
        luaL_Reg {
            name: b"getuservalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getuservalue),
        },
        luaL_Reg {
            name: b"gethook\x00" as *const u8 as *const libc::c_char,
            func: Some(db_gethook),
        },
        luaL_Reg {
            name: b"getinfo\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getinfo),
        },
        luaL_Reg {
            name: b"getlocal\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getlocal),
        },
        luaL_Reg {
            name: b"getregistry\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getregistry),
        },
        luaL_Reg {
            name: b"getmetatable\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getmetatable),
        },
        luaL_Reg {
            name: b"getupvalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_getupvalue),
        },
        luaL_Reg {
            name: b"upvaluejoin\x00" as *const u8 as *const libc::c_char,
            func: Some(db_upvaluejoin),
        },
        luaL_Reg {
            name: b"upvalueid\x00" as *const u8 as *const libc::c_char,
            func: Some(db_upvalueid),
        },
        luaL_Reg {
            name: b"setuservalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setuservalue),
        },
        luaL_Reg {
            name: b"sethook\x00" as *const u8 as *const libc::c_char,
            func: Some(db_sethook),
        },
        luaL_Reg {
            name: b"setlocal\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setlocal),
        },
        luaL_Reg {
            name: b"setmetatable\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setmetatable),
        },
        luaL_Reg {
            name: b"setupvalue\x00" as *const u8 as *const libc::c_char,
            func: Some(db_setupvalue),
        },
        luaL_Reg {
            name: b"traceback\x00" as *const u8 as *const libc::c_char,
            func: Some(db_traceback),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> libc::c_int {
    let mut level: libc::c_int = 0;
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut msg: *const libc::c_char = lua_tolstring(L, arg + 1i32, 0 as *mut size_t);
    /* non-string 'msg'? */
    if msg.is_null() && !(lua_type(L, arg + 1i32) <= 0i32) {
        /* return it untouched */
        lua_pushvalue(L, arg + 1i32);
    } else {
        level = luaL_optinteger(
            L,
            arg + 2i32,
            (if L == L1 { 1i32 } else { 0i32 }) as lua_Integer,
        ) as libc::c_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1i32;
}
/*
** Auxiliary function used by several library functions: check for
** an optional thread as function's first argument and set 'arg' with
** 1 if this argument is present (so that functions can skip it to
** access their other arguments)
*/
unsafe extern "C" fn getthread(mut L: *mut lua_State, mut arg: *mut libc::c_int) -> *mut lua_State {
    if lua_type(L, 1i32) == 8i32 {
        *arg = 1i32;
        return lua_tothread(L, 1i32);
    } else {
        *arg = 0i32;
        /* function will operate over current thread */
        return L;
    };
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 3i32);
    return auxupvalue(L, 0i32);
}
/*
** get (if 'get' is true) or set an upvalue from a closure
*/
unsafe extern "C" fn auxupvalue(mut L: *mut lua_State, mut get: libc::c_int) -> libc::c_int {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* upvalue index */
    let mut n: libc::c_int = luaL_checkinteger(L, 2i32) as libc::c_int;
    /* closure */
    luaL_checktype(L, 1i32, 6i32);
    name = if 0 != get {
        lua_getupvalue(L, 1i32, n)
    } else {
        lua_setupvalue(L, 1i32, n)
    };
    if name.is_null() {
        return 0i32;
    } else {
        lua_pushstring(L, name);
        /* no-op if get is false */
        lua_rotate(L, -(get + 1i32), 1i32);
        return get + 1i32;
    };
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> libc::c_int {
    let mut t: libc::c_int = lua_type(L, 2i32);
    (t == 0i32 || t == 5i32
        || 0 != luaL_argerror(
            L,
            2i32,
            b"nil or table expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    /* return 1st argument */
    return 1i32;
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
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
    let mut level: libc::c_int = luaL_checkinteger(L, arg + 1i32) as libc::c_int;
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2i32) as libc::c_int;
    /* out of range? */
    if 0 == lua_getstack(L1, level, &mut ar) {
        return luaL_argerror(
            L,
            arg + 1i32,
            b"level out of range\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        luaL_checkany(L, arg + 3i32);
        lua_settop(L, arg + 3i32);
        checkstack(L, L1, 1i32);
        lua_xmove(L, L1, 1i32);
        name = lua_setlocal(L1, &mut ar, nvar);
        if name.is_null() {
            /* pop value (if not popped by 'lua_setlocal') */
            lua_settop(L1, -1i32 - 1i32);
        }
        lua_pushstring(L, name);
        return 1i32;
    };
}
/*
** If L1 != L, L1 can be in any state, and therefore there are no
** guarantees about its stack space; any push in L1 must be
** checked.
*/
unsafe extern "C" fn checkstack(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut n: libc::c_int,
) -> () {
    if L != L1 && 0 == lua_checkstack(L1, n) {
        luaL_error(L, b"stack overflow\x00" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> libc::c_int {
    let mut smask: *const libc::c_char = 0 as *const libc::c_char;
    let mut arg: libc::c_int = 0;
    let mut mask: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut func: lua_Hook = None;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    if lua_type(L, arg + 1i32) <= 0i32 {
        /* no hook? */
        lua_settop(L, arg + 1i32);
        func = None;
        mask = 0i32;
        /* turn off hooks */
        count = 0i32
    } else {
        smask = luaL_checklstring(L, arg + 2i32, 0 as *mut size_t);
        luaL_checktype(L, arg + 1i32, 6i32);
        count = luaL_optinteger(L, arg + 3i32, 0i32 as lua_Integer) as libc::c_int;
        func = Some(hookf);
        mask = makemask(smask, count)
    }
    if lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    ) == 0i32
    {
        /* create a hook table */
        lua_createtable(L, 0i32, 2i32);
        lua_pushvalue(L, -1i32);
        /* set it in position */
        lua_rawsetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        lua_pushstring(L, b"k\x00" as *const u8 as *const libc::c_char);
        /* * hooktable.__mode = "k" */
        lua_setfield(L, -2i32, b"__mode\x00" as *const u8 as *const libc::c_char);
        lua_pushvalue(L, -1i32);
        /* setmetatable(hooktable) = hooktable */
        lua_setmetatable(L, -2i32);
    }
    checkstack(L, L1, 1i32);
    lua_pushthread(L1);
    /* key (thread) */
    lua_xmove(L1, L, 1i32);
    /* value (hook function) */
    lua_pushvalue(L, arg + 1i32);
    /* hooktable[L1] = new Lua hook */
    lua_rawset(L, -3i32);
    lua_sethook(L1, func, mask, count);
    return 0i32;
}
/*
** $Id: ldblib.c,v 1.151.1.1 2017/04/19 17:20:42 roberto Exp $
** Interface from Lua to its debug API
** See Copyright Notice in lua.h
*/
/*
** The hook table at registry[&HOOKKEY] maps threads to their current
** hook function. (We only need the unique address of 'HOOKKEY'.)
*/
static mut HOOKKEY: libc::c_int = unsafe { 0i32 };
/*
** Convert a string mask (for 'sethook') into a bit mask
*/
unsafe extern "C" fn makemask(
    mut smask: *const libc::c_char,
    mut count: libc::c_int,
) -> libc::c_int {
    let mut mask: libc::c_int = 0i32;
    if !strchr(smask, 'c' as i32).is_null() {
        mask |= 1i32 << 0i32
    }
    if !strchr(smask, 'r' as i32).is_null() {
        mask |= 1i32 << 1i32
    }
    if !strchr(smask, 'l' as i32).is_null() {
        mask |= 1i32 << 2i32
    }
    if count > 0i32 {
        mask |= 1i32 << 3i32
    }
    return mask;
}
/*
** Call hook function registered at hook table for the current
** thread (if there is one)
*/
unsafe extern "C" fn hookf(mut L: *mut lua_State, mut ar: *mut lua_Debug) -> () {
    static mut hooknames: [*const libc::c_char; 5] = unsafe {
        [
            b"call\x00" as *const u8 as *const libc::c_char,
            b"return\x00" as *const u8 as *const libc::c_char,
            b"line\x00" as *const u8 as *const libc::c_char,
            b"count\x00" as *const u8 as *const libc::c_char,
            b"tail call\x00" as *const u8 as *const libc::c_char,
        ]
    };
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const libc::c_int as *const libc::c_void,
    );
    lua_pushthread(L);
    if lua_rawget(L, -2i32) == 6i32 {
        /* is there a hook function? */
        /* push event name */
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0i32 {
            /* push current line */
            lua_pushinteger(L, (*ar).currentline as lua_Integer);
        } else {
            lua_pushnil(L);
        }
        /* call hook function */
        lua_callk(L, 2i32, 0i32, 0i32 as lua_KContext, None);
    };
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> libc::c_int {
    luaL_checktype(L, 1i32, 7i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_setuservalue(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = checkupval(L, 1i32, 2i32);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1i32, n));
    return 1i32;
}
/*
** Check whether a given upvalue from a given closure exists and
** returns its index
*/
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: libc::c_int,
    mut argnup: libc::c_int,
) -> libc::c_int {
    /* upvalue index */
    let mut nup: libc::c_int = luaL_checkinteger(L, argnup) as libc::c_int;
    /* closure */
    luaL_checktype(L, argf, 6i32);
    (!lua_getupvalue(L, argf, nup).is_null()
        || 0 != luaL_argerror(
            L,
            argnup,
            b"invalid upvalue index\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    return nup;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> libc::c_int {
    let mut n1: libc::c_int = checkupval(L, 1i32, 2i32);
    let mut n2: libc::c_int = checkupval(L, 3i32, 4i32);
    (0 == lua_iscfunction(L, 1i32)
        || 0 != luaL_argerror(
            L,
            1i32,
            b"Lua function expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (0 == lua_iscfunction(L, 3i32)
        || 0 != luaL_argerror(
            L,
            3i32,
            b"Lua function expected\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_upvaluejoin(L, 1i32, n1, 3i32, n2);
    return 0i32;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> libc::c_int {
    return auxupvalue(L, 1i32);
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> libc::c_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        /* no metatable */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> libc::c_int {
    lua_pushvalue(L, -1000000i32 - 1000i32);
    return 1i32;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
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
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* local-variable index */
    let mut nvar: libc::c_int = luaL_checkinteger(L, arg + 2i32) as libc::c_int;
    if lua_type(L, arg + 1i32) == 6i32 {
        /* function argument? */
        /* push function */
        lua_pushvalue(L, arg + 1i32);
        /* push local name */
        lua_pushstring(L, lua_getlocal(L, 0 as *const lua_Debug, nvar));
        /* return only name (there is no value) */
        return 1i32;
    } else {
        /* stack-level argument */
        let mut level: libc::c_int = luaL_checkinteger(L, arg + 1i32) as libc::c_int;
        /* out of range? */
        if 0 == lua_getstack(L1, level, &mut ar) {
            return luaL_argerror(
                L,
                arg + 1i32,
                b"level out of range\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            checkstack(L, L1, 1i32);
            name = lua_getlocal(L1, &mut ar, nvar);
            if !name.is_null() {
                /* move local value */
                lua_xmove(L1, L, 1i32);
                /* push name */
                lua_pushstring(L, name);
                /* re-order */
                lua_rotate(L, -2i32, 1i32);
                return 2i32;
            } else {
                /* no name (nor value) */
                lua_pushnil(L);
                return 1i32;
            }
        }
    };
}
/*
** Calls 'lua_getinfo' and collects all results in a new table.
** L1 needs stack space for an optional input (function) plus
** two optional outputs (function and line table) from function
** 'lua_getinfo'.
*/
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> libc::c_int {
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
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut options: *const libc::c_char = luaL_optlstring(
        L,
        arg + 2i32,
        b"flnStu\x00" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    checkstack(L, L1, 3i32);
    if lua_type(L, arg + 1i32) == 6i32 {
        /* info about a function? */
        /* add '>' to 'options' */
        options = lua_pushfstring(L, b">%s\x00" as *const u8 as *const libc::c_char, options);
        /* move function to 'L1' stack */
        lua_pushvalue(L, arg + 1i32);
        lua_xmove(L, L1, 1i32);
    } else if 0 == lua_getstack(L1, luaL_checkinteger(L, arg + 1i32) as libc::c_int, &mut ar) {
        /* level out of range */
        lua_pushnil(L);
        return 1i32;
    }
    if 0 == lua_getinfo(L1, options, &mut ar) {
        return luaL_argerror(
            L,
            arg + 2i32,
            b"invalid option\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        /* table to collect results */
        lua_createtable(L, 0i32, 0i32);
        if !strchr(options, 'S' as i32).is_null() {
            settabss(
                L,
                b"source\x00" as *const u8 as *const libc::c_char,
                ar.source,
            );
            settabss(
                L,
                b"short_src\x00" as *const u8 as *const libc::c_char,
                ar.short_src.as_mut_ptr(),
            );
            settabsi(
                L,
                b"linedefined\x00" as *const u8 as *const libc::c_char,
                ar.linedefined,
            );
            settabsi(
                L,
                b"lastlinedefined\x00" as *const u8 as *const libc::c_char,
                ar.lastlinedefined,
            );
            settabss(L, b"what\x00" as *const u8 as *const libc::c_char, ar.what);
        }
        if !strchr(options, 'l' as i32).is_null() {
            settabsi(
                L,
                b"currentline\x00" as *const u8 as *const libc::c_char,
                ar.currentline,
            );
        }
        if !strchr(options, 'u' as i32).is_null() {
            settabsi(
                L,
                b"nups\x00" as *const u8 as *const libc::c_char,
                ar.nups as libc::c_int,
            );
            settabsi(
                L,
                b"nparams\x00" as *const u8 as *const libc::c_char,
                ar.nparams as libc::c_int,
            );
            settabsb(
                L,
                b"isvararg\x00" as *const u8 as *const libc::c_char,
                ar.isvararg as libc::c_int,
            );
        }
        if !strchr(options, 'n' as i32).is_null() {
            settabss(L, b"name\x00" as *const u8 as *const libc::c_char, ar.name);
            settabss(
                L,
                b"namewhat\x00" as *const u8 as *const libc::c_char,
                ar.namewhat,
            );
        }
        if !strchr(options, 't' as i32).is_null() {
            settabsb(
                L,
                b"istailcall\x00" as *const u8 as *const libc::c_char,
                ar.istailcall as libc::c_int,
            );
        }
        if !strchr(options, 'L' as i32).is_null() {
            treatstackoption(
                L,
                L1,
                b"activelines\x00" as *const u8 as *const libc::c_char,
            );
        }
        if !strchr(options, 'f' as i32).is_null() {
            treatstackoption(L, L1, b"func\x00" as *const u8 as *const libc::c_char);
        }
        /* return table */
        return 1i32;
    };
}
/*
** In function 'db_getinfo', the call to 'lua_getinfo' may push
** results on the stack; later it creates the result table to put
** these objects. Function 'treatstackoption' puts the result from
** 'lua_getinfo' on top of the result table so that it can call
** 'lua_setfield'.
*/
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut fname: *const libc::c_char,
) -> () {
    if L == L1 {
        /* exchange object and table */
        lua_rotate(L, -2i32, 1i32);
    } else {
        /* move object to the "main" stack */
        lua_xmove(L1, L, 1i32);
    }
    /* put object into table */
    lua_setfield(L, -2i32, fname);
}
unsafe extern "C" fn settabsb(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) -> () {
    lua_pushboolean(L, v);
    lua_setfield(L, -2i32, k);
}
/*
** Variations of 'lua_settable', used by 'db_getinfo' to put results
** from 'lua_getinfo' into result table. Key is always a string;
** value can be a string, an int, or a boolean.
*/
unsafe extern "C" fn settabss(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: *const libc::c_char,
) -> () {
    lua_pushstring(L, v);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn settabsi(
    mut L: *mut lua_State,
    mut k: *const libc::c_char,
    mut v: libc::c_int,
) -> () {
    lua_pushinteger(L, v as lua_Integer);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> libc::c_int {
    let mut arg: libc::c_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut buff: [libc::c_char; 5] = [0; 5];
    let mut mask: libc::c_int = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    /* no hook? */
    if hook.is_none() {
        lua_pushnil(L);
    } else if hook != Some(hookf) {
        lua_pushstring(L, b"external hook\x00" as *const u8 as *const libc::c_char);
    } else {
        /* hook table must exist */
        lua_rawgetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const libc::c_int as *const libc::c_void,
        );
        checkstack(L, L1, 1i32);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1i32);
        /* 1st result = hooktable[L1] */
        lua_rawget(L, -2i32);
        /* remove hook table */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* 2nd result = mask */
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    /* 3rd result = count */
    lua_pushinteger(L, lua_gethookcount(L1) as lua_Integer);
    return 3i32;
}
/*
** Convert a bit mask (for 'gethook') into a string mask
*/
unsafe extern "C" fn unmakemask(
    mut mask: libc::c_int,
    mut smask: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0i32;
    if 0 != mask & 1i32 << 0i32 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as libc::c_char
    }
    if 0 != mask & 1i32 << 1i32 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as libc::c_char
    }
    if 0 != mask & 1i32 << 2i32 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as libc::c_char
    }
    *smask.offset(i as isize) = '\u{0}' as i32 as libc::c_char;
    return smask;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> libc::c_int {
    if lua_type(L, 1i32) != 7i32 {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1i32);
    }
    return 1i32;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> libc::c_int {
    loop {
        let mut buffer: [libc::c_char; 250] = [0; 250];
        fprintf(
            stderr,
            b"%s\x00" as *const u8 as *const libc::c_char,
            b"lua_debug> \x00" as *const u8 as *const libc::c_char,
        );
        fflush(stderr);
        if fgets(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 250]>() as libc::c_ulong as libc::c_int,
            stdin,
        ).is_null()
            || strcmp(
                buffer.as_mut_ptr(),
                b"cont\n\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
        {
            return 0i32;
        } else {
            if 0 != luaL_loadbufferx(
                L,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                b"=(debug command)\x00" as *const u8 as *const libc::c_char,
                0 as *const libc::c_char,
            ) || 0 != lua_pcallk(L, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None)
            {
                fprintf(
                    stderr,
                    b"%s\n\x00" as *const u8 as *const libc::c_char,
                    lua_tolstring(L, -1i32, 0 as *mut size_t),
                );
                fflush(stderr);
            }
            /* remove eventual returns */
            lua_settop(L, 0i32);
        }
    }
}
