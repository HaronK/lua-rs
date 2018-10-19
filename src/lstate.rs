use libc;
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
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaC_freeallobjects(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaF_close(L: *mut lua_State, level: StkId) -> ();
    #[no_mangle]
    fn lua_version(L: *mut lua_State) -> *const lua_Number;
    #[no_mangle]
    fn luaX_init(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaT_init(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaS_init(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State, t: *mut Table, key: lua_Integer, value: *mut TValue) -> ();
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    ) -> ();
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
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State, f: Pfunc, ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
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
    fn luaS_hash(str: *const libc::c_char, l: size_t, seed: libc::c_uint) -> libc::c_uint;
    /*
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State) -> ();
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type intptr_t = libc::c_long;
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
    pub nci: libc::c_ushort,
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
    pub stacksize: libc::c_int,
    pub basehookcount: libc::c_int,
    pub hookcount: libc::c_int,
    pub nny: libc::c_ushort,
    pub nCcalls: libc::c_ushort,
    pub hookmask: sig_atomic_t,
    pub allowhook: lu_byte,
}
/* 16-bit ints */
/* }{ */
/* } */
/* chars used as small naturals (so that 'char' is reserved for characters) */
pub type lu_byte = libc::c_uchar;
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
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub nups: libc::c_uchar,
    pub nparams: libc::c_uchar,
    pub isvararg: libc::c_char,
    pub istailcall: libc::c_char,
    pub short_src: [libc::c_char; 60],
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
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
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
    Option<unsafe extern "C" fn(_: *mut lua_State, _: libc::c_int, _: lua_KContext) -> libc::c_int>;
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
pub type Instruction = libc::c_uint;
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
    pub tt_: libc::c_int,
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
    pub p: *mut libc::c_void,
    pub b: libc::c_int,
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
pub type lua_Number = libc::c_double;
/* type for integer functions */
pub type lua_Integer = libc::c_longlong;
/*
** Type for C functions registered with Lua
*/
pub type lua_CFunction = Option<unsafe extern "C" fn(_: *mut lua_State) -> libc::c_int>;
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
** Lua Upvalues
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct UpVal {
    pub v: *mut TValue,
    pub refcount: lu_mem,
    pub u: unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_2 {
    pub open: unnamed_3,
    pub value: TValue,
}
/* (when open) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
    pub next: *mut UpVal,
    pub touched: libc::c_int,
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
    pub ud: *mut libc::c_void,
    pub totalbytes: l_mem,
    pub GCdebt: l_mem,
    pub GCmemtrav: lu_mem,
    pub GCestimate: lu_mem,
    pub strt: stringtable,
    pub l_registry: TValue,
    pub seed: libc::c_uint,
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
    pub gcfinnum: libc::c_uint,
    pub gcpause: libc::c_int,
    pub gcstepmul: libc::c_int,
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
    pub hash: libc::c_uint,
    pub u: unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
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
    pub sizearray: libc::c_uint,
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
    pub nk: unnamed_5,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
    pub value_: Value,
    pub tt_: libc::c_int,
    pub next: libc::c_int,
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
    pub nuse: libc::c_int,
    pub size: libc::c_int,
}
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void, _: size_t, _: size_t)
        -> *mut libc::c_void,
>;
/*
** Main thread combines a thread state and the global state
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LG {
    pub l: LX,
    pub g: global_State,
}
/*
** $Id: lstate.c,v 2.133.1.1 2017/04/19 17:39:34 roberto Exp $
** Global State
** See Copyright Notice in lua.h
*/
/* 200% */
/* GC runs 'twice the speed' of memory allocation */
/*
** a macro to help the creation of a unique random seed when a state is
** created; the seed is used to randomize hashes.
*/
/*
** thread state + extra space
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LX {
    pub extra_: [lu_byte; 8],
    pub l: lua_State,
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
    pub sizeupvalues: libc::c_int,
    pub sizek: libc::c_int,
    pub sizecode: libc::c_int,
    pub sizelineinfo: libc::c_int,
    pub sizep: libc::c_int,
    pub sizelocvars: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub k: *mut TValue,
    pub code: *mut Instruction,
    pub p: *mut *mut Proto,
    pub lineinfo: *mut libc::c_int,
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
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union Closure {
    pub c: CClosure,
    pub l: LClosure,
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
** $Id: ldo.h,v 2.29.1.1 2017/04/19 17:20:42 roberto Exp $
** Stack and Call structure of Lua
** See Copyright Notice in lua.h
*/
/*
** Macro to check stack size and grow stack if needed.  Parameters
** 'pre'/'pos' allow the macro to preserve a pointer into the
** stack across reallocations, doing the work only when needed.
** 'condmovestack' is used in heavy tests to force a stack reallocation
** at every check.
*/
/* In general, 'pre'/'pos' are empty (nothing to save) */
/* type of protected functions, to be ran by 'runprotected' */
pub type Pfunc = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut libc::c_void) -> ()>;
pub type time_t = __time_t;
/*
** state manipulation
*/
#[no_mangle]
pub unsafe extern "C" fn lua_newstate(
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> *mut lua_State {
    let mut i: libc::c_int = 0;
    let mut L: *mut lua_State = 0 as *mut lua_State;
    let mut g: *mut global_State = 0 as *mut global_State;
    let mut l: *mut LG = f.expect("non-null function pointer")(
        ud,
        0 as *mut libc::c_void,
        8i32 as size_t,
        ::std::mem::size_of::<LG>() as libc::c_ulong,
    ) as *mut LG;
    if l.is_null() {
        return 0 as *mut lua_State;
    } else {
        L = &mut (*l).l.l;
        g = &mut (*l).g;
        (*L).next = 0 as *mut GCObject;
        (*L).tt = 8i32 as lu_byte;
        (*g).currentwhite = (1i32 << 0i32) as lu_byte;
        (*L).marked = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
        preinit_thread(L, g);
        (*g).frealloc = f;
        (*g).ud = ud;
        (*g).mainthread = L;
        (*g).seed = makeseed(L);
        /* no GC while building state */
        (*g).gcrunning = 0i32 as lu_byte;
        (*g).GCestimate = 0i32 as lu_mem;
        (*g).strt.nuse = 0i32;
        (*g).strt.size = (*g).strt.nuse;
        (*g).strt.hash = 0 as *mut *mut TString;
        (*g).l_registry.tt_ = 0i32;
        (*g).panic = None;
        (*g).version = 0 as *const lua_Number;
        (*g).gcstate = 7i32 as lu_byte;
        (*g).gckind = 0i32 as lu_byte;
        (*g).fixedgc = 0 as *mut GCObject;
        (*g).tobefnz = (*g).fixedgc;
        (*g).finobj = (*g).tobefnz;
        (*g).allgc = (*g).finobj;
        (*g).sweepgc = 0 as *mut *mut GCObject;
        (*g).grayagain = 0 as *mut GCObject;
        (*g).gray = (*g).grayagain;
        (*g).allweak = 0 as *mut GCObject;
        (*g).ephemeron = (*g).allweak;
        (*g).weak = (*g).ephemeron;
        (*g).twups = 0 as *mut lua_State;
        (*g).totalbytes = ::std::mem::size_of::<LG>() as libc::c_ulong as l_mem;
        (*g).GCdebt = 0i32 as l_mem;
        (*g).gcfinnum = 0i32 as libc::c_uint;
        (*g).gcpause = 200i32;
        (*g).gcstepmul = 200i32;
        i = 0i32;
        while i < 9i32 {
            (*g).mt[i as usize] = 0 as *mut Table;
            i += 1
        }
        if luaD_rawrunprotected(L, Some(f_luaopen), 0 as *mut libc::c_void) != 0i32 {
            /* memory allocation error: free partial state */
            close_state(L);
            L = 0 as *mut lua_State
        }
        return L;
    };
}
unsafe extern "C" fn close_state(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* close all upvalues for this thread */
    luaF_close(L, (*L).stack);
    /* collect all objects */
    luaC_freeallobjects(L);
    /* closing a fully built state? */
    !(*g).version.is_null();
    luaM_realloc_(
        L,
        (*(*L).l_G).strt.hash as *mut libc::c_void,
        ((*(*L).l_G).strt.size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut TString>() as libc::c_ulong),
        0i32 as size_t,
    );
    freestack(L);
    /* free main block */
    (*g).frealloc.expect("non-null function pointer")(
        (*g).ud,
        (L as *mut lu_byte).offset(-8isize) as *mut LX as *mut libc::c_void,
        ::std::mem::size_of::<LG>() as libc::c_ulong,
        0i32 as size_t,
    );
}
unsafe extern "C" fn freestack(mut L: *mut lua_State) -> () {
    if (*L).stack.is_null() {
        /* stack not completely built yet */
        return;
    } else {
        /* free the entire 'ci' list */
        (*L).ci = &mut (*L).base_ci;
        luaE_freeCI(L);
        /* free stack array */
        luaM_realloc_(
            L,
            (*L).stack as *mut libc::c_void,
            ((*L).stacksize as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
            0i32 as size_t,
        );
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freeCI(mut L: *mut lua_State) -> () {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut next: *mut CallInfo = (*ci).next;
    (*ci).next = 0 as *mut CallInfo;
    loop {
        ci = next;
        if ci.is_null() {
            break;
        }
        next = (*ci).next;
        luaM_realloc_(
            L,
            ci as *mut libc::c_void,
            ::std::mem::size_of::<CallInfo>() as libc::c_ulong,
            0i32 as size_t,
        );
        (*L).nci = (*L).nci.wrapping_sub(1)
    }
}
/*
** open parts of the state that may cause memory-allocation errors.
** ('g->version' != NULL flags that the state was completely build)
*/
unsafe extern "C" fn f_luaopen(mut L: *mut lua_State, mut _ud: *mut libc::c_void) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* init stack */
    stack_init(L, L);
    init_registry(L, g);
    luaS_init(L);
    luaT_init(L);
    luaX_init(L);
    /* allow gc */
    (*g).gcrunning = 1i32 as lu_byte;
    (*g).version = lua_version(0 as *mut lua_State);
}
/*
** Create registry table and its predefined values
*/
unsafe extern "C" fn init_registry(mut L: *mut lua_State, mut g: *mut global_State) -> () {
    let mut temp: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    /* create registry */
    let mut registry: *mut Table = luaH_new(L);
    let mut io: *mut TValue = &mut (*g).l_registry;
    let mut x_: *mut Table = registry;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    luaH_resize(L, registry, 2i32 as libc::c_uint, 0i32 as libc::c_uint);
    /* registry[LUA_RIDX_MAINTHREAD] = L */
    let mut io_0: *mut TValue = &mut temp;
    /* temp = L */
    let mut x__0: *mut lua_State = L;
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 8i32 | 1i32 << 6i32;
    luaH_setint(L, registry, 1i32 as lua_Integer, &mut temp);
    /* registry[LUA_RIDX_GLOBALS] = table of globals */
    let mut io_1: *mut TValue = &mut temp;
    /* temp = new table (global table) */
    let mut x__1: *mut Table = luaH_new(L);
    (*io_1).value_.gc = &mut (*(x__1 as *mut GCUnion)).gc;
    (*io_1).tt_ = 5i32 | 1i32 << 6i32;
    luaH_setint(L, registry, 2i32 as lua_Integer, &mut temp);
}
unsafe extern "C" fn stack_init(mut L1: *mut lua_State, mut L: *mut lua_State) -> () {
    let mut i: libc::c_int = 0;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    /* initialize stack array */
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && ((2i32 * 20i32) as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*L1).stack = luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        ((2i32 * 20i32) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    (*L1).stacksize = 2i32 * 20i32;
    i = 0i32;
    while i < 2i32 * 20i32 {
        /* erase new stack */
        (*(*L1).stack.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    (*L1).top = (*L1).stack;
    (*L1).stack_last = (*L1).stack.offset((*L1).stacksize as isize).offset(-5isize);
    /* initialize first ci */
    ci = &mut (*L1).base_ci;
    (*ci).previous = 0 as *mut CallInfo;
    (*ci).next = (*ci).previous;
    (*ci).callstatus = 0i32 as libc::c_ushort;
    (*ci).func = (*L1).top;
    /* 'function' entry for this 'ci' */
    let fresh0 = (*L1).top;
    (*L1).top = (*L1).top.offset(1);
    (*fresh0).tt_ = 0i32;
    (*ci).top = (*L1).top.offset(20isize);
    (*L1).ci = ci;
}
/*
** Compute an initial seed as random as possible. Rely on Address Space
** Layout Randomization (if present) to increase randomness..
*/
unsafe extern "C" fn makeseed(mut L: *mut lua_State) -> libc::c_uint {
    let mut buff: [libc::c_char; 32] = [0; 32];
    let mut h: libc::c_uint = time(0 as *mut time_t) as libc::c_uint;
    let mut p: libc::c_int = 0i32;
    /* heap variable */
    let mut t: size_t = L as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* local variable */
    let mut t_0: size_t = &mut h as *mut libc::c_uint as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_0 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* global variable */
    let mut t_1: size_t = &luaO_nilobject_ as *const TValue as size_t;
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_1 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    /* public function */
    let mut t_2: size_t = lua_newstate as size_t;
    // let mut t_2: size_t = ::std::mem::transmute::<
    //     Option<unsafe extern "C" fn(_: lua_Alloc, _: *mut libc::c_void) -> *mut lua_State>,
    //     size_t,
    // >(Some(lua_newstate));
    memcpy(
        buff.as_mut_ptr().offset(p as isize) as *mut libc::c_void,
        &mut t_2 as *mut size_t as *const libc::c_void,
        ::std::mem::size_of::<size_t>() as libc::c_ulong,
    );
    p = (p as libc::c_ulong).wrapping_add(::std::mem::size_of::<size_t>() as libc::c_ulong)
        as libc::c_int as libc::c_int;
    return luaS_hash(buff.as_mut_ptr(), p as size_t, h);
}
/*
** preinitialize a thread with consistent values without allocating
** any memory (to avoid errors)
*/
unsafe extern "C" fn preinit_thread(mut L: *mut lua_State, mut g: *mut global_State) -> () {
    (*L).l_G = g;
    (*L).stack = 0 as StkId;
    (*L).ci = 0 as *mut CallInfo;
    (*L).nci = 0i32 as libc::c_ushort;
    (*L).stacksize = 0i32;
    /* thread has no upvalues */
    (*L).twups = L;
    (*L).errorJmp = 0 as *mut lua_longjmp;
    (*L).nCcalls = 0i32 as libc::c_ushort;
    ::std::ptr::write_volatile(&mut (*L).hook as *mut lua_Hook, None);
    (*L).hookmask = 0i32;
    (*L).basehookcount = 0i32;
    (*L).allowhook = 1i32 as lu_byte;
    (*L).hookcount = (*L).basehookcount;
    (*L).openupval = 0 as *mut UpVal;
    (*L).nny = 1i32 as libc::c_ushort;
    (*L).status = 0i32 as lu_byte;
    (*L).errfunc = 0i32 as ptrdiff_t;
}
#[no_mangle]
pub unsafe extern "C" fn lua_close(mut L: *mut lua_State) -> () {
    /* only the main thread can be closed */
    L = (*(*L).l_G).mainthread;
    close_state(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_newthread(mut L: *mut lua_State) -> *mut lua_State {
    let mut g: *mut global_State = (*L).l_G;
    let mut L1: *mut lua_State = 0 as *mut lua_State;
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    /* create new thread */
    L1 = &mut (*(luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        8i32 as size_t,
        ::std::mem::size_of::<LX>() as libc::c_ulong,
    ) as *mut LX))
        .l;
    (*L1).marked = ((*g).currentwhite as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    (*L1).tt = 8i32 as lu_byte;
    /* link it on list 'allgc' */
    (*L1).next = (*g).allgc;
    (*g).allgc = &mut (*(L1 as *mut GCUnion)).gc;
    /* anchor it on L stack */
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State = L1;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    preinit_thread(L1, g);
    (*L1).hookmask = (*L).hookmask;
    (*L1).basehookcount = (*L).basehookcount;
    ::std::ptr::write_volatile(&mut (*L1).hook as *mut lua_Hook, (*L).hook);
    (*L1).hookcount = (*L1).basehookcount;
    /* initialize L1 extra space */
    memcpy(
        (L1 as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize))
            as *mut libc::c_void,
        ((*g).mainthread as *mut libc::c_char)
            .offset(-(::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize))
            as *mut libc::c_void,
        ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    /* init stack */
    stack_init(L1, L);
    return L1;
}
/* macros to convert a GCObject into a specific value */
/* macro to convert a Lua object into a GCObject */
/* actual number of total bytes allocated */
#[no_mangle]
pub unsafe extern "C" fn luaE_setdebt(mut g: *mut global_State, mut debt: l_mem) -> () {
    let mut tb: l_mem = ((*g).totalbytes + (*g).GCdebt) as lu_mem as l_mem;
    if debt < tb - (!(0i32 as lu_mem) >> 1i32) as l_mem {
        /* will make 'totalbytes == MAX_LMEM' */
        debt = tb - (!(0i32 as lu_mem) >> 1i32) as l_mem
    }
    (*g).totalbytes = tb - debt;
    (*g).GCdebt = debt;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_freethread(mut L: *mut lua_State, mut L1: *mut lua_State) -> () {
    let mut l: *mut LX = (L1 as *mut lu_byte).offset(-8isize) as *mut LX;
    /* close all upvalues for this thread */
    luaF_close(L1, (*L1).stack);
    freestack(L1);
    luaM_realloc_(
        L,
        l as *mut libc::c_void,
        ::std::mem::size_of::<LX>() as libc::c_ulong,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaE_extendCI(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci: *mut CallInfo = luaM_realloc_(
        L,
        0 as *mut libc::c_void,
        0i32 as size_t,
        ::std::mem::size_of::<CallInfo>() as libc::c_ulong,
    ) as *mut CallInfo;
    (*(*L).ci).next = ci;
    (*ci).previous = (*L).ci;
    (*ci).next = 0 as *mut CallInfo;
    (*L).nci = (*L).nci.wrapping_add(1);
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn luaE_shrinkCI(mut L: *mut lua_State) -> () {
    let mut ci: *mut CallInfo = (*L).ci;
    /* next's next */
    let mut next2: *mut CallInfo = 0 as *mut CallInfo;
    /* while there are two nexts */
    while !(*ci).next.is_null() && {
        next2 = (*(*ci).next).next;
        !next2.is_null()
    } {
        /* free next */
        luaM_realloc_(
            L,
            (*ci).next as *mut libc::c_void,
            ::std::mem::size_of::<CallInfo>() as libc::c_ulong,
            0i32 as size_t,
        );
        (*L).nci = (*L).nci.wrapping_sub(1);
        /* remove 'next' from the list */
        (*ci).next = next2;
        (*next2).previous = ci;
        /* keep next's next */
        ci = next2
    }
}
