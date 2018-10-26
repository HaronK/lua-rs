use super::prelude::*;

/*
** Execute a protected call.
*/
/* data to 'f_call' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: lua_int,
}

/* value at a non-valid index */
//#define NONVALIDVALUE		cast(TValue *, luaO_nilobject)
macro_rules! NONVALIDVALUE {
    () => {
        &luaO_nilobject_ as *const TValue as *mut TValue
    };
}

/* corresponding test */
//#define isvalid(o)	((o) != luaO_nilobject)
macro_rules! isvalid {
    ($o:expr) => {
        $o != &luaO_nilobject_ as *const TValue as StkId
    };
}

/* test for pseudo index */
//#define ispseudo(i)		((i) <= LUA_REGISTRYINDEX)
macro_rules! ispseudo {
    ($i:expr) => {
        $i <= LUA_REGISTRYINDEX
    };
}

/* test for upvalue */
//#define isupvalue(i)		((i) < LUA_REGISTRYINDEX)
macro_rules! isupvalue {
    ($i:expr) => {
        $i < LUA_REGISTRYINDEX
    };
}

/* test for valid but not pseudo index */
//#define isstackindex(i, o)	(isvalid(o) && !ispseudo(i))
macro_rules! isstackindex {
    ($i:expr, $o:expr) => {
        isvalid!(o) && !ispseudo!(i)
    };
}

// #define api_checkvalidindex(l,o)  api_check(l, isvalid(o), "invalid index")

// #define api_checkstackindex(l, i, o)  \
// 	api_check(l, isstackindex(i, o), "index not in the stack")

#[no_mangle]
pub static mut lua_ident: [lua_char; 129] = [
    36, 76, 117, 97, 86, 101, 114, 115, 105, 111, 110, 58, 32, 76, 117, 97, 32, 53, 46, 51, 46, 53,
    32, 32, 67, 111, 112, 121, 114, 105, 103, 104, 116, 32, 40, 67, 41, 32, 49, 57, 57, 52, 45, 50,
    48, 49, 56, 32, 76, 117, 97, 46, 111, 114, 103, 44, 32, 80, 85, 67, 45, 82, 105, 111, 32, 36,
    36, 76, 117, 97, 65, 117, 116, 104, 111, 114, 115, 58, 32, 82, 46, 32, 73, 101, 114, 117, 115,
    97, 108, 105, 109, 115, 99, 104, 121, 44, 32, 76, 46, 32, 72, 46, 32, 100, 101, 32, 70, 105,
    103, 117, 101, 105, 114, 101, 100, 111, 44, 32, 87, 46, 32, 67, 101, 108, 101, 115, 32, 36, 0,
];
#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    old = (*(*L).l_G).panic;
    (*(*L).l_G).panic = panicf;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State) -> *const lua_Number {
    static mut version: lua_Number = { 503i32 as lua_Number };
    if L.is_null() {
        return &version;
    } else {
        return (*(*L).l_G).version;
    };
}
/*
** basic stack manipulation
*/
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    return if idx > 0i32 || ispseudo!(idx) {
        idx
    } else {
        (*L).top.wrapping_offset_from((*(*L).ci).func) as lua_long as lua_int + idx
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> lua_int {
    return (*L)
        .top
        .wrapping_offset_from((*(*L).ci).func.offset(1isize)) as lua_long as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut func: StkId = (*(*L).ci).func;
    if idx >= 0i32 {
        while (*L).top < func.offset(1isize).offset(idx as isize) {
            let fresh0 = (*L).top;
            (*L).top = (*L).top.offset(1);
            (*fresh0).tt_ = 0i32
        }
        (*L).top = func.offset(1isize).offset(idx as isize)
    } else {
        (*L).top = (*L).top.offset((idx + 1i32) as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut io1: *mut TValue = (*L).top;
    *io1 = *index2addr(L, idx);
    (*L).top = (*L).top.offset(1isize);
}

unsafe extern "C" fn index2addr(mut L: *mut lua_State, mut idx: lua_int) -> *mut TValue {
    let mut ci: *mut CallInfo = (*L).ci;
    if idx > 0i32 {
        let mut o: *mut TValue = (*ci).func.offset(idx as isize);
        // api_check(L, idx <= ci->top - (ci->func + 1), "unacceptable index");
        if o >= (*L).top {
            return NONVALIDVALUE!();
        } else {
            return o;
        }
    } else if !ispseudo!(idx) {
        /* negative index */
        // api_check(L, idx != 0 && -idx <= L->top - (ci->func + 1), "invalid index");
        return (*L).top.offset(idx as isize);
    } else if idx == LUA_REGISTRYINDEX {
        return &mut (*(*L).l_G).l_registry;
    } else {
        /* upvalues */
        idx = LUA_REGISTRYINDEX - idx;
        // api_check(L, idx <= MAXUPVAL + 1, "upvalue index too large");
        /* light C function? */
        if ttislcf!(*(*ci).func) {
            /* it has no upvalues */
            return NONVALIDVALUE!();
        } else {
            let mut func: *mut CClosure = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.c;
            return if idx <= (*func).nupvalues as lua_int {
                &mut (*func).upvalue[(idx - 1i32) as usize] as *mut TValue
            } else {
                NONVALIDVALUE!()
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rotate(mut L: *mut lua_State, mut idx: lua_int, mut n: lua_int) -> () {
    let mut p: StkId = 0 as *mut TValue;
    let mut t: StkId = 0 as *mut TValue;
    let mut m: StkId = 0 as *mut TValue;
    /* end of stack segment being rotated */
    t = (*L).top.offset(-1isize);
    /* start of segment */
    p = index2addr(L, idx);
    /* end of prefix */
    m = if n >= 0i32 {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-1isize)
    };
    /* reverse the prefix with length 'n' */
    reverse(L, p, m);
    /* reverse the suffix */
    reverse(L, m.offset(1isize), t);
    /* reverse the entire segment */
    reverse(L, p, t);
}
/*
** Reverse the stack segment from 'from' to 'to'
** (auxiliary to 'lua_rotate')
*/
unsafe extern "C" fn reverse(mut _L: *mut lua_State, mut from: StkId, mut to: StkId) -> () {
    while from < to {
        let mut temp: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io1: *mut TValue = &mut temp;
        *io1 = *from;
        let mut io1_0: *mut TValue = from;
        *io1_0 = *to;
        let mut io1_1: *mut TValue = to;
        *io1_1 = temp;
        from = from.offset(1isize);
        to = to.offset(-1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_copy(
    mut L: *mut lua_State,
    mut fromidx: lua_int,
    mut toidx: lua_int,
) -> () {
    let mut fr: *mut TValue = 0 as *mut TValue;
    let mut to: *mut TValue = 0 as *mut TValue;
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    let mut io1: *mut TValue = to;
    *io1 = *fr;
    /* function upvalue? */
    if isupvalue!(toidx) {
        if 0 != (*fr).tt_ & 1i32 << 6i32
            && 0 != (*((*(*(*L).ci).func).value_.gc as *mut GCUnion))
                .cl
                .c
                .marked as lua_int
                & 1i32 << 2i32
            && 0 != (*(*fr).value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            luaC_barrier_(
                L,
                &mut (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                    as *mut GCUnion))
                    .gc,
                (*fr).value_.gc,
            );
        } else {
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(mut L: *mut lua_State, mut n: lua_int) -> lua_int {
    let mut res: lua_int = 0;
    let mut ci: *mut CallInfo = (*L).ci;
    /* stack large enough? */
    if (*L).stack_last.wrapping_offset_from((*L).top) as lua_long > n as lua_long {
        /* yes; check is OK */
        res = 1i32
    } else {
        /* no; need to grow stack */
        let mut inuse: lua_int =
            (*L).top.wrapping_offset_from((*L).stack) as lua_long as lua_int + 5i32;
        /* can grow without overflow? */
        if inuse > 1000000i32 - n {
            /* no */
            res = 0i32
        } else {
            res =
                (luaD_rawrunprotected(L, Some(growstack), &mut n as *mut lua_int as *mut lua_void)
                    == LUA_OK) as lua_int
        }
    }
    if 0 != res && (*ci).top < (*L).top.offset(n as isize) {
        /* adjust frame top */
        (*ci).top = (*L).top.offset(n as isize)
    }
    return res;
}
/*
** to be called by 'lua_checkstack' in protected mode, to grow stack
** capturing memory errors
*/
unsafe extern "C" fn growstack(mut L: *mut lua_State, mut ud: *mut lua_void) -> () {
    let mut size: lua_int = *(ud as *mut lua_int);
    luaD_growstack(L, size);
}
#[no_mangle]
pub unsafe extern "C" fn lua_xmove(
    mut from: *mut lua_State,
    mut to: *mut lua_State,
    mut n: lua_int,
) -> () {
    let mut i: lua_int = 0;
    if from == to {
        return;
    } else {
        (*from).top = (*from).top.offset(-(n as isize));
        i = 0i32;
        while i < n {
            let mut io1: *mut TValue = (*to).top;
            *io1 = *(*from).top.offset(i as isize);
            /* stack already checked by previous 'api_check' */
            (*to).top = (*to).top.offset(1isize);
            i += 1
        }
        return;
    };
}
/*
** access functions (stack -> C)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    return if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ & 0xfi32 == 4i32 || (*o).tt_ & 0xfi32 == 3i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: StkId = index2addr(L, idx);
    return (ttislcf!(*o) || (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 3i32 | 1i32 << 4i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ == 7i32 | 1i32 << 6i32 || (*o).tt_ == 2i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_type(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: StkId = index2addr(L, idx);
    return if isvalid!(o) {
        (*o).tt_ & 0xfi32
    } else {
        -1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(mut _L: *mut lua_State, mut t: lua_int) -> *const lua_char {
    return luaT_typenames_[(t + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lua_tonumberx(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut pisnum: *mut lua_int,
) -> lua_Number {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: lua_int = if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if 0 == isnum {
        /* call to 'tonumber' may change 'n' even if it fails */
        n = 0i32 as lua_Number
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tointegerx(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut pisnum: *mut lua_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: lua_int = if (*o).tt_ == 3i32 | 1i32 << 4i32 {
        res = (*o).value_.i;
        1i32
    } else {
        luaV_tointeger(o, &mut res, 0i32)
    };
    if 0 == isnum {
        /* call to 'tointeger' may change 'n' even if it fails */
        res = 0i32 as lua_Integer
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_toboolean(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: *const TValue = index2addr(L, idx);
    return !((*o).tt_ == 0i32 || (*o).tt_ == 1i32 && (*o).value_.b == 0i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut len: *mut size_t,
) -> *const lua_char {
    let mut o: StkId = index2addr(L, idx);
    if !((*o).tt_ & 0xfi32 == 4i32) {
        if !((*o).tt_ & 0xfi32 == 3i32) {
            /* not convertible? */
            if !len.is_null() {
                *len = 0i32 as size_t
            }
            return 0 as *const lua_char;
        } else {
            /* 'luaO_tostring' may create a new string */
            luaO_tostring(L, o);
            if (*(*L).l_G).GCdebt > 0i32 as lua_long {
                luaC_step(L);
            }
            /* previous call may reallocate the stack */
            o = index2addr(L, idx)
        }
    }
    if !len.is_null() {
        *len = if (*((*o).value_.gc as *mut GCUnion)).ts.tt as lua_int == 4i32 | 0i32 << 4i32 {
            (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as lua_ulong
        } else {
            (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen
        }
    }
    return (&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString as *mut lua_char)
        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(mut L: *mut lua_State, mut idx: lua_int) -> size_t {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        4 => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t,
        20 => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen,
        7 => return (*((*o).value_.gc as *mut GCUnion)).u.len,
        5 => return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h) as size_t,
        _ => return 0i32 as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(mut L: *mut lua_State, mut idx: lua_int) -> lua_CFunction {
    let mut o: StkId = index2addr(L, idx);
    if ttislcf!(*o) {
        return (*o).value_.f;
    } else if (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f;
    } else {
        return None;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(mut L: *mut lua_State, mut idx: lua_int) -> *mut lua_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0xfi32 {
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut lua_char)
                .offset(::std::mem::size_of::<UUdata>() as lua_ulong as isize)
                as *mut lua_void
        }
        2 => return (*o).value_.p,
        _ => return 0 as *mut lua_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tothread(mut L: *mut lua_State, mut idx: lua_int) -> *mut lua_State {
    let mut o: StkId = index2addr(L, idx);
    return if !((*o).tt_ == 8i32 | 1i32 << 6i32) {
        0 as *mut lua_State
    } else {
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(mut L: *mut lua_State, mut idx: lua_int) -> *const lua_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        5 => return &mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table as *const lua_void,
        6 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.l as *mut LClosure as *const lua_void
        }
        38 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.c as *mut CClosure as *const lua_void
        }
        22 => return ::std::mem::transmute::<lua_CFunction, size_t>((*o).value_.f) as *mut lua_void,
        8 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).th as *mut lua_State as *const lua_void
        }
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut lua_char)
                .offset(::std::mem::size_of::<UUdata>() as lua_ulong as isize)
                as *const lua_void
        }
        2 => return (*o).value_.p,
        _ => return 0 as *const lua_void,
    };
}
/*
** Comparison and arithmetic functions
*/
/* ORDER TM, ORDER OP */
#[no_mangle]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State, mut op: lua_int) -> () {
    if !(op != 12i32 && op != 13i32) {
        /* all other operations expect two operands */
        /* for unary operations, add fake 2nd operand */
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        (*L).top = (*L).top.offset(1isize)
    }
    /* first operand at top - 2, second at top - 1; result go to top - 2 */
    luaO_arith(
        L,
        op,
        (*L).top.offset(-2isize) as *const TValue,
        (*L).top.offset(-1isize) as *const TValue,
        (*L).top.offset(-2isize),
    );
    /* remove second operand */
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State,
    mut index1: lua_int,
    mut index2: lua_int,
) -> lua_int {
    let mut o1: StkId = index2addr(L, index1);
    let mut o2: StkId = index2addr(L, index2);
    return if isvalid!(o1) && isvalid!(o2) {
        luaV_equalobj(
            0 as *mut lua_State,
            o1 as *const TValue,
            o2 as *const TValue,
        )
    } else {
        0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State,
    mut index1: lua_int,
    mut index2: lua_int,
    mut op: lua_int,
) -> lua_int {
    let mut current_block: u64;
    let mut o1: StkId = 0 as *mut TValue;
    let mut o2: StkId = 0 as *mut TValue;
    let mut i: lua_int = 0i32;
    /* may call tag method */
    o1 = index2addr(L, index1);
    o2 = index2addr(L, index2);
    if isvalid!(o1) && isvalid!(o2) {
        match op {
            0 => {
                current_block = 9754527956383385389;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            1 => {
                current_block = 17152279037612840043;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            2 => {
                current_block = 8492507194373517126;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            _ => {}
        }
    }
    return i;
}
/*
** push functions (C -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) -> () {
    (*(*L).top).tt_ = 0i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State, mut n: lua_Number) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.n = n;
    (*io).tt_ = 3i32 | 0i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State, mut n: lua_Integer) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.i = n;
    (*io).tt_ = 3i32 | 1i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State,
    mut s: *const lua_char,
    mut len: size_t,
) -> *const lua_char {
    let mut ts: *mut TString = 0 as *mut TString;
    ts = if len == 0i32 as lua_ulong {
        luaS_new(L, s!(b"\x00"))
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = ts;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    }
    return (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const lua_char,
) -> *const lua_char {
    if s.is_null() {
        (*(*L).top).tt_ = 0i32
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = ts;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
        /* internal copy's address */
        s = (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
    }
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const lua_char,
    mut argp: *mut __va_list_tag,
) -> *const lua_char {
    let mut ret: *const lua_char = 0 as *const lua_char;
    ret = luaO_pushvfstring!(L, fmt, argp);
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(
    mut L: *mut lua_State,
    mut fn_0: lua_CFunction,
    mut n: lua_int,
) -> () {
    if n == 0i32 {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.f = fn_0;
        (*io).tt_ = 6i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut cl: *mut CClosure = 0 as *mut CClosure;
        cl = luaF_newCclosure(L, n);
        (*cl).f = fn_0;
        (*L).top = (*L).top.offset(-(n as isize));
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(0 != fresh1) {
                break;
            }
            let mut io1: *mut TValue = &mut (*cl).upvalue[n as usize] as *mut TValue;
            *io1 = *(*L).top.offset(n as isize)
        }
        /* does not need barrier because closure is white */
        let mut io_0: *mut TValue = (*L).top;
        let mut x_: *mut CClosure = cl;
        (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0).tt_ = 6i32 | 2i32 << 4i32 | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        if (*(*L).l_G).GCdebt > 0i32 as lua_long {
            luaC_step(L);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State, mut b: lua_int) -> () {
    /* ensure that true is 1 */
    let mut io: *mut TValue = (*L).top;
    (*io).value_.b = (b != 0i32) as lua_int;
    (*io).tt_ = 1i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(mut L: *mut lua_State, mut p: *mut lua_void) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.p = p;
    (*io).tt_ = 2i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State) -> lua_int {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State = L;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    return ((*(*L).l_G).mainthread == L) as lua_int;
}
/*
** get functions (Lua -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State,
    mut name: *const lua_char,
) -> lua_int {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    return auxgetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** get functions (Lua -> stack)
*/
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const lua_char,
) -> lua_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ == 0i32) as lua_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishget(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-1isize) as *const TValue,
        );
        !((*slot).tt_ == 0i32) as lua_int
    } {
        let mut io1: *mut TValue = (*L).top.offset(-1isize);
        *io1 = *slot
    } else {
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getfield(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut k: *const lua_char,
) -> lua_int {
    return auxgetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_geti(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut n: lua_Integer,
) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        !((*slot).tt_ == 0i32) as lua_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = (*L).top.offset(-1isize);
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize) as *const TValue,
    );
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut n: lua_Integer,
) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut p: *const lua_void,
) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    t = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut lua_void;
    (*io).tt_ = 2i32;
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State,
    mut narray: lua_int,
    mut nrec: lua_int,
) -> () {
    let mut t: *mut Table = 0 as *mut Table;
    t = luaH_new(L);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Table = t;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if narray > 0i32 || nrec > 0i32 {
        luaH_resize(L, t, narray as lua_uint, nrec as lua_uint);
    }
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(mut L: *mut lua_State, mut size: size_t) -> *mut lua_void {
    let mut u: *mut Udata = 0 as *mut Udata;
    u = luaS_newudata(L, size);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Udata = u;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    }
    return (u as *mut lua_char).offset(::std::mem::size_of::<UUdata>() as lua_ulong as isize)
        as *mut lua_void;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(mut L: *mut lua_State, mut objindex: lua_int) -> lua_int {
    let mut obj: *const TValue = 0 as *const TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    let mut res: lua_int = 0i32;
    obj = index2addr(L, objindex);
    match (*obj).tt_ & 0xfi32 {
        5 => mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable,
        7 => mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable,
        _ => mt = (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize],
    }
    if !mt.is_null() {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table = mt;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5i32 | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        res = 1i32
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = (*L).top;
    let mut iu: *const Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*io).value_ = (*iu).user_;
    (*io).tt_ = (*iu).ttuv_ as lua_int;
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
/*
** set functions (stack -> Lua)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(mut L: *mut lua_State, mut name: *const lua_char) -> () {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** set functions (stack -> Lua)
*/
/*
** t[k] = value at the top of the stack (where 'k' is a string)
*/
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const lua_char,
) -> () {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        /* push 'str' (to make it a TValue) */
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishset(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 == if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-2isize) as *const TValue,
        );
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-2isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    /* pop index and value */
    (*L).top = (*L).top.offset(-2isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setfield(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut k: *const lua_char,
) -> () {
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_seti(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut n: lua_Integer,
) -> () {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    slot = luaH_set(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-2isize) as *const TValue,
    );
    *slot = *(*L).top.offset(-1isize);
    (*((*o).value_.gc as *mut GCUnion)).h.flags = 0i32 as lu_byte;
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-2isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut n: lua_Integer,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    luaH_setint(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        n,
        (*L).top.offset(-1isize),
    );
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State,
    mut idx: lua_int,
    mut p: *const lua_void,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut lua_void;
    (*io).tt_ = 2i32;
    slot = luaH_set(L, &mut (*((*o).value_.gc as *mut GCUnion)).h, &mut k);
    *slot = *(*L).top.offset(-1isize);
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as lua_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(mut L: *mut lua_State, mut objindex: lua_int) -> lua_int {
    let mut obj: *mut TValue = 0 as *mut TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    obj = index2addr(L, objindex);
    if (*(*L).top.offset(-1isize)).tt_ == 0i32 {
        mt = 0 as *mut Table
    } else {
        mt = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion)).h
    }
    match (*obj).tt_ & 0xfi32 {
        5 => {
            let ref mut fresh2 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh2 = mt;
            if !mt.is_null() {
                if 0 != (*(*obj).value_.gc).marked as lua_int & 1i32 << 2i32
                    && 0 != (*mt).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            let ref mut fresh3 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh3 = mt;
            if !mt.is_null() {
                if 0 != (*((*obj).value_.gc as *mut GCUnion)).u.marked as lua_int & 1i32 << 2i32
                    && 0 != (*mt).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata
                            as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize] = mt,
    }
    (*L).top = (*L).top.offset(-1isize);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *const TValue = (*L).top.offset(-1isize) as *const TValue;
    let mut iu: *mut Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*iu).user_ = (*io).value_;
    (*iu).ttuv_ = (*io).tt_ as lu_byte;
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*(*o).value_.gc).marked as lua_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrier_(
            L,
            &mut (*((*o).value_.gc as *mut GCUnion)).gc,
            (*(*L).top.offset(-1isize)).value_.gc,
        );
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
/*
** 'load' and 'call' functions (load and run Lua code)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    mut nargs: lua_int,
    mut nresults: lua_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> () {
    let mut func: StkId = 0 as *mut TValue;
    func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_some() && (*L).nny as lua_int == 0i32 {
        /* need to prepare continuation? */
        /* save continuation */
        (*(*L).ci).u.c.k = k;
        /* save context */
        (*(*L).ci).u.c.ctx = ctx;
        /* do the call */
        luaD_call(L, func, nresults);
    } else {
        /* no continuation or no yieldable */
        /* just do the call */
        luaD_callnoyield(L, func, nresults);
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State,
    mut nargs: lua_int,
    mut nresults: lua_int,
    mut errfunc: lua_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> lua_int {
    let mut o: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut c: CallS = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let mut status: lua_int = 0;
    let mut func: ptrdiff_t = 0;
    if errfunc == 0i32 {
        func = 0i32 as ptrdiff_t
    } else {
        o = index2addr(L, errfunc);
        func = (o as *mut lua_char).wrapping_offset_from((*L).stack as *mut lua_char) as lua_long
    }
    /* function to be called */
    c.func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_none() || (*L).nny as lua_int > 0i32 {
        /* no continuation or no yieldable? */
        /* do a 'conventional' protected call */
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(f_call),
            &mut c as *mut CallS as *mut lua_void,
            (c.func as *mut lua_char).wrapping_offset_from((*L).stack as *mut lua_char) as lua_long,
            func,
        )
    } else {
        /* prepare continuation (call is already protected by 'resume') */
        ci = (*L).ci;
        /* save continuation */
        (*ci).u.c.k = k;
        /* save context */
        (*ci).u.c.ctx = ctx;
        /* save information for error recovery */
        (*ci).extra =
            (c.func as *mut lua_char).wrapping_offset_from((*L).stack as *mut lua_char) as lua_long;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        /* save value of 'allowhook' */
        (*ci).callstatus = ((*ci).callstatus as lua_int & !(1i32 << 0i32)
            | (*L).allowhook as lua_int) as lua_ushort;
        /* function can do error recovery */
        (*ci).callstatus = ((*ci).callstatus as lua_int | 1i32 << 4i32) as lua_ushort;
        /* do the call */
        luaD_call(L, c.func, nresults);
        (*ci).callstatus = ((*ci).callstatus as lua_int & !(1i32 << 4i32)) as lua_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        /* if it is here, there were no errors */
        status = LUA_OK
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    return status;
}
unsafe extern "C" fn f_call(mut L: *mut lua_State, mut ud: *mut lua_void) -> () {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_load(
    mut L: *mut lua_State,
    mut reader: lua_Reader,
    mut data: *mut lua_void,
    mut chunkname: *const lua_char,
    mut mode: *const lua_char,
) -> lua_int {
    let mut z: ZIO = Zio {
        n: 0,
        p: 0 as *const lua_char,
        reader: None,
        data: 0 as *mut lua_void,
        L: 0 as *mut lua_State,
    };
    let mut status: lua_int = 0;
    if chunkname.is_null() {
        chunkname = s!(b"?\x00")
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == LUA_OK {
        /* no errors? */
        /* get newly created function */
        let mut f: *mut LClosure = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as lua_int >= 1i32 {
            /* does it have an upvalue? */
            /* get global table from registry */
            let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
            let mut gt: *const TValue = luaH_getint(reg, 2i32 as lua_Integer);
            /* set global table as 1st upvalue of 'f' (may be LUA_ENV) */
            let mut io1: *mut TValue = (*(*f).upvals[0usize]).v;
            *io1 = *gt;
            if 0 != (*(*(*f).upvals[0usize]).v).tt_ & 1i32 << 6i32
                && !((*(*f).upvals[0usize]).v != &mut (*(*f).upvals[0usize]).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, (*f).upvals[0usize]);
            } else {
            };
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State,
    mut writer: lua_Writer,
    mut data: *mut lua_void,
    mut strip: lua_int,
) -> lua_int {
    let mut status: lua_int = 0;
    let mut o: *mut TValue = 0 as *mut TValue;
    o = (*L).top.offset(-1isize);
    if (*o).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        status = luaU_dump(
            L,
            (*((*o).value_.gc as *mut GCUnion)).cl.l.p,
            writer,
            data,
            strip,
        )
    } else {
        status = 1i32
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_status(mut L: *mut lua_State) -> lua_int {
    return (*L).status as lua_int;
}
/*
** garbage-collection function and options
*/
#[no_mangle]
pub unsafe extern "C" fn lua_gc(
    mut L: *mut lua_State,
    mut what: lua_int,
    mut data: lua_int,
) -> lua_int {
    let mut res: lua_int = 0i32;
    let mut g: *mut global_State = 0 as *mut global_State;
    g = (*L).l_G;
    match what {
        0 => (*g).gcrunning = 0i32 as lu_byte,
        1 => {
            luaE_setdebt(g, 0i32 as l_mem);
            (*g).gcrunning = 1i32 as lu_byte
        }
        2 => {
            luaC_fullgc(L, 0i32);
        }
        3 => {
            /* GC values are expressed in Kbytes: #bytes/2^10 */
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10i32) as lua_int
        }
        4 => res = (((*g).totalbytes + (*g).GCdebt) as lu_mem & 0x3ffi32 as lua_ulong) as lua_int,
        5 => {
            /* =1 to signal that it did an actual step */
            let mut debt: l_mem = 1i32 as l_mem;
            let mut oldrunning: lu_byte = (*g).gcrunning;
            /* allow GC to run */
            (*g).gcrunning = 1i32 as lu_byte;
            if data == 0i32 {
                /* to do a "small" step */
                luaE_setdebt(
                    g,
                    -((100i32 as lua_ulong)
                        .wrapping_mul(::std::mem::size_of::<TString>() as lua_ulong)
                        as lua_int) as l_mem,
                );
                luaC_step(L);
            } else {
                /* add 'data' to total debt */
                debt = data as l_mem * 1024i32 as lua_long + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0i32 as lua_long {
                    luaC_step(L);
                }
            }
            /* restore previous state */
            (*g).gcrunning = oldrunning;
            /* end of cycle? */
            if debt > 0i32 as lua_long && (*g).gcstate as lua_int == 7i32 {
                /* signal it */
                res = 1i32
            }
        }
        6 => {
            res = (*g).gcpause;
            (*g).gcpause = data
        }
        7 => {
            res = (*g).gcstepmul;
            if data < 40i32 {
                /* avoid ridiculous low values (and 0) */
                data = 40i32
            }
            (*g).gcstepmul = data
        }
        9 => res = (*g).gcrunning as lua_int,
        _ => {
            /* invalid option */
            res = -1i32
        }
    }
    return res;
}
/*
** miscellaneous functions
*/
#[no_mangle]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State) -> lua_int {
    luaG_errormsg(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_next(mut L: *mut lua_State, mut idx: lua_int) -> lua_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut more: lua_int = 0;
    t = index2addr(L, idx);
    more = luaH_next(
        L,
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize),
    );
    if 0 != more {
        (*L).top = (*L).top.offset(1isize)
    } else {
        (*L).top = (*L).top.offset(-1isize)
    }
    return more;
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State, mut n: lua_int) -> () {
    if n >= 2i32 {
        luaV_concat(L, n);
    } else if n == 0i32 {
        /* push empty string */
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = luaS_newlstr(L, s!(b"\x00"), 0i32 as size_t);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize)
    }
    /* else n == 1; nothing to do */
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State, mut idx: lua_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_stringtonumber(
    mut L: *mut lua_State,
    mut s: *const lua_char,
) -> size_t {
    let mut sz: size_t = luaO_str2num(s, (*L).top);
    if sz != 0i32 as lua_ulong {
        (*L).top = (*L).top.offset(1isize)
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State,
    mut ud: *mut *mut lua_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud
    }
    f = (*(*L).l_G).frealloc;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State,
    mut f: lua_Alloc,
    mut ud: *mut lua_void,
) -> () {
    (*(*L).l_G).ud = ud;
    (*(*L).l_G).frealloc = f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getupvalue(
    mut L: *mut lua_State,
    mut funcindex: lua_int,
    mut n: lua_int,
) -> *const lua_char {
    let mut name: *const lua_char = 0 as *const lua_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        0 as *mut *mut CClosure,
        0 as *mut *mut UpVal,
    );
    if !name.is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *val;
        (*L).top = (*L).top.offset(1isize)
    }
    return name;
}
unsafe extern "C" fn aux_upvalue(
    mut fi: StkId,
    mut n: lua_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut CClosure,
    mut uv: *mut *mut UpVal,
) -> *const lua_char {
    match (*fi).tt_ & 0x3fi32 {
        38 => {
            /* C closure */
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !(1i32 <= n && n <= (*f).nupvalues as lua_int) {
                return 0 as *const lua_char;
            } else {
                *val = &mut (*f).upvalue[(n - 1i32) as usize] as *mut TValue;
                if !owner.is_null() {
                    *owner = f
                }
                return s!(b"\x00");
            }
        }
        6 => {
            /* Lua closure */
            let mut f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let mut name: *mut TString = 0 as *mut TString;
            let mut p: *mut Proto = (*f_0).p;
            if !(1i32 <= n && n <= (*p).sizeupvalues) {
                return 0 as *const lua_char;
            } else {
                *val = (*(*f_0).upvals[(n - 1i32) as usize]).v;
                if !uv.is_null() {
                    *uv = (*f_0).upvals[(n - 1i32) as usize]
                }
                name = (*(*p).upvalues.offset((n - 1i32) as isize)).name;
                return if name.is_null() {
                    s!(b"(*no name)\x00")
                } else {
                    (name as *mut lua_char)
                        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                };
            }
        }
        _ => {
            /* not a closure */
            return 0 as *const lua_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State,
    mut funcindex: lua_int,
    mut n: lua_int,
) -> *const lua_char {
    let mut name: *const lua_char = 0 as *const lua_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut CClosure = 0 as *mut CClosure;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut fi: StkId = 0 as *mut TValue;
    fi = index2addr(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        (*L).top = (*L).top.offset(-1isize);
        let mut io1: *mut TValue = val;
        *io1 = *(*L).top;
        if !owner.is_null() {
            if 0 != (*(*L).top).tt_ & 1i32 << 6i32
                && 0 != (*owner).marked as lua_int & 1i32 << 2i32
                && 0 != (*(*(*L).top).value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrier_(L, &mut (*(owner as *mut GCUnion)).gc, (*(*L).top).value_.gc);
            } else {
            };
        } else if !uv.is_null() {
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State,
    mut fidx: lua_int,
    mut n: lua_int,
) -> *mut lua_void {
    let mut fi: StkId = index2addr(L, fidx);
    match (*fi).tt_ & 0x3fi32 {
        6 => {
            /* lua closure */
            return *getupvalref(L, fidx, n, 0 as *mut *mut LClosure) as *mut lua_void;
        }
        38 => {
            /* C closure */
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            return &mut (*f).upvalue[(n - 1i32) as usize] as *mut TValue as *mut lua_void;
        }
        _ => return 0 as *mut lua_void,
    };
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State,
    mut fidx: lua_int,
    mut n: lua_int,
    mut pf: *mut *mut LClosure,
) -> *mut *mut UpVal {
    let mut f: *mut LClosure = 0 as *mut LClosure;
    let mut fi: StkId = index2addr(L, fidx);
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    if !pf.is_null() {
        *pf = f
    }
    /* get its upvalue pointer */
    return &mut (*f).upvals[(n - 1i32) as usize] as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvaluejoin(
    mut L: *mut lua_State,
    mut fidx1: lua_int,
    mut n1: lua_int,
    mut fidx2: lua_int,
    mut n2: lua_int,
) -> () {
    let mut f1: *mut LClosure = 0 as *mut LClosure;
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1, &mut f1);
    let mut up2: *mut *mut UpVal = getupvalref(L, fidx2, n2, 0 as *mut *mut LClosure);
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    (**up1).refcount = (**up1).refcount.wrapping_add(1);
    if (**up1).v != &mut (**up1).u.value as *mut TValue {
        (**up1).u.open.touched = 1i32
    }
    if 0 != (*(**up1).v).tt_ & 1i32 << 6i32 && !((**up1).v != &mut (**up1).u.value as *mut TValue) {
        luaC_upvalbarrier_(L, *up1);
    } else {
    };
}
