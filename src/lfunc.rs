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

    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State, tt: lua_int, sz: size_t) -> *mut GCObject;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut lua_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut lua_void;
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal) -> ();
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
** $Id: llimits.h,v 1.141.1.1 2017/04/19 17:20:42 roberto Exp $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
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

#[no_mangle]
pub unsafe extern "C" fn luaF_newproto(mut L: *mut lua_State) -> *mut Proto {
    let mut o: *mut GCObject = luaC_newobj(L, 9i32, ::std::mem::size_of::<Proto>() as lua_ulong);
    let mut f: *mut Proto = &mut (*(o as *mut GCUnion)).p;
    (*f).k = 0 as *mut TValue;
    (*f).sizek = 0i32;
    (*f).p = 0 as *mut *mut Proto;
    (*f).sizep = 0i32;
    (*f).code = 0 as *mut Instruction;
    (*f).cache = 0 as *mut LClosure;
    (*f).sizecode = 0i32;
    (*f).lineinfo = 0 as *mut lua_int;
    (*f).sizelineinfo = 0i32;
    (*f).upvalues = 0 as *mut Upvaldesc;
    (*f).sizeupvalues = 0i32;
    (*f).numparams = 0i32 as lu_byte;
    (*f).is_vararg = 0i32 as lu_byte;
    (*f).maxstacksize = 0i32 as lu_byte;
    (*f).locvars = 0 as *mut LocVar;
    (*f).sizelocvars = 0i32;
    (*f).linedefined = 0i32;
    (*f).lastlinedefined = 0i32;
    (*f).source = 0 as *mut TString;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newCclosure(mut L: *mut lua_State, mut n: lua_int) -> *mut CClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | 2i32 << 4i32,
        (::std::mem::size_of::<CClosure>() as lua_ulong as lua_int
            + (::std::mem::size_of::<TValue>() as lua_ulong).wrapping_mul((n - 1i32) as lua_ulong)
                as lua_int) as size_t,
    );
    let mut c: *mut CClosure = &mut (*(o as *mut GCUnion)).cl.c;
    (*c).nupvalues = n as lu_byte;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_newLclosure(mut L: *mut lua_State, mut n: lua_int) -> *mut LClosure {
    let mut o: *mut GCObject = luaC_newobj(
        L,
        6i32 | 0i32 << 4i32,
        (::std::mem::size_of::<LClosure>() as lua_ulong as lua_int
            + (::std::mem::size_of::<*mut TValue>() as lua_ulong)
                .wrapping_mul((n - 1i32) as lua_ulong) as lua_int) as size_t,
    );
    let mut c: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
    (*c).p = 0 as *mut Proto;
    (*c).nupvalues = n as lu_byte;
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(0 != fresh0) {
            break;
        }
        (*c).upvals[n as usize] = 0 as *mut UpVal
    }
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_initupvals(mut L: *mut lua_State, mut cl: *mut LClosure) -> () {
    let mut i: lua_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as lua_int {
        let mut uv: *mut UpVal = luaM_realloc_(
            L,
            0 as *mut lua_void,
            0i32 as size_t,
            ::std::mem::size_of::<UpVal>() as lua_ulong,
        ) as *mut UpVal;
        (*uv).refcount = 1i32 as lu_mem;
        /* make it closed */
        (*uv).v = &mut (*uv).u.value;
        (*(*uv).v).tt_ = 0i32;
        (*cl).upvals[i as usize] = uv;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_findupval(mut L: *mut lua_State, mut level: StkId) -> *mut UpVal {
    let mut pp: *mut *mut UpVal = &mut (*L).openupval;
    let mut p: *mut UpVal = 0 as *mut UpVal;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    while !(*pp).is_null() && {
        p = *pp;
        (*p).v >= level
    } {
        /* found a corresponding upvalue? */
        if (*p).v == level {
            /* return it */
            return p;
        } else {
            pp = &mut (*p).u.open.next
        }
    }
    /* not found: create a new upvalue */
    uv = luaM_realloc_(
        L,
        0 as *mut lua_void,
        0i32 as size_t,
        ::std::mem::size_of::<UpVal>() as lua_ulong,
    ) as *mut UpVal;
    (*uv).refcount = 0i32 as lu_mem;
    /* link it to list of open upvalues */
    (*uv).u.open.next = *pp;
    (*uv).u.open.touched = 1i32;
    *pp = uv;
    /* current value lives in the stack */
    (*uv).v = level;
    if !((*L).twups != L) {
        /* thread not in list of threads with upvalues? */
        /* link it to the list */
        (*L).twups = (*(*L).l_G).twups;
        (*(*L).l_G).twups = L
    }
    return uv;
}
#[no_mangle]
pub unsafe extern "C" fn luaF_close(mut L: *mut lua_State, mut level: StkId) -> () {
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    while !(*L).openupval.is_null() && {
        uv = (*L).openupval;
        (*uv).v >= level
    } {
        /* remove from 'open' list */
        (*L).openupval = (*uv).u.open.next;
        /* no references? */
        if (*uv).refcount == 0i32 as lua_ulong {
            /* free upvalue */
            luaM_realloc_(
                L,
                uv as *mut lua_void,
                ::std::mem::size_of::<UpVal>() as lua_ulong,
                0i32 as size_t,
            );
        } else {
            /* move value to upvalue slot */
            let mut io1: *mut TValue = &mut (*uv).u.value;
            *io1 = *(*uv).v;
            /* now current value lives here */
            (*uv).v = &mut (*uv).u.value;
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaF_freeproto(mut L: *mut lua_State, mut f: *mut Proto) -> () {
    luaM_realloc_(
        L,
        (*f).code as *mut lua_void,
        ((*f).sizecode as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).p as *mut lua_void,
        ((*f).sizep as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).k as *mut lua_void,
        ((*f).sizek as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).lineinfo as *mut lua_void,
        ((*f).sizelineinfo as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).locvars as *mut lua_void,
        ((*f).sizelocvars as lua_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        (*f).upvalues as *mut lua_void,
        ((*f).sizeupvalues as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<Upvaldesc>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        f as *mut lua_void,
        ::std::mem::size_of::<Proto>() as lua_ulong,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaF_getlocalname(
    mut f: *const Proto,
    mut local_number: lua_int,
    mut pc: lua_int,
) -> *const lua_char {
    let mut i: lua_int = 0;
    i = 0i32;
    while i < (*f).sizelocvars && (*(*f).locvars.offset(i as isize)).startpc <= pc {
        if pc < (*(*f).locvars.offset(i as isize)).endpc {
            /* is variable active? */
            local_number -= 1;
            if local_number == 0i32 {
                return ((*(*f).locvars.offset(i as isize)).varname as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
            }
        }
        i += 1
    }
    /* not found */
    return 0 as *const lua_char;
}
