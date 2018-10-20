use libc;
use luaconf::*;
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

    pub type lua_State;

    /* private part */
    pub type CallInfo;
    #[no_mangle]
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn lua_close(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, tp: libc::c_int) -> *const libc::c_char;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: libc::c_int, len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
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
    /*
     ** get functions (Lua -> stack)
     */
    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    /*
     ** set functions (stack -> Lua)
     */
    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State, idx: libc::c_int, n: lua_Integer) -> ();
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
    fn lua_concat(L: *mut lua_State, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: libc::c_int, count: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_callmeta(L: *mut lua_State, obj: libc::c_int, e: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadbufferx(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_newstate() -> *mut lua_State;
    #[no_mangle]
    fn luaL_len(L: *mut lua_State, idx: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_traceback(
        L: *mut lua_State,
        L1: *mut lua_State,
        msg: *const libc::c_char,
        level: libc::c_int,
    ) -> ();
    /* open all previous libraries */
    #[no_mangle]
    fn luaL_openlibs(L: *mut lua_State) -> ();
    #[no_mangle]
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn readline(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn add_history(_: *const libc::c_char) -> ();
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: libc::c_int) -> ()>;
pub type size_t = libc::c_ulong;
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

// ----------- Header declarations -----------

pub_const_c_str!(LUA_VERSION_MAJOR, b"5\x00");
pub_const_c_str!(LUA_VERSION_MINOR, b"3\x00");
pub const LUA_VERSION_NUM: i32 = 503;
pub_const_c_str!(LUA_VERSION_RELEASE, b"5\x00");

// #define LUA_VERSION	"Lua " LUA_VERSION_MAJOR "." LUA_VERSION_MINOR
// #define LUA_RELEASE	LUA_VERSION "." LUA_VERSION_RELEASE
// #define LUA_COPYRIGHT	LUA_RELEASE "  Copyright (C) 1994-2018 Lua.org, PUC-Rio"
pub_const_c_str!(
    LUA_AUTHORS,
    b"R. Ierusalimschy, L. H. de Figueiredo, W. Celes\x00"
);

/* mark for precompiled code ('<esc>Lua') */
pub_const_c_str!(LUA_SIGNATURE, b"\x1bLua\x00");

/* option for multiple returns in 'lua_pcall' and 'lua_call' */
pub const LUA_MULTRET: i32 = -1;

/*
 ** Pseudo-indices
 ** (-LUAI_MAXSTACK is the minimum valid index; we keep some free empty
 ** space after that to help overflow detection)
 */
// #define LUA_REGISTRYINDEX	(-LUAI_MAXSTACK - 1000)
// #define lua_upvalueindex(i)	(LUA_REGISTRYINDEX - (i))

/* thread status */
pub const LUA_OK: i32 = 0;
pub const LUA_YIELD: i32 = 1;
pub const LUA_ERRRUN: i32 = 2;
pub const LUA_ERRSYNTAX: i32 = 3;
pub const LUA_ERRMEM: i32 = 4;
pub const LUA_ERRGCMM: i32 = 5;
pub const LUA_ERRERR: i32 = 6;

/*
** basic types
*/
pub const LUA_TNONE: i32 = -1;

pub const LUA_TNIL: i32 = 0;
pub const LUA_TBOOLEAN: i32 = 1;
pub const LUA_TLIGHTUSERDATA: i32 = 2;
pub const LUA_TNUMBER: i32 = 3;
pub const LUA_TSTRING: i32 = 4;
pub const LUA_TTABLE: i32 = 5;
pub const LUA_TFUNCTION: i32 = 6;
pub const LUA_TUSERDATA: i32 = 7;
pub const LUA_TTHREAD: i32 = 8;

pub const LUA_NUMTAGS: i32 = 9;

/* minimum Lua stack available to a C function */
pub const LUA_MINSTACK: i32 = 20;

/* predefined values in the registry */
pub const LUA_RIDX_MAINTHREAD: i32 = 1;
pub const LUA_RIDX_GLOBALS: i32 = 2;
pub const LUA_RIDX_LAST: i32 = LUA_RIDX_GLOBALS;

/* type of numbers in Lua */
pub type lua_Number = LUA_NUMBER;

/* type for integer functions */
pub type lua_Integer = libc::c_longlong;

/* unsigned integer type */
pub type lua_Unsigned = u64; // TODO: check

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
** $Id: lua.c,v 1.230.1.1 2017/04/19 17:29:57 roberto Exp $
** Lua stand-alone interpreter
** See Copyright Notice in lua.h
*/
/*
** lua_stdin_is_tty detects whether the standard input is a 'tty' (that
** is, whether we're running lua interactively).
*/
/* { */
/* { */
/* }{ */
/* } */
/* } */
/*
** lua_readline defines how to show a prompt and then read a line from
** the standard input.
** lua_saveline defines how to "save" a read line in a "history".
** lua_freeline defines how to free a line read by lua_readline.
*/
/* { */
/* { */
/* }{ */
/* } */
/* } */
static mut globalL: *mut lua_State = 0 as *const lua_State as *mut lua_State;
static mut progname: *const libc::c_char = s!(b"lua\x00");
/*
** Hook set by signal function to stop the interpreter.
*/
unsafe extern "C" fn lstop(mut L: *mut lua_State, mut _ar: *mut lua_Debug) -> () {
    /* unused arg. */
    /* reset hook */
    lua_sethook(L, None, 0i32, 0i32);
    luaL_error(L, s!(b"interrupted!\x00"));
}
/*
** Function to be called at a C signal. Because a C signal cannot
** just change a Lua state (as there is no proper synchronization),
** this function only sets a hook that, when called, will stop the
** interpreter.
*/
unsafe extern "C" fn laction(mut i: libc::c_int) -> () {
    /* if another SIGINT happens, terminate process */
    signal(i, None);
    lua_sethook(
        globalL,
        Some(lstop),
        1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 3i32,
        1i32,
    );
}
unsafe extern "C" fn print_usage(mut badoption: *const libc::c_char) -> () {
    fprintf(stderr, s!(b"%s: \x00"), progname);
    fflush(stderr);
    if *badoption.offset(1isize) as libc::c_int == 'e' as i32
        || *badoption.offset(1isize) as libc::c_int == 'l' as i32
    {
        fprintf(stderr, s!(b"\'%s\' needs argument\n\x00"), badoption);
        fflush(stderr);
    } else {
        fprintf(stderr, s!(b"unrecognized option \'%s\'\n\x00"), badoption);
        fflush(stderr);
    }
    fprintf(stderr,
            b"usage: %s [options] [script [args]]\nAvailable options are:\n  -e stat  execute string \'stat\'\n  -i       enter interactive mode after executing \'script\'\n  -l name  require library \'name\' into global \'name\'\n  -v       show version information\n  -E       ignore environment variables\n  --       stop handling options\n  -        stop handling options and execute stdin\n\x00"
                as *const u8 as *const libc::c_char, progname);
    fflush(stderr);
}
/*
** Prints an error message, adding the program name in front of it
** (if present)
*/
unsafe extern "C" fn l_message(mut pname: *const libc::c_char, mut msg: *const libc::c_char) -> () {
    if !pname.is_null() {
        fprintf(stderr, s!(b"%s: \x00"), pname);
        fflush(stderr);
    }
    fprintf(stderr, s!(b"%s\n\x00"), msg);
    fflush(stderr);
}
/*
** Check whether 'status' is not OK and, if so, prints the error
** message on the top of the stack. It assumes that the error object
** is a string, as it was either generated by Lua or by 'msghandler'.
*/
unsafe extern "C" fn report(mut L: *mut lua_State, mut status: libc::c_int) -> libc::c_int {
    let mut msg: *const libc::c_char = 0 as *const libc::c_char;
    if status != 0i32 {
        msg = lua_tolstring(L, -1i32, 0 as *mut size_t);
        l_message(progname, msg);
        /* remove message */
        lua_settop(L, -1i32 - 1i32);
    }
    return status;
}
/*
** Message handler used to run all chunks
*/
unsafe extern "C" fn msghandler(mut L: *mut lua_State) -> libc::c_int {
    let mut msg: *const libc::c_char = lua_tolstring(L, 1i32, 0 as *mut size_t);
    if msg.is_null() {
        /* is error object not a string? */
        /* does it have a metamethod */
        if 0 != luaL_callmeta(L, 1i32, s!(b"__tostring\x00")) && lua_type(L, -1i32) == 4i32 {
            /* that produces a string? */
            /* that is the message */
            return 1i32;
        } else {
            msg = lua_pushfstring!(
                L,
                s!(b"(error object is a %s value)\x00"),
                lua_typename(L, lua_type(L, 1i32)),
            )
        }
    }
    /* append a standard traceback */
    luaL_traceback(L, L, msg, 1i32);
    /* return the traceback */
    return 1i32;
}
/*
** Interface to 'lua_pcall', which sets appropriate message function
** and C-signal handler. Used to run all chunks.
*/
unsafe extern "C" fn docall(
    mut L: *mut lua_State,
    mut narg: libc::c_int,
    mut nres: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    /* function index */
    let mut base: libc::c_int = lua_gettop(L) - narg;
    /* push message handler */
    lua_pushcclosure(L, Some(msghandler), 0i32);
    /* put it under function and args */
    lua_rotate(L, base, 1i32);
    /* to be available to 'laction' */
    globalL = L;
    /* set C-signal handler */
    signal(2i32, Some(laction));
    status = lua_pcallk(L, narg, nres, base, 0i32 as lua_KContext, None);
    /* reset C-signal handler */
    signal(2i32, None);
    /* remove message handler from the stack */
    lua_rotate(L, base, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return status;
}
unsafe extern "C" fn print_version() -> () {
    fwrite(
        s!(b"Lua 5.3.5  Copyright (C) 1994-2018 Lua.org, PUC-Rio\x00") as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        strlen(s!(
            b"Lua 5.3.5  Copyright (C) 1994-2018 Lua.org, PUC-Rio\x00"
        )),
        stdout,
    );
    fwrite(
        s!(b"\n\x00") as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1i32 as size_t,
        stdout,
    );
    fflush(stdout);
}
/*
** Create the 'arg' table, which stores all arguments from the
** command line ('argv'). It should be aligned so that, at index 0,
** it has 'argv[script]', which is the script name. The arguments
** to the script (everything after 'script') go to positive indices;
** other arguments (before the script name) go to negative indices.
** If there is no script name, assume interpreter's name as base.
*/
unsafe extern "C" fn createargtable(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
    mut script: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0;
    let mut narg: libc::c_int = 0;
    if script == argc {
        /* no script name? */
        script = 0i32
    }
    /* number of positive indices */
    narg = argc - (script + 1i32);
    lua_createtable(L, narg, script + 1i32);
    i = 0i32;
    while i < argc {
        lua_pushstring(L, *argv.offset(i as isize));
        lua_rawseti(L, -2i32, (i - script) as lua_Integer);
        i += 1
    }
    lua_setglobal(L, s!(b"arg\x00"));
}
unsafe extern "C" fn dochunk(mut L: *mut lua_State, mut status: libc::c_int) -> libc::c_int {
    if status == 0i32 {
        status = docall(L, 0i32, 0i32)
    }
    return report(L, status);
}
unsafe extern "C" fn dofile(mut L: *mut lua_State, mut name: *const libc::c_char) -> libc::c_int {
    return dochunk(L, luaL_loadfilex(L, name, 0 as *const libc::c_char));
}
unsafe extern "C" fn dostring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return dochunk(
        L,
        luaL_loadbufferx(L, s, strlen(s), name, 0 as *const libc::c_char),
    );
}
/*
** Calls 'require(name)' and stores the result in a global variable
** with the given name.
*/
unsafe extern "C" fn dolibrary(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_getglobal(L, s!(b"require\x00"));
    lua_pushstring(L, name);
    /* call 'require(name)' */
    status = docall(L, 1i32, 1i32);
    if status == 0i32 {
        /* global[name] = require return */
        lua_setglobal(L, name);
    }
    return report(L, status);
}
/*
** Returns the string to be used as a prompt by the interpreter.
*/
unsafe extern "C" fn get_prompt(
    mut L: *mut lua_State,
    mut firstline: libc::c_int,
) -> *const libc::c_char {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    lua_getglobal(
        L,
        if 0 != firstline {
            s!(b"_PROMPT\x00")
        } else {
            s!(b"_PROMPT2\x00")
        },
    );
    p = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if p.is_null() {
        p = if 0 != firstline {
            s!(b"> \x00")
        } else {
            s!(b">> \x00")
        }
    }
    return p;
}
/* mark in error messages for incomplete statements */
/*
** Check whether 'status' signals a syntax error and the error
** message at the top of the stack ends with the above mark for
** incomplete statements.
*/
unsafe extern "C" fn incomplete(mut L: *mut lua_State, mut status: libc::c_int) -> libc::c_int {
    if status == 3i32 {
        let mut lmsg: size_t = 0;
        let mut msg: *const libc::c_char = lua_tolstring(L, -1i32, &mut lmsg);
        if lmsg
            >= (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong)
            && strcmp(
                msg.offset(lmsg as isize).offset(
                    -((::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                        .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_sub(1i32 as libc::c_ulong) as isize),
                ),
                s!(b"<eof>\x00"),
            ) == 0i32
        {
            lua_settop(L, -1i32 - 1i32);
            return 1i32;
        }
    }
    /* else... */
    return 0i32;
}
/*
** Prompt the user, read a line, and push it into the Lua stack.
*/
unsafe extern "C" fn pushline(mut L: *mut lua_State, mut firstline: libc::c_int) -> libc::c_int {
    let mut buffer: [libc::c_char; 512] = [0; 512];
    let mut b: *mut libc::c_char = buffer.as_mut_ptr();
    let mut l: size_t = 0;
    let mut prmt: *const libc::c_char = get_prompt(L, firstline);
    b = readline(prmt);
    let mut readstatus: libc::c_int =
        (b != 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    if readstatus == 0i32 {
        /* no input (prompt will be popped by caller) */
        return 0i32;
    } else {
        /* remove prompt */
        lua_settop(L, -1i32 - 1i32);
        l = strlen(b);
        /* line ends with newline? */
        if l > 0i32 as libc::c_ulong
            && *b.offset(l.wrapping_sub(1i32 as libc::c_ulong) as isize) as libc::c_int
                == '\n' as i32
        {
            /* remove it */
            l = l.wrapping_sub(1);
            *b.offset(l as isize) = '\u{0}' as i32 as libc::c_char
        }
        /* for compatibility with 5.2, ... */
        if 0 != firstline && *b.offset(0isize) as libc::c_int == '=' as i32 {
            /* change '=' to 'return' */
            lua_pushfstring!(L, s!(b"return %s\x00"), b.offset(1isize),);
        } else {
            lua_pushlstring(L, b, l);
        }
        free(b as *mut libc::c_void);
        return 1i32;
    };
}
/*
** Try to compile line on the stack as 'return <line>;'; on return, stack
** has either compiled chunk or original line (if compilation failed).
*/
unsafe extern "C" fn addreturn(mut L: *mut lua_State) -> libc::c_int {
    /* original line */
    let mut line: *const libc::c_char = lua_tolstring(L, -1i32, 0 as *mut size_t);
    let mut retline: *const libc::c_char = lua_pushfstring!(L, s!(b"return %s;\x00"), line,);
    let mut status: libc::c_int = luaL_loadbufferx(
        L,
        retline,
        strlen(retline),
        s!(b"=stdin\x00"),
        0 as *const libc::c_char,
    );
    if status == 0i32 {
        /* remove modified line */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* non empty? */
        if *line.offset(0isize) as libc::c_int != '\u{0}' as i32 {
            /* keep history */
            add_history(line);
        }
    } else {
        /* pop result from 'luaL_loadbuffer' and modified line */
        lua_settop(L, -2i32 - 1i32);
    }
    return status;
}
/*
** Read multiple lines until a complete Lua statement
*/
unsafe extern "C" fn multiline(mut L: *mut lua_State) -> libc::c_int {
    loop {
        /* repeat until gets a complete statement */
        let mut len: size_t = 0;
        /* get what it has */
        let mut line: *const libc::c_char = lua_tolstring(L, 1i32, &mut len);
        /* try it */
        let mut status: libc::c_int =
            luaL_loadbufferx(L, line, len, s!(b"=stdin\x00"), 0 as *const libc::c_char);
        if 0 == incomplete(L, status) || 0 == pushline(L, 0i32) {
            /* keep history */
            add_history(line);
            /* cannot or should not try to add continuation line */
            return status;
        } else {
            /* add newline... */
            lua_pushstring(L, s!(b"\n\x00"));
            /* ...between the two lines */
            lua_rotate(L, -2i32, 1i32);
            /* join them */
            lua_concat(L, 3i32);
        }
    }
}
/*
** Read a line and try to load (compile) it first as an expression (by
** adding "return " in front of it) and second as a statement. Return
** the final status of load/call with the resulting function (if any)
** in the top of the stack.
*/
unsafe extern "C" fn loadline(mut L: *mut lua_State) -> libc::c_int {
    let mut status: libc::c_int = 0;
    lua_settop(L, 0i32);
    if 0 == pushline(L, 1i32) {
        /* no input */
        return -1i32;
    } else {
        /* 'return ...' did not work? */
        status = addreturn(L);
        if status != 0i32 {
            /* try as command, maybe with continuation lines */
            status = multiline(L)
        }
        /* remove line from the stack */
        lua_rotate(L, 1i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        return status;
    };
}
/*
** Prints (calling the Lua 'print' function) any values on the stack
*/
unsafe extern "C" fn l_print(mut L: *mut lua_State) -> () {
    let mut n: libc::c_int = lua_gettop(L);
    if n > 0i32 {
        /* any result to be printed? */
        luaL_checkstack(L, 20i32, s!(b"too many results to print\x00"));
        lua_getglobal(L, s!(b"print\x00"));
        lua_rotate(L, 1i32, 1i32);
        if lua_pcallk(L, n, 0i32, 0i32, 0i32 as lua_KContext, None) != 0i32 {
            l_message(
                progname,
                lua_pushfstring!(
                    L,
                    s!(b"error calling \'print\' (%s)\x00"),
                    lua_tolstring(L, -1i32, 0 as *mut size_t),
                ),
            );
        }
    };
}
/*
** Do the REPL: repeatedly read (load) a line, evaluate (call) it, and
** print any results.
*/
unsafe extern "C" fn doREPL(mut L: *mut lua_State) -> () {
    let mut status: libc::c_int = 0;
    let mut oldprogname: *const libc::c_char = progname;
    /* no 'progname' on errors in interactive mode */
    progname = 0 as *const libc::c_char;
    loop {
        status = loadline(L);
        if !(status != -1i32) {
            break;
        }
        if status == 0i32 {
            status = docall(L, 0i32, -1i32)
        }
        if status == 0i32 {
            l_print(L);
        } else {
            report(L, status);
        }
    }
    /* clear stack */
    lua_settop(L, 0i32);
    fwrite(
        s!(b"\n\x00") as *const libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        1i32 as size_t,
        stdout,
    );
    fflush(stdout);
    progname = oldprogname;
}
/*
** Push on the stack the contents of table 'arg' from 1 to #arg
*/
unsafe extern "C" fn pushargs(mut L: *mut lua_State) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    if lua_getglobal(L, s!(b"arg\x00")) != 5i32 {
        luaL_error(L, s!(b"\'arg\' is not a table\x00"));
    }
    n = luaL_len(L, -1i32) as libc::c_int;
    luaL_checkstack(L, n + 3i32, s!(b"too many arguments to script\x00"));
    i = 1i32;
    while i <= n {
        lua_rawgeti(L, -i, i as lua_Integer);
        i += 1
    }
    /* remove table from the stack */
    lua_rotate(L, -i, -1i32);
    lua_settop(L, -1i32 - 1i32);
    return n;
}
unsafe extern "C" fn handle_script(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut n: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut fname: *const libc::c_char = *argv.offset(0isize);
    if strcmp(fname, s!(b"-\x00")) == 0i32
        && strcmp(*argv.offset(-1i32 as isize), s!(b"--\x00")) != 0i32
    {
        /* stdin */
        fname = 0 as *const libc::c_char
    }
    status = luaL_loadfilex(L, fname, 0 as *const libc::c_char);
    if status == 0i32 {
        /* push arguments to script */
        n = pushargs(L);
        status = docall(L, n, -1i32)
    }
    return report(L, status);
}
/* bits of various argument indicators in 'args' */
/* bad option */
/* -i */
/* -v */
/* -e */
/* -E */
/*
** Traverses all arguments from 'argv', returning a mask with those
** needed before running any Lua code (or an error code if it finds
** any invalid argument). 'first' returns the first not-handled argument
** (either the script name or a bad argument in case of error).
*/
unsafe extern "C" fn collectargs(
    mut argv: *mut *mut libc::c_char,
    mut first: *mut libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut args: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    i = 1i32;
    while !(*argv.offset(i as isize)).is_null() {
        *first = i;
        /* not an option? */
        if *(*argv.offset(i as isize)).offset(0isize) as libc::c_int != '-' as i32 {
            /* stop handling options */
            return args;
        } else {
            match *(*argv.offset(i as isize)).offset(1isize) as libc::c_int {
                45 => {
                    /* extra characters after '--'? */
                    if *(*argv.offset(i as isize)).offset(2isize) as libc::c_int != '\u{0}' as i32 {
                        /* invalid option */
                        return 1i32;
                    } else {
                        *first = i + 1i32;
                        return args;
                    }
                }
                0 => {
                    /* script "name" is '-' */
                    return args;
                }
                69 => {
                    /* extra characters after 1st? */
                    if *(*argv.offset(i as isize)).offset(2isize) as libc::c_int != '\u{0}' as i32 {
                        /* invalid option */
                        return 1i32;
                    } else {
                        args |= 16i32
                    }
                    current_block = 16668937799742929182;
                }
                105 => {
                    /* (-i implies -v) *//* FALLTHROUGH */
                    args |= 2i32;
                    current_block = 3579427354951119183;
                }
                118 => {
                    current_block = 3579427354951119183;
                }
                101 => {
                    /* FALLTHROUGH */
                    args |= 8i32;
                    /* both options need an argument */
                    current_block = 3472961211730940312;
                }
                108 => {
                    current_block = 3472961211730940312;
                }
                _ => return 1i32,
            }
            match current_block {
                3472961211730940312 => {
                    if *(*argv.offset(i as isize)).offset(2isize) as libc::c_int == '\u{0}' as i32 {
                        /* no concatenated argument? */
                        /* try next 'argv' */
                        i += 1;
                        if (*argv.offset(i as isize)).is_null()
                            || *(*argv.offset(i as isize)).offset(0isize) as libc::c_int
                                == '-' as i32
                        {
                            /* no next argument or it is another option */
                            return 1i32;
                        }
                    }
                }
                3579427354951119183 => {
                    /* extra characters after 1st? */
                    if *(*argv.offset(i as isize)).offset(2isize) as libc::c_int != '\u{0}' as i32 {
                        /* invalid option */
                        return 1i32;
                    } else {
                        args |= 4i32
                    }
                }
                _ => {}
            }
            i += 1
        }
    }
    /* no script name */
    *first = i;
    return args;
}
/*
** Processes options 'e' and 'l', which involve running Lua code.
** Returns 0 if some code raises an error.
*/
unsafe extern "C" fn runargs(
    mut L: *mut lua_State,
    mut argv: *mut *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 1i32;
    while i < n {
        let mut option: libc::c_int = *(*argv.offset(i as isize)).offset(1isize) as libc::c_int;
        /* already checked */
        if option == 'e' as i32 || option == 'l' as i32 {
            let mut status: libc::c_int = 0;
            /* both options need an argument */
            let mut extra: *const libc::c_char = (*argv.offset(i as isize)).offset(2isize);
            if *extra as libc::c_int == '\u{0}' as i32 {
                i += 1;
                extra = *argv.offset(i as isize)
            }
            status = if option == 'e' as i32 {
                dostring(L, extra, s!(b"=(command line)\x00"))
            } else {
                dolibrary(L, extra)
            };
            if status != 0i32 {
                return 0i32;
            }
        }
        i += 1
    }
    return 1i32;
}
unsafe extern "C" fn handle_luainit(mut L: *mut lua_State) -> libc::c_int {
    let mut name: *const libc::c_char = s!(b"=LUA_INIT_5_3\x00");
    let mut init: *const libc::c_char = getenv(name.offset(1isize));
    if init.is_null() {
        name = s!(b"=LUA_INIT\x00");
        /* try alternative name */
        init = getenv(name.offset(1isize))
    }
    if init.is_null() {
        return 0i32;
    } else if *init.offset(0isize) as libc::c_int == '@' as i32 {
        return dofile(L, init.offset(1isize));
    } else {
        return dostring(L, init, name);
    };
}
/*
** Main body of stand-alone interpreter (to be called in protected mode).
** Reads the options and handles them all.
*/
unsafe extern "C" fn pmain(mut L: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_tointegerx(L, 1i32, 0 as *mut libc::c_int) as libc::c_int;
    let mut argv: *mut *mut libc::c_char = lua_touserdata(L, 2i32) as *mut *mut libc::c_char;
    let mut script: libc::c_int = 0;
    let mut args: libc::c_int = collectargs(argv, &mut script);
    /* check that interpreter has correct version */
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            .wrapping_mul(16i32 as libc::c_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    if !(*argv.offset(0isize)).is_null()
        && 0 != *(*argv.offset(0isize)).offset(0isize) as libc::c_int
    {
        progname = *argv.offset(0isize)
    }
    if args == 1i32 {
        /* bad arg? */
        /* 'script' has index of bad arg. */
        print_usage(*argv.offset(script as isize));
        return 0i32;
    } else {
        /* option '-v'? */
        if 0 != args & 4i32 {
            print_version();
        }
        if 0 != args & 16i32 {
            /* option '-E'? */
            /* signal for libraries to ignore env. vars. */
            lua_pushboolean(L, 1i32);
            lua_setfield(L, -1000000i32 - 1000i32, s!(b"LUA_NOENV\x00"));
        }
        /* open standard libraries */
        luaL_openlibs(L);
        /* create table 'arg' */
        createargtable(L, argv, argc, script);
        if 0 == args & 16i32 {
            /* no option '-E'? */
            /* run LUA_INIT */
            if handle_luainit(L) != 0i32 {
                /* error running LUA_INIT */
                return 0i32;
            }
        }
        /* execute arguments -e and -l */
        if 0 == runargs(L, argv, script) {
            /* something failed */
            return 0i32;
        } else if script < argc && handle_script(L, argv.offset(script as isize)) != 0i32 {
            return 0i32;
        } else {
            /* -i option? */
            if 0 != args & 2i32 {
                /* do read-eval-print loop */
                doREPL(L);
            } else if script == argc && 0 == args & (8i32 | 4i32) {
                /* no arguments? */
                if 0 != isatty(0i32) {
                    /* running in interactive mode? */
                    print_version();
                    /* do read-eval-print loop */
                    doREPL(L);
                } else {
                    /* executes stdin as a file */
                    dofile(L, 0 as *const libc::c_char);
                }
            }
            /* signal no errors */
            lua_pushboolean(L, 1i32);
            return 1i32;
        }
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut result: libc::c_int = 0;
    /* create state */
    let mut L: *mut lua_State = luaL_newstate();
    if L.is_null() {
        l_message(
            *argv.offset(0isize),
            s!(b"cannot create state: not enough memory\x00"),
        );
        return 1i32;
    } else {
        /* to call 'pmain' in protected mode */
        lua_pushcclosure(L, Some(pmain), 0i32);
        /* 1st argument */
        lua_pushinteger(L, argc as lua_Integer);
        /* 2nd argument */
        lua_pushlightuserdata(L, argv as *mut libc::c_void);
        /* do the call */
        status = lua_pcallk(L, 2i32, 1i32, 0i32, 0i32 as lua_KContext, None);
        /* get result */
        result = lua_toboolean(L, -1i32);
        report(L, status);
        lua_close(L);
        return if 0 != result && status == 0i32 {
            0i32
        } else {
            1i32
        };
    };
}
pub fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
