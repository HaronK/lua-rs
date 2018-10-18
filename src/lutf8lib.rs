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
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int) -> ();
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
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
        s: *const libc::c_char,
        len: size_t,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char) -> ();
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
    fn luaL_checkinteger(L: *mut lua_State, arg: libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_optinteger(L: *mut lua_State, arg: libc::c_int, def: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkstack(L: *mut lua_State, sz: libc::c_int, msg: *const libc::c_char) -> ();
    #[no_mangle]
    fn luaL_error(L: *mut lua_State, fmt: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: libc::c_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
}
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn luaopen_utf8(mut L: *mut lua_State) -> libc::c_int {
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
        (::std::mem::size_of::<[luaL_Reg; 7]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int,
    );
    luaL_setfuncs(L, funcs.as_ptr(), 0i32);
    lua_pushlstring(
        L,
        b"[\x00-\x7f\xc2-\xf4][\x80-\xbf]*\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 15]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    lua_setfield(
        L,
        -2i32,
        b"charpattern\x00" as *const u8 as *const libc::c_char,
    );
    return 1i32;
}
/* pattern to match a single UTF-8 character */
static mut funcs: [luaL_Reg; 7] = unsafe {
    [
        luaL_Reg {
            name: b"offset\x00" as *const u8 as *const libc::c_char,
            func: Some(byteoffset),
        },
        luaL_Reg {
            name: b"codepoint\x00" as *const u8 as *const libc::c_char,
            func: Some(codepoint),
        },
        luaL_Reg {
            name: b"char\x00" as *const u8 as *const libc::c_char,
            func: Some(utfchar),
        },
        luaL_Reg {
            name: b"len\x00" as *const u8 as *const libc::c_char,
            func: Some(utflen),
        },
        luaL_Reg {
            name: b"codes\x00" as *const u8 as *const libc::c_char,
            func: Some(iter_codes),
        },
        luaL_Reg {
            name: b"charpattern\x00" as *const u8 as *const libc::c_char,
            func: None,
        },
        luaL_Reg {
            name: 0 as *const libc::c_char,
            func: None,
        },
    ]
};
unsafe extern "C" fn iter_codes(mut L: *mut lua_State) -> libc::c_int {
    luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_pushcclosure(L, Some(iter_aux), 0i32);
    lua_pushvalue(L, 1i32);
    lua_pushinteger(L, 0i32 as lua_Integer);
    return 3i32;
}
unsafe extern "C" fn iter_aux(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer =
        lua_tointegerx(L, 2i32, 0 as *mut libc::c_int) - 1i32 as libc::c_longlong;
    /* first iteration? */
    if n < 0i32 as libc::c_longlong {
        /* start from here */
        n = 0i32 as lua_Integer
    } else if n < len as lua_Integer {
        /* skip current byte */
        n += 1;
        while *s.offset(n as isize) as libc::c_int & 0xc0i32 == 0x80i32 {
            /* and its continuations */
            n += 1
        }
    }
    if n >= len as lua_Integer {
        /* no more codepoints */
        return 0i32;
    } else {
        let mut code: libc::c_int = 0;
        let mut next: *const libc::c_char = utf8_decode(s.offset(n as isize), &mut code);
        if next.is_null() || *next as libc::c_int & 0xc0i32 == 0x80i32 {
            return luaL_error(
                L,
                b"invalid UTF-8 code\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            lua_pushinteger(L, n + 1i32 as libc::c_longlong);
            lua_pushinteger(L, code as lua_Integer);
            return 2i32;
        }
    };
}
/*
** Decode one UTF-8 sequence, returning NULL if byte sequence is invalid.
*/
unsafe extern "C" fn utf8_decode(
    mut o: *const libc::c_char,
    mut val: *mut libc::c_int,
) -> *const libc::c_char {
    static mut limits: [libc::c_uint; 4] = unsafe {
        [
            0xffi32 as libc::c_uint,
            0x7fi32 as libc::c_uint,
            0x7ffi32 as libc::c_uint,
            0xffffi32 as libc::c_uint,
        ]
    };
    let mut s: *const libc::c_uchar = o as *const libc::c_uchar;
    let mut c: libc::c_uint = *s.offset(0isize) as libc::c_uint;
    /* final result */
    let mut res: libc::c_uint = 0i32 as libc::c_uint;
    /* ascii? */
    if c < 0x80i32 as libc::c_uint {
        res = c
    } else {
        /* to count number of continuation bytes */
        let mut count: libc::c_int = 0i32;
        while 0 != c & 0x40i32 as libc::c_uint {
            /* still have continuation bytes? */
            /* read next byte */
            count += 1;
            let mut cc: libc::c_int = *s.offset(count as isize) as libc::c_int;
            /* not a continuation byte? */
            if cc & 0xc0i32 != 0x80i32 {
                /* invalid byte sequence */
                return 0 as *const libc::c_char;
            } else {
                /* add lower 6 bits from cont. byte */
                res = res << 6i32 | (cc & 0x3fi32) as libc::c_uint;
                /* to test next bit */
                c <<= 1i32
            }
        }
        /* add first byte */
        res |= (c & 0x7fi32 as libc::c_uint) << count * 5i32;
        if count > 3i32 || res > 0x10ffffi32 as libc::c_uint || res <= limits[count as usize] {
            /* invalid byte sequence */
            return 0 as *const libc::c_char;
        } else {
            s = s.offset(count as isize)
        }
    }
    if !val.is_null() {
        *val = res as libc::c_int
    }
    /* +1 to include first byte */
    return (s as *const libc::c_char).offset(1isize);
}
/*
** utf8len(s [, i [, j]]) --> number of characters that start in the
** range [i,j], or nil + current position if 's' is not well formed in
** that interval
*/
unsafe extern "C" fn utflen(mut L: *mut lua_State) -> libc::c_int {
    let mut n: libc::c_int = 0i32;
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut posj: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, -1i32 as lua_Integer), len);
    (1i32 as libc::c_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    }
        || 0 != luaL_argerror(
            L,
            2i32,
            b"initial position out of string\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    posj -= 1;
    (posj < len as lua_Integer
        || 0 != luaL_argerror(
            L,
            3i32,
            b"final position out of string\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    while posi <= posj {
        let mut s1: *const libc::c_char =
            utf8_decode(s.offset(posi as isize), 0 as *mut libc::c_int);
        if s1.is_null() {
            /* conversion error? */
            /* return nil ... */
            lua_pushnil(L);
            /* ... and current position */
            lua_pushinteger(L, posi + 1i32 as libc::c_longlong);
            return 2i32;
        } else {
            posi = s1.wrapping_offset_from(s) as libc::c_long as lua_Integer;
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
    if pos >= 0i32 as libc::c_longlong {
        return pos;
    } else if (0u32 as libc::c_ulong).wrapping_sub(pos as size_t) > len {
        return 0i32 as lua_Integer;
    } else {
        return len as lua_Integer + pos + 1i32 as libc::c_longlong;
    };
}
/*
** utfchar(n1, n2, ...)  -> char(n1)..char(n2)...
*/
unsafe extern "C" fn utfchar(mut L: *mut lua_State) -> libc::c_int {
    /* number of arguments */
    let mut n: libc::c_int = lua_gettop(L);
    /* optimize common case of single char */
    if n == 1i32 {
        pushutfchar(L, 1i32);
    } else {
        let mut i: libc::c_int = 0;
        let mut b: luaL_Buffer = luaL_Buffer {
            b: 0 as *mut libc::c_char,
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
unsafe extern "C" fn pushutfchar(mut L: *mut lua_State, mut arg: libc::c_int) -> () {
    let mut code: lua_Integer = luaL_checkinteger(L, arg);
    (0i32 as libc::c_longlong <= code && code <= 0x10ffffi32 as libc::c_longlong
        || 0 != luaL_argerror(
            L,
            arg,
            b"value out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    lua_pushfstring(
        L,
        b"%U\x00" as *const u8 as *const libc::c_char,
        code as libc::c_long,
    );
}
/*
** codepoint(s, [i, [j]])  -> returns codepoints for all characters
** that start in the range [i,j]
*/
unsafe extern "C" fn codepoint(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut posi: lua_Integer = u_posrelat(luaL_optinteger(L, 2i32, 1i32 as lua_Integer), len);
    let mut pose: lua_Integer = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    let mut n: libc::c_int = 0;
    let mut se: *const libc::c_char = 0 as *const libc::c_char;
    (posi >= 1i32 as libc::c_longlong
        || 0 != luaL_argerror(
            L,
            2i32,
            b"out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    (pose <= len as lua_Integer
        || 0 != luaL_argerror(
            L,
            3i32,
            b"out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    if posi > pose {
        /* empty interval; return no values */
        return 0i32;
    } else if pose - posi >= 2147483647i32 as libc::c_longlong {
        return luaL_error(
            L,
            b"string slice too long\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        n = (pose - posi) as libc::c_int + 1i32;
        luaL_checkstack(
            L,
            n,
            b"string slice too long\x00" as *const u8 as *const libc::c_char,
        );
        n = 0i32;
        se = s.offset(pose as isize);
        s = s.offset((posi - 1i32 as libc::c_longlong) as isize);
        while s < se {
            let mut code: libc::c_int = 0;
            s = utf8_decode(s, &mut code);
            if s.is_null() {
                return luaL_error(
                    L,
                    b"invalid UTF-8 code\x00" as *const u8 as *const libc::c_char,
                );
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
unsafe extern "C" fn byteoffset(mut L: *mut lua_State) -> libc::c_int {
    let mut len: size_t = 0;
    let mut s: *const libc::c_char = luaL_checklstring(L, 1i32, &mut len);
    let mut n: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut posi: lua_Integer = (if n >= 0i32 as libc::c_longlong {
        1i32 as libc::c_ulong
    } else {
        len.wrapping_add(1i32 as libc::c_ulong)
    }) as lua_Integer;
    posi = u_posrelat(luaL_optinteger(L, 3i32, posi), len);
    (1i32 as libc::c_longlong <= posi && {
        posi -= 1;
        posi <= len as lua_Integer
    }
        || 0 != luaL_argerror(
            L,
            3i32,
            b"position out of range\x00" as *const u8 as *const libc::c_char,
        )) as libc::c_int;
    if n == 0i32 as libc::c_longlong {
        /* find beginning of current byte sequence */
        while posi > 0i32 as libc::c_longlong
            && *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32
        {
            posi -= 1
        }
    } else if *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32 {
        return luaL_error(
            L,
            b"initial position is a continuation byte\x00" as *const u8 as *const libc::c_char,
        );
    } else if n < 0i32 as libc::c_longlong {
        while n < 0i32 as libc::c_longlong && posi > 0i32 as libc::c_longlong {
            /* move back */
            loop {
                /* find beginning of previous character */
                posi -= 1;
                if !(posi > 0i32 as libc::c_longlong
                    && *s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32)
                {
                    break;
                }
            }
            n += 1
        }
    } else {
        /* do not move for 1st character */
        n -= 1;
        while n > 0i32 as libc::c_longlong && posi < len as lua_Integer {
            loop {
                /* find beginning of next character */
                posi += 1;
                if !(*s.offset(posi as isize) as libc::c_int & 0xc0i32 == 0x80i32) {
                    break;
                }
            }
            /* (cannot pass final '\0') */
            n -= 1
        }
    }
    /* did it find given character? */
    if n == 0i32 as libc::c_longlong {
        lua_pushinteger(L, posi + 1i32 as libc::c_longlong);
    } else {
        /* no such character */
        lua_pushnil(L);
    }
    return 1i32;
}
