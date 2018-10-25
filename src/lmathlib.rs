use stdc::prelude::*;
use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn acos(_: lua_double) -> lua_double;
    #[no_mangle]
    fn asin(_: lua_double) -> lua_double;
    #[no_mangle]
    fn atan2(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn cos(_: lua_double) -> lua_double;
    #[no_mangle]
    fn sin(_: lua_double) -> lua_double;
    #[no_mangle]
    fn tan(_: lua_double) -> lua_double;
    #[no_mangle]
    fn cosh(_: lua_double) -> lua_double;
    #[no_mangle]
    fn sinh(_: lua_double) -> lua_double;
    #[no_mangle]
    fn tanh(_: lua_double) -> lua_double;
    #[no_mangle]
    fn exp(_: lua_double) -> lua_double;
    #[no_mangle]
    fn frexp(_: lua_double, _: *mut lua_int) -> lua_double;
    #[no_mangle]
    fn ldexp(_: lua_double, _: lua_int) -> lua_double;
    #[no_mangle]
    fn log(_: lua_double) -> lua_double;
    #[no_mangle]
    fn log10(_: lua_double) -> lua_double;
    #[no_mangle]
    fn log2(_: lua_double) -> lua_double;
    #[no_mangle]
    fn pow(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn sqrt(_: lua_double) -> lua_double;
    #[no_mangle]
    fn ceil(_: lua_double) -> lua_double;
    #[no_mangle]
    fn fabs(_: lua_double) -> lua_double;
    #[no_mangle]
    fn floor(_: lua_double) -> lua_double;
    #[no_mangle]
    fn fmod(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Integer;
    #[no_mangle]
    fn lua_compare(L: *mut lua_State, idx1: lua_int, idx2: lua_int, op: lua_int) -> lua_int;
    /*
     ** push functions (C -> stack)
     */
    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number) -> ();
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;
    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, arg: lua_int, extramsg: *const lua_char) -> lua_int;
    #[no_mangle]
    fn luaL_checknumber(L: *mut lua_State, arg: lua_int) -> lua_Number;
    #[no_mangle]
    fn luaL_optnumber(L: *mut lua_State, arg: lua_int, def: lua_Number) -> lua_Number;
    #[no_mangle]
    fn luaL_checkinteger(L: *mut lua_State, arg: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_checkany(L: *mut lua_State, arg: lua_int) -> ();
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_math(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 36]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, mathlib.as_ptr(), 0i32);
    lua_pushnumber(L, 3.141592653589793f64);
    lua_setfield(L, -2i32, s!(b"pi\x00"));
    lua_pushnumber(L, ::std::f64::INFINITY);
    lua_setfield(L, -2i32, s!(b"huge\x00"));
    lua_pushinteger(L, 9223372036854775807i64);
    lua_setfield(L, -2i32, s!(b"maxinteger\x00"));
    lua_pushinteger(L, -9223372036854775807i64 - 1i64);
    lua_setfield(L, -2i32, s!(b"mininteger\x00"));
    return 1i32;
}
/* }================================================================== */
static mut mathlib: [luaL_Reg; 36] = [
    lua_reg!(b"abs\x00", math_abs),
    lua_reg!(b"acos\x00", math_acos),
    lua_reg!(b"asin\x00", math_asin),
    lua_reg!(b"atan\x00", math_atan),
    lua_reg!(b"ceil\x00", math_ceil),
    lua_reg!(b"cos\x00", math_cos),
    lua_reg!(b"deg\x00", math_deg),
    lua_reg!(b"exp\x00", math_exp),
    lua_reg!(b"tointeger\x00", math_toint),
    lua_reg!(b"floor\x00", math_floor),
    lua_reg!(b"fmod\x00", math_fmod),
    lua_reg!(b"ult\x00", math_ult),
    lua_reg!(b"log\x00", math_log),
    lua_reg!(b"max\x00", math_max),
    lua_reg!(b"min\x00", math_min),
    lua_reg!(b"modf\x00", math_modf),
    lua_reg!(b"rad\x00", math_rad),
    lua_reg!(b"random\x00", math_random),
    lua_reg!(b"randomseed\x00", math_randomseed),
    lua_reg!(b"sin\x00", math_sin),
    lua_reg!(b"sqrt\x00", math_sqrt),
    lua_reg!(b"tan\x00", math_tan),
    lua_reg!(b"type\x00", math_type),
    lua_reg!(b"atan2\x00", math_atan),
    lua_reg!(b"cosh\x00", math_cosh),
    lua_reg!(b"sinh\x00", math_sinh),
    lua_reg!(b"tanh\x00", math_tanh),
    lua_reg!(b"pow\x00", math_pow),
    lua_reg!(b"frexp\x00", math_frexp),
    lua_reg!(b"ldexp\x00", math_ldexp),
    lua_reg!(b"log10\x00", math_log10),
    lua_reg_none!(b"pi\x00"),
    lua_reg_none!(b"huge\x00"),
    lua_reg_none!(b"maxinteger\x00"),
    lua_reg_none!(b"mininteger\x00"),
    lua_reg_none!(0),
];
unsafe extern "C" fn math_log10(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, log10(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_ldexp(mut L: *mut lua_State) -> lua_int {
    let mut x: lua_Number = luaL_checknumber(L, 1i32);
    let mut ep: lua_int = luaL_checkinteger(L, 2i32) as lua_int;
    lua_pushnumber(L, ldexp(x, ep));
    return 1i32;
}
unsafe extern "C" fn math_frexp(mut L: *mut lua_State) -> lua_int {
    let mut e: lua_int = 0;
    lua_pushnumber(L, frexp(luaL_checknumber(L, 1i32), &mut e));
    lua_pushinteger(L, e as lua_Integer);
    return 2i32;
}
unsafe extern "C" fn math_pow(mut L: *mut lua_State) -> lua_int {
    let mut x: lua_Number = luaL_checknumber(L, 1i32);
    let mut y: lua_Number = luaL_checknumber(L, 2i32);
    lua_pushnumber(L, pow(x, y));
    return 1i32;
}
unsafe extern "C" fn math_tanh(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, tanh(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sinh(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, sinh(luaL_checknumber(L, 1i32)));
    return 1i32;
}
/*
** {==================================================================
** Deprecated functions (for compatibility only)
** ===================================================================
*/
unsafe extern "C" fn math_cosh(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, cosh(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_atan(mut L: *mut lua_State) -> lua_int {
    let mut y: lua_Number = luaL_checknumber(L, 1i32);
    let mut x: lua_Number = luaL_optnumber(L, 2i32, 1i32 as lua_Number);
    lua_pushnumber(L, atan2(y, x));
    return 1i32;
}
unsafe extern "C" fn math_type(mut L: *mut lua_State) -> lua_int {
    if lua_type(L, 1i32) == 3i32 {
        if 0 != lua_isinteger(L, 1i32) {
            lua_pushstring(L, s!(b"integer\x00"));
        } else {
            lua_pushstring(L, s!(b"float\x00"));
        }
    } else {
        luaL_checkany(L, 1i32);
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn math_tan(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, tan(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sqrt(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, sqrt(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_sin(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, sin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_randomseed(mut L: *mut lua_State) -> lua_int {
    srandom(luaL_checknumber(L, 1i32) as lua_Integer as lua_uint);
    /* discard first value to avoid undesirable correlations */
    random();
    return 0i32;
}
/*
** This function uses 'double' (instead of 'lua_Number') to ensure that
** all bits from 'l_rand' can be represented, and that 'RANDMAX + 1.0'
** will keep full precision (ensuring that 'r' is always less than 1.0.)
*/
unsafe extern "C" fn math_random(mut L: *mut lua_State) -> lua_int {
    let mut low: lua_Integer = 0;
    let mut up: lua_Integer = 0;
    let mut r: lua_double =
        random() as lua_double * (1.0f64 / (2147483647i32 as lua_double + 1.0f64));
    match lua_gettop(L) {
        0 => {
            /* no arguments */
            /* Number between 0 and 1 */
            lua_pushnumber(L, r);
            return 1i32;
        }
        1 => {
            /* only upper limit */
            low = 1i32 as lua_Integer;
            up = luaL_checkinteger(L, 1i32)
        }
        2 => {
            /* lower and upper limits */
            low = luaL_checkinteger(L, 1i32);
            up = luaL_checkinteger(L, 2i32)
        }
        _ => return luaL_error!(L, s!(b"wrong number of arguments\x00")),
    }
    /* random integer in the interval [low, up] */
    (low <= up || 0 != luaL_argerror(L, 1i32, s!(b"interval is empty\x00"))) as lua_int;
    (low >= 0i32 as lua_longlong
        || up <= 9223372036854775807i64 + low
        || 0 != luaL_argerror(L, 1i32, s!(b"interval too large\x00"))) as lua_int;
    r *= (up - low) as lua_double + 1.0f64;
    lua_pushinteger(L, r as lua_Integer + low);
    return 1i32;
}
unsafe extern "C" fn math_rad(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1i32) * (3.141592653589793f64 / 180.0f64),
    );
    return 1i32;
}
/*
** next function does not use 'modf', avoiding problems with 'double*'
** (which is not compatible with 'float*') when lua_Number is not
** 'double'.
*/
unsafe extern "C" fn math_modf(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_Number = 0.;
    let mut ip: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* number is its own integer part */
        lua_settop(L, 1i32);
        /* no fractional part */
        lua_pushnumber(L, 0i32 as lua_Number);
    } else {
        n = luaL_checknumber(L, 1i32);
        /* integer part (rounds toward zero) */
        ip = if n < 0i32 as lua_double {
            ceil(n)
        } else {
            floor(n)
        };
        pushnumint(L, ip);
        /* fractional part (test needed for inf/-inf) */
        lua_pushnumber(L, if n == ip { 0.0f64 } else { n - ip });
    }
    return 2i32;
}
unsafe extern "C" fn pushnumint(mut L: *mut lua_State, mut d: lua_Number) -> () {
    let mut n: lua_Integer = 0;
    /* does 'd' fit in an integer? */
    if d >= (-9223372036854775807i64 - 1i64) as lua_double
        && d < -((-9223372036854775807i64 - 1i64) as lua_double)
        && {
            n = d as lua_longlong;
            0 != 1i32
        } {
        /* result is integer */
        lua_pushinteger(L, n);
    } else {
        /* result is float */
        lua_pushnumber(L, d);
    };
}
unsafe extern "C" fn math_min(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    /* index of current minimum value */
    let mut imin: lua_int = 1i32;
    let mut i: lua_int = 0;
    (n >= 1i32 || 0 != luaL_argerror(L, 1i32, s!(b"value expected\x00"))) as lua_int;
    i = 2i32;
    while i <= n {
        if 0 != lua_compare(L, i, imin, 1i32) {
            imin = i
        }
        i += 1
    }
    lua_pushvalue(L, imin);
    return 1i32;
}
unsafe extern "C" fn math_max(mut L: *mut lua_State) -> lua_int {
    /* number of arguments */
    let mut n: lua_int = lua_gettop(L);
    /* index of current maximum value */
    let mut imax: lua_int = 1i32;
    let mut i: lua_int = 0;
    (n >= 1i32 || 0 != luaL_argerror(L, 1i32, s!(b"value expected\x00"))) as lua_int;
    i = 2i32;
    while i <= n {
        if 0 != lua_compare(L, imax, i, 1i32) {
            imax = i
        }
        i += 1
    }
    lua_pushvalue(L, imax);
    return 1i32;
}
unsafe extern "C" fn math_log(mut L: *mut lua_State) -> lua_int {
    let mut x: lua_Number = luaL_checknumber(L, 1i32);
    let mut res: lua_Number = 0.;
    if lua_type(L, 2i32) <= 0i32 {
        res = log(x)
    } else {
        let mut base: lua_Number = luaL_checknumber(L, 2i32);
        if base == 2.0f64 {
            res = log2(x)
        } else if base == 10.0f64 {
            res = log10(x)
        } else {
            res = log(x) / log(base)
        }
    }
    lua_pushnumber(L, res);
    return 1i32;
}
unsafe extern "C" fn math_ult(mut L: *mut lua_State) -> lua_int {
    let mut a: lua_Integer = luaL_checkinteger(L, 1i32);
    let mut b: lua_Integer = luaL_checkinteger(L, 2i32);
    lua_pushboolean(L, ((a as lua_Unsigned) < b as lua_Unsigned) as lua_int);
    return 1i32;
}
unsafe extern "C" fn math_fmod(mut L: *mut lua_State) -> lua_int {
    if 0 != lua_isinteger(L, 1i32) && 0 != lua_isinteger(L, 2i32) {
        let mut d: lua_Integer = lua_tointegerx(L, 2i32, 0 as *mut lua_int);
        if (d as lua_Unsigned).wrapping_add(1u32 as lua_ulonglong) <= 1u32 as lua_ulonglong {
            /* special cases: -1 or 0 */
            (d != 0i32 as lua_longlong || 0 != luaL_argerror(L, 2i32, s!(b"zero\x00"))) as lua_int;
            /* avoid overflow with 0x80000... / -1 */
            lua_pushinteger(L, 0i32 as lua_Integer);
        } else {
            lua_pushinteger(L, lua_tointegerx(L, 1i32, 0 as *mut lua_int) % d);
        }
    } else {
        lua_pushnumber(
            L,
            fmod(luaL_checknumber(L, 1i32), luaL_checknumber(L, 2i32)),
        );
    }
    return 1i32;
}
unsafe extern "C" fn math_floor(mut L: *mut lua_State) -> lua_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* integer is its own floor */
        lua_settop(L, 1i32);
    } else {
        d = floor(luaL_checknumber(L, 1i32));
        pushnumint(L, d);
    }
    return 1i32;
}
unsafe extern "C" fn math_toint(mut L: *mut lua_State) -> lua_int {
    let mut valid: lua_int = 0;
    let mut n: lua_Integer = lua_tointegerx(L, 1i32, &mut valid);
    if 0 != valid {
        lua_pushinteger(L, n);
    } else {
        luaL_checkany(L, 1i32);
        /* value is not convertible to integer */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn math_exp(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, exp(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_deg(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(
        L,
        luaL_checknumber(L, 1i32) * (180.0f64 / 3.141592653589793f64),
    );
    return 1i32;
}
unsafe extern "C" fn math_cos(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, cos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_ceil(mut L: *mut lua_State) -> lua_int {
    let mut d: lua_Number = 0.;
    if 0 != lua_isinteger(L, 1i32) {
        /* integer is its own ceil */
        lua_settop(L, 1i32);
    } else {
        d = ceil(luaL_checknumber(L, 1i32));
        pushnumint(L, d);
    }
    return 1i32;
}
unsafe extern "C" fn math_asin(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, asin(luaL_checknumber(L, 1i32)));
    return 1i32;
}
unsafe extern "C" fn math_acos(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(L, acos(luaL_checknumber(L, 1i32)));
    return 1i32;
}
/*
** $Id: lmathlib.c,v 1.119.1.1 2017/04/19 17:20:42 roberto Exp $
** Standard mathematical library
** See Copyright Notice in lua.h
*/
/* { */
/* (2^31 - 1), following POSIX */
/* } */
unsafe extern "C" fn math_abs(mut L: *mut lua_State) -> lua_int {
    if 0 != lua_isinteger(L, 1i32) {
        let mut n: lua_Integer = lua_tointegerx(L, 1i32, 0 as *mut lua_int);
        if n < 0i32 as lua_longlong {
            n = (0u32 as lua_ulonglong).wrapping_sub(n as lua_Unsigned) as lua_Integer
        }
        lua_pushinteger(L, n);
    } else {
        lua_pushnumber(L, fabs(luaL_checknumber(L, 1i32)));
    }
    return 1i32;
}
