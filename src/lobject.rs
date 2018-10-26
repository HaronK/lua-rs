use super::prelude::*;

/*
** Extra tags for non-values
*/
pub const LUA_TPROTO: i32 = LUA_NUMTAGS; /* function prototypes */
pub const LUA_TDEADKEY: i32 = LUA_NUMTAGS + 1; /* removed keys in tables */

/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
pub const LUA_TOTALTAGS: i32 = LUA_TPROTO + 2;

/*
** tags for Tagged Values have the following use of bits:
** bits 0-3: actual tag (a LUA_T* value)
** bits 4-5: variant bits
** bit 6: whether value is collectable
*/

/*
** LUA_TFUNCTION variants:
** 0 - Lua function
** 1 - light C function
** 2 - regular C function (closure)
*/

/* Variant tags for functions */
pub const LUA_TLCL: i32 = (LUA_TFUNCTION | (0 << 4)); /* Lua closure */
pub const LUA_TLCF: i32 = (LUA_TFUNCTION | (1 << 4)); /* light C function */
pub const LUA_TCCL: i32 = (LUA_TFUNCTION | (2 << 4)); /* C closure */

/* Variant tags for strings */
pub const LUA_TSHRSTR: i32 = (LUA_TSTRING | (0 << 4)); /* short strings */
pub const LUA_TLNGSTR: i32 = (LUA_TSTRING | (1 << 4)); /* long strings */

/* Variant tags for numbers */
pub const LUA_TNUMFLT: i32 = (LUA_TNUMBER | (0 << 4)); /* float numbers */
pub const LUA_TNUMINT: i32 = (LUA_TNUMBER | (1 << 4)); /* integer numbers */

/* Bit mark for collectable types */
pub const BIT_ISCOLLECTABLE: i32 = (1 << 6);

/* types of 'usual argument conversions' for lua_Number and lua_Integer */
pub type l_uacNumber = lua_double;
pub type l_uacInt = lua_longlong;

/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER TM" and "ORDER OP"
*/
pub type TMS = lua_uint;
/* number of elements in the enum */
pub const TM_N: TMS = 24;
pub const TM_CALL: TMS = 23;
pub const TM_CONCAT: TMS = 22;
pub const TM_LE: TMS = 21;
pub const TM_LT: TMS = 20;
pub const TM_BNOT: TMS = 19;
pub const TM_UNM: TMS = 18;
pub const TM_SHR: TMS = 17;
pub const TM_SHL: TMS = 16;
pub const TM_BXOR: TMS = 15;
pub const TM_BOR: TMS = 14;
pub const TM_BAND: TMS = 13;
pub const TM_IDIV: TMS = 12;
pub const TM_DIV: TMS = 11;
pub const TM_POW: TMS = 10;
pub const TM_MOD: TMS = 9;
pub const TM_MUL: TMS = 8;
pub const TM_SUB: TMS = 7;
pub const TM_ADD: TMS = 6;
/* last tag method with fast access */
pub const TM_EQ: TMS = 5;
pub const TM_LEN: TMS = 4;
pub const TM_MODE: TMS = 3;
pub const TM_GC: TMS = 2;
pub const TM_NEWINDEX: TMS = 1;
pub const TM_INDEX: TMS = 0;

/*
** 'module' operation for hashing (size is always a power of 2)
*/
/*
** (address of) a fixed nil value
*/

/* size of buffer for 'luaO_utf8esc' function */
#[no_mangle]
pub unsafe extern "C" fn luaO_int2fb(mut x: lua_uint) -> lua_int {
    /* exponent */
    let mut e: lua_int = 0i32;
    if x < 8i32 as lua_uint {
        return x as lua_int;
    } else {
        while x >= (8i32 << 4i32) as lua_uint {
            /* coarse steps */
            /* x = ceil(x / 16) */
            x = x.wrapping_add(0xfi32 as lua_uint) >> 4i32;
            e += 4i32
        }
        while x >= (8i32 << 1i32) as lua_uint {
            /* fine steps */
            /* x = ceil(x / 2) */
            x = x.wrapping_add(1i32 as lua_uint) >> 1i32;
            e += 1
        }
        return e + 1i32 << 3i32 | x as lua_int - 8i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_fb2int(mut x: lua_int) -> lua_int {
    return if x < 8i32 {
        x
    } else {
        (x & 7i32) + 8i32 << (x >> 3i32) - 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_utf8esc(mut buff: *mut lua_char, mut x: lua_ulong) -> lua_int {
    /* number of bytes put in buffer (backwards) */
    let mut n: lua_int = 1i32;
    /* ascii? */
    if x < 0x80i32 as lua_ulong {
        *buff.offset((8i32 - 1i32) as isize) = x as lua_char
    } else {
        /* need continuation bytes */
        /* maximum that fits in first byte */
        let mut mfb: lua_uint = 0x3fi32 as lua_uint;
        loop {
            /* add continuation bytes */
            let fresh0 = n;
            n = n + 1;
            *buff.offset((8i32 - fresh0) as isize) =
                (0x80i32 as lua_ulong | x & 0x3fi32 as lua_ulong) as lua_char;
            /* remove added bits */
            x >>= 6i32;
            /* now there is one less bit available in first byte */
            mfb >>= 1i32;
            if !(x > mfb as lua_ulong) {
                break;
            }
        }
        /* still needs continuation byte? */
        /* add first byte */
        *buff.offset((8i32 - n) as isize) = ((!mfb << 1i32) as lua_ulong | x) as lua_char
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_ceillog2(mut x: lua_uint) -> lua_int {
    /* log_2[i] = ceil(log2(i - 1)) */
    static mut log_2: [lu_byte; 256] = [
        0i32 as lu_byte,
        1i32 as lu_byte,
        2i32 as lu_byte,
        2i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
    ];
    let mut l: lua_int = 0i32;
    x = x.wrapping_sub(1);
    while x >= 256i32 as lua_uint {
        l += 8i32;
        x >>= 8i32
    }
    return l + log_2[x as usize] as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_arith(
    mut L: *mut lua_State,
    mut op: lua_int,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: *mut TValue,
) -> () {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            /* operate only on integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            if 0 != if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
                i1 = (*p1).value_.i;
                1i32
            } else {
                luaV_tointeger(p1, &mut i1, 0i32)
            } && 0
                != if (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                    i2 = (*p2).value_.i;
                    1i32
                } else {
                    luaV_tointeger(p2, &mut i2, 0i32)
                } {
                let mut io: *mut TValue = res;
                (*io).value_.i = intarith(L, op, i1, i2);
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                return;
            }
        }
        5 | 4 => {
            /* go to the end */
            /* operate only on floats */
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                n1 = (*p1).value_.n;
                1i32
            } else {
                luaV_tonumber_(p1, &mut n1)
            } && 0
                != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    n2 = (*p2).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p2, &mut n2)
                } {
                let mut io_0: *mut TValue = res;
                (*io_0).value_.n = numarith(L, op, n1, n2);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
        _ => {
            /* go to the end */
            /* other operations */
            let mut n1_0: lua_Number = 0.;
            let mut n2_0: lua_Number = 0.;
            if (*p1).tt_ == 3i32 | 1i32 << 4i32 && (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                let mut io_1: *mut TValue = res;
                (*io_1).value_.i = intarith(L, op, (*p1).value_.i, (*p2).value_.i);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32;
                return;
            } else if 0
                != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    n1_0 = (*p1).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p1, &mut n1_0)
                }
                && 0 != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    n2_0 = (*p2).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p2, &mut n2_0)
                } {
                let mut io_2: *mut TValue = res;
                (*io_2).value_.n = numarith(L, op, n1_0, n2_0);
                (*io_2).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
    }
    /* go to the end */
    /* could not perform raw operation; try metamethod */
    /* should not fail when folding (compile time) */
    luaT_trybinTM(L, p1, p2, res, (op - 0i32 + TM_ADD as lua_int) as TMS);
}
unsafe extern "C" fn numarith(
    mut _L: *mut lua_State,
    mut op: lua_int,
    mut v1: lua_Number,
    mut v2: lua_Number,
) -> lua_Number {
    match op {
        0 => return v1 + v2,
        1 => return v1 - v2,
        2 => return v1 * v2,
        5 => return v1 / v2,
        4 => return pow(v1, v2),
        6 => return floor(v1 / v2),
        12 => return -v1,
        3 => {
            let mut m: lua_Number = 0.;
            m = fmod(v1, v2);
            if m * v2 < 0i32 as lua_double {
                m += v2
            }
            return m;
        }
        _ => return 0i32 as lua_Number,
    };
}
unsafe extern "C" fn intarith(
    mut L: *mut lua_State,
    mut op: lua_int,
    mut v1: lua_Integer,
    mut v2: lua_Integer,
) -> lua_Integer {
    match op {
        0 => return (v1 as lua_Unsigned).wrapping_add(v2 as lua_Unsigned) as lua_Integer,
        1 => return (v1 as lua_Unsigned).wrapping_sub(v2 as lua_Unsigned) as lua_Integer,
        2 => return (v1 as lua_Unsigned).wrapping_mul(v2 as lua_Unsigned) as lua_Integer,
        3 => return luaV_mod(L, v1, v2),
        6 => return luaV_div(L, v1, v2),
        7 => return (v1 as lua_Unsigned & v2 as lua_Unsigned) as lua_Integer,
        8 => return (v1 as lua_Unsigned | v2 as lua_Unsigned) as lua_Integer,
        9 => return (v1 as lua_Unsigned ^ v2 as lua_Unsigned) as lua_Integer,
        10 => return luaV_shiftl(v1, v2),
        11 => return luaV_shiftl(v1, -v2),
        12 => return (0i32 as lua_Unsigned).wrapping_sub(v1 as lua_Unsigned) as lua_Integer,
        13 => return (!(0i32 as lua_Unsigned) ^ v1 as lua_Unsigned) as lua_Integer,
        _ => return 0i32 as lua_Integer,
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_str2num(mut s: *const lua_char, mut o: *mut TValue) -> size_t {
    let mut i: lua_Integer = 0;
    let mut n: lua_Number = 0.;
    let mut e: *const lua_char = 0 as *const lua_char;
    e = l_str2int(s, &mut i);
    if !e.is_null() {
        /* try as an integer */
        let mut io: *mut TValue = o;
        (*io).value_.i = i;
        (*io).tt_ = 3i32 | 1i32 << 4i32
    } else {
        e = l_str2d(s, &mut n);
        if !e.is_null() {
            /* else try as a float */
            let mut io_0: *mut TValue = o;
            (*io_0).value_.n = n;
            (*io_0).tt_ = 3i32 | 0i32 << 4i32
        } else {
            return 0i32 as size_t;
        }
    }
    /* success; return string size */
    return (e.wrapping_offset_from(s) as lua_long + 1i32 as lua_long) as size_t;
}
/*
** Convert string 's' to a Lua number (put in 'result'). Return NULL
** on fail or the address of the ending '\0' on success.
** 'pmode' points to (and 'mode' contains) special things in the string:
** - 'x'/'X' means an hexadecimal numeral
** - 'n'/'N' means 'inf' or 'nan' (which should be rejected)
** - '.' just optimizes the search for the common case (nothing special)
** This function accepts both the current locale or a dot as the radix
** mark. If the convertion fails, it may mean number has a dot but
** locale accepts something else. In that case, the code copies 's'
** to a buffer (because 's' is read-only), changes the dot to the
** current locale radix mark, and tries to convert again.
*/
unsafe extern "C" fn l_str2d(
    mut s: *const lua_char,
    mut result: *mut lua_Number,
) -> *const lua_char {
    let mut endptr: *const lua_char = 0 as *const lua_char;
    let mut pmode: *const lua_char = strpbrk(s, s!(b".xXnN\x00"));
    let mut mode: lua_int = if !pmode.is_null() {
        *pmode as lua_uchar as lua_int | 'A' as i32 ^ 'a' as i32
    } else {
        0i32
    };
    /* reject 'inf' and 'nan' */
    if mode == 'n' as i32 {
        return 0 as *const lua_char;
    } else {
        /* try to convert */
        endptr = l_str2dloc(s, result, mode);
        if endptr.is_null() {
            /* failed? may be a different locale */
            let mut buff: [lua_char; 201] = [0; 201];
            let mut pdot: *const lua_char = strchr(s, '.' as i32);
            if strlen(s) > 200i32 as lua_ulong || pdot.is_null() {
                /* string too long or no dot; fail */
                return 0 as *const lua_char;
            } else {
                /* copy string to buffer */
                strcpy(buff.as_mut_ptr(), s);
                /* correct decimal point */
                buff[pdot.wrapping_offset_from(s) as lua_long as usize] =
                    *(*localeconv()).decimal_point.offset(0isize);
                /* try again */
                endptr = l_str2dloc(buff.as_mut_ptr(), result, mode);
                if !endptr.is_null() {
                    /* make relative to 's' */
                    endptr = s
                        .offset(endptr.wrapping_offset_from(buff.as_mut_ptr()) as lua_long as isize)
                }
            }
        }
        return endptr;
    };
}
/*
** {==================================================================
** Lua's implementation for 'lua_strx2number'
** ===================================================================
*/
/* }====================================================== */
/* maximum length of a numeral */
unsafe extern "C" fn l_str2dloc(
    mut s: *const lua_char,
    mut result: *mut lua_Number,
    mut mode: lua_int,
) -> *const lua_char {
    let mut endptr: *mut lua_char = 0 as *mut lua_char;
    /* try to convert */
    *result = if mode == 'x' as i32 {
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut lua_char {
        /* nothing recognized? */
        return 0 as *const lua_char;
    } else {
        while 0
            != luai_ctype_[(*endptr as lua_uchar as lua_int + 1i32) as usize] as lua_int
                & 1i32 << 3i32
        {
            /* skip trailing spaces */
            endptr = endptr.offset(1isize)
        }
        /* OK if no trailing characters */
        return if *endptr as lua_int == '\u{0}' as i32 {
            endptr
        } else {
            0 as *mut lua_char
        };
    };
}
unsafe extern "C" fn l_str2int(
    mut s: *const lua_char,
    mut result: *mut lua_Integer,
) -> *const lua_char {
    let mut a: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut empty: lua_int = 1i32;
    let mut neg: lua_int = 0;
    while 0 != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 3i32 {
        /* skip initial spaces */
        s = s.offset(1isize)
    }
    neg = isneg(&mut s);
    if *s.offset(0isize) as lua_int == '0' as i32
        && (*s.offset(1isize) as lua_int == 'x' as i32
            || *s.offset(1isize) as lua_int == 'X' as i32)
    {
        /* hex? */
        /* skip '0x' */
        s = s.offset(2isize);
        while 0
            != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 4i32
        {
            a = a
                .wrapping_mul(16i32 as lua_ulonglong)
                .wrapping_add(luaO_hexavalue(*s as lua_int) as lua_ulonglong);
            empty = 0i32;
            s = s.offset(1isize)
        }
    } else {
        while 0
            != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 1i32
        {
            let mut d: lua_int = *s as lua_int - '0' as i32;
            /* overflow? */
            if a >= (9223372036854775807i64 / 10i32 as lua_longlong) as lua_Unsigned
                && (a > (9223372036854775807i64 / 10i32 as lua_longlong) as lua_Unsigned
                    || d > (9223372036854775807i64 % 10i32 as lua_longlong) as lua_int + neg)
            {
                /* do not accept it (as integer) */
                return 0 as *const lua_char;
            } else {
                a = a
                    .wrapping_mul(10i32 as lua_ulonglong)
                    .wrapping_add(d as lua_ulonglong);
                empty = 0i32;
                s = s.offset(1isize)
            }
        }
    }
    while 0 != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 3i32 {
        /* skip trailing spaces */
        s = s.offset(1isize)
    }
    if 0 != empty || *s as lua_int != '\u{0}' as i32 {
        /* something wrong in the numeral */
        return 0 as *const lua_char;
    } else {
        *result = (if 0 != neg {
            (0u32 as lua_ulonglong).wrapping_sub(a)
        } else {
            a
        }) as lua_Integer;
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_hexavalue(mut c: lua_int) -> lua_int {
    if 0 != luai_ctype_[(c + 1i32) as usize] as lua_int & 1i32 << 1i32 {
        return c - '0' as i32;
    } else {
        return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10i32;
    };
}
unsafe extern "C" fn isneg(mut s: *mut *const lua_char) -> lua_int {
    if **s as lua_int == '-' as i32 {
        *s = (*s).offset(1isize);
        return 1i32;
    } else {
        if **s as lua_int == '+' as i32 {
            *s = (*s).offset(1isize)
        }
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_tostring(mut L: *mut lua_State, mut obj: StkId) -> () {
    let mut buff: [lua_char; 50] = [0; 50];
    let mut len: size_t = 0;
    if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 50]>() as lua_ulong,
            s!(b"%lld\x00"),
            (*obj).value_.i,
        ) as size_t
    } else {
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 50]>() as lua_ulong,
            s!(b"%.14g\x00"),
            (*obj).value_.n,
        ) as size_t;
        if buff[strspn(buff.as_mut_ptr(), s!(b"-0123456789\x00")) as usize] as lua_int
            == '\u{0}' as i32
        {
            /* looks like an int? */
            let fresh1 = len;
            len = len.wrapping_add(1);
            buff[fresh1 as usize] = *(*localeconv()).decimal_point.offset(0isize);
            /* adds '.0' to result */
            let fresh2 = len;
            len = len.wrapping_add(1);
            buff[fresh2 as usize] = '0' as i32 as lua_char
        }
    }
    let mut io: *mut TValue = obj;
    let mut x_: *mut TString = luaS_newlstr(L, buff.as_mut_ptr(), len);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
}
unsafe extern "C" fn pushstr(mut L: *mut lua_State, mut str: *const lua_char, mut l: size_t) -> () {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = luaS_newlstr(L, str, l);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
    luaD_inctop(L);
}
#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut lua_char,
    mut source: *const lua_char,
    mut bufflen: size_t,
) -> () {
    let mut l: size_t = strlen(source);
    if *source as lua_int == '=' as i32 {
        /* 'literal' source */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        } else {
            /* truncate it */
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                bufflen
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(bufflen.wrapping_sub(1i32 as lua_ulong) as isize);
            *out = '\u{0}' as i32 as lua_char
        }
    } else if *source as lua_int == '@' as i32 {
        /* file name */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        } else {
            /* add '...' before rest of name */
            memcpy(
                out as *mut lua_void,
                s!(b"...\x00") as *const lua_void,
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong) as isize,
            );
            bufflen = (bufflen as lua_ulong).wrapping_sub(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong),
            ) as size_t as size_t;
            memcpy(
                out as *mut lua_void,
                source
                    .offset(1isize)
                    .offset(l as isize)
                    .offset(-(bufflen as isize)) as *const lua_void,
                bufflen.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        }
    } else {
        /* string; format as [string "source"] */
        /* find first new line (if any) */
        let mut nl: *const lua_char = strchr(source, '\n' as i32);
        /* add prefix */
        memcpy(
            out as *mut lua_void,
            s!(b"[string \"\x00") as *const lua_void,
            (::std::mem::size_of::<[lua_char; 10]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
        out = out.offset(
            (::std::mem::size_of::<[lua_char; 10]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong) as isize,
        );
        /* save space for prefix+suffix+'\0' */
        bufflen = (bufflen as lua_ulong).wrapping_sub(
            (::std::mem::size_of::<[lua_char; 15]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_add(1i32 as lua_ulong),
        ) as size_t as size_t;
        if l < bufflen && nl.is_null() {
            /* small one-line source? */
            /* keep it */
            memcpy(
                out as *mut lua_void,
                source as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(l as isize)
        } else {
            if !nl.is_null() {
                /* stop at first newline */
                l = nl.wrapping_offset_from(source) as lua_long as size_t
            }
            if l > bufflen {
                l = bufflen
            }
            memcpy(
                out as *mut lua_void,
                source as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(l as isize);
            memcpy(
                out as *mut lua_void,
                s!(b"...\x00") as *const lua_void,
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong) as isize,
            )
        }
        memcpy(
            out as *mut lua_void,
            s!(b"\"]\x00") as *const lua_void,
            (::std::mem::size_of::<[lua_char; 3]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_add(1i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
    };
}
