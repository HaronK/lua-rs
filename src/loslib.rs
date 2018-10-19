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
    fn setlocale(__category: libc::c_int, __locale: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn mkstemp(__template: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn clock() -> clock_t;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    #[no_mangle]
    fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    fn lua_close(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
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
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkoption(
        L: *mut lua_State,
        arg: libc::c_int,
        def: *const libc::c_char,
        lst: *const *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_fileresult(
        L: *mut lua_State,
        stat: libc::c_int,
        fname: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_execresult(L: *mut lua_State, stat: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_prepbuffsize(B: *mut luaL_Buffer, sz: size_t) -> *mut libc::c_char;
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn close(__fd: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type clock_t = __clock_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
pub type ptrdiff_t = libc::c_long;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
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
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg; 12]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0i32);
    return 1i32;
}
static mut syslib: [luaL_Reg; 12] = [
    luaL_Reg {
        name: b"clock\x00" as *const u8 as *const libc::c_char,
        func: Some(os_clock),
    },
    luaL_Reg {
        name: b"date\x00" as *const u8 as *const libc::c_char,
        func: Some(os_date),
    },
    luaL_Reg {
        name: b"difftime\x00" as *const u8 as *const libc::c_char,
        func: Some(os_difftime),
    },
    luaL_Reg {
        name: b"execute\x00" as *const u8 as *const libc::c_char,
        func: Some(os_execute),
    },
    luaL_Reg {
        name: b"exit\x00" as *const u8 as *const libc::c_char,
        func: Some(os_exit),
    },
    luaL_Reg {
        name: b"getenv\x00" as *const u8 as *const libc::c_char,
        func: Some(os_getenv),
    },
    luaL_Reg {
        name: b"remove\x00" as *const u8 as *const libc::c_char,
        func: Some(os_remove),
    },
    luaL_Reg {
        name: b"rename\x00" as *const u8 as *const libc::c_char,
        func: Some(os_rename),
    },
    luaL_Reg {
        name: b"setlocale\x00" as *const u8 as *const libc::c_char,
        func: Some(os_setlocale),
    },
    luaL_Reg {
        name: b"time\x00" as *const u8 as *const libc::c_char,
        func: Some(os_time),
    },
    luaL_Reg {
        name: b"tmpname\x00" as *const u8 as *const libc::c_char,
        func: Some(os_tmpname),
    },
    luaL_Reg {
        name: 0 as *const libc::c_char,
        func: None,
    },
];
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State) -> libc::c_int {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut err: libc::c_int = 0;
    strcpy(
        buff.as_mut_ptr(),
        b"/tmp/lua_XXXXXX\x00" as *const u8 as *const libc::c_char,
    );
    err = mkstemp(buff.as_mut_ptr());
    if err != -1i32 {
        close(err);
    }
    err = (err == -1i32) as libc::c_int;
    if 0 != err {
        return luaL_error(
            L,
            b"unable to generate a unique filename\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        lua_pushstring(L, buff.as_mut_ptr());
        return 1i32;
    };
}
unsafe extern "C" fn os_time(mut L: *mut lua_State) -> libc::c_int {
    let mut ts: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut t: time_t = 0;
    /* called without args? */
    if lua_type(L, 1i32) <= 0i32 {
        /* get current time */
        t = time(0 as *mut time_t)
    } else {
        ts = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: 0 as *const libc::c_char,
        };
        luaL_checktype(L, 1i32, 5i32);
        /* make sure table is at the top */
        lua_settop(L, 1i32);
        ts.tm_sec = getfield(
            L,
            b"sec\x00" as *const u8 as *const libc::c_char,
            0i32,
            0i32,
        );
        ts.tm_min = getfield(
            L,
            b"min\x00" as *const u8 as *const libc::c_char,
            0i32,
            0i32,
        );
        ts.tm_hour = getfield(
            L,
            b"hour\x00" as *const u8 as *const libc::c_char,
            12i32,
            0i32,
        );
        ts.tm_mday = getfield(
            L,
            b"day\x00" as *const u8 as *const libc::c_char,
            -1i32,
            0i32,
        );
        ts.tm_mon = getfield(
            L,
            b"month\x00" as *const u8 as *const libc::c_char,
            -1i32,
            1i32,
        );
        ts.tm_year = getfield(
            L,
            b"year\x00" as *const u8 as *const libc::c_char,
            -1i32,
            1900i32,
        );
        ts.tm_isdst = getboolfield(L, b"isdst\x00" as *const u8 as *const libc::c_char);
        t = mktime(&mut ts);
        /* update fields with normalized values */
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -1i32 as time_t {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
        lua_pushinteger(L, t as lua_Integer);
        return 1i32;
    };
}
/*
** Set all fields from structure 'tm' in the table on top of the stack
*/
unsafe extern "C" fn setallfields(mut L: *mut lua_State, mut stm: *mut tm) -> () {
    setfield(
        L,
        b"sec\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_sec,
    );
    setfield(
        L,
        b"min\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_min,
    );
    setfield(
        L,
        b"hour\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_hour,
    );
    setfield(
        L,
        b"day\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_mday,
    );
    setfield(
        L,
        b"month\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_mon + 1i32,
    );
    setfield(
        L,
        b"year\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_year + 1900i32,
    );
    setfield(
        L,
        b"wday\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_wday + 1i32,
    );
    setfield(
        L,
        b"yday\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_yday + 1i32,
    );
    setboolfield(
        L,
        b"isdst\x00" as *const u8 as *const libc::c_char,
        (*stm).tm_isdst,
    );
}
unsafe extern "C" fn setboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) -> () {
    /* undefined? */
    if value < 0i32 {
        /* does not set field */
        return;
    } else {
        lua_pushboolean(L, value);
        lua_setfield(L, -2i32, key);
        return;
    };
}
/*
** {======================================================
** Time/Date operations
** { year=%Y, month=%m, day=%d, hour=%H, min=%M, sec=%S,
**   wday=%w+1, yday=%j, isdst=? }
** =======================================================
*/
unsafe extern "C" fn setfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut value: libc::c_int,
) -> () {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -2i32, key);
}
unsafe extern "C" fn getboolfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
) -> libc::c_int {
    let mut res: libc::c_int = 0;
    res = if lua_getfield(L, -1i32, key) == 0i32 {
        -1i32
    } else {
        lua_toboolean(L, -1i32)
    };
    lua_settop(L, -1i32 - 1i32);
    return res;
}
/* maximum value for date fields (to avoid arithmetic overflows with 'int') */
unsafe extern "C" fn getfield(
    mut L: *mut lua_State,
    mut key: *const libc::c_char,
    mut d: libc::c_int,
    mut delta: libc::c_int,
) -> libc::c_int {
    let mut isnum: libc::c_int = 0;
    /* get field and its type */
    let mut t: libc::c_int = lua_getfield(L, -1i32, key);
    let mut res: lua_Integer = lua_tointegerx(L, -1i32, &mut isnum);
    if 0 == isnum {
        /* field is not an integer? */
        /* some other value? */
        if t != 0i32 {
            return luaL_error(
                L,
                b"field \'%s\' is not an integer\x00" as *const u8 as *const libc::c_char,
                key,
            );
        } else if d < 0i32 {
            return luaL_error(
                L,
                b"field \'%s\' missing in date table\x00" as *const u8 as *const libc::c_char,
                key,
            );
        } else {
            res = d as lua_Integer
        }
    } else if !(-(2147483647i32 / 2i32) as libc::c_longlong <= res
        && res <= (2147483647i32 / 2i32) as libc::c_longlong)
    {
        return luaL_error(
            L,
            b"field \'%s\' is out-of-bound\x00" as *const u8 as *const libc::c_char,
            key,
        );
    } else {
        res -= delta as libc::c_longlong
    }
    lua_settop(L, -1i32 - 1i32);
    return res as libc::c_int;
}
/* }====================================================== */
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State) -> libc::c_int {
    static mut cat: [libc::c_int; 6] = [6i32, 3i32, 0i32, 4i32, 1i32, 2i32];
    static mut catnames: [*const libc::c_char; 7] = [
        b"all\x00" as *const u8 as *const libc::c_char,
        b"collate\x00" as *const u8 as *const libc::c_char,
        b"ctype\x00" as *const u8 as *const libc::c_char,
        b"monetary\x00" as *const u8 as *const libc::c_char,
        b"numeric\x00" as *const u8 as *const libc::c_char,
        b"time\x00" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    let mut l: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut size_t);
    let mut op: libc::c_int = luaL_checkoption(
        L,
        2i32,
        b"all\x00" as *const u8 as *const libc::c_char,
        catnames.as_ptr(),
    );
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1i32;
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State) -> libc::c_int {
    let mut fromname: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut toname: *const libc::c_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0i32) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    return luaL_fileresult(L, (remove(filename) == 0i32) as libc::c_int, filename);
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State) -> libc::c_int {
    /* if NULL push nil */
    lua_pushstring(L, getenv(luaL_checklstring(L, 1i32, 0 as *mut size_t)));
    return 1i32;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    if lua_type(L, 1i32) == 1i32 {
        status = if 0 != lua_toboolean(L, 1i32) {
            0i32
        } else {
            1i32
        }
    } else {
        status = luaL_optinteger(L, 1i32, 0i32 as lua_Integer) as libc::c_int
    }
    if 0 != lua_toboolean(L, 2i32) {
        lua_close(L);
    }
    if !L.is_null() {
        /* 'if' to avoid warnings for unreachable 'return' */
        exit(status);
    } else {
        return 0i32;
    };
}
/* } */
/* { */
/*
** By default, Lua uses gmtime/localtime, except when POSIX is available,
** where it uses gmtime_r/localtime_r
*/
/* { */
/* }{ */
/* } */
/* } */
/* }================================================================== */
/*
** {==================================================================
** Configuration for 'tmpnam':
** By default, Lua uses tmpnam except when POSIX is available, where
** it uses mkstemp.
** ===================================================================
*/
/* { */
/* { */
/* }{ */
/* } */
/* } */
/* }================================================================== */
unsafe extern "C" fn os_execute(mut L: *mut lua_State) -> libc::c_int {
    let mut cmd: *const libc::c_char =
        luaL_optlstring(L, 1i32, 0 as *const libc::c_char, 0 as *mut size_t);
    let mut stat: libc::c_int = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat);
    } else {
        /* true if there is a shell */
        lua_pushboolean(L, stat);
        return 1i32;
    };
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State) -> libc::c_int {
    let mut t1: time_t = l_checktime(L, 1i32);
    let mut t2: time_t = l_checktime(L, 2i32);
    lua_pushnumber(L, difftime(t1, t2));
    return 1i32;
}
/*
** $Id: loslib.c,v 1.65.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard Operating System library
** See Copyright Notice in lua.h
*/
/*
** {==================================================================
** List of valid conversion specifiers for the 'strftime' function;
** options are grouped by length; group of length 2 start with '||'.
** ===================================================================
*/
/* { */
/* options for ANSI C 89 (only 1-char options) */
/* options for ISO C 99 and POSIX */
/* two-char options */
/* options for Windows */
/* two-char options */
/* C99 specification */
/* } */
/* }================================================================== */
/*
** {==================================================================
** Configuration for time-related stuff
** ===================================================================
*/
/* { */
/*
** type to represent time_t in Lua
*/
unsafe extern "C" fn l_checktime(mut L: *mut lua_State, mut arg: libc::c_int) -> time_t {
    let mut t: lua_Integer = luaL_checkinteger(L, arg);
    (t as time_t as libc::c_longlong == t || 0 != luaL_argerror(
        L,
        arg,
        b"time out-of-bounds\x00" as *const u8 as *const libc::c_char,
    )) as libc::c_int;
    return t as time_t;
}
/* maximum size for an individual 'strftime' item */
unsafe extern "C" fn os_date(mut L: *mut lua_State) -> libc::c_int {
    let mut slen: size_t = 0;
    let mut s: *const libc::c_char = luaL_optlstring(
        L,
        1i32,
        b"%c\x00" as *const u8 as *const libc::c_char,
        &mut slen,
    );
    let mut t: time_t = if lua_type(L, 2i32) <= 0i32 {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2i32)
    };
    /* 's' end */
    let mut se: *const libc::c_char = s.offset(slen as isize);
    let mut tmr: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const libc::c_char,
    };
    let mut stm: *mut tm = 0 as *mut tm;
    if *s as libc::c_int == '!' as i32 {
        /* UTC? */
        stm = gmtime_r(&mut t, &mut tmr);
        /* skip '!' */
        s = s.offset(1isize)
    } else {
        stm = localtime_r(&mut t, &mut tmr)
    }
    /* invalid date? */
    if stm.is_null() {
        return luaL_error(
            L,
            b"time result cannot be represented in this installation\x00" as *const u8
                as *const libc::c_char,
        );
    } else {
        if strcmp(s, b"*t\x00" as *const u8 as *const libc::c_char) == 0i32 {
            /* 9 = number of fields */
            lua_createtable(L, 0i32, 9i32);
            setallfields(L, stm);
        } else {
            /* buffer for individual conversion specifiers */
            let mut cc: [libc::c_char; 4] = [0; 4];
            let mut b: luaL_Buffer = luaL_Buffer {
                b: 0 as *mut libc::c_char,
                size: 0,
                n: 0,
                L: 0 as *mut lua_State,
                initb: [0; 8192],
            };
            cc[0usize] = '%' as i32 as libc::c_char;
            luaL_buffinit(L, &mut b);
            while s < se {
                /* not a conversion specifier? */
                if *s as libc::c_int != '%' as i32 {
                    (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null())
                        as libc::c_int;
                    let fresh1 = b.n;
                    b.n = b.n.wrapping_add(1);
                    let fresh0 = s;
                    s = s.offset(1);
                    *b.b.offset(fresh1 as isize) = *fresh0
                } else {
                    let mut reslen: size_t = 0;
                    let mut buff: *mut libc::c_char = luaL_prepbuffsize(&mut b, 250i32 as size_t);
                    /* skip '%' */
                    s = s.offset(1isize);
                    /* copy specifier to 'cc' */
                    s = checkoption(
                        L,
                        s,
                        se.wrapping_offset_from(s) as libc::c_long,
                        cc.as_mut_ptr().offset(1isize),
                    );
                    reslen = strftime(buff, 250i32 as size_t, cc.as_mut_ptr(), stm);
                    b.n = (b.n as libc::c_ulong).wrapping_add(reslen) as size_t as size_t
                }
            }
            luaL_pushresult(&mut b);
        }
        return 1i32;
    };
}
unsafe extern "C" fn checkoption(
    mut L: *mut lua_State,
    mut conv: *const libc::c_char,
    mut convlen: ptrdiff_t,
    mut buff: *mut libc::c_char,
) -> *const libc::c_char {
    let mut option: *const libc::c_char =
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\x00"
            as *const u8 as *const libc::c_char;
    /* length of options being checked */
    let mut oplen: libc::c_int = 1i32;
    while *option as libc::c_int != '\u{0}' as i32 && oplen as libc::c_long <= convlen {
        /* next block? */
        if *option as libc::c_int == '|' as i32 {
            /* will check options with next length (+1) */
            oplen += 1
        } else if memcmp(
            conv as *const libc::c_void,
            option as *const libc::c_void,
            oplen as libc::c_ulong,
        ) == 0i32
        {
            /* match? */
            /* copy valid option to buffer */
            memcpy(
                buff as *mut libc::c_void,
                conv as *const libc::c_void,
                oplen as libc::c_ulong,
            );
            *buff.offset(oplen as isize) = '\u{0}' as i32 as libc::c_char;
            /* return next item */
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize)
    }
    luaL_argerror(
        L,
        1i32,
        lua_pushfstring(
            L,
            b"invalid conversion specifier \'%%%s\'\x00" as *const u8 as *const libc::c_char,
            conv,
        ),
    );
    /* to avoid warnings */
    return conv;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State) -> libc::c_int {
    lua_pushnumber(
        L,
        clock() as lua_Number / 1000000i32 as __clock_t as lua_Number,
    );
    return 1i32;
}
