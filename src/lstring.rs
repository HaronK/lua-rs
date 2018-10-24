use types::*;

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
    /* defined in ldo.c */
    pub type lua_longjmp;
    /*
     ** Lua Upvalues
     */
    pub type UpVal;
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn memcmp(_: *const lua_void, _: *const lua_void, _: lua_ulong) -> lua_int;
    #[no_mangle]
    fn strcmp(_: *const lua_char, _: *const lua_char) -> lua_int;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    /*
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
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
    fn luaM_toobig(L: *mut lua_State) -> !;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut lua_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut lua_void;
    /*
     ** $Id: lgc.h,v 2.91.1.1 2017/04/19 17:39:34 roberto Exp $
     ** Garbage Collector
     ** See Copyright Notice in lua.h
     */
    /*
     ** Collectable objects may have one of three colors: white, which
     ** means the object is not marked; gray, which means the
     ** object is marked, but its references may be not marked; and
     ** black, which means that the object and all its references are marked.
     ** The main invariant of the garbage collector, while marking objects,
     ** is that a black object can never point to a white one. Moreover,
     ** any gray object must be in a "gray list" (gray, grayagain, weak,
     ** allweak, ephemeron) so that it can be visited again before finishing
     ** the collection cycle. These lists have no meaning when the invariant
     ** is not being enforced (e.g., sweep phase).
     */
    /* how much to allocate before next GC step */
    /* ~100 small strings */
    /*
     ** Possible states of the Garbage Collector
     */
    /*
     ** macro to tell when main invariant (white objects cannot point to black
     ** ones) must be kept. During a collection, the sweep
     ** phase may break the invariant, as objects turned white may point to
     ** still-black objects. The invariant is restored when sweep ends and
     ** all objects are white again.
     */
    /*
     ** some useful bit tricks
     */
    /* Layout for bit use in 'marked' field: */
    /* object is white (type 0) */
    /* object is white (type 1) */
    /* object is black */
    /* object has been marked for finalization */
    /* bit 7 is currently used by tests (luaL_checkmemory) */
    /* neither white nor black */
    /*
     ** Does one step of collection when debt becomes positive. 'pre'/'pos'
     ** allows some adjustments to be done only when needed. macro
     ** 'condchangemem' is used only for heavy tests (forcing a full
     ** GC cycle on every opportunity)
     */
    /* more often than not, 'pre'/'pos' are empty */
    #[no_mangle]
    fn luaC_fix(L: *mut lua_State, o: *mut GCObject) -> ();
    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State, tt: lua_int, sz: size_t) -> *mut GCObject;
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
/* thread status */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_State {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nci: lua_ushort,
    pub status: lu_byte,
    pub top: StkId,
    pub l_G: *mut global_State,
    pub ci: *mut CallInfo,
    pub oldpc: *const Instruction,
    pub stack_last: StkId,
    pub stack: StkId,
    pub openupval: *mut UpVal,
    pub gclist: *mut GCObject,
    pub twups: *mut lua_State,
    pub errorJmp: *mut lua_longjmp,
    pub base_ci: CallInfo,
    pub hook: lua_Hook,
    pub errfunc: ptrdiff_t,
    pub stacksize: lua_int,
    pub basehookcount: lua_int,
    pub hookcount: lua_int,
    pub nny: lua_ushort,
    pub nCcalls: lua_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
/* 16-bit ints */
/* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = lua_uchar;
pub type sig_atomic_t = __sig_atomic_t;
/* Functions to be called by the debugger in specific events */
pub type lua_Hook = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_Debug) -> ()>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: lua_int,
    pub name: *const lua_char,
    pub namewhat: *const lua_char,
    pub what: *const lua_char,
    pub source: *const lua_char,
    pub currentline: lua_int,
    pub linedefined: lua_int,
    pub lastlinedefined: lua_int,
    pub nups: lua_uchar,
    pub nparams: lua_uchar,
    pub isvararg: lua_char,
    pub istailcall: lua_char,
    pub short_src: [lua_char; 60],
    pub i_ci: *mut CallInfo,
}
/* private part */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallInfo {
    pub func: StkId,
    pub top: StkId,
    pub previous: *mut CallInfo,
    pub next: *mut CallInfo,
    pub u: unnamed,
    pub extra: ptrdiff_t,
    pub nresults: lua_short,
    pub callstatus: lua_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed {
    pub l: unnamed_1,
    pub c: unnamed_0,
}
/* only for C functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_0 {
    pub k: lua_KFunction,
    pub old_errfunc: ptrdiff_t,
    pub ctx: lua_KContext,
}
/* type for continuation-function contexts */
pub type lua_KContext = intptr_t;
/*
** Type for continuation functions
*/
pub type lua_KFunction =
    Option<unsafe extern "C" fn(_: *mut lua_State, _: lua_int, _: lua_KContext) -> lua_int>;
/* only for Lua functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}
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
/*
** type for virtual-machine instructions;
** must be an unsigned with (at least) 4 bytes (see details in lopcodes.h)
*/
pub type Instruction = lua_uint;
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
/* index to stack elements */
pub type StkId = *mut TValue;
pub type TValue = lua_TValue;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_TValue {
    pub value_: Value,
    pub tt_: lua_int,
}
/*
** Tagged Values. This is the basic representation of values in Lua,
** an actual value plus a tag with its type.
*/
/*
** Union of all Lua values
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union Value {
    pub gc: *mut GCObject,
    pub p: *mut lua_void,
    pub b: lua_int,
    pub f: lua_CFunction,
    pub i: lua_Integer,
    pub n: lua_Number,
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
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> lua_int>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GCObject {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
}
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
** 'global state', shared by all threads of this state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct global_State {
    pub frealloc: lua_Alloc,
    pub ud: *mut lua_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: lua_uint,
    pub currentwhite: lu_byte,
    pub gcstate: lu_byte,
    pub gckind: lu_byte,
    pub gcrunning: lu_byte,
    pub allgc: *mut GCObject,
    pub sweepgc: *mut *mut GCObject,
    pub finobj: *mut GCObject,
    pub gray: *mut GCObject,
    pub grayagain: *mut GCObject,
    pub weak: *mut GCObject,
    pub ephemeron: *mut GCObject,
    pub allweak: *mut GCObject,
    pub tobefnz: *mut GCObject,
    pub fixedgc: *mut GCObject,
    pub twups: *mut lua_State,
    pub gcfinnum: lua_uint,
    pub gcpause: lua_int,
    pub gcstepmul: lua_int,
    pub panic: lua_CFunction,
    pub mainthread: *mut lua_State,
    pub version: *const lua_Number,
    pub memerrmsg: *mut TString,
    pub tmname: [*mut TString; 24],
    pub mt: [*mut Table; 9],
    pub strcache: [[*mut TString; 2]; 53],
}
/*
** Header for string value; string bytes follow the end of this structure
** (aligned according to 'UTString'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TString {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub extra: lu_byte,
    pub shrlen: lu_byte,
    pub hash: lua_uint,
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    pub lnglen: size_t,
    pub hnext: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Table {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub flags: lu_byte,
    pub lsizenode: lu_byte,
    pub sizearray: lua_uint,
    pub array: *mut TValue,
    pub node: *mut Node,
    pub lastfree: *mut Node,
    pub metatable: *mut Table,
    pub gclist: *mut GCObject,
}
/* copy a value into a key without messing up field 'next' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Node {
    pub i_val: TValue,
    pub i_key: TKey,
}
/*
** Tables
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union TKey {
    pub nk: unnamed_3,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub value_: Value,
    pub tt_: lua_int,
    pub next: lua_int,
}
/*
** Atomic type (relative to signals) to better ensure that 'lua_sethook'
** is thread safe
*/
/* extra stack space to handle TM calls and some other extras */
/* kinds of Garbage Collection */
/* gc was forced by an allocation failure */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringtable {
    pub hash: *mut *mut TString,
    pub nuse: lua_int,
    pub size: lua_int,
}
/*
** $Id: llimits.h,v 1.141.1.1 2017/04/19 17:20:42 roberto Exp $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/
/*
** 'lu_mem' and 'l_mem' are unsigned/signed integers big enough to count
** the total memory used by Lua (in bytes). Usually, 'size_t' and
** 'ptrdiff_t' should work, but we use 'long' for 16-bit machines.
*/
/* { external definitions? */
/* }{ */
pub type lu_mem = size_t;
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut lua_void, _: *mut lua_void, _: size_t, _: size_t) -> *mut lua_void,
>;
/* maximum value for size_t */
/* maximum size visible for Lua (must be representable in a lua_Integer */
/* maximum value of an int */
/*
** conversion of pointer to unsigned integer:
** this is for hashing only; there is no problem if the integer
** cannot hold the whole pointer value
*/
/* type to ensure maximum alignment */
#[derive(Copy, Clone)]
#[repr(C)]
pub union L_Umaxalign {
    pub n: lua_Number,
    pub u: lua_double,
    pub s: *mut lua_void,
    pub i: lua_Integer,
    pub l: lua_long,
}
/*
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    pub dummy: L_Umaxalign,
    pub tsv: TString,
}
/*
** Get the actual string (array of bytes) from a 'TString'.
** (Access to 'extra' ensures that value is really a 'TString'.)
*/
/* get the actual string (array of bytes) from a Lua value */
/* get string length from 'TString *s' */
/* get string length from 'TValue *o' */
/*
** Header for userdata; memory area follows the end of this structure
** (aligned according to 'UUdata'; see next).
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Udata {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub ttuv_: lu_byte,
    pub metatable: *mut Table,
    pub len: size_t,
    pub user_: Value,
}
/*
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UUdata {
    pub dummy: L_Umaxalign,
    pub uv: Udata,
}
/*
**  Get the address of memory block inside 'Udata'.
** (Access to 'ttuv_' ensures that value is really a 'Udata'.)
*/
/*
** Description of an upvalue for function prototypes
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Upvaldesc {
    pub name: *mut TString,
    pub instack: lu_byte,
    pub idx: lu_byte,
}
/*
** Description of a local variable for function prototypes
** (used for debug information)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LocVar {
    pub varname: *mut TString,
    pub startpc: lua_int,
    pub endpc: lua_int,
}
/*
** Function Prototypes
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Proto {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub numparams: lu_byte,
    pub is_vararg: lu_byte,
    pub maxstacksize: lu_byte,
    pub sizeupvalues: lua_int,
    pub sizek: lua_int,
    pub sizecode: lua_int,
    pub sizelineinfo: lua_int,
    pub sizep: lua_int,
    pub sizelocvars: lua_int,
    pub linedefined: lua_int,
    pub lastlinedefined: lua_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut lua_int,
    pub locvars: *mut LocVar,
    pub upvalues: *mut Upvaldesc,
    pub cache: *mut LClosure,
    pub source: *mut TString,
    pub gclist: *mut GCObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub p: *mut Proto,
    pub upvals: [*mut UpVal; 1],
}
/*
** Closures
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CClosure {
    pub next: *mut GCObject,
    pub tt: lu_byte,
    pub marked: lu_byte,
    pub nupvalues: lu_byte,
    pub gclist: *mut GCObject,
    pub f: lua_CFunction,
    pub upvalue: [TValue; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
}
/*
** Union of all collectable objects (only for conversions)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union GCUnion {
    pub gc: GCObject,
    pub ts: TString,
    pub u: Udata,
    pub cl: Closure,
    pub h: Table,
    pub p: Proto,
    pub th: lua_State,
}
/*
** $Id: lstring.h,v 1.61.1.1 2017/04/19 17:20:42 roberto Exp $
** String table (keep all strings handled by Lua)
** See Copyright Notice in lua.h
*/
/*
** test whether a string is a reserved word
*/
/*
** equality for short strings, which are always internalized
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_hash(
    mut str: *const lua_char,
    mut l: size_t,
    mut seed: lua_uint,
) -> lua_uint {
    let mut h: lua_uint = seed ^ l as lua_uint;
    let mut step: size_t = (l >> 5i32).wrapping_add(1i32 as lua_ulong);
    while l >= step {
        h ^= (h << 5i32).wrapping_add(h >> 2i32).wrapping_add(
            *str.offset(l.wrapping_sub(1i32 as lua_ulong) as isize) as lu_byte as lua_uint,
        );
        l = (l as lua_ulong).wrapping_sub(step) as size_t as size_t
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString) -> lua_uint {
    if (*ts).extra as lua_int == 0i32 {
        /* no hash? */
        (*ts).hash = luaS_hash(
            (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            (*ts).u.lnglen,
            (*ts).hash,
        );
        /* now it has its hash */
        (*ts).extra = 1i32 as lu_byte
    }
    return (*ts).hash;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(mut a: *mut TString, mut b: *mut TString) -> lua_int {
    let mut len: size_t = (*a).u.lnglen;
    /* same instance or... */
    return (a == b
        || len == (*b).u.lnglen
            && memcmp(
                (a as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                (b as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                len,
            ) == 0i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State, mut newsize: lua_int) -> () {
    let mut i: lua_int = 0;
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    if newsize > (*tb).size {
        /* grow table if needed */
        if ::std::mem::size_of::<lua_int>() as lua_ulong
            >= ::std::mem::size_of::<size_t>() as lua_ulong
            && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut lua_void,
            ((*tb).size as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
            (newsize as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
        ) as *mut *mut TString;
        i = (*tb).size;
        while i < newsize {
            let ref mut fresh0 = *(*tb).hash.offset(i as isize);
            *fresh0 = 0 as *mut TString;
            i += 1
        }
    }
    i = 0i32;
    while i < (*tb).size {
        /* rehash */
        let mut p: *mut TString = *(*tb).hash.offset(i as isize);
        let ref mut fresh1 = *(*tb).hash.offset(i as isize);
        *fresh1 = 0 as *mut TString;
        while !p.is_null() {
            /* for each node in the list */
            /* save next */
            let mut hnext: *mut TString = (*p).u.hnext;
            /* new position */
            let mut h: lua_uint = ((*p).hash & (newsize - 1i32) as lua_uint) as lua_int as lua_uint;
            /* chain it */
            (*p).u.hnext = *(*tb).hash.offset(h as isize);
            let ref mut fresh2 = *(*tb).hash.offset(h as isize);
            *fresh2 = p;
            p = hnext
        }
        i += 1
    }
    if newsize < (*tb).size {
        /* shrink table if needed */
        /* vanishing slice should be empty */
        if ::std::mem::size_of::<lua_int>() as lua_ulong
            >= ::std::mem::size_of::<size_t>() as lua_ulong
            && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut lua_void,
            ((*tb).size as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
            (newsize as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
        ) as *mut *mut TString
    }
    (*tb).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) -> () {
    let mut i: lua_int = 0;
    let mut j: lua_int = 0;
    i = 0i32;
    while i < 53i32 {
        j = 0i32;
        while j < 2i32 {
            /* will entry be collected? */
            if 0 != (*(*g).strcache[i as usize][j as usize]).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                /* replace it with something fixed */
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg
            }
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: lua_int = 0;
    let mut j: lua_int = 0;
    /* initial size of string table */
    luaS_resize(L, 128i32);
    /* pre-create memory-error message */
    (*g).memerrmsg = luaS_newlstr(
        L,
        s!(b"not enough memory\x00"),
        (::std::mem::size_of::<[lua_char; 18]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    /* it should never be collected */
    luaC_fix(L, &mut (*((*g).memerrmsg as *mut GCUnion)).gc);
    /* fill cache with valid strings */
    i = 0i32;
    while i < 53i32 {
        j = 0i32;
        while j < 2i32 {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(
    mut L: *mut lua_State,
    mut str: *const lua_char,
    mut l: size_t,
) -> *mut TString {
    /* short string? */
    if l <= 40i32 as lua_ulong {
        return internshrstr(L, str, l);
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        if l >= if (::std::mem::size_of::<size_t>() as lua_ulong)
            < ::std::mem::size_of::<lua_Integer>() as lua_ulong
        {
            !(0i32 as size_t)
        } else {
            9223372036854775807i64 as size_t
        }
        .wrapping_sub(::std::mem::size_of::<TString>() as lua_ulong)
        .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
            ts = luaS_createlngstrobj(L, l);
            memcpy(
                (ts as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *mut lua_void,
                str as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            return ts;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = createstrobj(L, l, 4i32 | 1i32 << 4i32, (*(*L).l_G).seed);
    (*ts).u.lnglen = l;
    return ts;
}
/*
** creates a new string object
*/
unsafe extern "C" fn createstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
    mut tag: lua_int,
    mut h: lua_uint,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    /* total size of TString object */
    let mut totalsize: size_t = 0;
    totalsize = (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
        l.wrapping_add(1i32 as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    o = luaC_newobj(L, tag, totalsize);
    ts = &mut (*(o as *mut GCUnion)).ts;
    (*ts).hash = h;
    (*ts).extra = 0i32 as lu_byte;
    /* ending 0 */
    *(ts as *mut lua_char)
        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
        .offset(l as isize) = '\u{0}' as i32 as lua_char;
    return ts;
}
/*
** checks whether short string exists and reuses it or creates a new one
*/
unsafe extern "C" fn internshrstr(
    mut L: *mut lua_State,
    mut str: *const lua_char,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut g: *mut global_State = (*L).l_G;
    let mut h: lua_uint = luaS_hash(str, l, (*g).seed);
    let mut list: *mut *mut TString = &mut *(*g)
        .strt
        .hash
        .offset((h & ((*g).strt.size - 1i32) as lua_uint) as lua_int as isize)
        as *mut *mut TString;
    /* otherwise 'memcmp'/'memcpy' are undefined */
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as lua_ulong
            && memcmp(
                str as *const lua_void,
                (ts as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            ) == 0i32
        {
            /* found! */
            /* dead (but not collected yet)? */
            if 0 == ((*ts).marked as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*g).currentwhite as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            {
                /* resurrect it */
                (*ts).marked = ((*ts).marked as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
            }
            return ts;
        } else {
            ts = (*ts).u.hnext
        }
    }
    if (*g).strt.nuse >= (*g).strt.size && (*g).strt.size <= 2147483647i32 / 2i32 {
        luaS_resize(L, (*g).strt.size * 2i32);
        /* recompute with new size */
        list = &mut *(*g)
            .strt
            .hash
            .offset((h & ((*g).strt.size - 1i32) as lua_uint) as lua_int as isize)
            as *mut *mut TString
    }
    ts = createstrobj(L, l, 4i32 | 0i32 << 4i32, h);
    memcpy(
        (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
            as *mut lua_void,
        str as *const lua_void,
        l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    (*ts).shrlen = l as lu_byte;
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State, mut ts: *mut TString) -> () {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    let mut p: *mut *mut TString = &mut *(*tb)
        .hash
        .offset(((*ts).hash & ((*tb).size - 1i32) as lua_uint) as lua_int as isize)
        as *mut *mut TString;
    /* find previous element */
    while *p != ts {
        p = &mut (**p).u.hnext
    }
    /* remove element from its list */
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(mut L: *mut lua_State, mut s: size_t) -> *mut Udata {
    let mut u: *mut Udata = 0 as *mut Udata;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    if s > if (::std::mem::size_of::<size_t>() as lua_ulong)
        < ::std::mem::size_of::<lua_Integer>() as lua_ulong
    {
        !(0i32 as size_t)
    } else {
        9223372036854775807i64 as size_t
    }
    .wrapping_sub(::std::mem::size_of::<Udata>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
        o = luaC_newobj(
            L,
            7i32,
            (::std::mem::size_of::<UUdata>() as lua_ulong).wrapping_add(s),
        );
        u = &mut (*(o as *mut GCUnion)).u;
        (*u).len = s;
        (*u).metatable = 0 as *mut Table;
        let mut io: *const TValue = &luaO_nilobject_;
        let mut iu: *mut Udata = u;
        (*iu).user_ = (*io).value_;
        (*iu).ttuv_ = (*io).tt_ as lu_byte;
        return u;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(mut L: *mut lua_State, mut str: *const lua_char) -> *mut TString {
    /* hash */
    let mut i: lua_uint = ((str as size_t
        & (2147483647i32 as lua_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32) as lua_ulong) as lua_uint)
        .wrapping_rem(53i32 as lua_uint);
    let mut j: lua_int = 0;
    let mut p: *mut *mut TString = (*(*L).l_G).strcache[i as usize].as_mut_ptr();
    j = 0i32;
    while j < 2i32 {
        /* hit? */
        if strcmp(
            str,
            (*p.offset(j as isize) as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
        ) == 0i32
        {
            /* that is it */
            return *p.offset(j as isize);
        } else {
            j += 1
        }
    }
    /* normal route */
    j = 2i32 - 1i32;
    while j > 0i32 {
        /* move out last element */
        let ref mut fresh3 = *p.offset(j as isize);
        *fresh3 = *p.offset((j - 1i32) as isize);
        j -= 1
    }
    /* new element is first in the list */
    let ref mut fresh4 = *p.offset(0isize);
    *fresh4 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0isize);
}
