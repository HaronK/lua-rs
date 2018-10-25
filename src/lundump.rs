use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn memcmp(_: *const lua_void, _: *const lua_void, _: lua_ulong) -> lua_int;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const lua_char, ...) -> *const lua_char;

    #[no_mangle]
    fn luaM_toobig(L: *mut lua_State) -> !;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut lua_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut lua_void;
    #[no_mangle]
    fn luaZ_read(z: *mut ZIO, b: *mut lua_void, n: size_t) -> size_t;
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: lua_int) -> !;
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State, nelems: lua_int) -> *mut LClosure;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const lua_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_createlngstrobj(L: *mut lua_State, l: size_t) -> *mut TString;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub L: *mut lua_State,
    pub Z: *mut ZIO,
    pub name: *const lua_char,
}

#[no_mangle]
pub unsafe extern "C" fn luaU_undump(
    mut L: *mut lua_State,
    mut Z: *mut ZIO,
    mut name: *const lua_char,
) -> *mut LClosure {
    let mut S: LoadState = LoadState {
        L: 0 as *mut lua_State,
        Z: 0 as *mut ZIO,
        name: 0 as *const lua_char,
    };
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    if *name as lua_int == '@' as i32 || *name as lua_int == '=' as i32 {
        S.name = name.offset(1isize)
    } else if *name as lua_int
        == (*::std::mem::transmute::<&[u8; 5], &[lua_char; 5]>(b"\x1bLua\x00"))[0usize] as lua_int
    {
        S.name = s!(b"binary string\x00")
    } else {
        S.name = name
    }
    S.L = L;
    S.Z = Z;
    checkHeader(&mut S);
    cl = luaF_newLclosure(L, LoadByte(&mut S) as lua_int);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut LClosure = cl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    LoadFunction(&mut S, (*cl).p, 0 as *mut TString);
    return cl;
}
unsafe extern "C" fn LoadFunction(
    mut S: *mut LoadState,
    mut f: *mut Proto,
    mut psource: *mut TString,
) -> () {
    (*f).source = LoadString(S);
    /* no source in dump? */
    if (*f).source.is_null() {
        /* reuse parent's source */
        (*f).source = psource
    }
    (*f).linedefined = LoadInt(S);
    (*f).lastlinedefined = LoadInt(S);
    (*f).numparams = LoadByte(S);
    (*f).is_vararg = LoadByte(S);
    (*f).maxstacksize = LoadByte(S);
    LoadCode(S, f);
    LoadConstants(S, f);
    LoadUpvalues(S, f);
    LoadProtos(S, f);
    LoadDebug(S, f);
}
unsafe extern "C" fn LoadDebug(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<lua_int>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).lineinfo = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
    ) as *mut lua_int;
    (*f).sizelineinfo = n;
    LoadBlock(
        S,
        (*f).lineinfo as *mut lua_void,
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
    );
    n = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<LocVar>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).locvars = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as lua_ulong),
    ) as *mut LocVar;
    (*f).sizelocvars = n;
    i = 0i32;
    while i < n {
        let ref mut fresh0 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh0 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh1 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh1 = LoadString(S);
        (*(*f).locvars.offset(i as isize)).startpc = LoadInt(S);
        (*(*f).locvars.offset(i as isize)).endpc = LoadInt(S);
        i += 1
    }
    n = LoadInt(S);
    i = 0i32;
    while i < n {
        let ref mut fresh2 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh2 = LoadString(S);
        i += 1
    }
}
unsafe extern "C" fn LoadString(mut S: *mut LoadState) -> *mut TString {
    let mut size: size_t = LoadByte(S) as size_t;
    if size == 0xffi32 as lua_ulong {
        LoadBlock(
            S,
            &mut size as *mut size_t as *mut lua_void,
            (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as lua_ulong),
        );
    }
    if size == 0i32 as lua_ulong {
        return 0 as *mut TString;
    } else {
        size = size.wrapping_sub(1);
        if size <= 40i32 as lua_ulong {
            /* short string? */
            let mut buff: [lua_char; 40] = [0; 40];
            LoadBlock(
                S,
                buff.as_mut_ptr() as *mut lua_void,
                size.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            return luaS_newlstr((*S).L, buff.as_mut_ptr(), size);
        } else {
            /* long string */
            let mut ts: *mut TString = luaS_createlngstrobj((*S).L, size);
            /* load directly in final place */
            LoadBlock(
                S,
                (ts as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *mut lua_void,
                size.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            return ts;
        }
    };
}
unsafe extern "C" fn LoadByte(mut S: *mut LoadState) -> lu_byte {
    let mut x: lu_byte = 0;
    LoadBlock(
        S,
        &mut x as *mut lu_byte as *mut lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lu_byte>() as lua_ulong),
    );
    return x;
}
/*
** All high-level loads go through LoadVector; you can change it to
** adapt to the endianness of the input
*/
unsafe extern "C" fn LoadBlock(
    mut S: *mut LoadState,
    mut b: *mut lua_void,
    mut size: size_t,
) -> () {
    if luaZ_read((*S).Z, b, size) != 0i32 as lua_ulong {
        error(S, s!(b"truncated\x00"));
    } else {
        return;
    };
}
unsafe extern "C" fn error(mut S: *mut LoadState, mut _why: *const lua_char) -> ! {
    luaO_pushfstring!((*S).L, s!(b"%s: %s precompiled chunk\x00"), (*S).name, why,);
    luaD_throw((*S).L, 3i32);
}
unsafe extern "C" fn LoadInt(mut S: *mut LoadState) -> lua_int {
    let mut x: lua_int = 0;
    LoadBlock(
        S,
        &mut x as *mut lua_int as *mut lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadProtos(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<*mut Proto>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).p = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as lua_ulong),
    ) as *mut *mut Proto;
    (*f).sizep = n;
    i = 0i32;
    while i < n {
        let ref mut fresh3 = *(*f).p.offset(i as isize);
        *fresh3 = 0 as *mut Proto;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh4 = *(*f).p.offset(i as isize);
        *fresh4 = luaF_newproto((*S).L);
        LoadFunction(S, *(*f).p.offset(i as isize), (*f).source);
        i += 1
    }
}
unsafe extern "C" fn LoadUpvalues(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Upvaldesc>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).upvalues = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc>() as lua_ulong),
    ) as *mut Upvaldesc;
    (*f).sizeupvalues = n;
    i = 0i32;
    while i < n {
        let ref mut fresh5 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh5 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        (*(*f).upvalues.offset(i as isize)).instack = LoadByte(S);
        (*(*f).upvalues.offset(i as isize)).idx = LoadByte(S);
        i += 1
    }
}
unsafe extern "C" fn LoadConstants(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut io: *mut TValue = 0 as *mut TValue;
    let mut io_0: *mut TValue = 0 as *mut TValue;
    let mut io_1: *mut TValue = 0 as *mut TValue;
    let mut i: lua_int = 0;
    let mut n: lua_int = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).k = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
    ) as *mut TValue;
    (*f).sizek = n;
    i = 0i32;
    while i < n {
        (*(*f).k.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    i = 0i32;
    while i < n {
        let mut o: *mut TValue = &mut *(*f).k.offset(i as isize) as *mut TValue;
        let mut t: lua_int = LoadByte(S) as lua_int;
        match t {
            0 => (*o).tt_ = 0i32,
            1 => {
                io = o;
                (*io).value_.b = LoadByte(S) as lua_int;
                (*io).tt_ = 1i32
            }
            3 => {
                io_0 = o;
                (*io_0).value_.n = LoadNumber(S);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32
            }
            19 => {
                io_1 = o;
                (*io_1).value_.i = LoadInteger(S);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32
            }
            4 | 20 => {
                let mut io_2: *mut TValue = o;
                let mut x_: *mut TString = LoadString(S);
                (*io_2).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                (*io_2).tt_ = (*x_).tt as lua_int | 1i32 << 6i32
            }
            _ => {}
        }
        i += 1
    }
}
unsafe extern "C" fn LoadInteger(mut S: *mut LoadState) -> lua_Integer {
    let mut x: lua_Integer = 0;
    LoadBlock(
        S,
        &mut x as *mut lua_Integer as *mut lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadNumber(mut S: *mut LoadState) -> lua_Number {
    let mut x: lua_Number = 0.;
    LoadBlock(
        S,
        &mut x as *mut lua_Number as *mut lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_Number>() as lua_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadCode(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut n: lua_int = LoadInt(S);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (n as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Instruction>() as lua_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).code = luaM_realloc_(
        (*S).L,
        0 as *mut lua_void,
        (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
    ) as *mut Instruction;
    (*f).sizecode = n;
    LoadBlock(
        S,
        (*f).code as *mut lua_void,
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
    );
}
unsafe extern "C" fn checkHeader(mut S: *mut LoadState) -> () {
    /* 1st char already checked */
    checkliteral(S, (s!(b"\x1bLua\x00")).offset(1isize), s!(b"not a\x00"));
    if LoadByte(S) as lua_int
        != ((*::std::mem::transmute::<&[u8; 2], &[lua_char; 2]>(b"5\x00"))[0usize] as lua_int
            - '0' as i32)
            * 16i32
            + ((*::std::mem::transmute::<&[u8; 2], &[lua_char; 2]>(b"3\x00"))[0usize] as lua_int
                - '0' as i32)
    {
        error(S, s!(b"version mismatch in\x00"));
    } else if LoadByte(S) as lua_int != 0i32 {
        error(S, s!(b"format mismatch in\x00"));
    } else {
        checkliteral(S, s!(b"\x19\x93\r\n\x1a\n\x00"), s!(b"corrupted\x00"));
        fchecksize(
            S,
            ::std::mem::size_of::<lua_int>() as lua_ulong,
            s!(b"int\x00"),
        );
        fchecksize(
            S,
            ::std::mem::size_of::<size_t>() as lua_ulong,
            s!(b"size_t\x00"),
        );
        fchecksize(
            S,
            ::std::mem::size_of::<Instruction>() as lua_ulong,
            s!(b"Instruction\x00"),
        );
        fchecksize(
            S,
            ::std::mem::size_of::<lua_Integer>() as lua_ulong,
            s!(b"lua_Integer\x00"),
        );
        fchecksize(
            S,
            ::std::mem::size_of::<lua_Number>() as lua_ulong,
            s!(b"lua_Number\x00"),
        );
        if LoadInteger(S) != 0x5678i32 as lua_longlong {
            error(S, s!(b"endianness mismatch in\x00"));
        } else if LoadNumber(S) != 370.5f64 {
            error(S, s!(b"float format mismatch in\x00"));
        } else {
            return;
        }
    };
}
unsafe extern "C" fn fchecksize(
    mut S: *mut LoadState,
    mut size: size_t,
    mut tname: *const lua_char,
) -> () {
    if LoadByte(S) as lua_ulong != size {
        error(
            S,
            luaO_pushfstring!((*S).L, s!(b"%s size mismatch in\x00"), tname,),
        );
    } else {
        return;
    };
}
unsafe extern "C" fn checkliteral(
    mut S: *mut LoadState,
    mut s: *const lua_char,
    mut msg: *const lua_char,
) -> () {
    /* larger than both */
    let mut buff: [lua_char; 12] = [0; 12];
    let mut len: size_t = strlen(s);
    LoadBlock(
        S,
        buff.as_mut_ptr() as *mut lua_void,
        len.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    if memcmp(
        s as *const lua_void,
        buff.as_mut_ptr() as *const lua_void,
        len,
    ) != 0i32
    {
        error(S, msg);
    } else {
        return;
    };
}
