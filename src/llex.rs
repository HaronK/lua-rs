use super::prelude::*;

/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER RESERVED"
*/
pub type RESERVED = lua_uint;
pub const TK_STRING: RESERVED = 293;
pub const TK_NAME: RESERVED = 292;
pub const TK_INT: RESERVED = 291;
pub const TK_FLT: RESERVED = 290;
pub const TK_EOS: RESERVED = 289;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_SHR: RESERVED = 287;
pub const TK_SHL: RESERVED = 286;
pub const TK_NE: RESERVED = 285;
pub const TK_LE: RESERVED = 284;
pub const TK_GE: RESERVED = 283;
pub const TK_EQ: RESERVED = 282;
pub const TK_DOTS: RESERVED = 281;
pub const TK_CONCAT: RESERVED = 280;
/* other terminal symbols */
pub const TK_IDIV: RESERVED = 279;
pub const TK_WHILE: RESERVED = 278;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_TRUE: RESERVED = 276;
pub const TK_THEN: RESERVED = 275;
pub const TK_RETURN: RESERVED = 274;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_OR: RESERVED = 272;
pub const TK_NOT: RESERVED = 271;
pub const TK_NIL: RESERVED = 270;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_IN: RESERVED = 268;
pub const TK_IF: RESERVED = 267;
pub const TK_GOTO: RESERVED = 266;
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_FOR: RESERVED = 264;
pub const TK_FALSE: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
/* terminal symbols denoted by reserved words */
pub const TK_AND: RESERVED = 257;
/* number of reserved words */

#[no_mangle]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State) -> () {
    let mut i: lua_int = 0;
    /* create env name */
    let mut e: *mut TString = luaS_newlstr(
        L,
        s!(b"_ENV\x00"),
        (::std::mem::size_of::<[lua_char; 5]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    /* never collect this name */
    luaC_fix(L, &mut (*(e as *mut GCUnion)).gc);
    i = 0i32;
    while i < TK_WHILE as lua_int - 257i32 + 1i32 {
        let mut ts: *mut TString = luaS_new(L, luaX_tokens[i as usize]);
        /* reserved words are never collected */
        luaC_fix(L, &mut (*(ts as *mut GCUnion)).gc);
        /* reserved word */
        (*ts).extra = (i + 1i32) as lu_byte;
        i += 1
    }
}

/* ORDER RESERVED */
static mut luaX_tokens: [*const lua_char; 37] = [
    s!(b"and\x00"),
    s!(b"break\x00"),
    s!(b"do\x00"),
    s!(b"else\x00"),
    s!(b"elseif\x00"),
    s!(b"end\x00"),
    s!(b"false\x00"),
    s!(b"for\x00"),
    s!(b"function\x00"),
    s!(b"goto\x00"),
    s!(b"if\x00"),
    s!(b"in\x00"),
    s!(b"local\x00"),
    s!(b"nil\x00"),
    s!(b"not\x00"),
    s!(b"or\x00"),
    s!(b"repeat\x00"),
    s!(b"return\x00"),
    s!(b"then\x00"),
    s!(b"true\x00"),
    s!(b"until\x00"),
    s!(b"while\x00"),
    s!(b"//\x00"),
    s!(b"..\x00"),
    s!(b"...\x00"),
    s!(b"==\x00"),
    s!(b">=\x00"),
    s!(b"<=\x00"),
    s!(b"~=\x00"),
    s!(b"<<\x00"),
    s!(b">>\x00"),
    s!(b"::\x00"),
    s!(b"<eof>\x00"),
    s!(b"<number>\x00"),
    s!(b"<integer>\x00"),
    s!(b"<name>\x00"),
    s!(b"<string>\x00"),
];
#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(
    mut L: *mut lua_State,
    mut ls: *mut LexState,
    mut z: *mut ZIO,
    mut source: *mut TString,
    mut firstchar: lua_int,
) -> () {
    (*ls).t.token = 0i32;
    (*ls).L = L;
    (*ls).current = firstchar;
    /* no look-ahead token */
    (*ls).lookahead.token = TK_EOS as lua_int;
    (*ls).z = z;
    (*ls).fs = 0 as *mut FuncState;
    (*ls).linenumber = 1i32;
    (*ls).lastline = 1i32;
    (*ls).source = source;
    /* get env name */
    (*ls).envn = luaS_newlstr(
        L,
        s!(b"_ENV\x00"),
        (::std::mem::size_of::<[lua_char; 5]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    /* initialize buffer */
    (*(*ls).buff).buffer = luaM_realloc_(
        (*ls).L,
        (*(*ls).buff).buffer as *mut lua_void,
        (*(*ls).buff)
            .buffsize
            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        (32i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    ) as *mut lua_char;
    (*(*ls).buff).buffsize = 32i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_newstring(
    mut ls: *mut LexState,
    mut str: *const lua_char,
    mut l: size_t,
) -> *mut TString {
    let mut L: *mut lua_State = (*ls).L;
    /* entry for 'str' */
    let mut o: *mut TValue = 0 as *mut TValue;
    /* create new string */
    let mut ts: *mut TString = luaS_newlstr(L, str, l);
    /* temporarily anchor it in stack */
    let fresh0 = (*L).top;
    (*L).top = (*L).top.offset(1);
    let mut io: *mut TValue = fresh0;
    let mut x_: *mut TString = ts;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
    o = luaH_set(L, (*ls).h, (*L).top.offset(-1isize) as *const TValue);
    if (*o).tt_ == 0i32 {
        /* not in use yet? */
        /* boolean value does not need GC barrier;
        table has no metatable, so it does not need to invalidate cache */
        /* t[string] = true */
        let mut io_0: *mut TValue = o;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        if (*(*L).l_G).GCdebt > 0i32 as lua_long {
            luaC_step(L);
        }
    } else {
        ts = &mut (*((*(&mut (*((o as *mut lua_char).offset(-0isize) as *mut Node))
            .i_key
            .tvk as *mut TValue as *const TValue))
            .value_
            .gc as *mut GCUnion))
            .ts
    }
    /* remove string from stack */
    (*L).top = (*L).top.offset(-1isize);
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState) -> () {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as lua_int {
        /* is there a look-ahead token? */
        /* use this one */
        (*ls).t = (*ls).lookahead;
        /* and discharge it */
        (*ls).lookahead.token = TK_EOS as lua_int
    } else {
        (*ls).t.token = llex(ls, &mut (*ls).t.seminfo)
    };
}
unsafe extern "C" fn llex(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> lua_int {
    (*(*ls).buff).n = 0i32 as size_t;
    loop {
        match (*ls).current {
            10 | 13 => {
                /* line breaks */
                inclinenumber(ls);
            }
            32 | 12 | 9 | 11 => {
                /* spaces */
                let fresh1 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh1 > 0i32 as lua_ulong {
                    let fresh2 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh2 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                }
            }
            45 => {
                /* '-' or '--' (comment) */
                let fresh3 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh3 > 0i32 as lua_ulong {
                    let fresh4 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh4 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current != '-' as i32 {
                    return '-' as i32;
                } else {
                    /* else is a comment */
                    let fresh5 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh5 > 0i32 as lua_ulong {
                        let fresh6 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh6 as lua_uchar as lua_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    if (*ls).current == '[' as i32 {
                        /* long comment? */
                        let mut sep: lua_int = skip_sep(ls);
                        /* 'skip_sep' may dirty the buffer */
                        (*(*ls).buff).n = 0i32 as size_t;
                        if sep >= 0i32 {
                            /* skip long comment */
                            read_long_string(ls, 0 as *mut SemInfo, sep);
                            /* previous call may dirty the buff. */
                            (*(*ls).buff).n = 0i32 as size_t;
                            continue;
                        }
                    }
                    /* else short comment */
                    while !((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32)
                        && (*ls).current != -1i32
                    {
                        /* skip until end of line (or end of file) */
                        let fresh7 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh7 > 0i32 as lua_ulong {
                            let fresh8 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh8 as lua_uchar as lua_int
                        } else {
                            luaZ_fill((*ls).z)
                        }
                    }
                }
            }
            91 => {
                /* long string or simply '[' */
                let mut sep_0: lua_int = skip_sep(ls);
                if sep_0 >= 0i32 {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as lua_int;
                } else if sep_0 != -1i32 {
                    lexerror(
                        ls,
                        s!(b"invalid long string delimiter\x00"),
                        TK_STRING as lua_int,
                    );
                } else {
                    return '[' as i32;
                }
            }
            61 => {
                let fresh9 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh9 > 0i32 as lua_ulong {
                    let fresh10 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh10 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_EQ as lua_int;
                } else {
                    return '=' as i32;
                }
            }
            60 => {
                let fresh11 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh11 > 0i32 as lua_ulong {
                    let fresh12 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh12 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_LE as lua_int;
                } else if 0 != check_next1(ls, '<' as i32) {
                    return TK_SHL as lua_int;
                } else {
                    return '<' as i32;
                }
            }
            62 => {
                let fresh13 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh13 > 0i32 as lua_ulong {
                    let fresh14 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh14 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_GE as lua_int;
                } else if 0 != check_next1(ls, '>' as i32) {
                    return TK_SHR as lua_int;
                } else {
                    return '>' as i32;
                }
            }
            47 => {
                let fresh15 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh15 > 0i32 as lua_ulong {
                    let fresh16 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh16 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '/' as i32) {
                    return TK_IDIV as lua_int;
                } else {
                    return '/' as i32;
                }
            }
            126 => {
                let fresh17 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh17 > 0i32 as lua_ulong {
                    let fresh18 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh18 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_NE as lua_int;
                } else {
                    return '~' as i32;
                }
            }
            58 => {
                let fresh19 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh19 > 0i32 as lua_ulong {
                    let fresh20 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh20 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, ':' as i32) {
                    return TK_DBCOLON as lua_int;
                } else {
                    return ':' as i32;
                }
            }
            34 | 39 => {
                /* short literal strings */
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as lua_int;
            }
            46 => {
                /* '.', '..', '...', or number */
                save(ls, (*ls).current);
                let fresh21 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh21 > 0i32 as lua_ulong {
                    let fresh22 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh22 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '.' as i32) {
                    if 0 != check_next1(ls, '.' as i32) {
                        /* '...' */
                        return TK_DOTS as lua_int;
                    } else {
                        return TK_CONCAT as lua_int;
                    }
                } else if 0
                    == luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 1i32
                {
                    return '.' as i32;
                } else {
                    return read_numeral(ls, seminfo);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return read_numeral(ls, seminfo),
            -1 => return TK_EOS as lua_int,
            _ => {
                if 0 != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 0i32 {
                    /* identifier or reserved word? */
                    let mut ts: *mut TString = 0 as *mut TString;
                    loop {
                        save(ls, (*ls).current);
                        let fresh23 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh23 > 0i32 as lua_ulong {
                            let fresh24 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh24 as lua_uchar as lua_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        if !(0
                            != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int
                                & (1i32 << 0i32 | 1i32 << 1i32))
                        {
                            break;
                        }
                    }
                    ts = luaX_newstring(ls, (*(*ls).buff).buffer, (*(*ls).buff).n);
                    (*seminfo).ts = ts;
                    /* reserved word? */
                    if (*ts).tt as lua_int == 4i32 | 0i32 << 4i32 && (*ts).extra as lua_int > 0i32 {
                        return (*ts).extra as lua_int - 1i32 + 257i32;
                    } else {
                        return TK_NAME as lua_int;
                    }
                } else {
                    /* single-char tokens (+ - / ...) */
                    let mut c: lua_int = (*ls).current;
                    let fresh25 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh25 > 0i32 as lua_ulong {
                        let fresh26 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh26 as lua_uchar as lua_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    return c;
                }
            }
        }
    }
}
unsafe extern "C" fn save(mut ls: *mut LexState, mut c: lua_int) -> () {
    let mut b: *mut Mbuffer = (*ls).buff;
    if (*b).n.wrapping_add(1i32 as lua_ulong) > (*b).buffsize {
        let mut newsize: size_t = 0;
        if (*b).buffsize
            >= if (::std::mem::size_of::<size_t>() as lua_ulong)
                < ::std::mem::size_of::<lua_Integer>() as lua_ulong
            {
                !(0i32 as size_t)
            } else {
                9223372036854775807i64 as size_t
            }
            .wrapping_div(2i32 as lua_ulong)
        {
            lexerror(ls, s!(b"lexical element too long\x00"), 0i32);
        } else {
            newsize = (*b).buffsize.wrapping_mul(2i32 as lua_ulong);
            (*b).buffer = luaM_realloc_(
                (*ls).L,
                (*b).buffer as *mut lua_void,
                (*b).buffsize
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
                newsize.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            ) as *mut lua_char;
            (*b).buffsize = newsize
        }
    }
    let fresh27 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).buffer.offset(fresh27 as isize) = c as lua_char;
}
unsafe extern "C" fn lexerror(
    mut ls: *mut LexState,
    mut msg: *const lua_char,
    mut token: lua_int,
) -> ! {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if 0 != token {
        luaO_pushfstring!((*ls).L, s!(b"%s near %s\x00"), msg, txtToken(ls, token),);
    }
    luaD_throw((*ls).L, 3i32);
}
unsafe extern "C" fn txtToken(mut ls: *mut LexState, mut token: lua_int) -> *const lua_char {
    match token {
        292 | 293 | 290 | 291 => {
            save(ls, '\u{0}' as i32);
            return luaO_pushfstring!((*ls).L, s!(b"\'%s\'\x00"), (*(*ls).buff).buffer,);
        }
        _ => return luaX_token2str(ls, token),
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(
    mut ls: *mut LexState,
    mut token: lua_int,
) -> *const lua_char {
    if token < 257i32 {
        /* single-byte symbols? */
        return luaO_pushfstring!((*ls).L, s!(b"\'%c\'\x00"), token,);
    } else {
        let mut s: *const lua_char = luaX_tokens[(token - 257i32) as usize];
        /* fixed format (symbols and reserved words)? */
        if token < TK_EOS as lua_int {
            return luaO_pushfstring!((*ls).L, s!(b"\'%s\'\x00"), s,);
        } else {
            return s;
        }
    };
}
/* LUA_NUMBER */
/*
** this function is quite liberal in what it accepts, as 'luaO_str2num'
** will reject ill-formed numerals.
*/
unsafe extern "C" fn read_numeral(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> lua_int {
    let mut obj: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut expo: *const lua_char = s!(b"Ee\x00");
    let mut first: lua_int = (*ls).current;
    save(ls, (*ls).current);
    let fresh28 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh28 > 0i32 as lua_ulong {
        let fresh29 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh29 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* hexadecimal? */
    if first == '0' as i32 && 0 != check_next2(ls, s!(b"xX\x00")) {
        expo = s!(b"Pp\x00")
    }
    loop {
        /* exponent part? */
        if 0 != check_next2(ls, expo) {
            /* optional exponent sign */
            check_next2(ls, s!(b"-+\x00"));
        }
        if 0 != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 4i32 {
            save(ls, (*ls).current);
            let fresh30 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh30 > 0i32 as lua_ulong {
                let fresh31 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh31 as lua_uchar as lua_int
            } else {
                luaZ_fill((*ls).z)
            }
        } else {
            if !((*ls).current == '.' as i32) {
                break;
            }
            save(ls, (*ls).current);
            let fresh32 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh32 > 0i32 as lua_ulong {
                let fresh33 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh33 as lua_uchar as lua_int
            } else {
                luaZ_fill((*ls).z)
            }
        }
    }
    save(ls, '\u{0}' as i32);
    /* format error? */
    if luaO_str2num((*(*ls).buff).buffer, &mut obj) == 0i32 as lua_ulong {
        lexerror(ls, s!(b"malformed number\x00"), TK_FLT as lua_int);
    } else if obj.tt_ == 3i32 | 1i32 << 4i32 {
        (*seminfo).i = obj.value_.i;
        return TK_INT as lua_int;
    } else {
        (*seminfo).r = obj.value_.n;
        return TK_FLT as lua_int;
    };
}
/*
** Check whether current char is in set 'set' (with two chars) and
** saves it
*/
unsafe extern "C" fn check_next2(mut ls: *mut LexState, mut set: *const lua_char) -> lua_int {
    if (*ls).current == *set.offset(0isize) as lua_int
        || (*ls).current == *set.offset(1isize) as lua_int
    {
        save(ls, (*ls).current);
        let fresh34 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh34 > 0i32 as lua_ulong {
            let fresh35 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh35 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
/*
** =======================================================
** LEXICAL ANALYZER
** =======================================================
*/
unsafe extern "C" fn check_next1(mut ls: *mut LexState, mut c: lua_int) -> lua_int {
    if (*ls).current == c {
        let fresh36 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh36 > 0i32 as lua_ulong {
            let fresh37 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh37 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn read_string(
    mut ls: *mut LexState,
    mut del: lua_int,
    mut seminfo: *mut SemInfo,
) -> () {
    let mut current_block: u64;
    /* keep delimiter (for error messages) */
    save(ls, (*ls).current);
    let fresh38 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh38 > 0i32 as lua_ulong {
        let fresh39 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh39 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    's_4: while (*ls).current != del {
        match (*ls).current {
            -1 => {
                lexerror(ls, s!(b"unfinished string\x00"), TK_EOS as lua_int);
            }
            10 | 13 => {
                lexerror(ls, s!(b"unfinished string\x00"), TK_STRING as lua_int);
            }
            92 => {
                /* escape sequences */
                /* final character to be saved */
                let mut c: lua_int = 0;
                /* keep '\\' for error messages */
                save(ls, (*ls).current);
                let fresh40 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh40 > 0i32 as lua_ulong {
                    let fresh41 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh41 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                match (*ls).current {
                    97 => {
                        c = '\u{7}' as i32;
                        current_block = 12992004776645797882;
                    }
                    98 => {
                        c = '\u{8}' as i32;
                        current_block = 12992004776645797882;
                    }
                    102 => {
                        c = '\u{c}' as i32;
                        current_block = 12992004776645797882;
                    }
                    110 => {
                        c = '\n' as i32;
                        current_block = 12992004776645797882;
                    }
                    114 => {
                        c = '\r' as i32;
                        current_block = 12992004776645797882;
                    }
                    116 => {
                        c = '\t' as i32;
                        current_block = 12992004776645797882;
                    }
                    118 => {
                        c = '\u{b}' as i32;
                        current_block = 12992004776645797882;
                    }
                    120 => {
                        c = readhexaesc(ls);
                        current_block = 12992004776645797882;
                    }
                    117 => {
                        utf8esc(ls);
                        continue;
                    }
                    10 | 13 => {
                        inclinenumber(ls);
                        c = '\n' as i32;
                        current_block = 8303449541411860306;
                    }
                    92 | 34 | 39 => {
                        c = (*ls).current;
                        current_block = 12992004776645797882;
                    }
                    -1 => {
                        /* will raise an error next loop */
                        continue;
                    }
                    122 => {
                        /* zap following span of spaces */
                        /* remove '\\' */
                        (*(*ls).buff).n = ((*(*ls).buff).n as lua_ulong)
                            .wrapping_sub(1i32 as lua_ulong)
                            as size_t as size_t;
                        /* skip the 'z' */
                        let fresh42 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh42 > 0i32 as lua_ulong {
                            let fresh43 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh43 as lua_uchar as lua_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        loop {
                            if !(0
                                != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int
                                    & 1i32 << 3i32)
                            {
                                continue 's_4;
                            }
                            if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
                                inclinenumber(ls);
                            } else {
                                let fresh44 = (*(*ls).z).n;
                                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                                (*ls).current = if fresh44 > 0i32 as lua_ulong {
                                    let fresh45 = (*(*ls).z).p;
                                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                    *fresh45 as lua_uchar as lua_int
                                } else {
                                    luaZ_fill((*ls).z)
                                }
                            }
                        }
                    }
                    _ => {
                        esccheck(
                            ls,
                            luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 1i32,
                            s!(b"invalid escape sequence\x00"),
                        );
                        /* digital escape '\ddd' */
                        c = readdecesc(ls);
                        current_block = 8303449541411860306;
                    }
                }
                match current_block {
                    12992004776645797882 => {
                        let fresh46 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh46 > 0i32 as lua_ulong {
                            let fresh47 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh47 as lua_uchar as lua_int
                        } else {
                            luaZ_fill((*ls).z)
                        }
                    }
                    _ => {}
                }
                /* go through */
                /* remove '\\' */
                (*(*ls).buff).n = ((*(*ls).buff).n as lua_ulong).wrapping_sub(1i32 as lua_ulong)
                    as size_t as size_t;
                save(ls, c);
            }
            _ => {
                /* go through */
                save(ls, (*ls).current);
                let fresh48 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh48 > 0i32 as lua_ulong {
                    let fresh49 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh49 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                }
            }
        }
    }
    /* skip delimiter */
    save(ls, (*ls).current);
    let fresh50 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh50 > 0i32 as lua_ulong {
        let fresh51 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh51 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    (*seminfo).ts = luaX_newstring(
        ls,
        (*(*ls).buff).buffer.offset(1isize),
        (*(*ls).buff).n.wrapping_sub(2i32 as lua_ulong),
    );
}
unsafe extern "C" fn readdecesc(mut ls: *mut LexState) -> lua_int {
    let mut i: lua_int = 0;
    /* result accumulator */
    let mut r: lua_int = 0i32;
    i = 0i32;
    while i < 3i32 && 0 != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 1i32 {
        /* read up to 3 digits */
        r = 10i32 * r + (*ls).current - '0' as i32;
        save(ls, (*ls).current);
        let fresh52 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh52 > 0i32 as lua_ulong {
            let fresh53 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh53 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        };
        i += 1
    }
    esccheck(
        ls,
        (r <= 127i32 * 2i32 + 1i32) as lua_int,
        s!(b"decimal escape too large\x00"),
    );
    /* remove read digits from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as lua_ulong).wrapping_sub(i as lua_ulong) as size_t as size_t;
    return r;
}
unsafe extern "C" fn esccheck(
    mut ls: *mut LexState,
    mut c: lua_int,
    mut msg: *const lua_char,
) -> () {
    if 0 == c {
        if (*ls).current != -1i32 {
            /* add current to buffer for error message */
            save(ls, (*ls).current);
            let fresh54 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh54 > 0i32 as lua_ulong {
                let fresh55 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh55 as lua_uchar as lua_int
            } else {
                luaZ_fill((*ls).z)
            }
        }
        lexerror(ls, msg, TK_STRING as lua_int);
    } else {
        return;
    };
}
/*
** increment line number and skips newline sequence (any of
** \n, \r, \n\r, or \r\n)
*/
unsafe extern "C" fn inclinenumber(mut ls: *mut LexState) -> () {
    let mut old: lua_int = (*ls).current;
    /* skip '\n' or '\r' */
    let fresh56 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh56 > 0i32 as lua_ulong {
        let fresh57 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh57 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    if ((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32) && (*ls).current != old {
        /* skip '\n\r' or '\r\n' */
        let fresh58 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh58 > 0i32 as lua_ulong {
            let fresh59 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh59 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        }
    }
    (*ls).linenumber += 1;
    if (*ls).linenumber >= 2147483647i32 {
        lexerror(ls, s!(b"chunk has too many lines\x00"), 0i32);
    } else {
        return;
    };
}
unsafe extern "C" fn utf8esc(mut ls: *mut LexState) -> () {
    let mut buff: [lua_char; 8] = [0; 8];
    let mut n: lua_int = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    /* add 'buff' to string */
    while n > 0i32 {
        save(ls, buff[(8i32 - n) as usize] as lua_int);
        n -= 1
    }
}
unsafe extern "C" fn readutf8esc(mut ls: *mut LexState) -> lua_ulong {
    let mut r: lua_ulong = 0;
    /* chars to be removed: '\', 'u', '{', and first digit */
    let mut i: lua_int = 4i32;
    /* skip 'u' */
    save(ls, (*ls).current);
    let fresh60 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh60 > 0i32 as lua_ulong {
        let fresh61 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh61 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        ((*ls).current == '{' as i32) as lua_int,
        s!(b"missing \'{\'\x00"),
    );
    /* must have at least one digit */
    r = gethexa(ls) as lua_ulong;
    loop {
        save(ls, (*ls).current);
        let fresh62 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh62 > 0i32 as lua_ulong {
            let fresh63 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh63 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        };
        if !(0 != luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 4i32) {
            break;
        }
        i += 1;
        r = (r << 4i32).wrapping_add(luaO_hexavalue((*ls).current) as lua_ulong);
        esccheck(
            ls,
            (r <= 0x10ffffi32 as lua_ulong) as lua_int,
            s!(b"UTF-8 value too large\x00"),
        );
    }
    esccheck(
        ls,
        ((*ls).current == '}' as i32) as lua_int,
        s!(b"missing \'}\'\x00"),
    );
    /* skip '}' */
    let fresh64 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh64 > 0i32 as lua_ulong {
        let fresh65 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh65 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* remove saved chars from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as lua_ulong).wrapping_sub(i as lua_ulong) as size_t as size_t;
    return r;
}
unsafe extern "C" fn gethexa(mut ls: *mut LexState) -> lua_int {
    save(ls, (*ls).current);
    let fresh66 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh66 > 0i32 as lua_ulong {
        let fresh67 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh67 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        luai_ctype_[((*ls).current + 1i32) as usize] as lua_int & 1i32 << 4i32,
        s!(b"hexadecimal digit expected\x00"),
    );
    return luaO_hexavalue((*ls).current);
}
unsafe extern "C" fn readhexaesc(mut ls: *mut LexState) -> lua_int {
    let mut r: lua_int = gethexa(ls);
    r = (r << 4i32) + gethexa(ls);
    /* remove saved chars from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as lua_ulong).wrapping_sub(2i32 as lua_ulong) as size_t as size_t;
    return r;
}
/*
** skip a sequence '[=*[' or ']=*]'; if sequence is well formed, return
** its number of '='s; otherwise, return a negative number (-1 iff there
** are no '='s after initial bracket)
*/
unsafe extern "C" fn skip_sep(mut ls: *mut LexState) -> lua_int {
    let mut count: lua_int = 0i32;
    let mut s: lua_int = (*ls).current;
    save(ls, (*ls).current);
    let fresh68 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh68 > 0i32 as lua_ulong {
        let fresh69 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh69 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    while (*ls).current == '=' as i32 {
        save(ls, (*ls).current);
        let fresh70 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh70 > 0i32 as lua_ulong {
            let fresh71 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh71 as lua_uchar as lua_int
        } else {
            luaZ_fill((*ls).z)
        };
        count += 1
    }
    return if (*ls).current == s {
        count
    } else {
        -count - 1i32
    };
}
unsafe extern "C" fn read_long_string(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
    mut sep: lua_int,
) -> () {
    /* initial line (for error message) */
    let mut line: lua_int = (*ls).linenumber;
    /* skip 2nd '[' */
    save(ls, (*ls).current);
    let fresh72 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh72 > 0i32 as lua_ulong {
        let fresh73 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh73 as lua_uchar as lua_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* string starts with a newline? */
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
        /* skip it */
        inclinenumber(ls);
    }
    loop {
        match (*ls).current {
            -1 => {
                /* error */
                let mut what: *const lua_char = if !seminfo.is_null() {
                    s!(b"string\x00")
                } else {
                    s!(b"comment\x00")
                };
                let mut msg: *const lua_char = luaO_pushfstring!(
                    (*ls).L,
                    s!(b"unfinished long %s (starting at line %d)\x00"),
                    what,
                    line,
                );
                lexerror(ls, msg, TK_EOS as lua_int);
            }
            93 => {
                if !(skip_sep(ls) == sep) {
                    continue;
                }
                /* skip 2nd ']' */
                save(ls, (*ls).current);
                let fresh74 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh74 > 0i32 as lua_ulong {
                    let fresh75 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh75 as lua_uchar as lua_int
                } else {
                    luaZ_fill((*ls).z)
                };
                break;
            }
            10 | 13 => {
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if !seminfo.is_null() {
                    continue;
                }
                /* avoid wasting space */
                (*(*ls).buff).n = 0i32 as size_t
            }
            _ => {
                if !seminfo.is_null() {
                    save(ls, (*ls).current);
                    let fresh76 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh76 > 0i32 as lua_ulong {
                        let fresh77 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh77 as lua_uchar as lua_int
                    } else {
                        luaZ_fill((*ls).z)
                    }
                } else {
                    let fresh78 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh78 > 0i32 as lua_ulong {
                        let fresh79 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh79 as lua_uchar as lua_int
                    } else {
                        luaZ_fill((*ls).z)
                    }
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo).ts = luaX_newstring(
            ls,
            (*(*ls).buff).buffer.offset((2i32 + sep) as isize),
            (*(*ls).buff)
                .n
                .wrapping_sub((2i32 * (2i32 + sep)) as lua_ulong),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState) -> lua_int {
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(mut ls: *mut LexState, mut msg: *const lua_char) -> ! {
    lexerror(ls, msg, (*ls).t.token);
}
