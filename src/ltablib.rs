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
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: lua_int, n: lua_int) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_typename(L: *mut lua_State, tp: lua_int) -> *const lua_char;
    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_compare(L: *mut lua_State, idx1: lua_int, idx2: lua_int, op: lua_int) -> lua_int;
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
    fn lua_geti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;
    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();
    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();
    #[no_mangle]
    fn lua_seti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> ();
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
    fn luaL_checkversion_(L: *mut lua_State, ver: lua_Number, sz: size_t) -> ();
    #[no_mangle]
    fn luaL_argerror(L: *mut lua_State, arg: lua_int, extramsg: *const lua_char) -> lua_int;
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
    fn luaL_len(L: *mut lua_State, idx: lua_int) -> lua_Integer;
    #[no_mangle]
    fn luaL_setfuncs(L: *mut lua_State, l: *const luaL_Reg, nup: lua_int) -> ();
    #[no_mangle]
    fn luaL_buffinit(L: *mut lua_State, B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_addlstring(B: *mut luaL_Buffer, s: *const lua_char, l: size_t) -> ();
    #[no_mangle]
    fn luaL_addvalue(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn luaL_pushresult(B: *mut luaL_Buffer) -> ();
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn clock() -> clock_t;
}

/*
** basic types
*/
/* minimum Lua stack available to a C function */
/* predefined values in the registry */
/* type of numbers in Lua */
pub type lua_Number = lua_double;
/* type for integer functions */
pub type lua_Integer = lua_longlong;
/* unsigned integer type */
pub type lua_Unsigned = lua_ulonglong;
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
/* }====================================================== */
/*
** {======================================================
** Quicksort
** (based on 'Algorithms in MODULA-3', Robert Sedgewick;
**  Addison-Wesley, 1993.)
** =======================================================
*/
/* type for array indices */
pub type IdxT = lua_uint;
pub type time_t = __time_t;
pub type clock_t = __clock_t;
#[no_mangle]
pub unsafe extern "C" fn luaopen_table(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 8]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, tab_funcs.as_ptr(), 0i32);
    return 1i32;
}
/* }====================================================== */
static mut tab_funcs: [luaL_Reg; 8] = [
    lua_reg!(b"concat\x00", tconcat),
    lua_reg!(b"insert\x00", tinsert),
    lua_reg!(b"pack\x00", pack),
    lua_reg!(b"unpack\x00", unpack),
    lua_reg!(b"remove\x00", tremove),
    lua_reg!(b"move\x00", tmove),
    lua_reg!(b"sort\x00", sort),
    lua_reg_none!(0),
];
/* tail call auxsort(L, lo, up, rnd) */
unsafe extern "C" fn sort(mut L: *mut lua_State) -> lua_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut n: lua_Integer = luaL_len(L, 1i32);
    if n > 1i32 as lua_longlong {
        /* non-trivial interval? */
        (n < 2147483647i32 as lua_longlong || 0 != luaL_argerror(L, 1i32, s!(b"array too big\x00")))
            as lua_int;
        /* is there a 2nd argument? */
        if !(lua_type(L, 2i32) <= 0i32) {
            /* must be a function */
            luaL_checktype(L, 2i32, 6i32);
        }
        /* make sure there are two arguments */
        lua_settop(L, 2i32);
        auxsort(L, 1i32 as IdxT, n as IdxT, 0i32 as lua_uint);
    }
    return 0i32;
}
/*
** Check that 'arg' either is a table or can behave like one (that is,
** has a metatable with the required metamethods)
*/
unsafe extern "C" fn checktab(mut L: *mut lua_State, mut arg: lua_int, mut what: lua_int) -> () {
    if lua_type(L, arg) != 5i32 {
        /* is it not a table? */
        /* number of elements to pop */
        let mut n: lua_int = 1i32;
        /* must have metatable */
        if 0 != lua_getmetatable(L, arg)
            && (0 == what & 1i32 || {
                n += 1;
                0 != checkfield(L, s!(b"__index\x00"), n)
            })
            && (0 == what & 2i32 || {
                n += 1;
                0 != checkfield(L, s!(b"__newindex\x00"), n)
            })
            && (0 == what & 4i32 || {
                n += 1;
                0 != checkfield(L, s!(b"__len\x00"), n)
            }) {
            /* pop metatable and tested metamethods */
            lua_settop(L, -n - 1i32);
        } else {
            /* force an error */
            luaL_checktype(L, arg, 5i32);
        }
    };
}
/*
** $Id: ltablib.c,v 1.93.1.1 2017/04/19 17:20:42 roberto Exp $
** Library for Table Manipulation
** See Copyright Notice in lua.h
*/
/*
** Operations that an object must define to mimic a table
** (some functions only need some of them)
*/
/* read */
/* write */
/* length */
/* read/write */
unsafe extern "C" fn checkfield(
    mut L: *mut lua_State,
    mut key: *const lua_char,
    mut n: lua_int,
) -> lua_int {
    lua_pushstring(L, key);
    return (lua_rawget(L, -n) != 0i32) as lua_int;
}
/*
** QuickSort algorithm (recursive function)
*/
unsafe extern "C" fn auxsort(
    mut L: *mut lua_State,
    mut lo: IdxT,
    mut up: IdxT,
    mut rnd: lua_uint,
) -> () {
    while lo < up {
        /* loop for tail recursion */
        /* Pivot index */
        let mut p: IdxT = 0;
        /* to be used later */
        let mut n: IdxT = 0;
        /* sort elements 'lo', 'p', and 'up' */
        lua_geti(L, 1i32, lo as lua_Integer);
        lua_geti(L, 1i32, up as lua_Integer);
        /* a[up] < a[lo]? */
        if 0 != sort_comp(L, -1i32, -2i32) {
            /* swap a[lo] - a[up] */
            set2(L, lo, up);
        } else {
            /* remove both values */
            lua_settop(L, -2i32 - 1i32);
        }
        /* only 2 elements? */
        if up.wrapping_sub(lo) == 1i32 as lua_uint {
            /* already sorted */
            return;
        } else {
            /* small interval or no randomize? */
            if up.wrapping_sub(lo) < 100u32 || rnd == 0i32 as lua_uint {
                /* middle element is a good pivot */
                p = lo.wrapping_add(up).wrapping_div(2i32 as lua_uint)
            } else {
                p = choosePivot(lo, up, rnd)
            }
            lua_geti(L, 1i32, p as lua_Integer);
            lua_geti(L, 1i32, lo as lua_Integer);
            /* a[p] < a[lo]? */
            if 0 != sort_comp(L, -2i32, -1i32) {
                /* swap a[p] - a[lo] */
                set2(L, p, lo);
            } else {
                /* remove a[lo] */
                lua_settop(L, -1i32 - 1i32);
                lua_geti(L, 1i32, up as lua_Integer);
                /* a[up] < a[p]? */
                if 0 != sort_comp(L, -1i32, -2i32) {
                    /* swap a[up] - a[p] */
                    set2(L, p, up);
                } else {
                    lua_settop(L, -2i32 - 1i32);
                }
            }
            /* only 3 elements? */
            if up.wrapping_sub(lo) == 2i32 as lua_uint {
                /* already sorted */
                return;
            } else {
                /* get middle element (Pivot) */
                lua_geti(L, 1i32, p as lua_Integer);
                /* push Pivot */
                lua_pushvalue(L, -1i32);
                /* push a[up - 1] */
                lua_geti(L, 1i32, up.wrapping_sub(1i32 as lua_uint) as lua_Integer);
                /* swap Pivot (a[p]) with a[up - 1] */
                set2(L, p, up.wrapping_sub(1i32 as lua_uint));
                p = partition(L, lo, up);
                /* a[lo .. p - 1] <= a[p] == P <= a[p + 1 .. up] */
                if p.wrapping_sub(lo) < up.wrapping_sub(p) {
                    /* lower interval is smaller? */
                    /* call recursively for lower interval */
                    auxsort(L, lo, p.wrapping_sub(1i32 as lua_uint), rnd);
                    /* size of smaller interval */
                    n = p.wrapping_sub(lo);
                    /* tail call for [p + 1 .. up] (upper interval) */
                    lo = p.wrapping_add(1i32 as lua_uint)
                } else {
                    /* call recursively for upper interval */
                    auxsort(L, p.wrapping_add(1i32 as lua_uint), up, rnd);
                    /* size of smaller interval */
                    n = up.wrapping_sub(p);
                    /* tail call for [lo .. p - 1]  (lower interval) */
                    up = p.wrapping_sub(1i32 as lua_uint)
                }
                /* partition too imbalanced? */
                if !(up.wrapping_sub(lo).wrapping_div(128i32 as lua_uint) > n) {
                    continue;
                }
                /* try a new randomization */
                rnd = l_randomizePivot()
            }
        }
    }
}
/*
** Produce a "random" 'unsigned int' to randomize pivot choice. This
** macro is used only when 'sort' detects a big imbalance in the result
** of a partition. (If you don't want/need this "randomness", ~0 is a
** good choice.)
*/
/* { */
/* size of 'e' measured in number of 'unsigned int's */
/*
** Use 'time' and 'clock' as sources of "randomness". Because we don't
** know the types 'clock_t' and 'time_t', we cannot cast them to
** anything without risking overflows. A safe way to use their values
** is to copy them to an array of a known type and use the array values.
*/
unsafe extern "C" fn l_randomizePivot() -> lua_uint {
    let mut c: clock_t = clock();
    let mut t: time_t = time(0 as *mut time_t);
    let mut buff: [lua_uint; 4] = [0; 4];
    let mut i: lua_uint = 0;
    let mut rnd: lua_uint = 0i32 as lua_uint;
    memcpy(
        buff.as_mut_ptr() as *mut lua_void,
        &mut c as *mut clock_t as *const lua_void,
        (::std::mem::size_of::<clock_t>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_uint>() as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_uint>() as lua_ulong),
    );
    memcpy(
        buff.as_mut_ptr().offset(
            (::std::mem::size_of::<clock_t>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_uint>() as lua_ulong) as isize,
        ) as *mut lua_void,
        &mut t as *mut time_t as *const lua_void,
        (::std::mem::size_of::<time_t>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_uint>() as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_uint>() as lua_ulong),
    );
    i = 0i32 as lua_uint;
    while (i as lua_ulong)
        < (::std::mem::size_of::<[lua_uint; 4]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_uint>() as lua_ulong)
    {
        rnd = rnd.wrapping_add(buff[i as usize]);
        i = i.wrapping_add(1)
    }
    return rnd;
}
/*
** Does the partition: Pivot P is at the top of the stack.
** precondition: a[lo] <= P == a[up-1] <= a[up],
** so it only needs to do the partition from lo + 1 to up - 2.
** Pos-condition: a[lo .. i - 1] <= a[i] == P <= a[i + 1 .. up]
** returns 'i'.
*/
unsafe extern "C" fn partition(mut L: *mut lua_State, mut lo: IdxT, mut up: IdxT) -> IdxT {
    /* will be incremented before first use */
    let mut i: IdxT = lo;
    /* will be decremented before first use */
    let mut j: IdxT = up.wrapping_sub(1i32 as lua_uint);
    /* loop invariant: a[lo .. i] <= P <= a[j .. up] */
    loop {
        /* next loop: repeat ++i while a[i] < P */
        loop {
            i = i.wrapping_add(1);
            lua_geti(L, 1i32, i as lua_Integer);
            if !(0 != sort_comp(L, -1i32, -2i32)) {
                break;
            }
            /* a[i] < P  but a[up - 1] == P  ?? */
            if i == up.wrapping_sub(1i32 as lua_uint) {
                luaL_error!(L, s!(b"invalid order function for sorting\x00"));
            }
            /* remove a[i] */
            lua_settop(L, -1i32 - 1i32);
        }
        /* after the loop, a[i] >= P and a[lo .. i - 1] < P */
        /* next loop: repeat --j while P < a[j] */
        loop {
            j = j.wrapping_sub(1);
            lua_geti(L, 1i32, j as lua_Integer);
            if !(0 != sort_comp(L, -3i32, -1i32)) {
                break;
            }
            /* j < i  but  a[j] > P ?? */
            if j < i {
                luaL_error!(L, s!(b"invalid order function for sorting\x00"));
            }
            /* remove a[j] */
            lua_settop(L, -1i32 - 1i32);
        }
        /* after the loop, a[j] <= P and a[j + 1 .. up] >= P */
        if j < i {
            /* no elements out of place? */
            /* a[lo .. i - 1] <= P <= a[j + 1 .. i .. up] */
            /* pop a[j] */
            lua_settop(L, -1i32 - 1i32);
            /* swap pivot (a[up - 1]) with a[i] to satisfy pos-condition */
            set2(L, up.wrapping_sub(1i32 as lua_uint), i);
            return i;
        } else {
            /* otherwise, swap a[i] - a[j] to restore invariant and repeat */
            set2(L, i, j);
        }
    }
}
/* } */
/* arrays larger than 'RANLIMIT' may use randomized pivots */
unsafe extern "C" fn set2(mut L: *mut lua_State, mut i: IdxT, mut j: IdxT) -> () {
    lua_seti(L, 1i32, i as lua_Integer);
    lua_seti(L, 1i32, j as lua_Integer);
}
/*
** Return true iff value at stack index 'a' is less than the value at
** index 'b' (according to the order of the sort).
*/
unsafe extern "C" fn sort_comp(mut L: *mut lua_State, mut a: lua_int, mut b: lua_int) -> lua_int {
    /* no function? */
    if lua_type(L, 2i32) == 0i32 {
        /* a < b */
        return lua_compare(L, a, b, 1i32);
    } else {
        /* function */
        let mut res: lua_int = 0;
        /* push function */
        lua_pushvalue(L, 2i32);
        /* -1 to compensate function */
        lua_pushvalue(L, a - 1i32);
        /* -2 to compensate function and 'a' */
        lua_pushvalue(L, b - 2i32);
        /* call function */
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        /* get result */
        res = lua_toboolean(L, -1i32);
        /* pop result */
        lua_settop(L, -1i32 - 1i32);
        return res;
    };
}
/*
** Choose an element in the middle (2nd-3th quarters) of [lo,up]
** "randomized" by 'rnd'
*/
unsafe extern "C" fn choosePivot(mut lo: IdxT, mut up: IdxT, mut rnd: lua_uint) -> IdxT {
    /* range/4 */
    let mut r4: IdxT = up.wrapping_sub(lo).wrapping_div(4i32 as lua_uint);
    let mut p: IdxT = rnd
        .wrapping_rem(r4.wrapping_mul(2i32 as lua_uint))
        .wrapping_add(lo.wrapping_add(r4));
    return p;
}
/*
** Copy elements (1[f], ..., 1[e]) into (tt[t], tt[t+1], ...). Whenever
** possible, copy in increasing order, which is better for rehashing.
** "possible" means destination after original range, or smaller
** than origin, or copying to another table.
*/
unsafe extern "C" fn tmove(mut L: *mut lua_State) -> lua_int {
    let mut f: lua_Integer = luaL_checkinteger(L, 2i32);
    let mut e: lua_Integer = luaL_checkinteger(L, 3i32);
    let mut t: lua_Integer = luaL_checkinteger(L, 4i32);
    /* destination table */
    let mut tt: lua_int = if !(lua_type(L, 5i32) <= 0i32) {
        5i32
    } else {
        1i32
    };
    checktab(L, 1i32, 1i32);
    checktab(L, tt, 2i32);
    if e >= f {
        /* otherwise, nothing to move */
        let mut n: lua_Integer = 0;
        let mut i: lua_Integer = 0;
        (f > 0i32 as lua_longlong
            || e < 9223372036854775807i64 + f
            || 0 != luaL_argerror(L, 3i32, s!(b"too many elements to move\x00")))
            as lua_int;
        /* number of elements to move */
        n = e - f + 1i32 as lua_longlong;
        (t <= 9223372036854775807i64 - n + 1i32 as lua_longlong
            || 0 != luaL_argerror(L, 4i32, s!(b"destination wrap around\x00"))) as lua_int;
        if t > e || t <= f || tt != 1i32 && 0 == lua_compare(L, 1i32, tt, 0i32) {
            i = 0i32 as lua_Integer;
            while i < n {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i += 1
            }
        } else {
            i = n - 1i32 as lua_longlong;
            while i >= 0i32 as lua_longlong {
                lua_geti(L, 1i32, f + i);
                lua_seti(L, tt, t + i);
                i -= 1
            }
        }
    }
    /* return destination table */
    lua_pushvalue(L, tt);
    return 1i32;
}
unsafe extern "C" fn tremove(mut L: *mut lua_State) -> lua_int {
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut size: lua_Integer = luaL_len(L, 1i32);
    let mut pos: lua_Integer = luaL_optinteger(L, 2i32, size);
    /* validate 'pos' if given */
    if pos != size {
        (1i32 as lua_longlong <= pos && pos <= size + 1i32 as lua_longlong
            || 0 != luaL_argerror(L, 1i32, s!(b"position out of bounds\x00"))) as lua_int;
    }
    /* result = t[pos] */
    lua_geti(L, 1i32, pos);
    while pos < size {
        lua_geti(L, 1i32, pos + 1i32 as lua_longlong);
        /* t[pos] = t[pos + 1] */
        lua_seti(L, 1i32, pos);
        pos += 1
    }
    lua_pushnil(L);
    /* t[pos] = nil */
    lua_seti(L, 1i32, pos);
    return 1i32;
}
unsafe extern "C" fn unpack(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_Unsigned = 0;
    let mut i: lua_Integer = luaL_optinteger(L, 2i32, 1i32 as lua_Integer);
    let mut e: lua_Integer = if lua_type(L, 3i32) <= 0i32 {
        luaL_len(L, 1i32)
    } else {
        luaL_checkinteger(L, 3i32)
    };
    if i > e {
        /* empty range */
        return 0i32;
    } else {
        /* number of elements minus 1 (avoid overflows) */
        n = (e as lua_Unsigned).wrapping_sub(i as lua_ulonglong);
        if n >= 2147483647i32 as lua_uint as lua_ulonglong || {
            n = n.wrapping_add(1);
            0 == lua_checkstack(L, n as lua_int)
        } {
            return luaL_error!(L, s!(b"too many results to unpack\x00"));
        } else {
            while i < e {
                /* push arg[i..e - 1] (to avoid overflows) */
                lua_geti(L, 1i32, i);
                i += 1
            }
            /* push last element */
            lua_geti(L, 1i32, e);
            return n as lua_int;
        }
    };
}
/*
** {======================================================
** Pack/unpack
** =======================================================
*/
unsafe extern "C" fn pack(mut L: *mut lua_State) -> lua_int {
    let mut i: lua_int = 0;
    /* number of elements to pack */
    let mut n: lua_int = lua_gettop(L);
    /* create result table */
    lua_createtable(L, n, 1i32);
    /* put it at index 1 */
    lua_rotate(L, 1i32, 1i32);
    /* assign elements */
    i = n;
    while i >= 1i32 {
        lua_seti(L, 1i32, i as lua_Integer);
        i -= 1
    }
    lua_pushinteger(L, n as lua_Integer);
    /* t.n = number of elements */
    lua_setfield(L, 1i32, s!(b"n\x00"));
    /* return table */
    return 1i32;
}
unsafe extern "C" fn tinsert(mut L: *mut lua_State) -> lua_int {
    /* first empty element */
    checktab(L, 1i32, 1i32 | 2i32 | 4i32);
    let mut e: lua_Integer = luaL_len(L, 1i32) + 1i32 as lua_longlong;
    /* where to insert new element */
    let mut pos: lua_Integer = 0;
    match lua_gettop(L) {
        2 => {
            /* called with only 2 arguments */
            /* insert new element at the end */
            pos = e
        }
        3 => {
            let mut i: lua_Integer = 0;
            /* 2nd argument is the position */
            pos = luaL_checkinteger(L, 2i32);
            (1i32 as lua_longlong <= pos && pos <= e
                || 0 != luaL_argerror(L, 2i32, s!(b"position out of bounds\x00")))
                as lua_int;
            i = e;
            while i > pos {
                /* move up elements */
                lua_geti(L, 1i32, i - 1i32 as lua_longlong);
                /* t[i] = t[i - 1] */
                lua_seti(L, 1i32, i);
                i -= 1
            }
        }
        _ => return luaL_error!(L, s!(b"wrong number of arguments to \'insert\'\x00")),
    }
    /* t[pos] = v */
    lua_seti(L, 1i32, pos);
    return 0i32;
}
unsafe extern "C" fn tconcat(mut L: *mut lua_State) -> lua_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    checktab(L, 1i32, 1i32 | 4i32);
    let mut last: lua_Integer = luaL_len(L, 1i32);
    let mut lsep: size_t = 0;
    let mut sep: *const lua_char = luaL_optlstring(L, 2i32, s!(b"\x00"), &mut lsep);
    let mut i: lua_Integer = luaL_optinteger(L, 3i32, 1i32 as lua_Integer);
    last = luaL_optinteger(L, 4i32, last);
    luaL_buffinit(L, &mut b);
    while i < last {
        addfield(L, &mut b, i);
        luaL_addlstring(&mut b, sep, lsep);
        i += 1
    }
    /* add last value (if interval was not empty) */
    if i == last {
        addfield(L, &mut b, i);
    }
    luaL_pushresult(&mut b);
    return 1i32;
}
unsafe extern "C" fn addfield(
    mut L: *mut lua_State,
    mut b: *mut luaL_Buffer,
    mut i: lua_Integer,
) -> () {
    lua_geti(L, 1i32, i);
    if 0 == lua_isstring(L, -1i32) {
        luaL_error!(
            L,
            s!(b"invalid value (%s) at index %d in table for \'concat\'\x00"),
            lua_typename(L, lua_type(L, -1i32)),
            i,
        );
    }
    luaL_addvalue(b);
}
