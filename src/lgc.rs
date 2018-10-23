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
    #[no_mangle]
    fn strchr(_: *const lua_char, _: lua_int) -> *mut lua_char;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const lua_char, ...) -> *const lua_char;
    #[no_mangle]
    fn luaT_gettm(events: *mut Table, event: TMS, ename: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut lua_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut lua_void;
    /* macros to convert a GCObject into a specific value */
    /* macro to convert a Lua object into a GCObject */
    /* actual number of total bytes allocated */
    #[no_mangle]
    fn luaE_setdebt(g: *mut global_State, debt: l_mem) -> ();
    #[no_mangle]
    fn luaE_freethread(L: *mut lua_State, L1: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaD_pcall(
        L: *mut lua_State,
        func: Pfunc,
        u: *mut lua_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> lua_int;
    #[no_mangle]
    fn luaD_shrinkstack(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: lua_int) -> !;
    #[no_mangle]
    fn luaF_freeproto(L: *mut lua_State, f: *mut Proto) -> ();
    #[no_mangle]
    fn luaS_remove(L: *mut lua_State, ts: *mut TString) -> ();
    #[no_mangle]
    fn luaH_free(L: *mut lua_State, t: *mut Table) -> ();
    #[no_mangle]
    fn luaS_resize(L: *mut lua_State, newsize: lua_int) -> ();
    #[no_mangle]
    fn luaS_clearcache(g: *mut global_State) -> ();
}
pub type size_t = lua_ulong;
pub type ptrdiff_t = lua_long;
pub type __sig_atomic_t = lua_int;
pub type intptr_t = lua_long;
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
    pub touched: lua_int,
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
    pub nk: unnamed_5,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_5 {
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
pub type l_mem = ptrdiff_t;
/*
** Type for memory-allocation functions
*/
pub type lua_Alloc = Option<
    unsafe extern "C" fn(_: *mut lua_void, _: *mut lua_void, _: size_t, _: size_t)
        -> *mut lua_void,
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
** $Id: ltm.h,v 2.22.1.1 2017/04/19 17:20:42 roberto Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
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
pub type Pfunc = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut lua_void) -> ()>;
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
pub unsafe extern "C" fn luaC_fix(mut L: *mut lua_State, mut o: *mut GCObject) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* object must be 1st in 'allgc' list! */
    /* they will be gray forever */
    (*o).marked = ((*o).marked as lua_int
        & !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as lua_int) as lu_byte;
    /* remove object from 'allgc' list */
    (*g).allgc = (*o).next;
    /* link it to 'fixedgc' list */
    (*o).next = (*g).fixedgc;
    (*g).fixedgc = o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_freeallobjects(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* separate all objects with finalizers */
    separatetobefnz(g, 1i32);
    callallpendingfinalizers(L);
    /* this "white" makes all objects look dead */
    (*g).currentwhite = (1i32 << 0i32 | 1i32 << 1i32) as lu_byte;
    (*g).gckind = 0i32 as lu_byte;
    sweeplist(L, &mut (*g).finobj, !(0i32 as lu_mem));
    sweeplist(L, &mut (*g).allgc, !(0i32 as lu_mem));
    /* collect fixed objects */
    sweeplist(L, &mut (*g).fixedgc, !(0i32 as lu_mem));
}
unsafe extern "C" fn sweeplist(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
    mut count: lu_mem,
) -> *mut *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut ow: lua_int = (*g).currentwhite as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32);
    /* current white */
    let mut white: lua_int = ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32))
        as lu_byte as lua_int;
    while !(*p).is_null() && {
        let fresh0 = count;
        count = count.wrapping_sub(1);
        fresh0 > 0i32 as lua_ulong
    } {
        let mut curr: *mut GCObject = *p;
        let mut marked: lua_int = (*curr).marked as lua_int;
        if 0 == (marked ^ (1i32 << 0i32 | 1i32 << 1i32)) & ow {
            /* is 'curr' dead? */
            /* remove 'curr' from list */
            *p = (*curr).next;
            /* erase 'curr' */
            freeobj(L, curr);
        } else {
            /* change mark to 'white' */
            (*curr).marked =
                (marked & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32)) | white) as lu_byte;
            /* go to next element */
            p = &mut (*curr).next
        }
    }
    return if (*p).is_null() {
        0 as *mut *mut GCObject
    } else {
        p
    };
}
unsafe extern "C" fn freeobj(mut L: *mut lua_State, mut o: *mut GCObject) -> () {
    match (*o).tt as lua_int {
        9 => {
            luaF_freeproto(L, &mut (*(o as *mut GCUnion)).p);
        }
        6 => {
            freeLclosure(L, &mut (*(o as *mut GCUnion)).cl.l);
        }
        38 => {
            luaM_realloc_(
                L,
                o as *mut lua_void,
                (::std::mem::size_of::<CClosure>() as lua_ulong as lua_int
                    + (::std::mem::size_of::<TValue>() as lua_ulong).wrapping_mul(
                        ((*(o as *mut GCUnion)).cl.c.nupvalues as lua_int - 1i32)
                            as lua_ulong,
                    ) as lua_int) as size_t,
                0i32 as size_t,
            );
        }
        5 => {
            luaH_free(L, &mut (*(o as *mut GCUnion)).h);
        }
        8 => {
            luaE_freethread(L, &mut (*(o as *mut GCUnion)).th);
        }
        7 => {
            luaM_realloc_(
                L,
                o as *mut lua_void,
                (::std::mem::size_of::<UUdata>() as lua_ulong)
                    .wrapping_add((*(o as *mut GCUnion)).u.len),
                0i32 as size_t,
            );
        }
        4 => {
            /* remove it from hash table */
            luaS_remove(L, &mut (*(o as *mut GCUnion)).ts);
            luaM_realloc_(
                L,
                o as *mut lua_void,
                (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
                    (((*(o as *mut GCUnion)).ts.shrlen as lua_int + 1i32) as lua_ulong)
                        .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
                ),
                0i32 as size_t,
            );
        }
        20 => {
            luaM_realloc_(
                L,
                o as *mut lua_void,
                (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
                    (*(o as *mut GCUnion))
                        .ts
                        .u
                        .lnglen
                        .wrapping_add(1i32 as lua_ulong)
                        .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
                ),
                0i32 as size_t,
            );
        }
        _ => {}
    };
}
unsafe extern "C" fn freeLclosure(mut L: *mut lua_State, mut cl: *mut LClosure) -> () {
    let mut i: lua_int = 0;
    i = 0i32;
    while i < (*cl).nupvalues as lua_int {
        let mut uv: *mut UpVal = (*cl).upvals[i as usize];
        if !uv.is_null() {
            luaC_upvdeccount(L, uv);
        }
        i += 1
    }
    luaM_realloc_(
        L,
        cl as *mut lua_void,
        (::std::mem::size_of::<LClosure>() as lua_ulong as lua_int
            + (::std::mem::size_of::<*mut TValue>() as lua_ulong)
                .wrapping_mul(((*cl).nupvalues as lua_int - 1i32) as lua_ulong)
                as lua_int) as size_t,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvdeccount(mut L: *mut lua_State, mut uv: *mut UpVal) -> () {
    (*uv).refcount = (*uv).refcount.wrapping_sub(1);
    if (*uv).refcount == 0i32 as lua_ulong && !((*uv).v != &mut (*uv).u.value as *mut TValue) {
        luaM_realloc_(
            L,
            uv as *mut lua_void,
            ::std::mem::size_of::<UpVal>() as lua_ulong,
            0i32 as size_t,
        );
    };
}
/*
** call all pending finalizers
*/
unsafe extern "C" fn callallpendingfinalizers(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while !(*g).tobefnz.is_null() {
        GCTM(L, 0i32);
    }
}
unsafe extern "C" fn GCTM(mut L: *mut lua_State, mut propagateerrors: lua_int) -> () {
    let mut msg: *const lua_char = 0 as *const lua_char;
    let mut g: *mut global_State = (*L).l_G;
    let mut tm: *const TValue = 0 as *const TValue;
    let mut v: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut io: *mut TValue = &mut v;
    let mut i_g: *mut GCObject = udata2finalize(g);
    (*io).value_.gc = i_g;
    (*io).tt_ = (*i_g).tt as lua_int | 1i32 << 6i32;
    tm = luaT_gettmbyobj(L, &mut v, TM_GC);
    if !tm.is_null() && (*tm).tt_ & 0xfi32 == 6i32 {
        /* is there a finalizer? */
        let mut status: lua_int = 0;
        let mut oldah: lu_byte = (*L).allowhook;
        let mut running: lua_int = (*g).gcrunning as lua_int;
        /* stop debug hooks during GC metamethod */
        (*L).allowhook = 0i32 as lu_byte;
        /* avoid GC steps */
        (*g).gcrunning = 0i32 as lu_byte;
        /* push finalizer... */
        let mut io1: *mut TValue = (*L).top;
        *io1 = *tm;
        /* ... and its argument */
        let mut io1_0: *mut TValue = (*L).top.offset(1isize);
        *io1_0 = v;
        /* and (next line) call the finalizer */
        (*L).top = (*L).top.offset(2isize);
        /* will run a finalizer */
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as lua_int | 1i32 << 8i32) as lua_ushort;
        status = luaD_pcall(
            L,
            Some(dothecall),
            0 as *mut lua_void,
            ((*L).top.offset(-2isize) as *mut lua_char)
                .wrapping_offset_from((*L).stack as *mut lua_char) as lua_long,
            0i32 as ptrdiff_t,
        );
        /* not running a finalizer anymore */
        (*(*L).ci).callstatus =
            ((*(*L).ci).callstatus as lua_int & !(1i32 << 8i32)) as lua_ushort;
        /* restore hooks */
        (*L).allowhook = oldah;
        /* restore state */
        (*g).gcrunning = running as lu_byte;
        if status != 0i32 && 0 != propagateerrors {
            /* error while running __gc? */
            if status == 2i32 {
                /* is there an error object? */
                msg = if (*(*L).top.offset(-1isize)).tt_ & 0xfi32 == 4i32 {
                    (&mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion)).ts
                        as *mut TString as *mut lua_char)
                        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                } else {
                    s!(b"no message\x00")
                };
                luaO_pushfstring!(L, s!(b"error in __gc metamethod (%s)\x00"), msg,);
                /* error in __gc metamethod */
                status = 5i32
            }
            /* re-throw error */
            luaD_throw(L, status);
        }
    };
}
unsafe extern "C" fn dothecall(mut L: *mut lua_State, mut _ud: *mut lua_void) -> () {
    luaD_callnoyield(L, (*L).top.offset(-2isize), 0i32);
}
unsafe extern "C" fn udata2finalize(mut g: *mut global_State) -> *mut GCObject {
    /* get first element */
    let mut o: *mut GCObject = (*g).tobefnz;
    /* remove it from 'tobefnz' list */
    (*g).tobefnz = (*o).next;
    /* return it to 'allgc' list */
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    /* object is "normal" again */
    (*o).marked =
        ((*o).marked as lua_int & !(1i32 << 3i32) as lu_byte as lua_int) as lu_byte;
    if 2i32 <= (*g).gcstate as lua_int && (*g).gcstate as lua_int <= 5i32 {
        /* "sweep" object */
        (*o).marked = ((*o).marked as lua_int & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
            | ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                as lua_int) as lu_byte
    }
    return o;
}
/*
** move all unreachable objects (or 'all' objects) that need
** finalization from list 'finobj' to list 'tobefnz' (to be finalized)
*/
unsafe extern "C" fn separatetobefnz(mut g: *mut global_State, mut all: lua_int) -> () {
    let mut curr: *mut GCObject = 0 as *mut GCObject;
    let mut p: *mut *mut GCObject = &mut (*g).finobj;
    let mut lastnext: *mut *mut GCObject = findlast(&mut (*g).tobefnz);
    loop {
        curr = *p;
        if curr.is_null() {
            break;
        }
        /* traverse all finalizable objects */
        /* not being collected? */
        if !(0 != (*curr).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) || 0 != all) {
            /* don't bother with it */
            p = &mut (*curr).next
        } else {
            /* remove 'curr' from 'finobj' list */
            *p = (*curr).next;
            /* link at the end of 'tobefnz' list */
            (*curr).next = *lastnext;
            *lastnext = curr;
            lastnext = &mut (*curr).next
        }
    }
}
/*
** find last 'next' field in list 'p' list (to add elements in its end)
*/
unsafe extern "C" fn findlast(mut p: *mut *mut GCObject) -> *mut *mut GCObject {
    while !(*p).is_null() {
        p = &mut (**p).next
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_step(mut L: *mut lua_State) -> () {
    let mut work: lu_mem = 0;
    let mut g: *mut global_State = (*L).l_G;
    /* GC deficit (be paid now) */
    let mut debt: l_mem = getdebt(g);
    if 0 == (*g).gcrunning {
        /* not running? */
        /* avoid being called too often */
        luaE_setdebt(
            g,
            (-((100i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as lua_ulong)
                as lua_int)
                * 10i32) as l_mem,
        );
        return;
    } else {
        loop {
            /* repeat until pause or enough "credit" (negative debt) */
            /* perform one single step */
            work = singlestep(L);
            debt = (debt as lua_ulong).wrapping_sub(work) as l_mem as l_mem;
            if !(debt > -((100i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as lua_ulong)
                as lua_int) as lua_long
                && (*g).gcstate as lua_int != 7i32)
            {
                break;
            }
        }
        if (*g).gcstate as lua_int == 7i32 {
            /* pause until next cycle */
            setpause(g);
        } else {
            /* convert 'work units' to Kb */
            debt = debt / (*g).gcstepmul as lua_long * 200i32 as lua_long;
            luaE_setdebt(g, debt);
            runafewfinalizers(L);
        }
        return;
    };
}
/*
** call a few (up to 'g->gcfinnum') finalizers
*/
unsafe extern "C" fn runafewfinalizers(mut L: *mut lua_State) -> lua_int {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: lua_uint = 0;
    i = 0i32 as lua_uint;
    while !(*g).tobefnz.is_null() && i < (*g).gcfinnum {
        /* call one finalizer */
        GCTM(L, 1i32);
        i = i.wrapping_add(1)
    }
    /* nothing more to finalize? */
    (*g).gcfinnum = if (*g).tobefnz.is_null() {
        0i32 as lua_uint
    } else {
        (*g).gcfinnum.wrapping_mul(2i32 as lua_uint)
    };
    /* else call a few more next time */
    return i as lua_int;
}
/*
** get GC debt and convert it from Kb to 'work units' (avoid zero debt
** and overflows)
*/
unsafe extern "C" fn getdebt(mut g: *mut global_State) -> l_mem {
    let mut debt: l_mem = (*g).GCdebt;
    let mut stepmul: lua_int = (*g).gcstepmul;
    if debt <= 0i32 as lua_long {
        /* minimal debt */
        return 0i32 as l_mem;
    } else {
        debt = debt / 200i32 as lua_long + 1i32 as lua_long;
        debt = if debt < (!(0i32 as lu_mem) >> 1i32) as l_mem / stepmul as lua_long {
            debt * stepmul as lua_long
        } else {
            (!(0i32 as lu_mem) >> 1i32) as l_mem
        };
        return debt;
    };
}
/* }====================================================== */
/*
** {======================================================
** GC control
** =======================================================
*/
/*
** Set a reasonable "time" to wait before starting a new GC cycle; cycle
** will start when memory use hits threshold. (Division by 'estimate'
** should be OK: it cannot be zero (because Lua cannot even start with
** less than PAUSEADJ bytes).
*/
unsafe extern "C" fn setpause(mut g: *mut global_State) -> () {
    let mut threshold: l_mem = 0;
    let mut debt: l_mem = 0;
    /* adjust 'estimate' */
    let mut estimate: l_mem = (*g).GCestimate.wrapping_div(100i32 as lua_ulong) as l_mem;
    /* overflow? */
    threshold = if ((*g).gcpause as lua_long) < (!(0i32 as lu_mem) >> 1i32) as l_mem / estimate
    {
        estimate * (*g).gcpause as lua_long
    } else {
        (!(0i32 as lu_mem) >> 1i32) as l_mem
    };
    /* no overflow */
    /* overflow; truncate to maximum */
    debt = (((*g).totalbytes + (*g).GCdebt) as lu_mem).wrapping_sub(threshold as lua_ulong)
        as l_mem;
    luaE_setdebt(g, debt);
}
unsafe extern "C" fn singlestep(mut L: *mut lua_State) -> lu_mem {
    let mut g: *mut global_State = (*L).l_G;
    match (*g).gcstate as lua_int {
        7 => {
            (*g).GCmemtrav = ((*g).strt.size as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut GCObject>() as lua_ulong);
            restartcollection(g);
            (*g).gcstate = 0i32 as lu_byte;
            return (*g).GCmemtrav;
        }
        0 => {
            (*g).GCmemtrav = 0i32 as lu_mem;
            propagatemark(g);
            /* no more gray objects? */
            if (*g).gray.is_null() {
                /* finish propagate phase */
                (*g).gcstate = 1i32 as lu_byte
            }
            /* memory traversed in this step */
            return (*g).GCmemtrav;
        }
        1 => {
            let mut work: lu_mem = 0;
            /* make sure gray list is empty */
            propagateall(g);
            /* work is what was traversed by 'atomic' */
            work = atomic(L) as lu_mem;
            entersweep(L);
            (*g).GCestimate = ((*g).totalbytes + (*g).GCdebt) as lu_mem;
            /* first estimate */
            return work;
        }
        2 => {
            /* sweep "regular" objects */
            return sweepstep(L, g, 3i32, &mut (*g).finobj);
        }
        3 => {
            /* sweep objects with finalizers */
            return sweepstep(L, g, 4i32, &mut (*g).tobefnz);
        }
        4 => {
            /* sweep objects to be finalized */
            return sweepstep(L, g, 5i32, 0 as *mut *mut GCObject);
        }
        5 => {
            /* finish sweeps */
            /* sweep main thread */
            (*(*g).mainthread).marked = ((*(*g).mainthread).marked as lua_int
                & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
                | ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                    as lua_int) as lu_byte;
            checkSizes(L, g);
            (*g).gcstate = 6i32 as lu_byte;
            return 0i32 as lu_mem;
        }
        6 => {
            /* call remaining finalizers */
            if !(*g).tobefnz.is_null() && (*g).gckind as lua_int != 1i32 {
                let mut n: lua_int = runafewfinalizers(L);
                return (n as lua_ulong).wrapping_mul(
                    (::std::mem::size_of::<TString>() as lua_ulong)
                        .wrapping_add(4i32 as lua_ulong)
                        .wrapping_div(4i32 as lua_ulong),
                );
            } else {
                /* emergency mode or no more finalizers */
                /* finish collection */
                (*g).gcstate = 7i32 as lu_byte;
                return 0i32 as lu_mem;
            }
        }
        _ => return 0i32 as lu_mem,
    };
}
/* }====================================================== */
/*
** {======================================================
** Finalization
** =======================================================
*/
/*
** If possible, shrink string table
*/
unsafe extern "C" fn checkSizes(mut L: *mut lua_State, mut g: *mut global_State) -> () {
    if (*g).gckind as lua_int != 1i32 {
        let mut olddebt: l_mem = (*g).GCdebt;
        /* string table too big? */
        if (*g).strt.nuse < (*g).strt.size / 4i32 {
            /* shrink it a little */
            luaS_resize(L, (*g).strt.size / 2i32);
        }
        /* update estimate */
        (*g).GCestimate = ((*g).GCestimate as lua_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as lua_ulong)
            as lu_mem as lu_mem
    };
}
unsafe extern "C" fn sweepstep(
    mut L: *mut lua_State,
    mut g: *mut global_State,
    mut nextstate: lua_int,
    mut nextlist: *mut *mut GCObject,
) -> lu_mem {
    if !(*g).sweepgc.is_null() {
        let mut olddebt: l_mem = (*g).GCdebt;
        (*g).sweepgc = sweeplist(
            L,
            (*g).sweepgc,
            ((100i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as lua_ulong)
                as lua_int as lua_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<TString>() as lua_ulong)
                        .wrapping_add(4i32 as lua_ulong)
                        .wrapping_div(4i32 as lua_ulong),
                )
                .wrapping_div(4i32 as lua_ulong) as lua_int as lu_mem,
        );
        /* update estimate */
        (*g).GCestimate = ((*g).GCestimate as lua_ulong)
            .wrapping_add(((*g).GCdebt - olddebt) as lua_ulong)
            as lu_mem as lu_mem;
        /* is there still something to sweep? */
        if !(*g).sweepgc.is_null() {
            return (((100i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<TString>() as lua_ulong)
                as lua_int as lua_ulong)
                .wrapping_div(
                    (::std::mem::size_of::<TString>() as lua_ulong)
                        .wrapping_add(4i32 as lua_ulong)
                        .wrapping_div(4i32 as lua_ulong),
                )
                .wrapping_div(4i32 as lua_ulong) as lua_int
                as lua_ulong)
                .wrapping_mul(
                    (::std::mem::size_of::<TString>() as lua_ulong)
                        .wrapping_add(4i32 as lua_ulong)
                        .wrapping_div(4i32 as lua_ulong),
                );
        }
    }
    /* else enter next state */
    (*g).gcstate = nextstate as lu_byte;
    (*g).sweepgc = nextlist;
    return 0i32 as lu_mem;
}
/*
** Enter first sweep phase.
** The call to 'sweeplist' tries to make pointer point to an object
** inside the list (instead of to the header), so that the real sweep do
** not need to skip objects created between "now" and the start of the
** real sweep.
*/
unsafe extern "C" fn entersweep(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    (*g).gcstate = 2i32 as lu_byte;
    (*g).sweepgc = sweeplist(L, &mut (*g).allgc, 1i32 as lu_mem);
}
unsafe extern "C" fn atomic(mut L: *mut lua_State) -> l_mem {
    let mut g: *mut global_State = (*L).l_G;
    let mut work: l_mem = 0;
    let mut origweak: *mut GCObject = 0 as *mut GCObject;
    let mut origall: *mut GCObject = 0 as *mut GCObject;
    /* save original list */
    let mut grayagain: *mut GCObject = (*g).grayagain;
    (*g).gcstate = (7i32 + 1i32) as lu_byte;
    /* start counting work */
    (*g).GCmemtrav = 0i32 as lu_mem;
    if 0 != (*L).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
        reallymarkobject(g, &mut (*(L as *mut GCUnion)).gc);
    }
    /* mark running thread */
    /* registry and global metatables may be changed by API */
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32
        && 0 != (*(*g).l_registry.value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    /* mark global metatables */
    markmt(g);
    /* remark occasional upvalues of (maybe) dead threads */
    remarkupvals(g);
    /* propagate changes */
    propagateall(g);
    /* stop counting (do not recount 'grayagain') */
    work = (*g).GCmemtrav as l_mem;
    (*g).gray = grayagain;
    /* traverse 'grayagain' list */
    propagateall(g);
    /* restart counting */
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    /* at this point, all strongly accessible objects are marked. */
    /* Clear values from weak tables, before checking finalizers */
    clearvalues(g, (*g).weak, 0 as *mut GCObject);
    clearvalues(g, (*g).allweak, 0 as *mut GCObject);
    origweak = (*g).weak;
    origall = (*g).allweak;
    /* stop counting (objects being finalized) */
    work = (work as lua_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    /* separate objects to be finalized */
    separatetobefnz(g, 0i32);
    /* there may be objects to be finalized */
    (*g).gcfinnum = 1i32 as lua_uint;
    /* mark objects that will be finalized */
    markbeingfnz(g);
    /* remark, to propagate 'resurrection' */
    propagateall(g);
    /* restart counting */
    (*g).GCmemtrav = 0i32 as lu_mem;
    convergeephemerons(g);
    /* at this point, all resurrected objects are marked. */
    /* remove dead objects from weak tables */
    /* clear keys from all ephemeron tables */
    clearkeys(g, (*g).ephemeron, 0 as *mut GCObject);
    /* clear keys from all 'allweak' tables */
    clearkeys(g, (*g).allweak, 0 as *mut GCObject);
    /* clear values from resurrected weak tables */
    clearvalues(g, (*g).weak, origweak);
    clearvalues(g, (*g).allweak, origall);
    luaS_clearcache(g);
    /* flip current white */
    (*g).currentwhite =
        ((*g).currentwhite as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    /* complete counting */
    work = (work as lua_ulong).wrapping_add((*g).GCmemtrav) as l_mem as l_mem;
    /* estimate of memory marked by 'atomic' */
    return work;
}
/*
** clear entries with unmarked values from all weaktables in list 'l' up
** to element 'f'
*/
unsafe extern "C" fn clearvalues(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) -> () {
    while l != f {
        let mut h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node = &mut *(*h)
            .node
            .offset((1i32 << (*h).lsizenode as lua_int) as size_t as isize)
            as *mut Node;
        let mut i: lua_uint = 0;
        i = 0i32 as lua_uint;
        while i < (*h).sizearray {
            let mut o: *mut TValue = &mut *(*h).array.offset(i as isize) as *mut TValue;
            /* value was collected? */
            if 0 != iscleared(g, o) {
                /* remove value */
                (*o).tt_ = 0i32
            }
            i = i.wrapping_add(1)
        }
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32) && 0 != iscleared(g, &mut (*n).i_val) {
                /* remove value ... */
                (*n).i_val.tt_ = 0i32;
                /* and remove entry from table */
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        l = (*(l as *mut GCUnion)).h.gclist
    }
}
/*
** {======================================================
** Generic functions
** =======================================================
*/
/*
** one after last element in a hash array
*/
/*
** link collectable object 'o' into list pointed by 'p'
*/
/*
** If key is not marked, mark its entry as dead. This allows key to be
** collected, but keeps its entry in the table.  A dead node is needed
** when Lua looks up for a key (it may be part of a chain) and when
** traversing a weak table (key might be removed from the table during
** traversal). Other places never manipulate dead keys, because its
** associated nil value is enough to signal that the entry is logically
** empty.
*/
unsafe extern "C" fn removeentry(mut n: *mut Node) -> () {
    if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
        && 0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
            .value_
            .gc)
            .marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        /* unused and unmarked key; remove it */
        (*n).i_key.nk.tt_ = 9i32 + 1i32
    };
}
/*
** tells whether a key or value can be cleared from a weak
** table. Non-collectable objects are never removed from weak
** tables. Strings behave as 'values', so are never removed too. for
** other objects: if really collected, cannot keep them; for objects
** being finalized, keep them in keys, but not in values
*/
unsafe extern "C" fn iscleared(mut g: *mut global_State, mut o: *const TValue) -> lua_int {
    if 0 == (*o).tt_ & 1i32 << 6i32 {
        return 0i32;
    } else if (*o).tt_ & 0xfi32 == 4i32 {
        if 0 != (*((*o).value_.gc as *mut GCUnion)).ts.marked as lua_int
            & (1i32 << 0i32 | 1i32 << 1i32)
        {
            reallymarkobject(
                g,
                &mut (*(&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString
                    as *mut GCUnion))
                    .gc,
            );
        }
        /* strings are 'values', so are never weak */
        return 0i32;
    } else {
        return (*(*o).value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32);
    };
}
/*
** $Id: lgc.c,v 2.215.1.2 2017/08/31 16:15:27 roberto Exp $
** Garbage Collector
** See Copyright Notice in lua.h
*/
/*
** internal state for collector while inside the atomic phase. The
** collector should never be in this state while running regular code.
*/
/*
** cost of sweeping one element (the size of a small object divided
** by some adjust for the sweep speed)
*/
/* maximum number of elements to sweep in each single step */
/* cost of calling one finalizer */
/*
** macro to adjust 'stepmul': 'stepmul' is actually used like
** 'stepmul / STEPMULADJ' (value chosen by tests)
*/
/*
** macro to adjust 'pause': 'pause' is actually used like
** 'pause / PAUSEADJ' (value chosen by tests)
*/
/*
** 'makewhite' erases all color bits then sets only the current white
** bit
*/
/*
** mark an object that can be NULL (either because it is really optional,
** or it was stripped as debug info, or inside an uncompleted structure)
*/
unsafe extern "C" fn reallymarkobject(mut g: *mut global_State, mut o: *mut GCObject) -> () {
    loop {
        (*o).marked = ((*o).marked as lua_int
            & !(1i32 << 0i32 | 1i32 << 1i32) as lu_byte as lua_int)
            as lu_byte;
        match (*o).tt as lua_int {
            4 => {
                (*o).marked = ((*o).marked as lua_int | 1i32 << 2i32) as lu_byte;
                (*g).GCmemtrav = ((*g).GCmemtrav as lua_ulong).wrapping_add(
                    (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
                        (((*(o as *mut GCUnion)).ts.shrlen as lua_int + 1i32) as lua_ulong)
                            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
                    ),
                ) as lu_mem as lu_mem;
                break;
            }
            20 => {
                (*o).marked = ((*o).marked as lua_int | 1i32 << 2i32) as lu_byte;
                (*g).GCmemtrav = ((*g).GCmemtrav as lua_ulong).wrapping_add(
                    (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
                        (*(o as *mut GCUnion))
                            .ts
                            .u
                            .lnglen
                            .wrapping_add(1i32 as lua_ulong)
                            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
                    ),
                ) as lu_mem as lu_mem;
                break;
            }
            7 => {
                let mut uvalue: TValue = lua_TValue {
                    value_: Value {
                        gc: 0 as *mut GCObject,
                    },
                    tt_: 0,
                };
                if !(*(o as *mut GCUnion)).u.metatable.is_null() {
                    if 0 != (*(*(o as *mut GCUnion)).u.metatable).marked as lua_int
                        & (1i32 << 0i32 | 1i32 << 1i32)
                    {
                        reallymarkobject(
                            g,
                            &mut (*((*(o as *mut GCUnion)).u.metatable as *mut GCUnion)).gc,
                        );
                    }
                }
                /* mark its metatable */
                (*o).marked = ((*o).marked as lua_int | 1i32 << 2i32) as lu_byte;
                (*g).GCmemtrav = ((*g).GCmemtrav as lua_ulong).wrapping_add(
                    (::std::mem::size_of::<UUdata>() as lua_ulong)
                        .wrapping_add((*(o as *mut GCUnion)).u.len),
                ) as lu_mem as lu_mem;
                let mut io: *mut TValue = &mut uvalue;
                let mut iu: *const Udata = &mut (*(o as *mut GCUnion)).u;
                (*io).value_ = (*iu).user_;
                (*io).tt_ = (*iu).ttuv_ as lua_int;
                if !(0 != uvalue.tt_ & 1i32 << 6i32
                    && 0 != (*uvalue.value_.gc).marked as lua_int
                        & (1i32 << 0i32 | 1i32 << 1i32))
                {
                    break;
                }
                /* markvalue(g, &uvalue); */
                o = uvalue.value_.gc
            }
            6 => {
                let ref mut fresh1 = (*(o as *mut GCUnion)).cl.l.gclist;
                *fresh1 = (*g).gray;
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).cl.l as *mut LClosure as *mut GCUnion)).gc;
                break;
            }
            38 => {
                let ref mut fresh2 = (*(o as *mut GCUnion)).cl.c.gclist;
                *fresh2 = (*g).gray;
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).cl.c as *mut CClosure as *mut GCUnion)).gc;
                break;
            }
            5 => {
                let ref mut fresh3 = (*(o as *mut GCUnion)).h.gclist;
                *fresh3 = (*g).gray;
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).h as *mut Table as *mut GCUnion)).gc;
                break;
            }
            8 => {
                let ref mut fresh4 = (*(o as *mut GCUnion)).th.gclist;
                *fresh4 = (*g).gray;
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).th as *mut lua_State as *mut GCUnion)).gc;
                break;
            }
            9 => {
                let ref mut fresh5 = (*(o as *mut GCUnion)).p.gclist;
                *fresh5 = (*g).gray;
                (*g).gray =
                    &mut (*(&mut (*(o as *mut GCUnion)).p as *mut Proto as *mut GCUnion)).gc;
                break;
            }
            _ => {
                break;
            }
        }
    }
}
/* }====================================================== */
/*
** {======================================================
** Sweep Functions
** =======================================================
*/
/*
** clear entries with unmarked keys from all weaktables in list 'l' up
** to element 'f'
*/
unsafe extern "C" fn clearkeys(
    mut g: *mut global_State,
    mut l: *mut GCObject,
    mut f: *mut GCObject,
) -> () {
    while l != f {
        let mut h: *mut Table = &mut (*(l as *mut GCUnion)).h;
        let mut n: *mut Node = 0 as *mut Node;
        let mut limit: *mut Node = &mut *(*h)
            .node
            .offset((1i32 << (*h).lsizenode as lua_int) as size_t as isize)
            as *mut Node;
        n = &mut *(*h).node.offset(0isize) as *mut Node;
        while n < limit {
            if !((*n).i_val.tt_ == 0i32)
                && 0 != iscleared(g, &mut (*n).i_key.tvk as *mut TValue as *const TValue)
            {
                /* remove value ... */
                (*n).i_val.tt_ = 0i32
            }
            /* is entry empty? */
            if (*n).i_val.tt_ == 0i32 {
                /* remove entry from table */
                removeentry(n);
            }
            n = n.offset(1isize)
        }
        l = (*(l as *mut GCUnion)).h.gclist
    }
}
unsafe extern "C" fn convergeephemerons(mut g: *mut global_State) -> () {
    let mut changed: lua_int = 0;
    loop {
        let mut w: *mut GCObject = 0 as *mut GCObject;
        /* get ephemeron list */
        let mut next: *mut GCObject = (*g).ephemeron;
        /* tables may return to this list when traversed */
        (*g).ephemeron = 0 as *mut GCObject;
        changed = 0i32;
        loop {
            w = next;
            if w.is_null() {
                break;
            }
            next = (*(w as *mut GCUnion)).h.gclist;
            if !(0 != traverseephemeron(g, &mut (*(w as *mut GCUnion)).h)) {
                continue;
            }
            /* traverse marked some value? */
            /* propagate changes */
            propagateall(g);
            /* will have to revisit all ephemeron tables */
            changed = 1i32
        }
        if !(0 != changed) {
            break;
        }
    }
}
unsafe extern "C" fn propagateall(mut g: *mut global_State) -> () {
    while !(*g).gray.is_null() {
        propagatemark(g);
    }
}
/*
** traverse one gray object, turning it to black (except for threads,
** which are always gray).
*/
unsafe extern "C" fn propagatemark(mut g: *mut global_State) -> () {
    let mut size: lu_mem = 0;
    let mut o: *mut GCObject = (*g).gray;
    (*o).marked = ((*o).marked as lua_int | 1i32 << 2i32) as lu_byte;
    match (*o).tt as lua_int {
        5 => {
            let mut h: *mut Table = &mut (*(o as *mut GCUnion)).h;
            /* remove from 'gray' list */
            (*g).gray = (*h).gclist;
            size = traversetable(g, h)
        }
        6 => {
            let mut cl: *mut LClosure = &mut (*(o as *mut GCUnion)).cl.l;
            /* remove from 'gray' list */
            (*g).gray = (*cl).gclist;
            size = traverseLclosure(g, cl)
        }
        38 => {
            let mut cl_0: *mut CClosure = &mut (*(o as *mut GCUnion)).cl.c;
            /* remove from 'gray' list */
            (*g).gray = (*cl_0).gclist;
            size = traverseCclosure(g, cl_0)
        }
        8 => {
            let mut th: *mut lua_State = &mut (*(o as *mut GCUnion)).th;
            /* remove from 'gray' list */
            (*g).gray = (*th).gclist;
            /* insert into 'grayagain' list */
            (*th).gclist = (*g).grayagain;
            (*g).grayagain = &mut (*(th as *mut GCUnion)).gc;
            (*o).marked =
                ((*o).marked as lua_int & !(1i32 << 2i32) as lu_byte as lua_int) as lu_byte;
            size = traversethread(g, th)
        }
        9 => {
            let mut p: *mut Proto = &mut (*(o as *mut GCUnion)).p;
            /* remove from 'gray' list */
            (*g).gray = (*p).gclist;
            size = traverseproto(g, p) as lu_mem
        }
        _ => return,
    }
    (*g).GCmemtrav = ((*g).GCmemtrav as lua_ulong).wrapping_add(size) as lu_mem as lu_mem;
}
/*
** Traverse a prototype. (While a prototype is being build, its
** arrays can be larger than needed; the extra slots are filled with
** NULL, so the use of 'markobjectN')
*/
unsafe extern "C" fn traverseproto(mut g: *mut global_State, mut f: *mut Proto) -> lua_int {
    let mut i: lua_int = 0;
    if !(*f).cache.is_null()
        && 0 != (*(*f).cache).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        /* allow cache to be collected */
        (*f).cache = 0 as *mut LClosure
    }
    if !(*f).source.is_null() {
        if 0 != (*(*f).source).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
            reallymarkobject(g, &mut (*((*f).source as *mut GCUnion)).gc);
        }
    }
    /* mark literals */
    i = 0i32;
    while i < (*f).sizek {
        if 0 != (*(*f).k.offset(i as isize)).tt_ & 1i32 << 6i32
            && 0 != (*(*(*f).k.offset(i as isize)).value_.gc).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        {
            reallymarkobject(g, (*(*f).k.offset(i as isize)).value_.gc);
        }
        i += 1
    }
    /* mark upvalue names */
    i = 0i32;
    while i < (*f).sizeupvalues {
        if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
            if 0 != (*(*(*f).upvalues.offset(i as isize)).name).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(
                    g,
                    &mut (*((*(*f).upvalues.offset(i as isize)).name as *mut GCUnion)).gc,
                );
            }
        }
        i += 1
    }
    /* mark nested protos */
    i = 0i32;
    while i < (*f).sizep {
        if !(*(*f).p.offset(i as isize)).is_null() {
            if 0 != (**(*f).p.offset(i as isize)).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(g, &mut (*(*(*f).p.offset(i as isize) as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
    /* mark local-variable names */
    i = 0i32;
    while i < (*f).sizelocvars {
        if !(*(*f).locvars.offset(i as isize)).varname.is_null() {
            if 0 != (*(*(*f).locvars.offset(i as isize)).varname).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(
                    g,
                    &mut (*((*(*f).locvars.offset(i as isize)).varname as *mut GCUnion)).gc,
                );
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<Proto>() as lua_ulong)
        .wrapping_add(
            (::std::mem::size_of::<Instruction>() as lua_ulong)
                .wrapping_mul((*f).sizecode as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<*mut Proto>() as lua_ulong)
                .wrapping_mul((*f).sizep as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<TValue>() as lua_ulong)
                .wrapping_mul((*f).sizek as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<lua_int>() as lua_ulong)
                .wrapping_mul((*f).sizelineinfo as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<LocVar>() as lua_ulong)
                .wrapping_mul((*f).sizelocvars as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<Upvaldesc>() as lua_ulong)
                .wrapping_mul((*f).sizeupvalues as lua_ulong),
        ) as lua_int;
}
unsafe extern "C" fn traversethread(mut g: *mut global_State, mut th: *mut lua_State) -> lu_mem {
    let mut o: StkId = (*th).stack;
    if o.is_null() {
        /* stack not completely built yet */
        return 1i32 as lu_mem;
    } else {
        /* mark live elements in the stack */
        while o < (*th).top {
            if 0 != (*o).tt_ & 1i32 << 6i32
                && 0 != (*(*o).value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(g, (*o).value_.gc);
            }
            o = o.offset(1isize)
        }
        if (*g).gcstate as lua_int == 7i32 + 1i32 {
            /* final traversal? */
            /* real end of stack */
            let mut lim: StkId = (*th).stack.offset((*th).stacksize as isize);
            /* clear not-marked stack slice */
            while o < lim {
                (*o).tt_ = 0i32;
                o = o.offset(1isize)
            }
            /* 'remarkupvals' may have removed thread from 'twups' list */
            if !((*th).twups != th) && !(*th).openupval.is_null() {
                /* link it back to the list */
                (*th).twups = (*g).twups;
                (*g).twups = th
            }
        } else if (*g).gckind as lua_int != 1i32 {
            /* do not change stack in emergency cycle */
            luaD_shrinkstack(th);
        }
        return (::std::mem::size_of::<lua_State>() as lua_ulong)
            .wrapping_add(
                (::std::mem::size_of::<TValue>() as lua_ulong)
                    .wrapping_mul((*th).stacksize as lua_ulong),
            )
            .wrapping_add(
                (::std::mem::size_of::<CallInfo>() as lua_ulong)
                    .wrapping_mul((*th).nci as lua_ulong),
            );
    };
}
unsafe extern "C" fn traverseCclosure(mut g: *mut global_State, mut cl: *mut CClosure) -> lu_mem {
    let mut i: lua_int = 0;
    /* mark its upvalues */
    i = 0i32;
    while i < (*cl).nupvalues as lua_int {
        if 0 != (*cl).upvalue[i as usize].tt_ & 1i32 << 6i32
            && 0 != (*(*cl).upvalue[i as usize].value_.gc).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        {
            reallymarkobject(g, (*cl).upvalue[i as usize].value_.gc);
        }
        i += 1
    }
    return (::std::mem::size_of::<CClosure>() as lua_ulong as lua_int
        + (::std::mem::size_of::<TValue>() as lua_ulong)
            .wrapping_mul(((*cl).nupvalues as lua_int - 1i32) as lua_ulong)
            as lua_int) as lu_mem;
}
/*
** open upvalues point to values in a thread, so those values should
** be marked when the thread is traversed except in the atomic phase
** (because then the value cannot be changed by the thread and the
** thread may not be traversed again)
*/
unsafe extern "C" fn traverseLclosure(mut g: *mut global_State, mut cl: *mut LClosure) -> lu_mem {
    let mut i: lua_int = 0;
    if !(*cl).p.is_null() {
        if 0 != (*(*cl).p).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
            reallymarkobject(g, &mut (*((*cl).p as *mut GCUnion)).gc);
        }
    }
    /* mark its prototype */
    i = 0i32;
    while i < (*cl).nupvalues as lua_int {
        /* mark its upvalues */
        let mut uv: *mut UpVal = (*cl).upvals[i as usize];
        if !uv.is_null() {
            if (*uv).v != &mut (*uv).u.value as *mut TValue
                && (*g).gcstate as lua_int != 7i32 + 1i32
            {
                /* can be marked in 'remarkupvals' */
                (*uv).u.open.touched = 1i32
            } else if 0 != (*(*uv).v).tt_ & 1i32 << 6i32
                && 0 != (*(*(*uv).v).value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(g, (*(*uv).v).value_.gc);
            }
        }
        i += 1
    }
    return (::std::mem::size_of::<LClosure>() as lua_ulong as lua_int
        + (::std::mem::size_of::<*mut TValue>() as lua_ulong)
            .wrapping_mul(((*cl).nupvalues as lua_int - 1i32) as lua_ulong)
            as lua_int) as lu_mem;
}
unsafe extern "C" fn traversetable(mut g: *mut global_State, mut h: *mut Table) -> lu_mem {
    let mut weakkey: *const lua_char = 0 as *const lua_char;
    let mut weakvalue: *const lua_char = 0 as *const lua_char;
    let mut mode: *const TValue = if (*h).metatable.is_null() {
        0 as *const TValue
    } else if 0 != (*(*h).metatable).flags as lua_uint & 1u32 << TM_MODE as lua_int {
        0 as *const TValue
    } else {
        luaT_gettm(
            (*h).metatable,
            TM_MODE,
            (*g).tmname[TM_MODE as lua_int as usize],
        )
    };
    if !(*h).metatable.is_null() {
        if 0 != (*(*h).metatable).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
            reallymarkobject(g, &mut (*((*h).metatable as *mut GCUnion)).gc);
        }
    }
    /* is there a weak mode? */
    if !mode.is_null() && (*mode).tt_ & 0xfi32 == 4i32 && {
        weakkey = strchr(
            (&mut (*((*mode).value_.gc as *mut GCUnion)).ts as *mut TString as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            'k' as i32,
        );
        weakvalue = strchr(
            (&mut (*((*mode).value_.gc as *mut GCUnion)).ts as *mut TString as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            'v' as i32,
        );
        !weakkey.is_null() || !weakvalue.is_null()
    } {
        /* is really weak? */
        /* keep table gray */
        (*h).marked =
            ((*h).marked as lua_int & !(1i32 << 2i32) as lu_byte as lua_int) as lu_byte;
        /* strong keys? */
        if weakkey.is_null() {
            traverseweakvalue(g, h);
        } else if weakvalue.is_null() {
            traverseephemeron(g, h);
        } else {
            /* all weak */
            /* nothing to traverse now */
            (*h).gclist = (*g).allweak;
            (*g).allweak = &mut (*(h as *mut GCUnion)).gc
        }
    } else {
        /* not weak */
        traversestrongtable(g, h);
    }
    return (::std::mem::size_of::<Table>() as lua_ulong)
        .wrapping_add(
            (::std::mem::size_of::<TValue>() as lua_ulong)
                .wrapping_mul((*h).sizearray as lua_ulong),
        )
        .wrapping_add(
            (::std::mem::size_of::<Node>() as lua_ulong).wrapping_mul(
                (if (*h).lastfree.is_null() {
                    0i32
                } else {
                    1i32 << (*h).lsizenode as lua_int
                }) as size_t,
            ),
        );
}
unsafe extern "C" fn traversestrongtable(mut g: *mut global_State, mut h: *mut Table) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as lua_int) as size_t as isize)
        as *mut Node;
    let mut i: lua_uint = 0;
    /* traverse array part */
    i = 0i32 as lua_uint;
    while i < (*h).sizearray {
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32
            && 0 != (*(*(*h).array.offset(i as isize)).value_.gc).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        {
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        /* traverse hash part */
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else {
            if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                && 0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .gc)
                    .marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc,
                );
            }
            /* mark key */
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32
                && 0 != (*(*n).i_val.value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(g, (*n).i_val.value_.gc);
            }
        }
        /* mark value */
        n = n.offset(1isize)
    }
}
/*
** Traverse an ephemeron table and link it to proper list. Returns true
** iff any object was marked during this traversal (which implies that
** convergence has to continue). During propagation phase, keep table
** in 'grayagain' list, to be visited again in the atomic phase. In
** the atomic phase, if table has any white->white entry, it has to
** be revisited during ephemeron convergence (as that key may turn
** black). Otherwise, if it has any white key, table has to be cleared
** (in the atomic phase).
*/
unsafe extern "C" fn traverseephemeron(mut g: *mut global_State, mut h: *mut Table) -> lua_int {
    /* true if an object is marked in this traversal */
    let mut marked: lua_int = 0i32;
    /* true if table has white keys */
    let mut hasclears: lua_int = 0i32;
    /* true if table has entry "white-key -> white-value" */
    let mut hasww: lua_int = 0i32;
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as lua_int) as size_t as isize)
        as *mut Node;
    let mut i: lua_uint = 0;
    /* traverse array part */
    i = 0i32 as lua_uint;
    while i < (*h).sizearray {
        if 0 != (*(*h).array.offset(i as isize)).tt_ & 1i32 << 6i32
            && 0 != (*(*(*h).array.offset(i as isize)).value_.gc).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
        {
            marked = 1i32;
            reallymarkobject(g, (*(*h).array.offset(i as isize)).value_.gc);
        }
        i = i.wrapping_add(1)
    }
    /* traverse hash part */
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else if 0 != iscleared(g, &mut (*n).i_key.tvk as *mut TValue as *const TValue) {
            /* key is not marked (yet)? */
            /* table must be cleared */
            hasclears = 1i32;
            /* value not marked yet? */
            if 0 != (*n).i_val.tt_ & 1i32 << 6i32
                && 0 != (*(*n).i_val.value_.gc).marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                /* white-white entry */
                hasww = 1i32
            }
        } else if 0 != (*n).i_val.tt_ & 1i32 << 6i32
            && 0 != (*(*n).i_val.value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            /* value not marked yet? */
            marked = 1i32;
            /* mark it now */
            reallymarkobject(g, (*n).i_val.value_.gc);
        }
        n = n.offset(1isize)
    }
    /* link table into proper list */
    if (*g).gcstate as lua_int == 0i32 {
        /* must retraverse it in atomic phase */
        (*h).gclist = (*g).grayagain;
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasww {
        /* have to propagate again */
        (*h).gclist = (*g).ephemeron;
        (*g).ephemeron = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasclears {
        /* may have to clean white keys */
        (*h).gclist = (*g).allweak;
        (*g).allweak = &mut (*(h as *mut GCUnion)).gc
    }
    return marked;
}
/* }====================================================== */
/*
** {======================================================
** Traverse functions
** =======================================================
*/
/*
** Traverse a table with weak values and link it to proper list. During
** propagate phase, keep it in 'grayagain' list, to be revisited in the
** atomic phase. In the atomic phase, if table has any white value,
** put it in 'weak' list, to be cleared.
*/
unsafe extern "C" fn traverseweakvalue(mut g: *mut global_State, mut h: *mut Table) -> () {
    let mut n: *mut Node = 0 as *mut Node;
    let mut limit: *mut Node = &mut *(*h)
        .node
        .offset((1i32 << (*h).lsizenode as lua_int) as size_t as isize)
        as *mut Node;
    /* if there is array part, assume it may have white values (it is not
    worth traversing it now just to check) */
    let mut hasclears: lua_int = ((*h).sizearray > 0i32 as lua_uint) as lua_int;
    n = &mut *(*h).node.offset(0isize) as *mut Node;
    while n < limit {
        /* traverse hash part */
        /* entry is empty? */
        if (*n).i_val.tt_ == 0i32 {
            /* remove it */
            removeentry(n);
        } else {
            if 0 != (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ & 1i32 << 6i32
                && 0 != (*(*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .gc)
                    .marked as lua_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                reallymarkobject(
                    g,
                    (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc,
                );
            }
            /* mark key */
            /* is there a white value? */
            if 0 == hasclears && 0 != iscleared(g, &mut (*n).i_val) {
                /* table will have to be cleared */
                hasclears = 1i32
            }
        }
        n = n.offset(1isize)
    }
    if (*g).gcstate as lua_int == 0i32 {
        /* must retraverse it in atomic phase */
        (*h).gclist = (*g).grayagain;
        (*g).grayagain = &mut (*(h as *mut GCUnion)).gc
    } else if 0 != hasclears {
        /* has to be cleared later */
        (*h).gclist = (*g).weak;
        (*g).weak = &mut (*(h as *mut GCUnion)).gc
    };
}
/*
** mark all objects in list of being-finalized
*/
unsafe extern "C" fn markbeingfnz(mut g: *mut global_State) -> () {
    let mut o: *mut GCObject = 0 as *mut GCObject;
    o = (*g).tobefnz;
    while !o.is_null() {
        if 0 != (*o).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
        o = (*o).next
    }
}
/*
** Mark all values stored in marked open upvalues from non-marked threads.
** (Values from marked threads were already marked when traversing the
** thread.) Remove from the list threads that no longer have upvalues and
** not-marked threads.
*/
unsafe extern "C" fn remarkupvals(mut g: *mut global_State) -> () {
    let mut thread: *mut lua_State = 0 as *mut lua_State;
    let mut p: *mut *mut lua_State = &mut (*g).twups;
    loop {
        thread = *p;
        if thread.is_null() {
            break;
        }
        /* threads are never black */
        if 0 == (*thread).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32 | 1i32 << 2i32)
            && !(*thread).openupval.is_null()
        {
            /* keep marked thread with upvalues in the list */
            p = &mut (*thread).twups
        } else {
            /* thread is not marked or without upvalues */
            let mut uv: *mut UpVal = 0 as *mut UpVal;
            /* remove thread from the list */
            *p = (*thread).twups;
            /* mark that it is out of list */
            (*thread).twups = thread;
            uv = (*thread).openupval;
            while !uv.is_null() {
                if 0 != (*uv).u.open.touched {
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32
                        && 0 != (*(*(*uv).v).value_.gc).marked as lua_int
                            & (1i32 << 0i32 | 1i32 << 1i32)
                    {
                        reallymarkobject(g, (*(*uv).v).value_.gc);
                    }
                    /* remark upvalue's value */
                    (*uv).u.open.touched = 0i32
                }
                uv = (*uv).u.open.next
            }
        }
    }
}
/*
** mark metamethods for basic types
*/
unsafe extern "C" fn markmt(mut g: *mut global_State) -> () {
    let mut i: lua_int = 0;
    i = 0i32;
    while i < 9i32 {
        if !(*g).mt[i as usize].is_null() {
            if 0 != (*(*g).mt[i as usize]).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
                reallymarkobject(g, &mut (*((*g).mt[i as usize] as *mut GCUnion)).gc);
            }
        }
        i += 1
    }
}
/*
** mark root set and reset all gray lists, to start a new collection
*/
unsafe extern "C" fn restartcollection(mut g: *mut global_State) -> () {
    (*g).grayagain = 0 as *mut GCObject;
    (*g).gray = (*g).grayagain;
    (*g).ephemeron = 0 as *mut GCObject;
    (*g).allweak = (*g).ephemeron;
    (*g).weak = (*g).allweak;
    if 0 != (*(*g).mainthread).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
        reallymarkobject(g, &mut (*((*g).mainthread as *mut GCUnion)).gc);
    }
    if 0 != (*g).l_registry.tt_ & 1i32 << 6i32
        && 0 != (*(*g).l_registry.value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        reallymarkobject(g, (*g).l_registry.value_.gc);
    }
    markmt(g);
    /* mark any finalizing object left from previous cycle */
    markbeingfnz(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_runtilstate(
    mut L: *mut lua_State,
    mut statesmask: lua_int,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    while 0 == statesmask & 1i32 << (*g).gcstate as lua_int {
        singlestep(L);
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaC_fullgc(mut L: *mut lua_State, mut isemergency: lua_int) -> () {
    let mut g: *mut global_State = (*L).l_G;
    if 0 != isemergency {
        /* set flag */
        (*g).gckind = 1i32 as lu_byte
    }
    if (*g).gcstate as lua_int <= 1i32 {
        /* black objects? */
        /* sweep everything to turn them back to white */
        entersweep(L);
    }
    /* finish any pending sweep phase to start a new cycle */
    luaC_runtilstate(L, 1i32 << 7i32);
    /* start new collection */
    luaC_runtilstate(L, !(1i32 << 7i32));
    /* run up to finalizers */
    luaC_runtilstate(L, 1i32 << 6i32);
    /* estimate must be correct after a full GC cycle */
    /* finish collection */
    luaC_runtilstate(L, 1i32 << 7i32);
    (*g).gckind = 0i32 as lu_byte;
    setpause(g);
}
#[no_mangle]
pub unsafe extern "C" fn luaC_newobj(
    mut L: *mut lua_State,
    mut tt: lua_int,
    mut sz: size_t,
) -> *mut GCObject {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject =
        luaM_realloc_(L, 0 as *mut lua_void, (tt & 0xfi32) as size_t, sz) as *mut GCObject;
    (*o).marked = ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte;
    (*o).tt = tt as lu_byte;
    (*o).next = (*g).allgc;
    (*g).allgc = o;
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrier_(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut v: *mut GCObject,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* must keep invariant? */
    if (*g).gcstate as lua_int <= 1i32 {
        /* restore invariant */
        reallymarkobject(g, v);
    } else {
        (*o).marked = ((*o).marked as lua_int & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
            | ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                as lua_int) as lu_byte
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_barrierback_(mut L: *mut lua_State, mut t: *mut Table) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* make table gray (again) */
    (*t).marked =
        ((*t).marked as lua_int & !(1i32 << 2i32) as lu_byte as lua_int) as lu_byte;
    (*t).gclist = (*g).grayagain;
    (*g).grayagain = &mut (*(t as *mut GCUnion)).gc;
}
#[no_mangle]
pub unsafe extern "C" fn luaC_upvalbarrier_(mut L: *mut lua_State, mut uv: *mut UpVal) -> () {
    let mut g: *mut global_State = (*L).l_G;
    let mut o: *mut GCObject = (*(*uv).v).value_.gc;
    /* ensured by macro luaC_upvalbarrier */
    if (*g).gcstate as lua_int <= 1i32 {
        if 0 != (*o).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32) {
            reallymarkobject(g, &mut (*(o as *mut GCUnion)).gc);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaC_checkfinalizer(
    mut L: *mut lua_State,
    mut o: *mut GCObject,
    mut mt: *mut Table,
) -> () {
    let mut g: *mut global_State = (*L).l_G;
    /* obj. is already marked... */
    if 0 != (*o).marked as lua_int & 1i32 << 3i32 || if mt.is_null() {
        0 as *const TValue
    } else if 0 != (*mt).flags as lua_uint & 1u32 << TM_GC as lua_int {
        0 as *const TValue
    } else {
        luaT_gettm(mt, TM_GC, (*g).tmname[TM_GC as lua_int as usize])
    }
    .is_null()
    {
        /* or has no finalizer? */
        /* nothing to be done */
        return;
    } else {
        /* move 'o' to 'finobj' list */
        let mut p: *mut *mut GCObject = 0 as *mut *mut GCObject;
        if 2i32 <= (*g).gcstate as lua_int && (*g).gcstate as lua_int <= 5i32 {
            /* "sweep" object 'o' */
            (*o).marked = ((*o).marked as lua_int
                & !(1i32 << 2i32 | (1i32 << 0i32 | 1i32 << 1i32))
                | ((*g).currentwhite as lua_int & (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
                    as lua_int) as lu_byte;
            /* should not remove 'sweepgc' object */
            if (*g).sweepgc == &mut (*o).next as *mut *mut GCObject {
                /* change 'sweepgc' */
                (*g).sweepgc = sweeptolive(L, (*g).sweepgc)
            }
        }
        /* search for pointer pointing to 'o' */
        p = &mut (*g).allgc;
        while *p != o {
            /* empty */
            p = &mut (**p).next
        }
        /* remove 'o' from 'allgc' list */
        *p = (*o).next;
        /* link it in 'finobj' list */
        (*o).next = (*g).finobj;
        (*g).finobj = o;
        /* mark it as such */
        (*o).marked = ((*o).marked as lua_int | 1i32 << 3i32) as lu_byte;
        return;
    };
}
/*
** sweep a list until a live object (or end of list)
*/
unsafe extern "C" fn sweeptolive(
    mut L: *mut lua_State,
    mut p: *mut *mut GCObject,
) -> *mut *mut GCObject {
    let mut old: *mut *mut GCObject = p;
    loop {
        p = sweeplist(L, p, 1i32 as lu_mem);
        if !(p == old) {
            break;
        }
    }
    return p;
}
