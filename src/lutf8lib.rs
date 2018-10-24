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
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Integer;
    /*
     ** push functions (C -> stack)
     */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushlstring(
        L: *mut lua_State,
        s: *const lua_char,
        len: size_t,
    ) -> *const lua_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
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
    fn luaL_checkinteger(L: *mut lua_State, arg: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: lua_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, sz: lua_int, msg: *const lua_char) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
}
pub type size_t = lua_ulong;
/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = lua_double;
/* type for integer functions */
pub type lua_Integer = lua_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> lua_int>;
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
#[no_mangle]
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 7]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0i32);
    lua_pushlstring(
        L,
        s!(b"[\x00-\x7f\xc2-\xf4][\x80-\xbf]*\x00"),
        (::std::mem::size_of::<[lua_char; 15]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    lua_setfield(L, -2i32, s!(b"charpattern\x00"));
    return 1i32;
}
/* pattern to match a single UTF-8 character */
static mut funcs: [luaL_Reg; 7] = [
    lua_reg!(b"offset\x00", byteoffset),
    lua_reg!(b"codepoint\x00", codepoint),
    lua_reg!(b"char\x00", utfchar),
    lua_reg!(b"len\x00", utflen),
    lua_reg!(b"codes\x00", iter_codes),
    lua_reg_none!(b"charpattern\x00"),
    lua_reg_none!(0),
];
unsafe extern "C" fn iter_codes(mut L: *mut lua_State) -> lua_int {
    luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_pushcclosure(L, Some(iter_aux), 0i32);
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, 0i32 as lua_Integer);
    return 3i32;
}
unsafe extern "C" fn iter_aux(mut L: *mut lua_State) -> lua_int {
    let mut len: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer =
        lua_tointegerx(L, 2i32, 0 as *mut lua_int) - 1i32 as lua_longlong;
    /* first iteration? */
    if n < 0i32 as lua_longlong {
        /* start from here */
        n = 0i32 as lua_Integer
    } else if n < len as lua_Integer {
        /* skip current byte */
        n += 1;
        while *s.offset(n as isize) as lua_int & 0xc0i32 == 0x80i32 {
            /* and its continuations */
            n += 1
        }
    }
    if n >= len as lua_Integer {
        /* no more codepoints */
        return 0i32;
    } else {
        let mut code: lua_int = 0;
        let mut next: *const lua_char = utf8_decode(s.offset(n as isize), &mut code);
        if next.is_null() || *next as lua_int & 0xc0i32 == 0x80i32 {
            return luaL_error!(L, s!(b"invalid UTF-8 code\x00"));
        } else {
            lua_pushinteger(L, n + 1i32 as lua_longlong);
            lua_pushinteger(L, code as lua_Integer);
            return 2i32;
        }
    };
}
/*
** Decode one UTF-8 sequence, returning NULL if byte sequence is invalid.
*/
unsafe extern "C" fn utf8_decode(
    mut o: *const lua_char,
    mut val: *mut lua_int,
) -> *const lua_char {
    static mut limits: [lua_uint; 4] = [
        0xffi32 as lua_uint,
        0x7fi32 as lua_uint,
        0x7ffi32 as lua_uint,
        0xffffi32 as lua_uint,
    ];
    let mut s: *const lua_uchar = o as *const lua_uchar;
    let mut c: lua_uint = *s.offset(0isize) as lua_uint;
    /* final result */
    let mut res: lua_uint = 0i32 as lua_uint;
    /* ascii? */
    if c < 0x80i32 as lua_uint {
        res = c
    } else {
        /* to count number of continuation bytes */
        let mut count: lua_int = 0i32;
        while 0 != c & 0x40i32 as lua_uint {
            /* still have continuation bytes? */
            /* read next byte */
            count += 1;
            let mut cc: lua_int = *s.offset(count as isize) as lua_int;
            /* not a continuation byte? */
            if cc & 0xc0i32 != 0x80i32 {
                /* invalid byte sequence */
                return 0 as *const lua_char;
            } else {
                /* add lower 6 bits from cont. byte */
                res = res << 6i32 | (cc & 0x3fi32) as lua_uint;
                /* to test next bit */
                c <<= 1i32
            }
        }
        /* add first byte */
        res |= (c & 0x7fi32 as lua_uint) << count * 5i32;
        if count > 3i32 || res > 0x10ffffi32 as lua_uint || res <= limits[count as usize] {
            /* invalid byte sequence */
            return 0 as *const lua_char;
        } else {
            s = s.offset(count as isize)
        }
    }
    if !val.is_null() {
        *val = res as lua_int
    }
    /* +1 to include first byte */
    return (s as *const lua_char).offset(1isize);
}
/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_int = 0i32;
    let mut len: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut posj: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, -1i32 as lua_Integer), len);
    (1i32 as lua_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    } || 0 != luaL_argerror(L, 2i32, s!(b"initial position out of string\x00"))) as lua_int;
    posj -= 1;
    (posj < len as lua_Integer
        || 0 != luaL_argerror(L, 3i32, s!(b"final position out of string\x00"))) as lua_int;
    while posi <= posj {
        let mut s1: *const lua_char =
            utf8_decode(s.offset(posi as isize), 0 as *mut lua_int);
        if s1.is_null() {
            /* conversion error? */
            /* return nil ... */
            lua_pushnil(L);
            /* ... and current position */
            lua_pushinteger(L, posi + 1i32 as lua_longlong);
            return 2i32;
        } else {
            posi = s1.wrapping_offset_from(s) as lua_long as lua_Integer;
            n += 1
        }
    }
    lua_pushinteger(L, n as lua_Integer);
    return 1i32;
}
/*
** $Id: lutf8lib.c,v 1.16.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard library for UTF-8 manipulation
** See Copyright Notice in lua.h
*/
/* from strlib */
/* translate a relative string position: negative means back from end */
unsafe extern "C" fn u_posrelat(mut pos: lua_Integer, mut len: size_t) -> lua_Integer {
    if pos >= 0i32 as lua_longlong {
        return pos;
    } else if (0u32 as lua_ulong).wrapping_sub(pos as size_t) > len {
        return 0i32 as lua_Integer;
    } else {
        return len as lua_Integer + pos + 1i32 as lua_longlong;
    };
}
/*
** utfchar(n1, n2, ...)  -> char(n1)..char(n2)...
*/
unsafe extern "C" fn utfchar(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    /* optimize common case of single char */
    if n == 1i32 {
        pushutfchar(L, 1i32);
    } else {
        let mut i: lua_int = 0;
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut lua_char,
            size: 0,
            n: 0,
            L: 0 as *mut lua_State,
            initb: [0; 8192],
        };
        luaL_buffinit(L, &mut b);
        i = 1i32;
        while i <= n {
            pushutfchar(L, i);
            luaL_addvalue(&mut b);
            i += 1
        }
        luaL_pushresult(&mut b);
    }
    return 1i32;
}
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State, mut arg: lua_int) -> () {
    let mut code: lua_Integer = luaL_checkinteger(L, arg);
    (0i32 as lua_longlong <= code && code <= 0x10ffffi32 as lua_longlong
        || 0 != luaL_argerror(L, arg, s!(b"value out of range\x00"))) as lua_int;
    lua_pushfstring!(L, s!(b"%U\x00"), code as lua_long,);
}
/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> lua_int {
    let mut len: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut pose: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    let mut n: lua_int = 0;
    let mut se: *const lua_char = 0 as *const lua_char;
    (posi >= 1i32 as lua_longlong || 0 != luaL_argerror(L, 2i32, s!(b"out of range\x00")))
        as lua_int;
    (pose <= len as lua_Integer || 0 != luaL_argerror(L, 3i32, s!(b"out of range\x00")))
        as lua_int;
    if posi > pose {
        /* empty interval; return no values */
        return 0i32;
    } else if pose - posi >= 2147483647i32 as lua_longlong {
        return luaL_error!(L, s!(b"string slice too long\x00"));
    } else {
        n = (pose - posi) as lua_int + 1i32;
        luaL_checkstack(L, n, s!(b"string slice too long\x00"));
        n = 0i32;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1i32 as lua_longlong) as isize);
        while s < se {
            let mut code: lua_int = 0;
            s = utf8_decode(s, &mut code);
            if s.is_null() {
                return luaL_error!(L, s!(b"invalid UTF-8 code\x00"));
            } else {
                lua_pushinteger(L, code as lua_Integer);
                n += 1
            }
        }
        return n;
    };
}
/*
** offset(s, n, [i])  -> index where n-th character counting from
**   position 'i' starts; 0 means character at 'i'.
*/
unsafe extern "C" fn byteoffset(mut L: *mut lua_State) -> lua_int {
    let mut len: size_t = 0;
    let mut s: *const lua_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut posi: lua_Integer = (if n >= 0i32 as lua_longlong {
        1i32 as lua_ulong
    } else {
        len.wrapping_add(1i32 as lua_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    (1i32 as lua_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    } || 0 != luaL_argerror(L, 3i32, s!(b"position out of range\x00"))) as lua_int;
    if n == 0i32 as lua_longlong {
        /* find beginning of current byte sequence */
        while posi > 0i32 as lua_longlong
            && *s.offset(posi as isize) as lua_int & 0xc0i32 == 0x80i32
        {
            posi -= 1
        }
    } else if *s.offset(posi as isize) as lua_int & 0xc0i32 == 0x80i32 {
        return luaL_error!(L, s!(b"initial position is a continuation byte\x00"));
    } else if n < 0i32 as lua_longlong {
        while n < 0i32 as lua_longlong && posi > 0i32 as lua_longlong {
            /* move back */
            loop {
                /* find beginning of previous character */
                posi -= 1;
                if !(posi > 0i32 as lua_longlong
                    && *s.offset(posi as isize) as lua_int & 0xc0i32 == 0x80i32)
                {
                    break;
                }
            }
            n += 1
        }
    } else {
        /* do not move for 1st character */
        n -= 1;
        while n > 0i32 as lua_longlong && posi < len as lua_Integer {
            loop {
                /* find beginning of next character */
                posi += 1;
                if !(*s.offset(posi as isize) as lua_int & 0xc0i32 == 0x80i32) {
                    break;
                }
            }
            /* (cannot pass final '\0') */
            n -= 1
        }
    }
    /* did it find given character? */
    if n == 0i32 as lua_longlong {
        lua_pushinteger(L, posi + 1i32 as lua_longlong);
    } else {
        /* no such character */
        lua_pushnil(L);
    }
    return 1i32;
}
