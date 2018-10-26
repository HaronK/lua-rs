use super::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn luaopen_base(mut L: *mut lua_State) -> lua_int {
    /* open lib into global table */
    lua_rawgeti(L, -1000000i32 - 1000i32, 2i32 as lua_Integer);
    luaL_setfuncs(L, base_funcs.as_ptr(), 0i32);
    /* set global _G */
    lua_pushvalue(L, -1i32);
    lua_setfield(L, -2i32, s!(b"_G\x00"));
    /* set global _VERSION */
    lua_pushstring(L, s!(b"Lua 5.3\x00"));
    lua_setfield(L, -2i32, s!(b"_VERSION\x00"));
    return 1i32;
}
static mut base_funcs: [luaL_Reg; 25] = [
    lua_reg!(b"assert\x00", luaB_assert),
    lua_reg!(b"collectgarbage\x00", luaB_collectgarbage),
    lua_reg!(b"dofile\x00", luaB_dofile),
    lua_reg!(b"error\x00", luaB_error),
    lua_reg!(b"getmetatable\x00", luaB_getmetatable),
    lua_reg!(b"ipairs\x00", luaB_ipairs),
    lua_reg!(b"loadfile\x00", luaB_loadfile),
    lua_reg!(b"load\x00", luaB_load),
    lua_reg!(b"next\x00", luaB_next),
    lua_reg!(b"pairs\x00", luaB_pairs),
    lua_reg!(b"pcall\x00", luaB_pcall),
    lua_reg!(b"print\x00", luaB_print),
    lua_reg!(b"rawequal\x00", luaB_rawequal),
    lua_reg!(b"rawlen\x00", luaB_rawlen),
    lua_reg!(b"rawget\x00", luaB_rawget),
    lua_reg!(b"rawset\x00", luaB_rawset),
    lua_reg!(b"select\x00", luaB_select),
    lua_reg!(b"setmetatable\x00", luaB_setmetatable),
    lua_reg!(b"tonumber\x00", luaB_tonumber),
    lua_reg!(b"tostring\x00", luaB_tostring),
    lua_reg!(b"type\x00", luaB_type),
    lua_reg!(b"xpcall\x00", luaB_xpcall),
    lua_reg_none!(b"_G\x00"),
    lua_reg_none!(b"_VERSION\x00"),
    lua_reg_none!(0),
];
/*
** Do a protected call with error handling. After 'lua_rotate', the
** stack will have <f, err, true, f, [args...]>; so, the function passes
** 2 to 'finishpcall' to skip the 2 first values when returning results.
*/
unsafe extern "C" fn luaB_xpcall(mut L: *mut lua_State) -> lua_int {
    let mut status: lua_int = 0;
    let mut n: lua_int = lua_gettop(L);
    /* check error function */
    luaL_checktype(L, 2i32, 6i32);
    /* first result */
    lua_pushboolean(L, 1i32);
    /* function */
    lua_pushvalue(L, 1i32);
    /* move them below function's arguments */
    lua_rotate(L, 3i32, 2i32);
    status = lua_pcallk(
        L,
        n - 2i32,
        -1i32,
        2i32,
        2i32 as lua_KContext,
        Some(finishpcall),
    );
    return finishpcall(L, status, 2i32 as lua_KContext);
}
/*
** Continuation function for 'pcall' and 'xpcall'. Both functions
** already pushed a 'true' before doing the call, so in case of success
** 'finishpcall' only has to return everything in the stack minus
** 'extra' values (where 'extra' is exactly the number of items to be
** ignored).
*/
unsafe extern "C" fn finishpcall(
    mut L: *mut lua_State,
    mut status: lua_int,
    mut extra: lua_KContext,
) -> lua_int {
    if status != LUA_OK && status != LUA_YIELD {
        /* error? */
        /* first result (false) */
        lua_pushboolean(L, 0i32);
        /* error message */
        lua_pushvalue(L, -2i32);
        /* return false, msg */
        return 2i32;
    } else {
        return lua_gettop(L) - extra as lua_int;
    };
}
unsafe extern "C" fn luaB_type(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 1i32);
    (t != -1i32 || 0 != luaL_argerror(L, 1i32, s!(b"value expected\x00"))) as lua_int;
    lua_pushstring(L, lua_typename(L, t));
    return 1i32;
}
unsafe extern "C" fn luaB_tostring(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    luaL_tolstring(L, 1i32, 0 as *mut size_t);
    return 1i32;
}
unsafe extern "C" fn luaB_tonumber(mut L: *mut lua_State) -> lua_int {
    if lua_type(L, 2i32) <= 0i32 {
        /* standard conversion? */
        luaL_checkany(L, 1i32);
        if lua_type(L, 1i32) == 3i32 {
            /* already a number? */
            /* yes; return it */
            lua_settop(L, 1i32);
            return 1i32;
        } else {
            let mut l: size_t = 0;
            let mut s: *const lua_char = lua_tolstring(L, 1i32, &mut l);
            if !s.is_null() && lua_stringtonumber(L, s) == l.wrapping_add(1i32 as lua_ulong) {
                /* successful conversion to number */
                return 1i32;
            }
        }
    } else {
        /* else not a number */
        let mut l_0: size_t = 0;
        let mut s_0: *const lua_char = 0 as *const lua_char;
        /* to avoid warnings */
        let mut n: lua_Integer = 0i32 as lua_Integer;
        let mut base: lua_Integer = luaL_checkinteger(L, 2i32);
        /* no numbers as strings */
        luaL_checktype(L, 1i32, 4i32);
        s_0 = lua_tolstring(L, 1i32, &mut l_0);
        (2i32 as lua_longlong <= base && base <= 36i32 as lua_longlong
            || 0 != luaL_argerror(L, 2i32, s!(b"base out of range\x00"))) as lua_int;
        if b_str2int(s_0, base as lua_int, &mut n) == s_0.offset(l_0 as isize) {
            lua_pushinteger(L, n);
            return 1i32;
        }
    }
    /* else not a number */
    /* else not a number */
    /* not a number */
    lua_pushnil(L);
    return 1i32;
}
unsafe extern "C" fn b_str2int(
    mut s: *const lua_char,
    mut base: lua_int,
    mut pn: *mut lua_Integer,
) -> *const lua_char {
    let mut n: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut neg: lua_int = 0i32;
    /* skip initial spaces */
    s = s.offset(strspn(s, s!(b" \x0c\n\r\t\x0b\x00")) as isize);
    if *s as lua_int == '-' as i32 {
        s = s.offset(1isize);
        /* handle signal */
        neg = 1i32
    } else if *s as lua_int == '+' as i32 {
        s = s.offset(1isize)
    }
    /* no digit? */
    if 0 == isalnum(*s as lua_uchar as lua_int) {
        return 0 as *const lua_char;
    } else {
        loop {
            let mut digit: lua_int = if 0 != isdigit(*s as lua_uchar as lua_int) {
                *s as lua_int - ('0' as i32)
            } else {
                {
                    let mut __c: lua_int = 0;
                    let mut __res: lua_int = 0;
                    if ::std::mem::size_of::<lua_uchar>() as lua_ulong > 1i32 as lua_ulong {
                        if 0 != 0 {
                            __c = *s as lua_uchar as lua_int;
                            __res = if __c < -128i32 || __c > 255i32 {
                                __c
                            } else {
                                toupper(__c)
                            }
                        } else {
                            __res = toupper(*s as lua_uchar as lua_int)
                        }
                    } else {
                        __res = *s as lua_uchar as lua_int
                    }
                    //__res // TODO: What is this?
                }
                -('A' as i32) + 10i32
            };
            if digit >= base {
                /* invalid numeral */
                return 0 as *const lua_char;
            } else {
                n = n
                    .wrapping_mul(base as lua_ulonglong)
                    .wrapping_add(digit as lua_ulonglong);
                s = s.offset(1isize);
                if !(0 != isalnum(*s as lua_uchar as lua_int)) {
                    break;
                }
            }
        }
        /* skip trailing spaces */
        s = s.offset(strspn(s, s!(b" \x0c\n\r\t\x0b\x00")) as isize);
        *pn = (if 0 != neg {
            (0u32 as lua_ulonglong).wrapping_sub(n)
        } else {
            n
        }) as lua_Integer;
        return s;
    };
}
unsafe extern "C" fn luaB_setmetatable(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 2i32);
    luaL_checktype(L, 1i32, 5i32);
    (t == 0i32 || t == 5i32 || 0 != luaL_argerror(L, 2i32, s!(b"nil or table expected\x00")))
        as lua_int;
    if luaL_getmetafield(L, 1i32, s!(b"__metatable\x00")) != 0i32 {
        return luaL_error!(L, s!(b"cannot change a protected metatable\x00"));
    } else {
        lua_settop(L, 2i32);
        lua_setmetatable(L, 1i32);
        return 1i32;
    };
}
unsafe extern "C" fn luaB_select(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_int = lua_gettop(L);
    if lua_type(L, 1i32) == 4i32
        && *lua_tolstring(L, 1i32, 0 as *mut size_t) as lua_int == '#' as i32
    {
        lua_pushinteger(L, (n - 1i32) as lua_Integer);
        return 1i32;
    } else {
        let mut i: lua_Integer = luaL_checkinteger(L, 1i32);
        if i < 0i32 as lua_longlong {
            i = n as lua_longlong + i
        } else if i > n as lua_longlong {
            i = n as lua_Integer
        }
        (1i32 as lua_longlong <= i || 0 != luaL_argerror(L, 1i32, s!(b"index out of range\x00")))
            as lua_int;
        return n - i as lua_int;
    };
}
unsafe extern "C" fn luaB_rawset(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    luaL_checkany(L, 3i32);
    lua_settop(L, 3i32);
    lua_rawset(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawget(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_rawget(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn luaB_rawlen(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 1i32);
    (t == 5i32 || t == 4i32 || 0 != luaL_argerror(L, 1i32, s!(b"table or string expected\x00")))
        as lua_int;
    lua_pushinteger(L, lua_rawlen(L, 1i32) as lua_Integer);
    return 1i32;
}
unsafe extern "C" fn luaB_rawequal(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    luaL_checkany(L, 2i32);
    lua_pushboolean(L, lua_rawequal(L, 1i32, 2i32));
    return 1i32;
}
/*
** $Id: lbaselib.c,v 1.314.1.1 2017/04/19 17:39:34 roberto Exp $
** Basic library
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn luaB_print(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    let mut i: lua_int = 0;
    lua_getglobal(L, s!(b"tostring\x00"));
    i = 1i32;
    while i <= n {
        let mut s: *const lua_char = 0 as *const lua_char;
        let mut l: size_t = 0;
        /* function to be called */
        lua_pushvalue(L, -1i32);
        /* value to print */
        lua_pushvalue(L, i);
        lua_callk(L, 1i32, 1i32, 0i32 as lua_KContext, None);
        /* get result */
        s = lua_tolstring(L, -1i32, &mut l);
        if s.is_null() {
            return luaL_error!(L, s!(b"\'tostring\' must return a string to \'print\'\x00"));
        } else {
            if i > 1i32 {
                fwrite(
                    s!(b"\t\x00") as *const lua_void,
                    ::std::mem::size_of::<lua_char>() as lua_ulong,
                    1i32 as size_t,
                    stdout,
                );
            }
            fwrite(
                s as *const lua_void,
                ::std::mem::size_of::<lua_char>() as lua_ulong,
                l,
                stdout,
            );
            /* pop result */
            lua_settop(L, -1i32 - 1i32);
            i += 1
        }
    }
    fwrite(
        s!(b"\n\x00") as *const lua_void,
        ::std::mem::size_of::<lua_char>() as lua_ulong,
        1i32 as size_t,
        stdout,
    );
    fflush(stdout);
    return 0i32;
}
unsafe extern "C" fn luaB_pcall(mut L: *mut lua_State) -> lua_int {
    let mut status: lua_int = 0;
    luaL_checkany(L, 1i32);
    /* first result if no errors */
    lua_pushboolean(L, 1i32);
    /* put it in place */
    lua_rotate(L, 1i32, 1i32);
    status = lua_pcallk(
        L,
        lua_gettop(L) - 2i32,
        -1i32,
        0i32,
        0i32 as lua_KContext,
        Some(finishpcall),
    );
    return finishpcall(L, status, 0i32 as lua_KContext);
}
unsafe extern "C" fn luaB_pairs(mut L: *mut lua_State) -> lua_int {
    return pairsmeta(L, s!(b"__pairs\x00"), 0i32, Some(luaB_next));
}
unsafe extern "C" fn luaB_next(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 5i32);
    /* create a 2nd argument if there isn't one */
    lua_settop(L, 2i32);
    if 0 != lua_next(L, 1i32) {
        return 2i32;
    } else {
        lua_pushnil(L);
        return 1i32;
    };
}
unsafe extern "C" fn pairsmeta(
    mut L: *mut lua_State,
    mut method: *const lua_char,
    mut iszero: lua_int,
    mut iter: lua_CFunction,
) -> lua_int {
    luaL_checkany(L, 1i32);
    if luaL_getmetafield(L, 1i32, method) == 0i32 {
        /* no metamethod? */
        /* will return generator, */
        lua_pushcclosure(L, iter, 0i32);
        /* state, */
        lua_pushvalue(L, 1i32);
        if 0 != iszero {
            /* and initial value */
            lua_pushinteger(L, 0i32 as lua_Integer);
        } else {
            lua_pushnil(L);
        }
    } else {
        /* argument 'self' to metamethod */
        lua_pushvalue(L, 1i32);
        /* get 3 values from metamethod */
        lua_callk(L, 1i32, 3i32, 0i32 as lua_KContext, None);
    }
    return 3i32;
}
unsafe extern "C" fn luaB_load(mut L: *mut lua_State) -> lua_int {
    let mut chunkname: *const lua_char = 0 as *const lua_char;
    let mut chunkname_0: *const lua_char = 0 as *const lua_char;
    let mut status: lua_int = 0;
    let mut l: size_t = 0;
    let mut s: *const lua_char = lua_tolstring(L, 1i32, &mut l);
    let mut mode: *const lua_char = luaL_optlstring(L, 3i32, s!(b"bt\x00"), 0 as *mut size_t);
    /* 'env' index or 0 if no 'env' */
    let mut env: lua_int = if !(lua_type(L, 4i32) == -1i32) {
        4i32
    } else {
        0i32
    };
    if !s.is_null() {
        /* loading a string? */
        chunkname = luaL_optlstring(L, 2i32, s, 0 as *mut size_t);
        status = luaL_loadbufferx(L, s, l, chunkname, mode)
    } else {
        /* loading from a reader function */
        chunkname_0 = luaL_optlstring(L, 2i32, s!(b"=(load)\x00"), 0 as *mut size_t);
        luaL_checktype(L, 1i32, 6i32);
        /* create reserved slot */
        lua_settop(L, 5i32);
        status = lua_load(
            L,
            Some(generic_reader),
            0 as *mut lua_void,
            chunkname_0,
            mode,
        )
    }
    return load_aux(L, status, env);
}
unsafe extern "C" fn load_aux(
    mut L: *mut lua_State,
    mut status: lua_int,
    mut envidx: lua_int,
) -> lua_int {
    if status == LUA_OK {
        if envidx != 0i32 {
            /* 'env' parameter? */
            /* environment for loaded function */
            lua_pushvalue(L, envidx);
            /* set it as 1st upvalue */
            if lua_setupvalue(L, -2i32, 1i32).is_null() {
                /* remove 'env' if not used by previous call */
                lua_settop(L, -1i32 - 1i32);
            }
        }
        return 1i32;
    } else {
        /* error (message is on top of the stack) */
        lua_pushnil(L);
        /* put before error message */
        lua_rotate(L, -2i32, 1i32);
        /* return nil plus error message */
        return 2i32;
    };
}
/*
** {======================================================
** Generic Read function
** =======================================================
*/
/*
** reserved slot, above all arguments, to hold a copy of the returned
** string to avoid it being collected while parsed. 'load' has four
** optional arguments (chunk, source name, mode, and environment).
*/
/*
** Reader for generic 'load' function: 'lua_load' uses the
** stack for internal stuff, so the reader cannot change the
** stack top. Instead, it keeps its resulting string in a
** reserved slot inside the stack.
*/
unsafe extern "C" fn generic_reader(
    mut L: *mut lua_State,
    mut _ud: *mut lua_void,
    mut size: *mut size_t,
) -> *const lua_char {
    /* not used */
    luaL_checkstack(L, 2i32, s!(b"too many nested functions\x00"));
    /* get function */
    lua_pushvalue(L, 1i32);
    /* call it */
    lua_callk(L, 0i32, 1i32, 0i32 as lua_KContext, None);
    if lua_type(L, -1i32) == 0i32 {
        /* pop result */
        lua_settop(L, -1i32 - 1i32);
        *size = 0i32 as size_t;
        return 0 as *const lua_char;
    } else {
        if 0 == lua_isstring(L, -1i32) {
            luaL_error!(L, s!(b"reader function must return a string\x00"));
        }
        /* save string in reserved slot */
        lua_copy(L, -1i32, 5i32);
        lua_settop(L, -1i32 - 1i32);
        return lua_tolstring(L, 5i32, size);
    };
}
unsafe extern "C" fn luaB_loadfile(mut L: *mut lua_State) -> lua_int {
    let mut fname: *const lua_char =
        luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    let mut mode: *const lua_char =
        luaL_optlstring(L, 2i32, 0 as *const lua_char, 0 as *mut size_t);
    /* 'env' index or 0 if no 'env' */
    let mut env: lua_int = if !(lua_type(L, 3i32) == -1i32) {
        3i32
    } else {
        0i32
    };
    let mut status: lua_int = luaL_loadfilex(L, fname, mode);
    return load_aux(L, status, env);
}
/*
** 'ipairs' function. Returns 'ipairsaux', given "table", 0.
** (The given "table" may not be a table.)
*/
unsafe extern "C" fn luaB_ipairs(mut L: *mut lua_State) -> lua_int {
    return pairsmeta(L, s!(b"__ipairs\x00"), 1i32, Some(ipairsaux));
}
/*
** Traversal function for 'ipairs'
*/
unsafe extern "C" fn ipairsaux(mut L: *mut lua_State) -> lua_int {
    let mut i: lua_Integer = luaL_checkinteger(L, 2i32) + 1i32 as lua_longlong;
    lua_pushinteger(L, i);
    return if lua_geti(L, 1i32, i) == 0i32 {
        1i32
    } else {
        2i32
    };
}
unsafe extern "C" fn luaB_getmetatable(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        lua_pushnil(L);
        /* no metatable */
        return 1i32;
    } else {
        luaL_getmetafield(L, 1i32, s!(b"__metatable\x00"));
        /* returns either __metatable field (if present) or metatable */
        return 1i32;
    };
}
unsafe extern "C" fn luaB_error(mut L: *mut lua_State) -> lua_int {
    let mut level: lua_int = luaL_optinteger(L, 2i32, 1i32 as lua_Integer) as lua_int;
    lua_settop(L, 1i32);
    if lua_type(L, 1i32) == 4i32 && level > 0i32 {
        /* add extra information */
        luaL_where(L, level);
        lua_pushvalue(L, 1i32);
        lua_concat(L, 2i32);
    }
    return lua_error(L);
}
unsafe extern "C" fn luaB_dofile(mut L: *mut lua_State) -> lua_int {
    let mut fname: *const lua_char =
        luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    lua_settop(L, 1i32);
    if luaL_loadfilex(L, fname, 0 as *const lua_char) != LUA_OK {
        return lua_error(L);
    } else {
        lua_callk(L, 0i32, LUA_MULTRET, 0i32 as lua_KContext, Some(dofilecont));
        return dofilecont(L, 0i32, 0i32 as lua_KContext);
    };
}
/* }====================================================== */
unsafe extern "C" fn dofilecont(
    mut L: *mut lua_State,
    mut _d1: lua_int,
    mut _d2: lua_KContext,
) -> lua_int {
    /* only to match 'lua_Kfunction' prototype */
    return lua_gettop(L) - 1i32;
}
unsafe extern "C" fn luaB_collectgarbage(mut L: *mut lua_State) -> lua_int {
    static mut opts: [*const lua_char; 9] = [
        s!(b"stop\x00"),
        s!(b"restart\x00"),
        s!(b"collect\x00"),
        s!(b"count\x00"),
        s!(b"step\x00"),
        s!(b"setpause\x00"),
        s!(b"setstepmul\x00"),
        s!(b"isrunning\x00"),
        0 as *const lua_char,
    ];
    static mut optsnum: [lua_int; 8] = [0i32, 1i32, 2i32, 3i32, 5i32, 6i32, 7i32, 9i32];
    let mut o: lua_int =
        optsnum[luaL_checkoption(L, 1i32, s!(b"collect\x00"), opts.as_ptr()) as usize];
    let mut ex: lua_int = luaL_optinteger(L, 2i32, 0i32 as lua_Integer) as lua_int;
    let mut res: lua_int = lua_gc(L, o, ex);
    match o {
        3 => {
            let mut b: lua_int = lua_gc(L, 4i32, 0i32);
            lua_pushnumber(
                L,
                res as lua_Number + b as lua_Number / 1024i32 as lua_double,
            );
            return 1i32;
        }
        5 | 9 => {
            lua_pushboolean(L, res);
            return 1i32;
        }
        _ => {
            lua_pushinteger(L, res as lua_Integer);
            return 1i32;
        }
    };
}
unsafe extern "C" fn luaB_assert(mut L: *mut lua_State) -> lua_int {
    /* condition is true? */
    if 0 != lua_toboolean(L, 1i32) {
        /* return all arguments */
        return lua_gettop(L);
    } else {
        /* error */
        /* there must be a condition */
        luaL_checkany(L, 1i32);
        /* remove it */
        lua_rotate(L, 1i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* default message */
        lua_pushstring(L, s!(b"assertion failed!\x00"));
        /* leave only message (default if no other one) */
        lua_settop(L, 1i32);
        /* call 'error' */
        return luaB_error(L);
    };
}
