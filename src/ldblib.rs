use types::*;
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
    fn fflush(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn fgets(__s: *mut lua_char, __n: lua_int, __stream: *mut FILE) -> *mut lua_char;
    #[no_mangle]
    fn strcmp(_: *const lua_char, _: *const lua_char) -> lua_int;
    #[no_mangle]
    fn strchr(_: *const lua_char, _: lua_int) -> *mut lua_char;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: lua_int, n: lua_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: lua_int) -> ();
    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_tothread(L: *mut lua_State, idx: lua_int) -> *mut lua_State;
    /*
     ** push functions (C -> stack)
     */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State, fmt: *const lua_char, ...) -> *const lua_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut lua_void) -> ();
    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State, idx: lua_int, p: *const lua_void) -> lua_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_getuservalue(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State, idx: lua_int, p: *const lua_void) -> ();
    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_setuservalue(L: *mut lua_State, idx: lua_int) -> ();
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
    fn lua_getstack(L: *mut lua_State, level: lua_int, ar: *mut lua_Debug) -> lua_int;
    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const lua_char, ar: *mut lua_Debug)
        -> lua_int;
    #[no_mangle]
    fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: lua_int)
        -> *const lua_char;
    #[no_mangle]
    fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: lua_int)
        -> *const lua_char;
    #[no_mangle]
    fn lua_getupvalue(
        L: *mut lua_State,
        funcindex: lua_int,
        n: lua_int,
    ) -> *const lua_char;
    #[no_mangle]
    fn lua_setupvalue(
        L: *mut lua_State,
        funcindex: lua_int,
        n: lua_int,
    ) -> *const lua_char;
    #[no_mangle]
    fn lua_upvalueid(L: *mut lua_State, fidx: lua_int, n: lua_int) -> *mut lua_void;
    #[no_mangle]
    fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: lua_int,
        n1: lua_int,
        fidx2: lua_int,
        n2: lua_int,
    ) -> ();
    #[no_mangle]
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: lua_int, count: lua_int) -> ();
    #[no_mangle]
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;
    #[no_mangle]
    fn lua_gethookmask(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_gethookcount(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(
        L: *mut lua_State,
        arg: lua_int,
        extramsg: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_checklstring(
        L: *mut lua_State,
        arg: lua_int,
        l: *mut size_t,
    ) -> *const lua_char;
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
    fn luaL_checktype(L: *mut lua_State, arg: lua_int, t: lua_int) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, arg: lua_int) -> ();
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
    #[no_mangle]
    fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const lua_char,
        level: lua_int,
    ) -> ();
}
pub type size_t = lua_ulong;
pub type __off_t = lua_long;
pub type __off64_t = lua_long;
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
    pub name: *const lua_char,
    pub func: lua_CFunction,
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 17]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0i32);
    return 1i32;
}
static mut dblib: [luaL_Reg; 17] = [
    lua_reg!(b"debug\x00", db_debug),
    lua_reg!(b"getuservalue\x00", db_getuservalue),
    lua_reg!(b"gethook\x00", db_gethook),
    lua_reg!(b"getinfo\x00", db_getinfo),
    lua_reg!(b"getlocal\x00", db_getlocal),
    lua_reg!(b"getregistry\x00", db_getregistry),
    lua_reg!(b"getmetatable\x00", db_getmetatable),
    lua_reg!(b"getupvalue\x00", db_getupvalue),
    lua_reg!(b"upvaluejoin\x00", db_upvaluejoin),
    lua_reg!(b"upvalueid\x00", db_upvalueid),
    lua_reg!(b"setuservalue\x00", db_setuservalue),
    lua_reg!(b"sethook\x00", db_sethook),
    lua_reg!(b"setlocal\x00", db_setlocal),
    lua_reg!(b"setmetatable\x00", db_setmetatable),
    lua_reg!(b"setupvalue\x00", db_setupvalue),
    lua_reg!(b"traceback\x00", db_traceback),
    lua_reg_none!(0),
];
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> lua_int {
    let mut level: lua_int = 0;
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut msg: *const lua_char = lua_tolstring(L, arg + 1i32, 0 as *mut size_t);
    /* non-string 'msg'? */
    if msg.is_null() && !(lua_type(L, arg + 1i32) <= 0i32) {
        /* return it untouched */
        lua_pushvalue(L, arg + 1i32);
    } else {
        level = luaL_optinteger(
            L,
            arg + 2i32,
            (if L == L1 { 1i32 } else { 0i32 }) as lua_Integer,
        ) as lua_int;
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
unsafe extern "C" fn getthread(mut L: *mut lua_State, mut arg: *mut lua_int) -> *mut lua_State {
    if lua_type(L, 1i32) == 8i32 {
        *arg = 1i32;
        return lua_tothread(L, 1i32);
    } else {
        *arg = 0i32;
        /* function will operate over current thread */
        return L;
    };
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 3i32);
    return auxupvalue(L, 0i32);
}
/*
** get (if 'get' is true) or set an upvalue from a closure
*/
unsafe extern "C" fn auxupvalue(mut L: *mut lua_State, mut get: lua_int) -> lua_int {
    let mut name: *const lua_char = 0 as *const lua_char;
    /* upvalue index */
    let mut n: lua_int = luaL_checkinteger(L, 2i32) as lua_int;
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
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 2i32);
    (t == 0i32 || t == 5i32 || 0 != luaL_argerror(L, 2i32, s!(b"nil or table expected\x00")))
        as lua_int;
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    /* return 1st argument */
    return 1i32;
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut name: *const lua_char = 0 as *const lua_char;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
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
    let mut level: lua_int = luaL_checkinteger(L, arg + 1i32) as lua_int;
    let mut nvar: lua_int = luaL_checkinteger(L, arg + 2i32) as lua_int;
    /* out of range? */
    if 0 == lua_getstack(L1, level, &mut ar) {
        return luaL_argerror(L, arg + 1i32, s!(b"level out of range\x00"));
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
    mut n: lua_int,
) -> () {
    if L != L1 && 0 == lua_checkstack(L1, n) {
        luaL_error!(L, s!(b"stack overflow\x00"));
    };
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> lua_int {
    let mut smask: *const lua_char = 0 as *const lua_char;
    let mut arg: lua_int = 0;
    let mut mask: lua_int = 0;
    let mut count: lua_int = 0;
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
        count = luaL_optinteger(L, arg + 3i32, 0i32 as lua_Integer) as lua_int;
        func = Some(hookf);
        mask = makemask(smask, count)
    }
    if lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const lua_int as *const lua_void,
    ) == 0i32
    {
        /* create a hook table */
        lua_createtable(L, 0i32, 2i32);
        lua_pushvalue(L, -1i32);
        /* set it in position */
        lua_rawsetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const lua_int as *const lua_void,
        );
        lua_pushstring(L, s!(b"k\x00"));
        /* * hooktable.__mode = "k" */
        lua_setfield(L, -2i32, s!(b"__mode\x00"));
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
static mut HOOKKEY: lua_int = 0i32;
/*
** Convert a string mask (for 'sethook') into a bit mask
*/
unsafe extern "C" fn makemask(
    mut smask: *const lua_char,
    mut count: lua_int,
) -> lua_int {
    let mut mask: lua_int = 0i32;
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
    static mut hooknames: [*const lua_char; 5] = [
        s!(b"call\x00"),
        s!(b"return\x00"),
        s!(b"line\x00"),
        s!(b"count\x00"),
        s!(b"tail call\x00"),
    ];
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const lua_int as *const lua_void,
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
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 7i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_setuservalue(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_int = checkupval(L, 1i32, 2i32);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1i32, n));
    return 1i32;
}
/*
** Check whether a given upvalue from a given closure exists and
** returns its index
*/
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: lua_int,
    mut argnup: lua_int,
) -> lua_int {
    /* upvalue index */
    let mut nup: lua_int = luaL_checkinteger(L, argnup) as lua_int;
    /* closure */
    luaL_checktype(L, argf, 6i32);
    (!lua_getupvalue(L, argf, nup).is_null()
        || 0 != luaL_argerror(L, argnup, s!(b"invalid upvalue index\x00"))) as lua_int;
    return nup;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> lua_int {
    let mut n1: lua_int = checkupval(L, 1i32, 2i32);
    let mut n2: lua_int = checkupval(L, 3i32, 4i32);
    (0 == lua_iscfunction(L, 1i32) || 0 != luaL_argerror(L, 1i32, s!(b"Lua function expected\x00")))
        as lua_int;
    (0 == lua_iscfunction(L, 3i32) || 0 != luaL_argerror(L, 3i32, s!(b"Lua function expected\x00")))
        as lua_int;
    lua_upvaluejoin(L, 1i32, n1, 3i32, n2);
    return 0i32;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> lua_int {
    return auxupvalue(L, 1i32);
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        /* no metatable */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> lua_int {
    lua_pushvalue(L, -1000000i32 - 1000i32);
    return 1i32;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
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
    let mut name: *const lua_char = 0 as *const lua_char;
    /* local-variable index */
    let mut nvar: lua_int = luaL_checkinteger(L, arg + 2i32) as lua_int;
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
        let mut level: lua_int = luaL_checkinteger(L, arg + 1i32) as lua_int;
        /* out of range? */
        if 0 == lua_getstack(L1, level, &mut ar) {
            return luaL_argerror(L, arg + 1i32, s!(b"level out of range\x00"));
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
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> lua_int {
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
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut options: *const lua_char =
        luaL_optlstring(L, arg + 2i32, s!(b"flnStu\x00"), 0 as *mut size_t);
    checkstack(L, L1, 3i32);
    if lua_type(L, arg + 1i32) == 6i32 {
        /* info about a function? */
        /* add '>' to 'options' */
        options = lua_pushfstring!(L, s!(b">%s\x00"), options);
        /* move function to 'L1' stack */
        lua_pushvalue(L, arg + 1i32);
        lua_xmove(L, L1, 1i32);
    } else if 0 == lua_getstack(L1, luaL_checkinteger(L, arg + 1i32) as lua_int, &mut ar) {
        /* level out of range */
        lua_pushnil(L);
        return 1i32;
    }
    if 0 == lua_getinfo(L1, options, &mut ar) {
        return luaL_argerror(L, arg + 2i32, s!(b"invalid option\x00"));
    } else {
        /* table to collect results */
        lua_createtable(L, 0i32, 0i32);
        if !strchr(options, 'S' as i32).is_null() {
            settabss(L, s!(b"source\x00"), ar.source);
            settabss(L, s!(b"short_src\x00"), ar.short_src.as_mut_ptr());
            settabsi(L, s!(b"linedefined\x00"), ar.linedefined);
            settabsi(L, s!(b"lastlinedefined\x00"), ar.lastlinedefined);
            settabss(L, s!(b"what\x00"), ar.what);
        }
        if !strchr(options, 'l' as i32).is_null() {
            settabsi(L, s!(b"currentline\x00"), ar.currentline);
        }
        if !strchr(options, 'u' as i32).is_null() {
            settabsi(L, s!(b"nups\x00"), ar.nups as lua_int);
            settabsi(L, s!(b"nparams\x00"), ar.nparams as lua_int);
            settabsb(L, s!(b"isvararg\x00"), ar.isvararg as lua_int);
        }
        if !strchr(options, 'n' as i32).is_null() {
            settabss(L, s!(b"name\x00"), ar.name);
            settabss(L, s!(b"namewhat\x00"), ar.namewhat);
        }
        if !strchr(options, 't' as i32).is_null() {
            settabsb(L, s!(b"istailcall\x00"), ar.istailcall as lua_int);
        }
        if !strchr(options, 'L' as i32).is_null() {
            treatstackoption(L, L1, s!(b"activelines\x00"));
        }
        if !strchr(options, 'f' as i32).is_null() {
            treatstackoption(L, L1, s!(b"func\x00"));
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
    mut fname: *const lua_char,
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
    mut k: *const lua_char,
    mut v: lua_int,
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
    mut k: *const lua_char,
    mut v: *const lua_char,
) -> () {
    lua_pushstring(L, v);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn settabsi(
    mut L: *mut lua_State,
    mut k: *const lua_char,
    mut v: lua_int,
) -> () {
    lua_pushinteger(L, v as lua_Integer);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut buff: [lua_char; 5] = [0; 5];
    let mut mask: lua_int = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    /* no hook? */
    if hook.is_none() {
        lua_pushnil(L);
    } else if hook != Some(hookf) {
        lua_pushstring(L, s!(b"external hook\x00"));
    } else {
        /* hook table must exist */
        lua_rawgetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const lua_int as *const lua_void,
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
    mut mask: lua_int,
    mut smask: *mut lua_char,
) -> *mut lua_char {
    let mut i: lua_int = 0i32;
    if 0 != mask & 1i32 << 0i32 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as lua_char
    }
    if 0 != mask & 1i32 << 1i32 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as lua_char
    }
    if 0 != mask & 1i32 << 2i32 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as lua_char
    }
    *smask.offset(i as isize) = '\u{0}' as i32 as lua_char;
    return smask;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> lua_int {
    if lua_type(L, 1i32) != 7i32 {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1i32);
    }
    return 1i32;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> lua_int {
    loop {
        let mut buffer: [lua_char; 250] = [0; 250];
        fprintf(stderr, s!(b"%s\x00"), s!(b"lua_debug> \x00"));
        fflush(stderr);
        if fgets(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 250]>() as lua_ulong as lua_int,
            stdin,
        )
        .is_null()
            || strcmp(buffer.as_mut_ptr(), s!(b"cont\n\x00")) == 0i32
        {
            return 0i32;
        } else {
            if 0 != luaL_loadbufferx(
                L,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                s!(b"=(debug command)\x00"),
                0 as *const lua_char,
            ) || 0 != lua_pcallk(L, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None)
            {
                fprintf(
                    stderr,
                    s!(b"%s\n\x00"),
                    lua_tolstring(L, -1i32, 0 as *mut size_t),
                );
                fflush(stderr);
            }
            /* remove eventual returns */
            lua_settop(L, 0i32);
        }
    }
}
