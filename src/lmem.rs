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
    fn luaD_throw(L: *mut lua_State, errcode: lua_int) -> !;
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State, isemergency: lua_int) -> ();
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

/*
** $Id: lmem.h,v 1.43.1.1 2017/04/19 17:20:42 roberto Exp $
** Interface to Memory Manager
** See Copyright Notice in lua.h
*/
/*
** This macro reallocs a vector 'b' from 'on' to 'n' elements, where
** each element has size 'e'. In case of arithmetic overflow of the
** product 'n'*'e', it raises an error (calling 'luaM_toobig'). Because
** 'e' is always constant, it avoids the runtime division MAX_SIZET/(e).
**
** (The macro is somewhat complex to avoid warnings:  The 'sizeof'
** comparison avoids a runtime comparison when overflow cannot occur.
** The compiler should be able to optimize the real test by itself, but
** when it does it, it may give a warning about "comparison is always
** false due to limited range of data type"; the +1 tricks the compiler,
** avoiding this warning but also this optimization.)
*/
/*
** Arrays of chars do not need any test
*/
#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State) -> ! {
    luaG_runerror!(L, s!(b"memory allocation error: block too big\x00"),);
}
/* not to be called directly */
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(
    mut L: *mut lua_State,
    mut block: *mut lua_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut lua_void {
    let mut newblock: *mut lua_void = 0 as *mut lua_void;
    let mut g: *mut global_State = (*L).l_G;
    let mut realosize: size_t = if !block.is_null() {
        osize
    } else {
        0i32 as lua_ulong
    };
    newblock = (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize);
    if newblock.is_null() && nsize > 0i32 as lua_ulong {
        /* cannot fail when shrinking a block */
        if !(*g).version.is_null() {
            /* is state fully built? */
            /* try to free some memory... */
            luaC_fullgc(L, 1i32);
            /* try again */
            newblock =
                (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize)
        }
        if newblock.is_null() {
            luaD_throw(L, 4i32);
        }
    }
    (*g).GCdebt = ((*g).GCdebt as lua_ulong)
        .wrapping_add(nsize)
        .wrapping_sub(realosize) as l_mem;
    return newblock;
}
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block: *mut lua_void,
    mut size: *mut lua_int,
    mut size_elems: size_t,
    mut limit: lua_int,
    mut what: *const lua_char,
) -> *mut lua_void {
    let mut newblock: *mut lua_void = 0 as *mut lua_void;
    let mut newsize: lua_int = 0;
    if *size >= limit / 2i32 {
        /* cannot double it? */
        /* cannot grow even a little? */
        if *size >= limit {
            luaG_runerror!(L, s!(b"too many %s (limit is %d)\x00"), what, limit,);
        } else {
            newsize = limit
        }
    } else {
        newsize = *size * 2i32;
        if newsize < 4i32 {
            /* minimum size */
            newsize = 4i32
        }
    }
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(size_elems)
    {
        luaM_toobig(L);
    } else {
    };
    newblock = luaM_realloc_(
        L,
        block,
        (*size as lua_ulong).wrapping_mul(size_elems),
        (newsize as lua_ulong).wrapping_mul(size_elems),
    );
    /* update only when everything else is OK */
    *size = newsize;
    return newblock;
}
