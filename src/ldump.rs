use lobject::*;
use lua::*;
use types::prelude::*;

extern "C" {
    /*
     ** $Id: lstate.h,v 2.133.1.1 2017/04/19 17:39:34 roberto Exp $
     ** Global State
     ** See Copyright Notice in lua.h
     */
    /*
    
    ** Some notes about garbage-collected objects: All objects in Lua must
    ** be kept somehow accessible until being freed, so all objects always
    ** belong to one (and only one) of these lists, using field 'next' of
    ** the 'CommonHeader' for the link:
    **
    ** 'allgc': all objects not marked for finalization;
    ** 'finobj': all objects marked for finalization;
    ** 'tobefnz': all objects ready to be finalized;
    ** 'fixedgc': all objects that are not to be collected (currently
    ** only small strings, such as reserved words).
    **
    ** Moreover, there is another set of lists that control gray objects.
    ** These lists are linked by fields 'gclist'. (All objects that
    ** can become gray have such a field. The field is not the same
    ** in all objects, but it always has this name.)  Any gray object
    ** must belong to one of these lists, and all objects in these lists
    ** must be gray:
    **
    ** 'gray': regular gray objects, still waiting to be visited.
    ** 'grayagain': objects that must be revisited at the atomic phase.
    **   That includes
    **   - black objects got in a write barrier;
    **   - all kinds of weak tables during propagation phase;
    **   - all threads.
    ** 'weak': tables with weak values to be cleared;
    ** 'ephemeron': ephemeron tables with white->white entries;
    ** 'allweak': tables with weak keys and/or weak values to be cleared.
    ** The last three lists are used only during the atomic phase.
    
    */
}

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

/*
** {==============================================================
** some useful macros
** ===============================================================
*/
/* }============================================================== */
/*
** {==============================================================
** compatibility macros for unsigned conversions
** ===============================================================
*/
/* }============================================================== */
/*
** {======================================================================
** Debug API
** =======================================================================
*/
/*
** Event codes
*/
/*
** Event masks
*/

/* internal assertions for in-house debugging */
/*
** assertion for checking API calls
*/
/* macro to avoid warnings about unused variables */
/* type casts (a macro highlights casts in the code) */
/* cast a signed lua_Integer to lua_Unsigned */
/*
** cast a lua_Unsigned to a signed lua_Integer; this cast is
** not strict ISO C, but two-complement architectures should
** work fine.
*/
/*
** non-return type
*/
/*
** maximum depth for nested C calls and syntactical nested non-terminals
** in a program. (Value must fit in an unsigned short int.)
*/

/* macro defining a nil value */
/* raw type tag of a TValue */
/* tag with no variants (bits 0-3) */
/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
/* type tag of a TValue with no variants (bits 0-3) */
/* Macros to test type */
/* Macros to access values */
/* a dead value may get the 'gc' field, but cannot access its contents */
/* Macros for internal tests */
/* Macros to set values */
/*
** different types of assignments, according to destination
*/
/* from stack to (same) stack */
/* to stack (not from same stack) */
/* from table to same table */
/* to new object */
/* to table (define it as an expression to be used in macros) */
/*
** {======================================================
** types and prototypes
** =======================================================
*/

/*
** $Id: lobject.h,v 2.117.1.1 2017/04/19 17:39:34 roberto Exp $
** Type definitions for Lua objects
** See Copyright Notice in lua.h
*/
/*
** Extra tags for non-values
*/
/* function prototypes */
/* removed keys in tables */
/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
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
/* Lua closure */
/* light C function */
/* C closure */
/* Variant tags for strings */
/* short strings */
/* long strings */
/* Variant tags for numbers */
/* float numbers */
/* integer numbers */
/* Bit mark for collectable types */
/* mark a tag as collectable */
/*
** Common type for all collectable objects
*/

/*
** Bits in CallInfo status
*/
/* original value of 'allowhook' */
/* call is running a Lua function */
/* call is running a debug hook */
/* call is running on a fresh invocation
of luaV_execute */
/* call is a yieldable protected call */
/* call was tail called */
/* last hook called yielded */
/* using __lt for __le */
/* call is running a finalizer */
/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */

/*
** $Id: llimits.h,v 1.141.1.1 2017/04/19 17:20:42 roberto Exp $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/

/* maximum value for size_t */
/* maximum size visible for Lua (must be representable in a lua_Integer */
/* maximum value of an int */
/*
** conversion of pointer to unsigned integer:
** this is for hashing only; there is no problem if the integer
** cannot hold the whole pointer value
*/

/*
** $Id: ldump.c,v 2.37.1.1 2017/04/19 17:20:42 roberto Exp $
** save precompiled Lua chunks
** See Copyright Notice in lua.h
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DumpState {
    pub L: *mut lua_State,
    pub writer: lua_Writer,
    pub data: *mut lua_void,
    pub strip: lua_int,
    pub status: lua_int,
}
/* dump one chunk; from ldump.c */
#[no_mangle]
pub unsafe extern "C" fn luaU_dump(
    mut L: *mut lua_State,
    mut f: *const Proto,
    mut w: lua_Writer,
    mut data: *mut lua_void,
    mut strip: lua_int,
) -> lua_int {
    let mut D: DumpState = DumpState {
        L: L,
        writer: w,
        data: data,
        strip: strip,
        status: 0,
    };
    DumpHeader(&mut D);
    DumpByte((*f).sizeupvalues, &mut D);
    DumpFunction(f, 0 as *mut TString, &mut D);
    return D.status;
}
unsafe extern "C" fn DumpFunction(
    mut f: *const Proto,
    mut psource: *mut TString,
    mut D: *mut DumpState,
) -> () {
    if 0 != (*D).strip || (*f).source == psource {
        /* no debug info or same source as its parent */
        DumpString(0 as *const TString, D);
    } else {
        DumpString((*f).source, D);
    }
    DumpInt((*f).linedefined, D);
    DumpInt((*f).lastlinedefined, D);
    DumpByte((*f).numparams as lua_int, D);
    DumpByte((*f).is_vararg as lua_int, D);
    DumpByte((*f).maxstacksize as lua_int, D);
    DumpCode(f, D);
    DumpConstants(f, D);
    DumpUpvalues(f, D);
    DumpProtos(f, D);
    DumpDebug(f, D);
}
unsafe extern "C" fn DumpDebug(mut f: *const Proto, mut D: *mut DumpState) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = 0;
    n = if 0 != (*D).strip {
        0i32
    } else {
        (*f).sizelineinfo
    };
    DumpInt(n, D);
    DumpBlock(
        (*f).lineinfo as *const lua_void,
        (n as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
        D,
    );
    n = if 0 != (*D).strip {
        0i32
    } else {
        (*f).sizelocvars
    };
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpString((*(*f).locvars.offset(i as isize)).varname, D);
        DumpInt((*(*f).locvars.offset(i as isize)).startpc, D);
        DumpInt((*(*f).locvars.offset(i as isize)).endpc, D);
        i += 1
    }
    n = if 0 != (*D).strip {
        0i32
    } else {
        (*f).sizeupvalues
    };
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpString((*(*f).upvalues.offset(i as isize)).name, D);
        i += 1
    }
}
unsafe extern "C" fn DumpString(mut s: *const TString, mut D: *mut DumpState) -> () {
    if s.is_null() {
        DumpByte(0i32, D);
    } else {
        /* include trailing '\0' */
        let mut size: size_t = if (*s).tt as lua_int == 4i32 | 0i32 << 4i32 {
            (*s).shrlen as lua_ulong
        } else {
            (*s).u.lnglen
        }
        .wrapping_add(1i32 as lua_ulong);
        let mut str: *const lua_char =
            (s as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
        if size < 0xffi32 as lua_ulong {
            DumpByte(size as lua_int, D);
        } else {
            DumpByte(0xffi32, D);
            DumpBlock(
                &mut size as *mut size_t as *const lua_void,
                (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as lua_ulong),
                D,
            );
        }
        /* no need to save '\0' */
        DumpBlock(
            str as *const lua_void,
            size.wrapping_sub(1i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            D,
        );
    };
}
/*
** All high-level dumps go through DumpVector; you can change it to
** change the endianness of the result
*/
unsafe extern "C" fn DumpBlock(
    mut b: *const lua_void,
    mut size: size_t,
    mut D: *mut DumpState,
) -> () {
    if (*D).status == 0i32 && size > 0i32 as lua_ulong {
        (*D).status = (*D).writer.expect("non-null function pointer")((*D).L, b, size, (*D).data)
    };
}
unsafe extern "C" fn DumpByte(mut y: lua_int, mut D: *mut DumpState) -> () {
    let mut x: lu_byte = y as lu_byte;
    DumpBlock(
        &mut x as *mut lu_byte as *const lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lu_byte>() as lua_ulong),
        D,
    );
}
unsafe extern "C" fn DumpInt(mut x: lua_int, mut D: *mut DumpState) -> () {
    DumpBlock(
        &mut x as *mut lua_int as *const lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
        D,
    );
}
unsafe extern "C" fn DumpProtos(mut f: *const Proto, mut D: *mut DumpState) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = (*f).sizep;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpFunction(*(*f).p.offset(i as isize), (*f).source, D);
        i += 1
    }
}
unsafe extern "C" fn DumpUpvalues(mut f: *const Proto, mut D: *mut DumpState) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = (*f).sizeupvalues;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        DumpByte((*(*f).upvalues.offset(i as isize)).instack as lua_int, D);
        DumpByte((*(*f).upvalues.offset(i as isize)).idx as lua_int, D);
        i += 1
    }
}
unsafe extern "C" fn DumpConstants(mut f: *const Proto, mut D: *mut DumpState) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = (*f).sizek;
    DumpInt(n, D);
    i = 0i32;
    while i < n {
        let mut o: *const TValue = &mut *(*f).k.offset(i as isize) as *mut TValue;
        DumpByte((*o).tt_ & 0x3fi32, D);
        match (*o).tt_ & 0x3fi32 {
            LUA_TNIL => {}
            LUA_TBOOLEAN => {
                DumpByte((*o).value_.b, D);
            }
            LUA_TNUMFLT => {
                DumpNumber((*o).value_.n, D);
            }
            LUA_TNUMINT => {
                DumpInteger((*o).value_.i, D);
            }
            LUA_TSHRSTR | LUA_TLNGSTR => {
                DumpString(&mut (*((*o).value_.gc as *mut GCUnion)).ts, D);
            }
            _ => {}
        }
        i += 1
    }
}
unsafe extern "C" fn DumpInteger(mut x: lua_Integer, mut D: *mut DumpState) -> () {
    DumpBlock(
        &mut x as *mut lua_Integer as *const lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong),
        D,
    );
}
unsafe extern "C" fn DumpNumber(mut x: lua_Number, mut D: *mut DumpState) -> () {
    DumpBlock(
        &mut x as *mut lua_Number as *const lua_void,
        (1i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_Number>() as lua_ulong),
        D,
    );
}
unsafe extern "C" fn DumpCode(mut f: *const Proto, mut D: *mut DumpState) -> () {
    DumpInt((*f).sizecode, D);
    DumpBlock(
        (*f).code as *const lua_void,
        ((*f).sizecode as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
        D,
    );
}
unsafe extern "C" fn DumpHeader(mut D: *mut DumpState) -> () {
    DumpBlock(
        s!(b"\x1bLua\x00") as *const lua_void,
        (::std::mem::size_of::<[lua_char; 5]>() as lua_ulong)
            .wrapping_sub(::std::mem::size_of::<lua_char>() as lua_ulong),
        D,
    );
    DumpByte(
        ((*::std::mem::transmute::<&[u8; 2], &[lua_char; 2]>(b"5\x00"))[0usize] as lua_int
            - '0' as i32)
            * 16i32
            + ((*::std::mem::transmute::<&[u8; 2], &[lua_char; 2]>(b"3\x00"))[0usize] as lua_int
                - '0' as i32),
        D,
    );
    DumpByte(0i32, D);
    DumpBlock(
        s!(b"\x19\x93\r\n\x1a\n\x00") as *const lua_void,
        (::std::mem::size_of::<[lua_char; 7]>() as lua_ulong)
            .wrapping_sub(::std::mem::size_of::<lua_char>() as lua_ulong),
        D,
    );
    DumpByte(::std::mem::size_of::<lua_int>() as lua_ulong as lua_int, D);
    DumpByte(::std::mem::size_of::<size_t>() as lua_ulong as lua_int, D);
    DumpByte(
        ::std::mem::size_of::<Instruction>() as lua_ulong as lua_int,
        D,
    );
    DumpByte(
        ::std::mem::size_of::<lua_Integer>() as lua_ulong as lua_int,
        D,
    );
    DumpByte(
        ::std::mem::size_of::<lua_Number>() as lua_ulong as lua_int,
        D,
    );
    DumpInteger(0x5678i32 as lua_Integer, D);
    DumpNumber(370.5f64, D);
}
