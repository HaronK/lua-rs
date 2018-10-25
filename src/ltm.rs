use types::prelude::*;

extern "C" {
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const lua_char) -> *mut TString;
    #[no_mangle]
    fn luaH_getshortstr(t: *mut Table, key: *mut TString) -> *const TValue;

    #[no_mangle]
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaG_opinterror(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        msg: *const lua_char,
    ) -> !;
    #[no_mangle]
    fn luaG_tointerror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> lua_int;
    #[no_mangle]
    fn luaG_concaterror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
}

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

#[no_mangle]
pub static mut luaT_typenames_: [*const lua_char; 11] = unsafe {
    [
        s!(b"no value\x00"),
        s!(b"nil\x00"),
        s!(b"boolean\x00"),
        udatatypename.as_ptr(),
        s!(b"number\x00"),
        s!(b"string\x00"),
        s!(b"table\x00"),
        s!(b"function\x00"),
        udatatypename.as_ptr(),
        s!(b"thread\x00"),
        s!(b"proto\x00"),
    ]
};
/*
** $Id: ltm.c,v 2.38.1.1 2017/04/19 17:39:34 roberto Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
static mut udatatypename: [lua_char; 9] = [117, 115, 101, 114, 100, 97, 116, 97, 0];
#[no_mangle]
pub unsafe extern "C" fn luaT_objtypename(
    mut L: *mut lua_State,
    mut o: *const TValue,
) -> *const lua_char {
    let mut mt: *mut Table = 0 as *mut Table;
    if (*o).tt_ == 5i32 | 1i32 << 6i32 && {
        mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable;
        !mt.is_null()
    } || (*o).tt_ == 7i32 | 1i32 << 6i32 && {
        mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable;
        !mt.is_null()
    } {
        let mut name: *const TValue = luaH_getshortstr(mt, luaS_new(L, s!(b"__name\x00")));
        /* is '__name' a string? */
        if (*name).tt_ & 0xfi32 == 4i32 {
            /* use it as type name */
            return (&mut (*((*name).value_.gc as *mut GCUnion)).ts as *mut TString
                as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
        }
    }
    /* else use standard type name */
    return luaT_typenames_[(((*o).tt_ & 0xfi32) + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettm(
    mut events: *mut Table,
    mut event: TMS,
    mut ename: *mut TString,
) -> *const TValue {
    let mut tm: *const TValue = luaH_getshortstr(events, ename);
    if (*tm).tt_ == 0i32 {
        /* no tag method? */
        /* cache this fact */
        (*events).flags = ((*events).flags as lua_int
            | (1u32 << event as lua_uint) as lu_byte as lua_int)
            as lu_byte;
        return 0 as *const TValue;
    } else {
        return tm;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_gettmbyobj(
    mut L: *mut lua_State,
    mut o: *const TValue,
    mut event: TMS,
) -> *const TValue {
    let mut mt: *mut Table = 0 as *mut Table;
    match (*o).tt_ & 0xfi32 {
        5 => mt = (*((*o).value_.gc as *mut GCUnion)).h.metatable,
        7 => mt = (*((*o).value_.gc as *mut GCUnion)).u.metatable,
        _ => mt = (*(*L).l_G).mt[((*o).tt_ & 0xfi32) as usize],
    }
    return if !mt.is_null() {
        luaH_getshortstr(mt, (*(*L).l_G).tmname[event as usize])
    } else {
        &luaO_nilobject_
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_init(mut L: *mut lua_State) -> () {
    /* ORDER TM */
    static mut luaT_eventname: [*const lua_char; 24] = [
        s!(b"__index\x00"),
        s!(b"__newindex\x00"),
        s!(b"__gc\x00"),
        s!(b"__mode\x00"),
        s!(b"__len\x00"),
        s!(b"__eq\x00"),
        s!(b"__add\x00"),
        s!(b"__sub\x00"),
        s!(b"__mul\x00"),
        s!(b"__mod\x00"),
        s!(b"__pow\x00"),
        s!(b"__div\x00"),
        s!(b"__idiv\x00"),
        s!(b"__band\x00"),
        s!(b"__bor\x00"),
        s!(b"__bxor\x00"),
        s!(b"__shl\x00"),
        s!(b"__shr\x00"),
        s!(b"__unm\x00"),
        s!(b"__bnot\x00"),
        s!(b"__lt\x00"),
        s!(b"__le\x00"),
        s!(b"__concat\x00"),
        s!(b"__call\x00"),
    ];
    let mut i: lua_int = 0;
    i = 0i32;
    while i < TM_N as lua_int {
        (*(*L).l_G).tmname[i as usize] = luaS_new(L, luaT_eventname[i as usize]);
        /* never collect these names */
        luaC_fix(
            L,
            &mut (*((*(*L).l_G).tmname[i as usize] as *mut GCUnion)).gc,
        );
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callTM(
    mut L: *mut lua_State,
    mut f: *const TValue,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut p3: *mut TValue,
    mut hasres: lua_int,
) -> () {
    let mut io1_2: *mut TValue = 0 as *mut TValue;
    let mut result: ptrdiff_t =
        (p3 as *mut lua_char).wrapping_offset_from((*L).stack as *mut lua_char) as lua_long;
    let mut func: StkId = (*L).top;
    /* push function (assume EXTRA_STACK) */
    let mut io1: *mut TValue = func;
    *io1 = *f;
    /* 1st argument */
    let mut io1_0: *mut TValue = func.offset(1isize);
    *io1_0 = *p1;
    /* 2nd argument */
    let mut io1_1: *mut TValue = func.offset(2isize);
    *io1_1 = *p2;
    (*L).top = (*L).top.offset(3isize);
    /* no result? 'p3' is third argument */
    if 0 == hasres {
        /* 3rd argument */
        let fresh0 = (*L).top;
        (*L).top = (*L).top.offset(1);
        io1_2 = fresh0;
        *io1_2 = *p3
    }
    /* metamethod may yield only when called from Lua code */
    if 0 != (*(*L).ci).callstatus as lua_int & 1i32 << 1i32 {
        luaD_call(L, func, hasres);
    } else {
        luaD_callnoyield(L, func, hasres);
    }
    if 0 != hasres {
        /* if has result, move it to its place */
        p3 = ((*L).stack as *mut lua_char).offset(result as isize) as *mut TValue;
        let mut io1_3: *mut TValue = p3;
        (*L).top = (*L).top.offset(-1isize);
        *io1_3 = *(*L).top
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callbinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> lua_int {
    /* try first operand */
    let mut tm: *const TValue = luaT_gettmbyobj(L, p1, event);
    if (*tm).tt_ == 0i32 {
        /* try second operand */
        tm = luaT_gettmbyobj(L, p2, event)
    }
    if (*tm).tt_ == 0i32 {
        return 0i32;
    } else {
        luaT_callTM(L, tm, p1, p2, res, 1i32);
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_trybinTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: StkId,
    mut event: TMS,
) -> () {
    if 0 == luaT_callbinTM(L, p1, p2, res, event) {
        match event as lua_uint {
            22 => {
                luaG_concaterror(L, p1, p2);
            }
            13 | 14 | 15 | 16 | 17 | 19 => {
                let mut dummy: lua_Number = 0.;
                if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    dummy = (*p1).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p1, &mut dummy)
                } && 0
                    != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                        dummy = (*p2).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(p2, &mut dummy)
                    } {
                    luaG_tointerror(L, p1, p2);
                } else {
                    luaG_opinterror(L, p1, p2, s!(b"perform bitwise operation on\x00"));
                }
            }
            _ => {
                luaG_opinterror(L, p1, p2, s!(b"perform arithmetic on\x00"));
            }
        }
    } else {
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaT_callorderTM(
    mut L: *mut lua_State,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut event: TMS,
) -> lua_int {
    if 0 == luaT_callbinTM(L, p1, p2, (*L).top, event) {
        /* no metamethod */
        return -1i32;
    } else {
        return !((*(*L).top).tt_ == 0i32 || (*(*L).top).tt_ == 1i32 && (*(*L).top).value_.b == 0i32)
            as lua_int;
    };
}
