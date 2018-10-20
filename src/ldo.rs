use libc;
use lua::*;
use llimits::*;
extern "C" {
    #[no_mangle]
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    #[no_mangle]
    fn _longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    #[no_mangle]
    fn abort() -> !;
    #[no_mangle]
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaV_execute(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaV_finishOp(L: *mut lua_State) -> ();
    /*
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
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
    fn luaE_shrinkCI(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaE_freeCI(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaF_close(L: *mut lua_State, level: StkId) -> ();
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    /*
     ** $Id: ldebug.h,v 2.14.1.1 2017/04/19 17:20:42 roberto Exp $
     ** Auxiliary functions from Debug Interface module
     ** See Copyright Notice in lua.h
     */
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State, o: *const TValue, opname: *const libc::c_char) -> !;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaE_extendCI(L: *mut lua_State) -> *mut CallInfo;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    #[no_mangle]
    fn luaZ_fill(z: *mut ZIO) -> libc::c_int;
    #[no_mangle]
    fn luaF_initupvals(L: *mut lua_State, cl: *mut LClosure) -> ();
    #[no_mangle]
    fn luaY_parser(
        L: *mut lua_State,
        z: *mut ZIO,
        buff: *mut Mbuffer,
        dyd: *mut Dyndata,
        name: *const libc::c_char,
        firstchar: libc::c_int,
    ) -> *mut LClosure;
    /*
     ** $Id: lundump.h,v 1.45.1.1 2017/04/19 17:20:42 roberto Exp $
     ** load precompiled Lua chunks
     ** See Copyright Notice in lua.h
     */
    /* data to catch conversion errors */
    /* this is the official format */
    /* load one chunk; from lundump.c */
    #[no_mangle]
    fn luaU_undump(L: *mut lua_State, Z: *mut ZIO, name: *const libc::c_char) -> *mut LClosure;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __sig_atomic_t = libc::c_int;
pub type ptrdiff_t = libc::c_long;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_longjmp {
    pub previous: *mut lua_longjmp,
    pub b: jmp_buf,
    pub status: libc::c_int,
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
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State, _: *mut libc::c_void, _: *mut size_t)
        -> *const libc::c_char,
>;
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
/*
** $Id: ltm.h,v 2.22.1.1 2017/04/19 17:20:42 roberto Exp $
** Tag methods
** See Copyright Notice in lua.h
*/
/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER TM" and "ORDER OP"
*/
pub type TMS = libc::c_uint;
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
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*
** size and position of opcode arguments.
*/
/*
** limits for opcode arguments.
** we use (signed) int to manipulate most arguments,
** so they must fit in LUAI_BITSINT-1 bits (-1 for sign)
*/
/* 'sBx' is signed */
/* creates a mask with 'n' 1 bits at position 'p' */
/* creates a mask with 'n' 0 bits at position 'p' */
/*
** the following macros help to manipulate instructions
*/
/*
** Macros to operate RK indices
*/
/* this bit 1 means constant (0 means register) */
/* test whether value is a constant */
/* gets the index of the constant */
/* (for debugging only) */
/* code a constant index as a RK value */
/*
** invalid register that fits in 8 bits
*/
/*
** R(x) - register
** Kst(x) - constant (in constant table)
** RK(x) == if ISK(x) then Kst(INDEXK(x)) else R(x)
*/
/*
** grep "ORDER OP" if you change these enums
*/
pub type OpCode = libc::c_uint;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
/*
** $Id: lzio.h,v 1.31.1.1 2017/04/19 17:20:42 roberto Exp $
** Buffered streams
** See Copyright Notice in lua.h
*/
/* end of stream */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Zio {
    pub n: size_t,
    pub p: *const libc::c_char,
    pub reader: lua_Reader,
    pub data: *mut libc::c_void,
    pub L: *mut lua_State,
}
pub type ZIO = Zio;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Mbuffer {
    pub buffer: *mut libc::c_char,
    pub n: size_t,
    pub buffsize: size_t,
}
/* description of pending goto statements and label statements */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: libc::c_int,
    pub line: libc::c_int,
    pub nactvar: lu_byte,
}
/* list of labels or gotos */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
/* dynamic structures used by the parser */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_6,
    pub gt: Labellist,
    pub label: Labellist,
}
/* list of active local variables */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_6 {
    pub arr: *mut Vardesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
}
/* description of active local variable */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    pub idx: libc::c_short,
}
/*
** Execute a protected parser.
*/
/* data to 'f_parser' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SParser {
    pub z: *mut ZIO,
    pub buff: Mbuffer,
    pub dyd: Dyndata,
    pub mode: *const libc::c_char,
    pub name: *const libc::c_char,
}
/*
** coroutine functions
*/
#[no_mangle]
pub unsafe extern "C" fn lua_yieldk(
    mut L: *mut lua_State,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut ci: *mut CallInfo = (*L).ci;
    if (*L).nny as libc::c_int > 0i32 {
        if L != (*(*L).l_G).mainthread {
            luaG_runerror!(L, s!(b"attempt to yield across a C-call boundary\x00"),);
        } else {
            luaG_runerror!(L, s!(b"attempt to yield from outside a coroutine\x00"),);
        }
    } else {
        (*L).status = 1i32 as lu_byte;
        /* save current 'func' */
        (*ci).extra = ((*ci).func as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            /* inside a hook? */
            /* must be inside a hook */
            /* return to 'luaD_hook' */
            return 0i32;
        } else {
            /* is there a continuation? */
            (*ci).u.c.k = k;
            if (*ci).u.c.k.is_some() {
                /* save context */
                (*ci).u.c.ctx = ctx
            }
            /* protect stack below results */
            (*ci).func = (*L).top.offset(-(nresults as isize)).offset(-1isize);
            luaD_throw(L, 1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_throw(mut L: *mut lua_State, mut errcode: libc::c_int) -> ! {
    if !(*L).errorJmp.is_null() {
        /* thread has an error handler? */
        /* set status */
        ::std::ptr::write_volatile(&mut (*(*L).errorJmp).status as *mut libc::c_int, errcode);
        /* jump to it */
        _longjmp((*(*L).errorJmp).b.as_mut_ptr(), 1i32);
    } else {
        /* thread has no error handler */
        let mut g: *mut global_State = (*L).l_G;
        /* mark it as dead */
        (*L).status = errcode as lu_byte;
        if !(*(*g).mainthread).errorJmp.is_null() {
            /* main thread has a handler? */
            /* copy error obj. */
            let fresh0 = (*(*g).mainthread).top;
            (*(*g).mainthread).top = (*(*g).mainthread).top.offset(1);
            let mut io1: *mut TValue = fresh0;
            *io1 = *(*L).top.offset(-1isize);
            /* re-throw in main thread */
            luaD_throw((*g).mainthread, errcode);
        } else {
            /* no handler at all; abort */
            if (*g).panic.is_some() {
                /* panic function? */
                /* assume EXTRA_STACK */
                seterrorobj(L, errcode, (*L).top);
                if (*(*L).ci).top < (*L).top {
                    /* pushing msg. can break this invariant */
                    (*(*L).ci).top = (*L).top
                }
                /* call panic function (last chance to jump out) */
                (*g).panic.expect("non-null function pointer")(L);
            }
            abort();
        }
    };
}
unsafe extern "C" fn seterrorobj(
    mut L: *mut lua_State,
    mut errcode: libc::c_int,
    mut oldtop: StkId,
) -> () {
    match errcode {
        4 => {
            /* memory error? */
            /* reuse preregistered msg. */
            let mut io: *mut TValue = oldtop;
            let mut x_: *mut TString = (*(*L).l_G).memerrmsg;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32
        }
        6 => {
            let mut io_0: *mut TValue = oldtop;
            let mut x__0: *mut TString = luaS_newlstr(
                L,
                s!(b"error in error handling\x00"),
                (::std::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong),
            );
            (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
            (*io_0).tt_ = (*x__0).tt as libc::c_int | 1i32 << 6i32
        }
        _ => {
            /* error message on current top */
            let mut io1: *mut TValue = oldtop;
            *io1 = *(*L).top.offset(-1isize)
        }
    }
    (*L).top = oldtop.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_resume(
    mut L: *mut lua_State,
    mut from: *mut lua_State,
    mut nargs: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    /* save "number of non-yieldable" calls */
    let mut oldnny: libc::c_ushort = (*L).nny;
    if (*L).status as libc::c_int == LUA_OK {
        /* may be starting a coroutine */
        /* not in base level? */
        if (*L).ci != &mut (*L).base_ci as *mut CallInfo {
            return resume_error(L, s!(b"cannot resume non-suspended coroutine\x00"), nargs);
        }
    } else if (*L).status as libc::c_int != 1i32 {
        return resume_error(L, s!(b"cannot resume dead coroutine\x00"), nargs);
    }
    (*L).nCcalls = (if !from.is_null() {
        (*from).nCcalls as libc::c_int + 1i32
    } else {
        1i32
    }) as libc::c_ushort;
    if (*L).nCcalls as libc::c_int >= LUAI_MAXCCALLS {
        return resume_error(L, s!(b"C stack overflow\x00"), nargs);
    } else {
        /* allow yields */
        (*L).nny = 0i32 as libc::c_ushort;
        status = luaD_rawrunprotected(
            L,
            Some(resume),
            &mut nargs as *mut libc::c_int as *mut libc::c_void,
        );
        /* error calling 'lua_resume'? */
        if status == -1i32 {
            status = 2i32
        } else {
            /* continue running after recoverable errors */
            while status > 1i32 && 0 != recover(L, status) {
                /* unroll continuation */
                status = luaD_rawrunprotected(
                    L,
                    Some(unroll),
                    &mut status as *mut libc::c_int as *mut libc::c_void,
                )
            }
            if status > 1i32 {
                /* unrecoverable error? */
                /* mark thread as 'dead' */
                (*L).status = status as lu_byte;
                /* push error message */
                seterrorobj(L, status, (*L).top);
                (*(*L).ci).top = (*L).top
            }
        }
        /* normal end or yield */
        /* restore 'nny' */
        (*L).nny = oldnny;
        (*L).nCcalls = (*L).nCcalls.wrapping_sub(1);
        return status;
    };
}
/*
** Executes "full continuation" (everything in the stack) of a
** previously interrupted coroutine until the stack is empty (or another
** interruption long-jumps out of the loop). If the coroutine is
** recovering from an error, 'ud' points to the error status, which must
** be passed to the first continuation function (otherwise the default
** status is LUA_YIELD).
*/
unsafe extern "C" fn unroll(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    /* error status? */
    if !ud.is_null() {
        /* finish 'lua_pcallk' callee */
        finishCcall(L, *(ud as *mut libc::c_int));
    }
    while (*L).ci != &mut (*L).base_ci as *mut CallInfo {
        /* something in the stack */
        /* C function? */
        if 0 == (*(*L).ci).callstatus as libc::c_int & 1i32 << 1i32 {
            /* complete its execution */
            finishCcall(L, 1i32);
        } else {
            /* Lua function */
            /* finish interrupted instruction */
            luaV_finishOp(L);
            /* execute down to higher C 'boundary' */
            luaV_execute(L);
        }
    }
}
/*
** Completes the execution of an interrupted C function, calling its
** continuation function.
*/
unsafe extern "C" fn finishCcall(mut L: *mut lua_State, mut status: libc::c_int) -> () {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut n: libc::c_int = 0;
    /* must have a continuation and must be able to call it */
    /* error status can only happen in a protected call */
    if 0 != (*ci).callstatus as libc::c_int & 1i32 << 4i32 {
        /* was inside a pcall? */
        /* continuation is also inside it */
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 4i32)) as libc::c_ushort;
        /* with the same error function */
        (*L).errfunc = (*ci).u.c.old_errfunc
    }
    /* finish 'lua_callk'/'lua_pcall'; CIST_YPCALL and 'errfunc' already
    handled */
    if (*ci).nresults as libc::c_int == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    /* call continuation function */
    n = (*ci).u.c.k.expect("non-null function pointer")(L, status, (*ci).u.c.ctx);
    /* finish 'luaD_precall' */
    luaD_poscall(L, ci, (*L).top.offset(-(n as isize)), n);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_poscall(
    mut L: *mut lua_State,
    mut ci: *mut CallInfo,
    mut firstResult: StkId,
    mut nres: libc::c_int,
) -> libc::c_int {
    let mut fr: ptrdiff_t = 0;
    let mut res: StkId = 0 as *mut TValue;
    let mut wanted: libc::c_int = (*ci).nresults as libc::c_int;
    if 0 != (*L).hookmask & (1i32 << 1i32 | 1i32 << 2i32) {
        if 0 != (*L).hookmask & 1i32 << 1i32 {
            /* hook may change stack */
            fr = (firstResult as *mut libc::c_char)
                .wrapping_offset_from((*L).stack as *mut libc::c_char)
                as libc::c_long;
            luaD_hook(L, 1i32, -1i32);
            firstResult = ((*L).stack as *mut libc::c_char).offset(fr as isize) as *mut TValue
        }
        /* 'oldpc' for caller function */
        (*L).oldpc = (*(*ci).previous).u.l.savedpc
    }
    /* res == final position of 1st result */
    res = (*ci).func;
    /* back to caller */
    (*L).ci = (*ci).previous;
    /* move results to proper place */
    return moveresults(L, firstResult as *const TValue, res, nres, wanted);
}
/*
** Given 'nres' results at 'firstResult', move 'wanted' of them to 'res'.
** Handle most typical cases (zero results for commands, one result for
** expressions, multiple results for tail calls/single parameters)
** separated.
*/
unsafe extern "C" fn moveresults(
    mut L: *mut lua_State,
    mut firstResult: *const TValue,
    mut res: StkId,
    mut nres: libc::c_int,
    mut wanted: libc::c_int,
) -> libc::c_int {
    match wanted {
        0 => {}
        1 => {
            /* nothing to move */
            /* one result needed */
            /* no results? */
            if nres == 0i32 {
                /* adjust with nil */
                firstResult = &luaO_nilobject_
            }
            /* move it to proper place */
            let mut io1: *mut TValue = res;
            *io1 = *firstResult
        }
        -1 => {
            let mut i: libc::c_int = 0;
            /* move all results to correct place */
            i = 0i32;
            while i < nres {
                let mut io1_0: *mut TValue = res.offset(i as isize);
                *io1_0 = *firstResult.offset(i as isize);
                i += 1
            }
            (*L).top = res.offset(nres as isize);
            /* wanted == LUA_MULTRET */
            return 0i32;
        }
        _ => {
            let mut i_0: libc::c_int = 0;
            if wanted <= nres {
                /* enough results? */
                /* move wanted results to correct place */
                i_0 = 0i32;
                while i_0 < wanted {
                    let mut io1_1: *mut TValue = res.offset(i_0 as isize);
                    *io1_1 = *firstResult.offset(i_0 as isize);
                    i_0 += 1
                }
            } else {
                /* not enough results; use all of them plus nils */
                /* move all results to correct place */
                i_0 = 0i32;
                while i_0 < nres {
                    let mut io1_2: *mut TValue = res.offset(i_0 as isize);
                    *io1_2 = *firstResult.offset(i_0 as isize);
                    i_0 += 1
                }
                /* complete wanted number of results */
                while i_0 < wanted {
                    (*res.offset(i_0 as isize)).tt_ = 0i32;
                    i_0 += 1
                }
            }
        }
    }
    /* top points after the last result */
    (*L).top = res.offset(wanted as isize);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_hook(
    mut L: *mut lua_State,
    mut event: libc::c_int,
    mut line: libc::c_int,
) -> () {
    let mut hook: lua_Hook = (*L).hook;
    if hook.is_some() && 0 != (*L).allowhook as libc::c_int {
        /* make sure there is a hook */
        let mut ci: *mut CallInfo = (*L).ci;
        let mut top: ptrdiff_t = ((*L).top as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        let mut ci_top: ptrdiff_t = ((*ci).top as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        let mut ar: lua_Debug = lua_Debug {
            event: 0,
            name: 0 as *const libc::c_char,
            namewhat: 0 as *const libc::c_char,
            what: 0 as *const libc::c_char,
            source: 0 as *const libc::c_char,
            currentline: 0,
            linedefined: 0,
            lastlinedefined: 0,
            nups: 0,
            nparams: 0,
            isvararg: 0,
            istailcall: 0,
            short_src: [0; 60],
            i_ci: 0 as *mut CallInfo,
        };
        ar.event = event;
        ar.currentline = line;
        ar.i_ci = ci;
        if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long <= 20i32 as libc::c_long {
            luaD_growstack(L, 20i32);
        }
        /* ensure minimum stack size */
        (*ci).top = (*L).top.offset(20isize);
        /* cannot call hooks inside a hook */
        (*L).allowhook = 0i32 as lu_byte;
        (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 2i32) as libc::c_ushort;
        hook.expect("non-null function pointer")(L, &mut ar);
        (*L).allowhook = 1i32 as lu_byte;
        (*ci).top = ((*L).stack as *mut libc::c_char).offset(ci_top as isize) as *mut TValue;
        (*L).top = ((*L).stack as *mut libc::c_char).offset(top as isize) as *mut TValue;
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 2i32)) as libc::c_ushort
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_growstack(mut L: *mut lua_State, mut n: libc::c_int) -> () {
    let mut size: libc::c_int = (*L).stacksize;
    /* error after extra size? */
    if size > 1000000i32 {
        luaD_throw(L, 6i32);
    } else {
        let mut needed: libc::c_int =
            (*L).top.wrapping_offset_from((*L).stack) as libc::c_long as libc::c_int + n + 5i32;
        let mut newsize: libc::c_int = 2i32 * size;
        if newsize > 1000000i32 {
            newsize = 1000000i32
        }
        if newsize < needed {
            newsize = needed
        }
        if newsize > 1000000i32 {
            /* stack overflow? */
            luaD_reallocstack(L, 1000000i32 + 200i32);
            luaG_runerror!(L, s!(b"stack overflow\x00"));
        } else {
            luaD_reallocstack(L, newsize);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_reallocstack(mut L: *mut lua_State, mut newsize: libc::c_int) -> () {
    let mut oldstack: *mut TValue = (*L).stack;
    let mut lim: libc::c_int = (*L).stacksize;
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (newsize as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*L).stack = luaM_realloc_(
        L,
        (*L).stack as *mut libc::c_void,
        ((*L).stacksize as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        (newsize as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    while lim < newsize {
        /* erase new segment */
        (*(*L).stack.offset(lim as isize)).tt_ = 0i32;
        lim += 1
    }
    (*L).stacksize = newsize;
    (*L).stack_last = (*L).stack.offset(newsize as isize).offset(-5isize);
    correctstack(L, oldstack);
}
/* }====================================================== */
/*
** {==================================================================
** Stack reallocation
** ===================================================================
*/
unsafe extern "C" fn correctstack(mut L: *mut lua_State, mut oldstack: *mut TValue) -> () {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut up: *mut UpVal = 0 as *mut UpVal;
    (*L).top = (*L)
        .stack
        .offset((*L).top.wrapping_offset_from(oldstack) as libc::c_long as isize);
    up = (*L).openupval;
    while !up.is_null() {
        (*up).v = (*L)
            .stack
            .offset((*up).v.wrapping_offset_from(oldstack) as libc::c_long as isize);
        up = (*up).u.open.next
    }
    ci = (*L).ci;
    while !ci.is_null() {
        (*ci).top = (*L)
            .stack
            .offset((*ci).top.wrapping_offset_from(oldstack) as libc::c_long as isize);
        (*ci).func = (*L)
            .stack
            .offset((*ci).func.wrapping_offset_from(oldstack) as libc::c_long as isize);
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            (*ci).u.l.base = (*L)
                .stack
                .offset((*ci).u.l.base.wrapping_offset_from(oldstack) as libc::c_long as isize)
        }
        ci = (*ci).previous
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaD_rawrunprotected(
    mut L: *mut lua_State,
    mut f: Pfunc,
    mut ud: *mut libc::c_void,
) -> libc::c_int {
    let mut oldnCcalls: libc::c_ushort = (*L).nCcalls;
    let mut lj: lua_longjmp = lua_longjmp {
        previous: 0 as *mut lua_longjmp,
        b: [__jmp_buf_tag {
            __jmpbuf: [0; 8],
            __mask_was_saved: 0,
            __saved_mask: __sigset_t { __val: [0; 16] },
        }; 1],
        status: LUA_OK,
    };
    ::std::ptr::write_volatile(&mut lj.status as *mut libc::c_int, 0i32);
    /* chain new error handler */
    lj.previous = (*L).errorJmp;
    (*L).errorJmp = &mut lj;
    if _setjmp(lj.b.as_mut_ptr()) == 0i32 {
        f.expect("non-null function pointer")(L, ud);
    }
    /* restore old error handler */
    (*L).errorJmp = lj.previous;
    (*L).nCcalls = oldnCcalls;
    return lj.status;
}
/*
** Recovers from an error in a coroutine. Finds a recover point (if
** there is one) and completes the execution of the interrupted
** 'luaD_pcall'. If there is no recover point, returns zero.
*/
unsafe extern "C" fn recover(mut L: *mut lua_State, mut status: libc::c_int) -> libc::c_int {
    let mut oldtop: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo = findpcall(L);
    if ci.is_null() {
        /* no recovery point */
        return 0i32;
    } else {
        /* "finish" luaD_pcall */
        oldtop = ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as *mut TValue;
        luaF_close(L, oldtop);
        seterrorobj(L, status, oldtop);
        (*L).ci = ci;
        /* restore original 'allowhook' */
        (*L).allowhook = ((*ci).callstatus as libc::c_int & 1i32 << 0i32) as lu_byte;
        /* should be zero to be yieldable */
        (*L).nny = 0i32 as libc::c_ushort;
        luaD_shrinkstack(L);
        (*L).errfunc = (*ci).u.c.old_errfunc;
        /* continue running the coroutine */
        return 1i32;
    };
}
/*
** Try to find a suspended protected call (a "recover point") for the
** given thread.
*/
unsafe extern "C" fn findpcall(mut L: *mut lua_State) -> *mut CallInfo {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    ci = (*L).ci;
    while !ci.is_null() {
        /* search for a pcall */
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 4i32 {
            return ci;
        } else {
            ci = (*ci).previous
        }
    }
    /* no pending pcall */
    return 0 as *mut CallInfo;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_shrinkstack(mut L: *mut lua_State) -> () {
    let mut inuse: libc::c_int = stackinuse(L);
    let mut goodsize: libc::c_int = inuse + inuse / 8i32 + 2i32 * 5i32;
    if goodsize > 1000000i32 {
        /* respect stack limit */
        goodsize = 1000000i32
    }
    /* had been handling stack overflow? */
    if (*L).stacksize > 1000000i32 {
        /* free all CIs (list grew because of an error) */
        luaE_freeCI(L);
    } else {
        /* shrink list */
        luaE_shrinkCI(L);
    }
    /* if thread is currently not handling a stack overflow and its
    good size is smaller than current size, shrink its stack */
    if inuse <= 1000000i32 - 5i32 && goodsize < (*L).stacksize {
        luaD_reallocstack(L, goodsize);
    };
}
unsafe extern "C" fn stackinuse(mut L: *mut lua_State) -> libc::c_int {
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut lim: StkId = (*L).top;
    ci = (*L).ci;
    while !ci.is_null() {
        if lim < (*ci).top {
            lim = (*ci).top
        }
        ci = (*ci).previous
    }
    /* part of stack in use */
    return lim.wrapping_offset_from((*L).stack) as libc::c_long as libc::c_int + 1i32;
}
/*
** Do the work for 'lua_resume' in protected mode. Most of the work
** depends on the status of the coroutine: initial state, suspended
** inside a hook, or regularly suspended (optionally with a continuation
** function), plus erroneous cases: non-suspended coroutine or dead
** coroutine.
*/
unsafe extern "C" fn resume(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    /* number of arguments */
    let mut n: libc::c_int = *(ud as *mut libc::c_int);
    /* first argument */
    let mut firstArg: StkId = (*L).top.offset(-(n as isize));
    let mut ci: *mut CallInfo = (*L).ci;
    if (*L).status as libc::c_int == LUA_OK {
        /* starting a coroutine? */
        /* Lua function? */
        if 0 == luaD_precall(L, firstArg.offset(-1isize), LUA_MULTRET) {
            /* call it */
            luaV_execute(L);
        }
    } else {
        /* resuming from previous yield */
        /* mark that it is running (again) */
        (*L).status = LUA_OK as lu_byte;
        (*ci).func = ((*L).stack as *mut libc::c_char).offset((*ci).extra as isize) as *mut TValue;
        /* yielded inside a hook? */
        if 0 != (*ci).callstatus as libc::c_int & 1i32 << 1i32 {
            /* just continue running Lua code */
            luaV_execute(L);
        } else {
            /* 'common' yield */
            if (*ci).u.c.k.is_some() {
                /* does it have a continuation function? */
                /* call continuation */
                n = (*ci).u.c.k.expect("non-null function pointer")(L, 1i32, (*ci).u.c.ctx);
                /* yield results come from continuation */
                firstArg = (*L).top.offset(-(n as isize))
            }
            /* finish 'luaD_precall' */
            luaD_poscall(L, ci, firstArg, n);
        }
        /* run continuation */
        unroll(L, 0 as *mut libc::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_precall(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nresults: libc::c_int,
) -> libc::c_int {
    let mut f: lua_CFunction = None;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    match (*func).tt_ & 0x3fi32 {
        38 => f = (*((*func).value_.gc as *mut GCUnion)).cl.c.f,
        22 => f = (*func).value_.f,
        6 => {
            /* Lua function: prepare its call */
            let mut base: StkId = 0 as *mut TValue;
            let mut p: *mut Proto = (*((*func).value_.gc as *mut GCUnion)).cl.l.p;
            /* number of real arguments */
            let mut n_0: libc::c_int =
                (*L).top.wrapping_offset_from(func) as libc::c_long as libc::c_int - 1i32;
            /* frame size */
            let mut fsize: libc::c_int = (*p).maxstacksize as libc::c_int;
            if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long
                <= fsize as libc::c_long
            {
                let mut t___0: ptrdiff_t = (func as *mut libc::c_char)
                    .wrapping_offset_from((*L).stack as *mut libc::c_char)
                    as libc::c_long;
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                    luaC_step(L);
                }
                luaD_growstack(L, fsize);
                func = ((*L).stack as *mut libc::c_char).offset(t___0 as isize) as *mut TValue
            }
            if 0 != (*p).is_vararg {
                base = adjust_varargs(L, p, n_0)
            } else {
                /* non vararg function */
                while n_0 < (*p).numparams as libc::c_int {
                    /* complete missing arguments */
                    let fresh1 = (*L).top;
                    (*L).top = (*L).top.offset(1);
                    (*fresh1).tt_ = 0i32;
                    n_0 += 1
                }
                base = func.offset(1isize)
            }
            /* now 'enter' new function */
            (*L).ci = if !(*(*L).ci).next.is_null() {
                (*(*L).ci).next
            } else {
                luaE_extendCI(L)
            };
            ci = (*L).ci;
            (*ci).nresults = nresults as libc::c_short;
            (*ci).func = func;
            (*ci).u.l.base = base;
            (*ci).top = base.offset(fsize as isize);
            (*L).top = (*ci).top;
            /* starting point */
            (*ci).u.l.savedpc = (*p).code;
            (*ci).callstatus = (1i32 << 1i32) as libc::c_ushort;
            if 0 != (*L).hookmask & 1i32 << 0i32 {
                callhook(L, ci);
            }
            return 0i32;
        }
        _ => {
            /* not a function */
            if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long
                <= 1i32 as libc::c_long
            {
                /* ensure space for metamethod */
                let mut t___1: ptrdiff_t = (func as *mut libc::c_char)
                    .wrapping_offset_from((*L).stack as *mut libc::c_char)
                    as libc::c_long;
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                    luaC_step(L);
                }
                luaD_growstack(L, 1i32);
                func = ((*L).stack as *mut libc::c_char).offset(t___1 as isize) as *mut TValue
            }
            /* try to get '__call' metamethod */
            tryfuncTM(L, func);
            /* now it must be a function */
            return luaD_precall(L, func, nresults);
        }
    }
    /* number of returns */
    let mut n: libc::c_int = 0;
    if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long <= 20i32 as libc::c_long {
        /* ensure minimum stack size */
        let mut t__: ptrdiff_t = (func as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
            luaC_step(L);
        }
        luaD_growstack(L, 20i32);
        func = ((*L).stack as *mut libc::c_char).offset(t__ as isize) as *mut TValue
    }
    /* now 'enter' new function */
    (*L).ci = if !(*(*L).ci).next.is_null() {
        (*(*L).ci).next
    } else {
        luaE_extendCI(L)
    };
    ci = (*L).ci;
    (*ci).nresults = nresults as libc::c_short;
    (*ci).func = func;
    (*ci).top = (*L).top.offset(20isize);
    (*ci).callstatus = 0i32 as libc::c_ushort;
    if 0 != (*L).hookmask & 1i32 << 0i32 {
        luaD_hook(L, 0i32, -1i32);
    }
    /* do the actual call */
    n = f.expect("non-null function pointer")(L);
    luaD_poscall(L, ci, (*L).top.offset(-(n as isize)), n);
    return 1i32;
}
/*
** Check whether __call metafield of 'func' is a function. If so, put
** it in stack below original 'func' so that 'luaD_precall' can call
** it. Raise an error if __call metafield is not a function.
*/
unsafe extern "C" fn tryfuncTM(mut L: *mut lua_State, mut func: StkId) -> () {
    let mut tm: *const TValue = luaT_gettmbyobj(L, func as *const TValue, TM_CALL);
    let mut p: StkId = 0 as *mut TValue;
    if !((*tm).tt_ & 0xfi32 == 6i32) {
        luaG_typeerror(L, func as *const TValue, s!(b"call\x00"));
    } else {
        /* Open a hole inside the stack at 'func' */
        p = (*L).top;
        while p > func {
            let mut io1: *mut TValue = p;
            *io1 = *p.offset(-1isize);
            p = p.offset(-1isize)
        }
        /* slot ensured by caller */
        (*L).top = (*L).top.offset(1isize);
        /* tag method is the new function to be called */
        let mut io1_0: *mut TValue = func;
        *io1_0 = *tm;
        return;
    };
}
unsafe extern "C" fn callhook(mut L: *mut lua_State, mut ci: *mut CallInfo) -> () {
    let mut hook: libc::c_int = 0i32;
    /* hooks assume 'pc' is already incremented */
    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
    if 0 != (*(*ci).previous).callstatus as libc::c_int & 1i32 << 1i32
        && (*(*(*ci).previous).u.l.savedpc.offset(-1isize) >> 0i32
            & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode as libc::c_uint
            == OP_TAILCALL as libc::c_int as libc::c_uint
    {
        (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 5i32) as libc::c_ushort;
        hook = 4i32
    }
    luaD_hook(L, hook, -1i32);
    /* correct 'pc' */
    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(-1isize);
}
unsafe extern "C" fn adjust_varargs(
    mut L: *mut lua_State,
    mut p: *mut Proto,
    mut actual: libc::c_int,
) -> StkId {
    let mut i: libc::c_int = 0;
    let mut nfixargs: libc::c_int = (*p).numparams as libc::c_int;
    let mut base: StkId = 0 as *mut TValue;
    let mut fixed: StkId = 0 as *mut TValue;
    /* move fixed parameters to final position */
    /* first fixed argument */
    fixed = (*L).top.offset(-(actual as isize));
    /* final position of first argument */
    base = (*L).top;
    i = 0i32;
    while i < nfixargs && i < actual {
        let fresh2 = (*L).top;
        (*L).top = (*L).top.offset(1);
        let mut io1: *mut TValue = fresh2;
        *io1 = *fixed.offset(i as isize);
        /* erase original copy (for GC) */
        (*fixed.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    while i < nfixargs {
        /* complete missing arguments */
        let fresh3 = (*L).top;
        (*L).top = (*L).top.offset(1);
        (*fresh3).tt_ = 0i32;
        i += 1
    }
    return base;
}
/*
** Signal an error in the call to 'lua_resume', not in the execution
** of the coroutine itself. (Such errors should not be handled by any
** coroutine error handler and should not kill the coroutine.)
*/
unsafe extern "C" fn resume_error(
    mut L: *mut lua_State,
    mut msg: *const libc::c_char,
    mut narg: libc::c_int,
) -> libc::c_int {
    /* remove args from the stack */
    (*L).top = (*L).top.offset(-(narg as isize));
    /* push error message */
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = luaS_new(L, msg);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    return 2i32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isyieldable(mut L: *mut lua_State) -> libc::c_int {
    return ((*L).nny as libc::c_int == 0i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_protectedparser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut name: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut p: SParser = SParser {
        z: 0 as *mut ZIO,
        buff: Mbuffer {
            buffer: 0 as *mut libc::c_char,
            n: 0,
            buffsize: 0,
        },
        dyd: Dyndata {
            actvar: unnamed_6 {
                arr: 0 as *mut Vardesc,
                n: 0,
                size: 0,
            },
            gt: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
            label: Labellist {
                arr: 0 as *mut Labeldesc,
                n: 0,
                size: 0,
            },
        },
        mode: 0 as *const libc::c_char,
        name: 0 as *const libc::c_char,
    };
    let mut status: libc::c_int = 0;
    /* cannot yield during parsing */
    (*L).nny = (*L).nny.wrapping_add(1);
    p.z = z;
    p.name = name;
    p.mode = mode;
    p.dyd.actvar.arr = 0 as *mut Vardesc;
    p.dyd.actvar.size = 0i32;
    p.dyd.gt.arr = 0 as *mut Labeldesc;
    p.dyd.gt.size = 0i32;
    p.dyd.label.arr = 0 as *mut Labeldesc;
    p.dyd.label.size = 0i32;
    p.buff.buffer = 0 as *mut libc::c_char;
    p.buff.buffsize = 0i32 as size_t;
    status = luaD_pcall(
        L,
        Some(f_parser),
        &mut p as *mut SParser as *mut libc::c_void,
        ((*L).top as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long,
        (*L).errfunc,
    );
    p.buff.buffer = luaM_realloc_(
        L,
        p.buff.buffer as *mut libc::c_void,
        p.buff
            .buffsize
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (0i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    p.buff.buffsize = 0i32 as size_t;
    luaM_realloc_(
        L,
        p.dyd.actvar.arr as *mut libc::c_void,
        (p.dyd.actvar.size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Vardesc>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        p.dyd.gt.arr as *mut libc::c_void,
        (p.dyd.gt.size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Labeldesc>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        p.dyd.label.arr as *mut libc::c_void,
        (p.dyd.label.size as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<Labeldesc>() as libc::c_ulong),
        0i32 as size_t,
    );
    (*L).nny = (*L).nny.wrapping_sub(1);
    return status;
}
unsafe extern "C" fn f_parser(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    let mut p: *mut SParser = ud as *mut SParser;
    /* read first character */
    let fresh4 = (*(*p).z).n;
    (*(*p).z).n = (*(*p).z).n.wrapping_sub(1);
    let mut c: libc::c_int = if fresh4 > 0i32 as libc::c_ulong {
        let fresh5 = (*(*p).z).p;
        (*(*p).z).p = (*(*p).z).p.offset(1);
        *fresh5 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*p).z)
    };
    if c == (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
        as libc::c_int
    {
        checkmode(L, (*p).mode, s!(b"binary\x00"));
        cl = luaU_undump(L, (*p).z, (*p).name)
    } else {
        checkmode(L, (*p).mode, s!(b"text\x00"));
        cl = luaY_parser(L, (*p).z, &mut (*p).buff, &mut (*p).dyd, (*p).name, c)
    }
    luaF_initupvals(L, cl);
}
unsafe extern "C" fn checkmode(
    mut L: *mut lua_State,
    mut mode: *const libc::c_char,
    mut x: *const libc::c_char,
) -> () {
    if !mode.is_null() && strchr(mode, *x.offset(0isize) as libc::c_int).is_null() {
        luaO_pushfstring!(
            L,
            s!(b"attempt to load a %s chunk (mode is \'%s\')\x00"),
            x,
            mode,
        );
        luaD_throw(L, 3i32);
    } else {
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_pcall(
    mut L: *mut lua_State,
    mut func: Pfunc,
    mut u: *mut libc::c_void,
    mut old_top: ptrdiff_t,
    mut ef: ptrdiff_t,
) -> libc::c_int {
    let mut oldtop: StkId = 0 as *mut TValue;
    let mut status: libc::c_int = 0;
    let mut old_ci: *mut CallInfo = (*L).ci;
    let mut old_allowhooks: lu_byte = (*L).allowhook;
    let mut old_nny: libc::c_ushort = (*L).nny;
    let mut old_errfunc: ptrdiff_t = (*L).errfunc;
    (*L).errfunc = ef;
    status = luaD_rawrunprotected(L, func, u);
    if status != 0i32 {
        /* an error occurred? */
        oldtop = ((*L).stack as *mut libc::c_char).offset(old_top as isize) as *mut TValue;
        /* close possible pending closures */
        luaF_close(L, oldtop);
        seterrorobj(L, status, oldtop);
        (*L).ci = old_ci;
        (*L).allowhook = old_allowhooks;
        (*L).nny = old_nny;
        luaD_shrinkstack(L);
    }
    (*L).errfunc = old_errfunc;
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn luaD_call(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) -> () {
    (*L).nCcalls = (*L).nCcalls.wrapping_add(1);
    if (*L).nCcalls as libc::c_int >= LUAI_MAXCCALLS {
        stackerror(L);
    }
    /* is a Lua function? */
    if 0 == luaD_precall(L, func, nResults) {
        /* call it */
        luaV_execute(L);
    }
    (*L).nCcalls = (*L).nCcalls.wrapping_sub(1);
}
/*
** Check appropriate error for stack overflow ("regular" overflow or
** overflow while handling stack overflow). If 'nCalls' is larger than
** LUAI_MAXCCALLS (which means it is handling a "regular" overflow) but
** smaller than 9/8 of LUAI_MAXCCALLS, does not report an error (to
** allow overflow handling to work)
*/
unsafe extern "C" fn stackerror(mut L: *mut lua_State) -> () {
    if (*L).nCcalls as libc::c_int == LUAI_MAXCCALLS {
        luaG_runerror!(L, s!(b"C stack overflow\x00"),);
    } else if (*L).nCcalls as libc::c_int >= LUAI_MAXCCALLS + (LUAI_MAXCCALLS >> 3i32) {
        /* error while handing stack error */
        luaD_throw(L, 6i32);
    } else {
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaD_callnoyield(
    mut L: *mut lua_State,
    mut func: StkId,
    mut nResults: libc::c_int,
) -> () {
    (*L).nny = (*L).nny.wrapping_add(1);
    luaD_call(L, func, nResults);
    (*L).nny = (*L).nny.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn luaD_inctop(mut L: *mut lua_State) -> () {
    if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long <= 1i32 as libc::c_long {
        luaD_growstack(L, 1i32);
    }
    (*L).top = (*L).top.offset(1isize);
}
