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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    fn localeconv() -> *mut lconv;
    #[no_mangle]
    static mut stdin: *mut FILE;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn tmpfile() -> *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut libc::c_char,
        __modes: libc::c_int,
        __n: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn getc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn __uflow(_: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn ftello(__stream: *mut FILE) -> __off64_t;
    #[no_mangle]
    fn clearerr(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn flockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn funlockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: libc::c_int, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_copy(L: *mut lua_State, fromidx: libc::c_int, toidx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Number;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: libc::c_int, len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State, idx: libc::c_int) -> size_t;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
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
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: libc::c_int) -> ();
    #[no_mangle]
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
    #[no_mangle]
    fn lua_stringtonumber(L: *mut lua_State, s: *const libc::c_char) -> size_t;
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
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, arg: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_newmetatable(L: *mut lua_State, tname: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn luaL_setmetatable(L: *mut lua_State, tname: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_testudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaL_checkudata(
        L: *mut lua_State,
        ud: libc::c_int,
        tname: *const libc::c_char,
    ) -> *mut libc::c_void;
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
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type unnamed = libc::c_uint;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut libc::c_char,
    pub thousands_sep: *mut libc::c_char,
    pub grouping: *mut libc::c_char,
    pub int_curr_symbol: *mut libc::c_char,
    pub currency_symbol: *mut libc::c_char,
    pub mon_decimal_point: *mut libc::c_char,
    pub mon_thousands_sep: *mut libc::c_char,
    pub mon_grouping: *mut libc::c_char,
    pub positive_sign: *mut libc::c_char,
    pub negative_sign: *mut libc::c_char,
    pub int_frac_digits: libc::c_char,
    pub frac_digits: libc::c_char,
    pub p_cs_precedes: libc::c_char,
    pub p_sep_by_space: libc::c_char,
    pub n_cs_precedes: libc::c_char,
    pub n_sep_by_space: libc::c_char,
    pub p_sign_posn: libc::c_char,
    pub n_sign_posn: libc::c_char,
    pub int_p_cs_precedes: libc::c_char,
    pub int_p_sep_by_space: libc::c_char,
    pub int_n_cs_precedes: libc::c_char,
    pub int_n_sep_by_space: libc::c_char,
    pub int_p_sign_posn: libc::c_char,
    pub int_n_sign_posn: libc::c_char,
}
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
pub type off_t = __off64_t;
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
/* }====================================================== */
/*
** {======================================================
** File handles for IO library
** =======================================================
*/
/*
** A file handle is a userdata with metatable 'LUA_FILEHANDLE' and
** initial structure 'luaL_Stream' (it may contain other fields
** after that initial structure).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaL_Stream {
    pub f: *mut FILE,
    pub closef: lua_CFunction,
}
/* skip if char is '+' */
/* check extensions */
/*
** {======================================================
** l_popen spawns a new process connected to the current
** one through the file streams.
** =======================================================
*/
/* { */
/* { */
/* }{ */
/* } */
/* } */
/* }====================================================== */
/* { */
/* } */
/*
** {======================================================
** l_fseek: configuration for longer offsets
** =======================================================
*/
/* { */
/* { */
/* }{ */
/* } */
/* } */
/* }====================================================== */
pub type LStream = luaL_Stream;
/*
** {======================================================
** READ
** =======================================================
*/
/* maximum length of a numeral */
/* auxiliary structure used by 'read_number' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub f: *mut FILE,
    pub c: libc::c_int,
    pub n: libc::c_int,
    pub buff: [libc::c_char; 201],
}
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> libc::c_int {
    return if 0 != ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as libc::c_int as libc::c_long {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh0 as *mut libc::c_uchar) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State) -> libc::c_int {
    /* new module */
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
    luaL_setfuncs(L, iolib.as_ptr(), 0i32);
    createmeta(L);
    /* create (and set) default files */
    createstdfile(
        L,
        stdin,
        b"_IO_input\x00" as *const u8 as *const libc::c_char,
        b"stdin\x00" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stdout,
        b"_IO_output\x00" as *const u8 as *const libc::c_char,
        b"stdout\x00" as *const u8 as *const libc::c_char,
    );
    createstdfile(
        L,
        stderr,
        0 as *const libc::c_char,
        b"stderr\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
unsafe extern "C" fn createstdfile(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut k: *const libc::c_char,
    mut fname: *const libc::c_char,
) -> () {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = f;
    (*p).closef = Some(io_noclose);
    if !k.is_null() {
        lua_pushvalue(L, -1i32);
        /* add file to registry */
        lua_setfield(L, -1000000i32 - 1000i32, k);
    }
    /* add file to module */
    lua_setfield(L, -2i32, fname);
}
/*
** function to (not) close the standard files stdin, stdout, and stderr
*/
unsafe extern "C" fn io_noclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    /* keep file opened */
    (*p).closef = Some(io_noclose);
    lua_pushnil(L);
    lua_pushstring(
        L,
        b"cannot close standard file\x00" as *const u8 as *const libc::c_char,
    );
    return 2i32;
}
/*
** When creating file handles, always creates a 'closed' file handle
** before opening the actual file; so, if there is a memory error, the
** handle is in a consistent state.
*/
unsafe extern "C" fn newprefile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream =
        lua_newuserdata(L, ::std::mem::size_of::<LStream>() as libc::c_ulong) as *mut LStream;
    /* mark file handle as 'closed' */
    (*p).closef = None;
    luaL_setmetatable(L, b"FILE*\x00" as *const u8 as *const libc::c_char);
    return p;
}
unsafe extern "C" fn createmeta(mut L: *mut lua_State) -> () {
    /* create metatable for file handles */
    luaL_newmetatable(L, b"FILE*\x00" as *const u8 as *const libc::c_char);
    /* push metatable */
    lua_pushvalue(L, -1i32);
    /* metatable.__index = metatable */
    lua_setfield(L, -2i32, b"__index\x00" as *const u8 as *const libc::c_char);
    /* add file methods to new metatable */
    luaL_setfuncs(L, flib.as_ptr(), 0i32);
    /* pop new metatable */
    lua_settop(L, -1i32 - 1i32);
}
/*
** methods for file handles
*/
static mut flib: [luaL_Reg; 10] = unsafe {
    [
        luaL_Reg {
            name: b"close\x00" as *const u8 as *const libc::c_char,
            func: Some(f_close),
        },
        luaL_Reg {
            name: b"flush\x00" as *const u8 as *const libc::c_char,
            func: Some(f_flush),
        },
        luaL_Reg {
            name: b"lines\x00" as *const u8 as *const libc::c_char,
            func: Some(f_lines),
        },
        luaL_Reg {
            name: b"read\x00" as *const u8 as *const libc::c_char,
            func: Some(f_read),
        },
        luaL_Reg {
            name: b"seek\x00" as *const u8 as *const libc::c_char,
            func: Some(f_seek),
        },
        luaL_Reg {
            name: b"setvbuf\x00" as *const u8 as *const libc::c_char,
            func: Some(f_setvbuf),
        },
        luaL_Reg {
            name: b"write\x00" as *const u8 as *const libc::c_char,
            func: Some(f_write),
        },
        luaL_Reg {
            name: b"__gc\x00" as *const u8 as *const libc::c_char,
            func: Some(f_gc),
        },
        luaL_Reg {
            name: b"__tostring\x00" as *const u8 as *const libc::c_char,
            func: Some(f_tostring),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn f_tostring(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    if (*p).closef.is_none() {
        lua_pushstring(L, b"file (closed)\x00" as *const u8 as *const libc::c_char);
    } else {
        lua_pushfstring(
            L,
            b"file (%p)\x00" as *const u8 as *const libc::c_char,
            (*p).f,
        );
    }
    return 1i32;
}
unsafe extern "C" fn f_gc(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    if (*p).closef.is_some() && !(*p).f.is_null() {
        /* ignore closed and incompletely open files */
        aux_close(L);
    }
    return 0i32;
}
/*
** Calls the 'close' function from a file handle. The 'volatile' avoids
** a bug in some versions of the Clang compiler (e.g., clang 3.0 for
** 32 bits).
*/
unsafe extern "C" fn aux_close(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    let mut cf: lua_CFunction = (*p).closef;
    /* mark stream as closed */
    (*p).closef = None;
    /* close it */
    return cf.expect("non-null function pointer")(L);
}
unsafe extern "C" fn f_write(mut L: *mut lua_State) -> libc::c_int {
    let mut f: *mut FILE = tofile(L);
    /* push file at the stack top (to be returned) */
    lua_pushvalue(L, 1i32);
    return g_write(L, f, 2i32);
}
unsafe extern "C" fn tofile(mut L: *mut lua_State) -> *mut FILE {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    if (*p).closef.is_none() {
        luaL_error(
            L,
            b"attempt to use a closed file\x00" as *const u8 as *const libc::c_char,
        );
    }
    return (*p).f;
}
/* }====================================================== */
unsafe extern "C" fn g_write(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut arg: libc::c_int,
) -> libc::c_int {
    let mut len: libc::c_int = 0;
    let mut nargs: libc::c_int = lua_gettop(L) - arg;
    let mut status: libc::c_int = 1i32;
    loop {
        let fresh1 = nargs;
        nargs = nargs - 1;
        if !(0 != fresh1) {
            break;
        }
        if lua_type(L, arg) == 3i32 {
            /* optimization: could be done exactly as for strings */
            len = if 0 != lua_isinteger(L, arg) {
                fprintf(
                    f,
                    b"%lld\x00" as *const u8 as *const libc::c_char,
                    lua_tointegerx(L, arg, 0 as *mut libc::c_int),
                )
            } else {
                fprintf(
                    f,
                    b"%.14g\x00" as *const u8 as *const libc::c_char,
                    lua_tonumberx(L, arg, 0 as *mut libc::c_int),
                )
            };
            status = (0 != status && len > 0i32) as libc::c_int
        } else {
            let mut l: size_t = 0;
            let mut s: *const libc::c_char = luaL_checklstring(L, arg, &mut l);
            status = (0 != status
                && fwrite(
                    s as *const libc::c_void,
                    ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                    l,
                    f,
                ) == l) as libc::c_int
        }
        arg += 1
    }
    if 0 != status {
        /* file handle already on stack top */
        return 1i32;
    } else {
        return luaL_fileresult(L, status, 0 as *const libc::c_char);
    };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = unsafe { [2i32, 0i32, 1i32] };
    static mut modenames: [*const libc::c_char; 4] = unsafe {
        [
            b"no\x00" as *const u8 as *const libc::c_char,
            b"full\x00" as *const u8 as *const libc::c_char,
            b"line\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ]
    };
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int =
        luaL_checkoption(L, 2i32, 0 as *const libc::c_char, modenames.as_ptr());
    let mut sz: lua_Integer = luaL_optinteger(
        L,
        3i32,
        (0x80i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            as libc::c_int as lua_Integer,
    );
    let mut res: libc::c_int = setvbuf(f, 0 as *mut libc::c_char, mode[op as usize], sz as size_t);
    return luaL_fileresult(L, (res == 0i32) as libc::c_int, 0 as *const libc::c_char);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State) -> libc::c_int {
    static mut mode: [libc::c_int; 3] = unsafe { [0i32, 1i32, 2i32] };
    static mut modenames: [*const libc::c_char; 4] = unsafe {
        [
            b"set\x00" as *const u8 as *const libc::c_char,
            b"cur\x00" as *const u8 as *const libc::c_char,
            b"end\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ]
    };
    let mut f: *mut FILE = tofile(L);
    let mut op: libc::c_int = luaL_checkoption(
        L,
        2i32,
        b"cur\x00" as *const u8 as *const libc::c_char,
        modenames.as_ptr(),
    );
    let mut p3: lua_Integer = luaL_optinteger(L, 3i32, 0i32 as lua_Integer);
    let mut offset: off_t = p3 as off_t;
    (offset as lua_Integer == p3
        || 0 != luaL_argerror(
            L,
            3i32,
            b"not an integer in proper range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    op = fseeko(f, offset, mode[op as usize]);
    if 0 != op {
        /* error */
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    } else {
        lua_pushinteger(L, ftello(f) as lua_Integer);
        return 1i32;
    };
}
unsafe extern "C" fn f_read(mut L: *mut lua_State) -> libc::c_int {
    return g_read(L, tofile(L), 2i32);
}
unsafe extern "C" fn g_read(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut first: libc::c_int,
) -> libc::c_int {
    let mut l: size_t = 0;
    let mut nargs: libc::c_int = lua_gettop(L) - 1i32;
    let mut success: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    clearerr(f);
    if nargs == 0i32 {
        /* no arguments? */
        success = read_line(L, f, 1i32);
        /* to return 1 result */
        n = first + 1i32
    } else {
        /* ensure stack space for all results and for auxlib's buffer */
        luaL_checkstack(
            L,
            nargs + 20i32,
            b"too many arguments\x00" as *const u8 as *const libc::c_char,
        );
        success = 1i32;
        n = first;
        loop {
            let fresh2 = nargs;
            nargs = nargs - 1;
            if !(0 != fresh2 && 0 != success) {
                break;
            }
            if lua_type(L, n) == 3i32 {
                l = luaL_checkinteger(L, n) as size_t;
                success = if l == 0i32 as libc::c_ulong {
                    test_eof(L, f)
                } else {
                    read_chars(L, f, l)
                }
            } else {
                let mut p: *const libc::c_char = luaL_checklstring(L, n, 0 as *mut size_t);
                if *p as libc::c_int == '*' as i32 {
                    /* skip optional '*' (for compatibility) */
                    p = p.offset(1isize)
                }
                match *p as libc::c_int {
                    110 => success = read_number(L, f),
                    108 => success = read_line(L, f, 1i32),
                    76 => success = read_line(L, f, 0i32),
                    97 => {
                        /* read entire file */
                        read_all(L, f);
                        /* always success */
                        success = 1i32
                    }
                    _ => {
                        return luaL_argerror(
                            L,
                            n,
                            b"invalid format\x00" as *const u8 as *const libc::c_char,
                        )
                    }
                }
            }
            n += 1
        }
    }
    if 0 != ferror(f) {
        return luaL_fileresult(L, 0i32, 0 as *const libc::c_char);
    } else {
        if 0 == success {
            /* remove last result */
            lua_settop(L, -1i32 - 1i32);
            /* push nil instead */
            lua_pushnil(L);
        }
        return n - first;
    };
}
unsafe extern "C" fn read_all(mut L: *mut lua_State, mut f: *mut FILE) -> () {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nr: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        /* read file in chunks of LUAL_BUFFERSIZE bytes */
        p = luaL_prepbuffsize(
            &mut b,
            (0x80i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
                as libc::c_int as size_t,
        );
        nr = fread(
            p as *mut libc::c_void,
            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
            (0x80i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
                as libc::c_int as size_t,
            f,
        );
        b.n = (b.n as libc::c_ulong).wrapping_add(nr) as size_t as size_t;
        if !(nr == (0x80i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            as libc::c_int as libc::c_ulong)
        {
            break;
        }
    }
    /* close buffer */
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn read_line(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut chop: libc::c_int,
) -> libc::c_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut c: libc::c_int = '\u{0}' as i32;
    luaL_buffinit(L, &mut b);
    while c != -1i32 && c != '\n' as i32 {
        /* repeat until end of line */
        /* preallocate buffer */
        let mut buff: *mut libc::c_char = luaL_prepbuffsize(
            &mut b,
            (0x80i32 as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
                as libc::c_int as size_t,
        );
        let mut i: libc::c_int = 0i32;
        /* no memory errors can happen inside the lock */
        flockfile(f);
        while i < (0x80i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong)
            as libc::c_int && {
            c = getc_unlocked(f);
            c != -1i32
        } && c != '\n' as i32
        {
            let fresh3 = i;
            i = i + 1;
            *buff.offset(fresh3 as isize) = c as libc::c_char
        }
        funlockfile(f);
        b.n = (b.n as libc::c_ulong).wrapping_add(i as libc::c_ulong) as size_t as size_t
    }
    /* want a newline and have one? */
    if 0 == chop && c == '\n' as i32 {
        /* add ending newline to result */
        (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as libc::c_int;
        let fresh4 = b.n;
        b.n = b.n.wrapping_add(1);
        *b.b.offset(fresh4 as isize) = c as libc::c_char
    }
    /* close buffer */
    luaL_pushresult(&mut b);
    /* return ok if read something (either a newline or something else) */
    return (c == '\n' as i32 || lua_rawlen(L, -1i32) > 0i32 as libc::c_ulong) as libc::c_int;
}
/*
** Read a number: first reads a valid prefix of a numeral into a buffer.
** Then it calls 'lua_stringtonumber' to check whether the format is
** correct and to convert it to a Lua number
*/
unsafe extern "C" fn read_number(mut L: *mut lua_State, mut f: *mut FILE) -> libc::c_int {
    let mut rn: RN = RN {
        f: 0 as *mut FILE,
        c: 0,
        n: 0,
        buff: [0; 201],
    };
    let mut count: libc::c_int = 0i32;
    let mut hex: libc::c_int = 0i32;
    let mut decp: [libc::c_char; 2] = [0; 2];
    rn.f = f;
    rn.n = 0i32;
    /* get decimal point from locale */
    decp[0usize] = *(*localeconv()).decimal_point.offset(0isize);
    /* always accept a dot */
    decp[1usize] = '.' as i32 as libc::c_char;
    flockfile(rn.f);
    loop {
        /* skip spaces */
        rn.c = getc_unlocked(rn.f);
        if !(0 != *(*__ctype_b_loc()).offset(rn.c as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int)
        {
            break;
        }
    }
    /* optional signal */
    test2(&mut rn, b"-+\x00" as *const u8 as *const libc::c_char);
    if 0 != test2(&mut rn, b"00\x00" as *const u8 as *const libc::c_char) {
        if 0 != test2(&mut rn, b"xX\x00" as *const u8 as *const libc::c_char) {
            /* numeral is hexadecimal */
            hex = 1i32
        } else {
            count = 1i32
        }
    }
    /* integral part */
    count += readdigits(&mut rn, hex);
    /* decimal point? */
    if 0 != test2(&mut rn, decp.as_mut_ptr()) {
        /* fractional part */
        count += readdigits(&mut rn, hex)
    }
    if count > 0i32
        && 0 != test2(
            &mut rn,
            if 0 != hex {
                b"pP\x00" as *const u8 as *const libc::c_char
            } else {
                b"eE\x00" as *const u8 as *const libc::c_char
            },
        ) {
        /* exponent mark? */
        /* exponent signal */
        test2(&mut rn, b"-+\x00" as *const u8 as *const libc::c_char);
        /* exponent digits */
        readdigits(&mut rn, 0i32);
    }
    /* unread look-ahead char */
    ungetc(rn.c, rn.f);
    funlockfile(rn.f);
    /* finish string */
    rn.buff[rn.n as usize] = '\u{0}' as i32 as libc::c_char;
    /* is this a valid number? */
    if 0 != lua_stringtonumber(L, rn.buff.as_mut_ptr()) {
        /* ok */
        return 1i32;
    } else {
        /* invalid format */
        /* "result" to be removed */
        lua_pushnil(L);
        /* read fails */
        return 0i32;
    };
}
/*
** Read a sequence of (hex)digits
*/
unsafe extern "C" fn readdigits(mut rn: *mut RN, mut hex: libc::c_int) -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    while 0 != if 0 != hex {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as libc::c_int
            & _ISxdigit as libc::c_int as libc::c_ushort as libc::c_int
    } else {
        *(*__ctype_b_loc()).offset((*rn).c as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
    } && 0 != nextc(rn)
    {
        count += 1
    }
    return count;
}
/*
** Add current char to buffer (if not out of space) and read next one
*/
unsafe extern "C" fn nextc(mut rn: *mut RN) -> libc::c_int {
    if (*rn).n >= 200i32 {
        /* buffer overflow? */
        /* invalidate result */
        (*rn).buff[0usize] = '\u{0}' as i32 as libc::c_char;
        /* fail */
        return 0i32;
    } else {
        /* save current char */
        let fresh5 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buff[fresh5 as usize] = (*rn).c as libc::c_char;
        /* read next one */
        (*rn).c = getc_unlocked((*rn).f);
        return 1i32;
    };
}
/*
** Accept current char if it is in 'set' (of size 2)
*/
unsafe extern "C" fn test2(mut rn: *mut RN, mut set: *const libc::c_char) -> libc::c_int {
    if (*rn).c == *set.offset(0isize) as libc::c_int
        || (*rn).c == *set.offset(1isize) as libc::c_int
    {
        return nextc(rn);
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn read_chars(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut n: size_t,
) -> libc::c_int {
    /* number of chars actually read */
    let mut nr: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut libc::c_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    /* prepare buffer to read whole block */
    p = luaL_prepbuffsize(&mut b, n);
    /* try to read 'n' chars */
    nr = fread(
        p as *mut libc::c_void,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
        n,
        f,
    );
    b.n = (b.n as libc::c_ulong).wrapping_add(nr) as size_t as size_t;
    /* close buffer */
    luaL_pushresult(&mut b);
    /* true iff read something */
    return (nr > 0i32 as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State, mut f: *mut FILE) -> libc::c_int {
    let mut c: libc::c_int = getc(f);
    /* no-op when c == EOF */
    ungetc(c, f);
    lua_pushstring(L, b"\x00" as *const u8 as *const libc::c_char);
    return (c != -1i32) as libc::c_int;
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State) -> libc::c_int {
    /* check that it's a valid file handle */
    tofile(L);
    aux_lines(L, 0i32);
    return 1i32;
}
/*
** maximum number of arguments to 'f:lines'/'io.lines' (it + 3 must fit
** in the limit for upvalues of a closure)
*/
unsafe extern "C" fn aux_lines(mut L: *mut lua_State, mut toclose: libc::c_int) -> () {
    /* number of arguments to read */
    let mut n: libc::c_int = lua_gettop(L) - 1i32;
    (n <= 250i32
        || 0 != luaL_argerror(
            L,
            250i32 + 2i32,
            b"too many arguments\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    /* number of arguments to read */
    lua_pushinteger(L, n as lua_Integer);
    /* close/not close file when finished */
    lua_pushboolean(L, toclose);
    /* move 'n' and 'toclose' to their positions */
    lua_rotate(L, 2i32, 2i32);
    lua_pushcclosure(L, Some(io_readline), 3i32 + n);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream = lua_touserdata(L, -1000000i32 - 1000i32 - 1i32) as *mut LStream;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int =
        lua_tointegerx(L, -1000000i32 - 1000i32 - 2i32, 0 as *mut libc::c_int) as libc::c_int;
    /* file is already closed? */
    if (*p).closef.is_none() {
        return luaL_error(
            L,
            b"file is already closed\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        lua_settop(L, 1i32);
        luaL_checkstack(
            L,
            n,
            b"too many arguments\x00" as *const u8 as *const libc::c_char,
        );
        /* push arguments to 'g_read' */
        i = 1i32;
        while i <= n {
            lua_pushvalue(L, -1000000i32 - 1000i32 - (3i32 + i));
            i += 1
        }
        /* 'n' is number of results */
        n = g_read(L, (*p).f, 2i32);
        /* should return at least a nil */
        /* read at least one value? */
        if 0 != lua_toboolean(L, -n) {
            /* return them */
            return n;
        } else if n > 1i32 {
            /* is there error information? */
            /* 2nd result is error message */
            return luaL_error(
                L,
                b"%s\x00" as *const u8 as *const libc::c_char,
                lua_tolstring(L, -n + 1i32, 0 as *mut size_t),
            );
        } else {
            if 0 != lua_toboolean(L, -1000000i32 - 1000i32 - 3i32) {
                /* generator created file? */
                lua_settop(L, 0i32);
                lua_pushvalue(L, -1000000i32 - 1000i32 - 1i32);
                /* close it */
                aux_close(L);
            }
            return 0i32;
        }
    };
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State) -> libc::c_int {
    return luaL_fileresult(
        L,
        (fflush(tofile(L)) == 0i32) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn f_close(mut L: *mut lua_State) -> libc::c_int {
    /* make sure argument is an open stream */
    tofile(L);
    return aux_close(L);
}
/*
** functions for 'io' library
*/
static mut iolib: [luaL_Reg; 12] = unsafe {
    [
        luaL_Reg {
            name: b"close\x00" as *const u8 as *const libc::c_char,
            func: Some(io_close),
        },
        luaL_Reg {
            name: b"flush\x00" as *const u8 as *const libc::c_char,
            func: Some(io_flush),
        },
        luaL_Reg {
            name: b"input\x00" as *const u8 as *const libc::c_char,
            func: Some(io_input),
        },
        luaL_Reg {
            name: b"lines\x00" as *const u8 as *const libc::c_char,
            func: Some(io_lines),
        },
        luaL_Reg {
            name: b"open\x00" as *const u8 as *const libc::c_char,
            func: Some(io_open),
        },
        luaL_Reg {
            name: b"output\x00" as *const u8 as *const libc::c_char,
            func: Some(io_output),
        },
        luaL_Reg {
            name: b"popen\x00" as *const u8 as *const libc::c_char,
            func: Some(io_popen),
        },
        luaL_Reg {
            name: b"read\x00" as *const u8 as *const libc::c_char,
            func: Some(io_read),
        },
        luaL_Reg {
            name: b"tmpfile\x00" as *const u8 as *const libc::c_char,
            func: Some(io_tmpfile),
        },
        luaL_Reg {
            name: b"type\x00" as *const u8 as *const libc::c_char,
            func: Some(io_type),
        },
        luaL_Reg {
            name: b"write\x00" as *const u8 as *const libc::c_char,
            func: Some(io_write),
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn io_write(mut L: *mut lua_State) -> libc::c_int {
    return g_write(
        L,
        getiofile(L, b"_IO_output\x00" as *const u8 as *const libc::c_char),
        1i32,
    );
}
unsafe extern "C" fn getiofile(
    mut L: *mut lua_State,
    mut findex: *const libc::c_char,
) -> *mut FILE {
    let mut p: *mut LStream = 0 as *mut LStream;
    lua_getfield(L, -1000000i32 - 1000i32, findex);
    p = lua_touserdata(L, -1i32) as *mut LStream;
    if (*p).closef.is_none() {
        luaL_error(
            L,
            b"standard %s file is closed\x00" as *const u8 as *const libc::c_char,
            findex.offset(
                (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as isize,
            ),
        );
    }
    return (*p).f;
}
unsafe extern "C" fn io_type(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream = 0 as *mut LStream;
    luaL_checkany(L, 1i32);
    p = luaL_testudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    if p.is_null() {
        /* not a file */
        lua_pushnil(L);
    } else if (*p).closef.is_none() {
        lua_pushstring(L, b"closed file\x00" as *const u8 as *const libc::c_char);
    } else {
        lua_pushstring(L, b"file\x00" as *const u8 as *const libc::c_char);
    }
    return 1i32;
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream = newfile(L);
    (*p).f = tmpfile();
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, 0 as *const libc::c_char)
    } else {
        1i32
    };
}
unsafe extern "C" fn newfile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = 0 as *mut FILE;
    (*p).closef = Some(io_fclose);
    return p;
}
/*
** function to close regular files
*/
unsafe extern "C" fn io_fclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    let mut res: libc::c_int = fclose((*p).f);
    return luaL_fileresult(L, (res == 0i32) as libc::c_int, 0 as *const libc::c_char);
}
unsafe extern "C" fn io_read(mut L: *mut lua_State) -> libc::c_int {
    return g_read(
        L,
        getiofile(L, b"_IO_input\x00" as *const u8 as *const libc::c_char),
        1i32,
    );
}
unsafe extern "C" fn io_popen(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"r\x00" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut p: *mut LStream = newprefile(L);
    fflush(0 as *mut FILE);
    (*p).f = popen(filename, mode);
    (*p).closef = Some(io_pclose);
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
/*
** function to close 'popen' files
*/
unsafe extern "C" fn io_pclose(mut L: *mut lua_State) -> libc::c_int {
    let mut p: *mut LStream =
        luaL_checkudata(L, 1i32, b"FILE*\x00" as *const u8 as *const libc::c_char) as *mut LStream;
    return luaL_execresult(L, pclose((*p).f));
}
unsafe extern "C" fn io_output(mut L: *mut lua_State) -> libc::c_int {
    return g_iofile(
        L,
        b"_IO_output\x00" as *const u8 as *const libc::c_char,
        b"w\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn g_iofile(
    mut L: *mut lua_State,
    mut f: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    if !(lua_type(L, 1i32) <= 0i32) {
        let mut filename: *const libc::c_char = lua_tolstring(L, 1i32, 0 as *mut size_t);
        if !filename.is_null() {
            opencheck(L, filename, mode);
        } else {
            /* check that it's a valid file handle */
            tofile(L);
            lua_pushvalue(L, 1i32);
        }
        lua_setfield(L, -1000000i32 - 1000i32, f);
    }
    /* return current value */
    lua_getfield(L, -1000000i32 - 1000i32, f);
    return 1i32;
}
unsafe extern "C" fn opencheck(
    mut L: *mut lua_State,
    mut fname: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> () {
    let mut p: *mut LStream = newfile(L);
    (*p).f = fopen(fname, mode);
    if (*p).f.is_null() {
        luaL_error(
            L,
            b"cannot open file \'%s\' (%s)\x00" as *const u8 as *const libc::c_char,
            fname,
            strerror(*__errno_location()),
        );
    };
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const libc::c_char = luaL_optlstring(
        L,
        2i32,
        b"r\x00" as *const u8 as *const libc::c_char,
        0 as *mut size_t,
    );
    let mut p: *mut LStream = newfile(L);
    /* to traverse/check mode */
    let mut md: *const libc::c_char = mode;
    (0 != l_checkmode(md)
        || 0 != luaL_argerror(
            L,
            2i32,
            b"invalid mode\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (*p).f = fopen(filename, mode);
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
/*
** $Id: liolib.c,v 2.151.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard I/O (and system) library
** See Copyright Notice in lua.h
*/
/*
** Change this macro to accept other modes for 'fopen' besides
** the standard ones.
*/
/* accepted extensions to 'mode' in 'fopen' */
/* Check whether 'mode' matches '[rwa]%+?[L_MODEEXT]*' */
unsafe extern "C" fn l_checkmode(mut mode: *const libc::c_char) -> libc::c_int {
    return (*mode as libc::c_int != '\u{0}' as i32 && {
        let fresh6 = mode;
        mode = mode.offset(1);
        !strchr(
            b"rwa\x00" as *const u8 as *const libc::c_char,
            *fresh6 as libc::c_int,
        ).is_null()
    } && (*mode as libc::c_int != '+' as i32 || {
        mode = mode.offset(1isize);
        0 != 1i32
    }) && strspn(mode, b"b\x00" as *const u8 as *const libc::c_char) == strlen(mode))
        as libc::c_int;
}
unsafe extern "C" fn io_lines(mut L: *mut lua_State) -> libc::c_int {
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut toclose: libc::c_int = 0;
    if lua_type(L, 1i32) == -1i32 {
        /* at least one argument */
        lua_pushnil(L);
    }
    if lua_type(L, 1i32) == 0i32 {
        /* no file name? */
        /* get default input */
        lua_getfield(
            L,
            -1000000i32 - 1000i32,
            b"_IO_input\x00" as *const u8 as *const libc::c_char,
        );
        /* put it at index 1 */
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        /* check that it's a valid file handle */
        tofile(L);
        /* do not close it after iteration */
        toclose = 0i32
    } else {
        /* open a new file */
        filename = luaL_checklstring(L, 1i32, 0 as *mut size_t);
        opencheck(L, filename, b"r\x00" as *const u8 as *const libc::c_char);
        /* put file at index 1 */
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        /* close it after iteration */
        toclose = 1i32
    }
    aux_lines(L, toclose);
    return 1i32;
}
unsafe extern "C" fn io_input(mut L: *mut lua_State) -> libc::c_int {
    return g_iofile(
        L,
        b"_IO_input\x00" as *const u8 as *const libc::c_char,
        b"r\x00" as *const u8 as *const libc::c_char,
    );
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State) -> libc::c_int {
    return luaL_fileresult(
        L,
        (fflush(getiofile(
            L,
            b"_IO_output\x00" as *const u8 as *const libc::c_char,
        )) == 0i32) as libc::c_int,
        0 as *const libc::c_char,
    );
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> libc::c_int {
    /* no argument? */
    if lua_type(L, 1i32) == -1i32 {
        /* use standard output */
        lua_getfield(
            L,
            -1000000i32 - 1000i32,
            b"_IO_output\x00" as *const u8 as *const libc::c_char,
        );
    }
    return f_close(L);
}