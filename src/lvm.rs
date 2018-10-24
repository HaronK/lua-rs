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
    fn pow(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn floor(_: lua_double) -> lua_double;
    #[no_mangle]
    fn fmod(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn strcoll(__s1: *const lua_char, __s2: *const lua_char) -> lua_int;
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
    #[no_mangle]
    fn luaO_fb2int(x: lua_int) -> lua_int;
    #[no_mangle]
    fn luaO_str2num(s: *const lua_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaO_tostring(L: *mut lua_State, obj: StkId) -> ();
    #[no_mangle]
    fn luaT_gettm(events: *mut Table, event: TMS, ename: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaT_gettmbyobj(L: *mut lua_State, o: *const TValue, event: TMS) -> *const TValue;
    #[no_mangle]
    fn luaT_callTM(
        L: *mut lua_State,
        f: *const TValue,
        p1: *const TValue,
        p2: *const TValue,
        p3: *mut TValue,
        hasres: lua_int,
    ) -> ();
    #[no_mangle]
    fn luaT_trybinTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    ) -> ();
    #[no_mangle]
    fn luaT_callorderTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        event: TMS,
    ) -> lua_int;
    /*
     ** $Id: ldebug.h,v 2.14.1.1 2017/04/19 17:20:42 roberto Exp $
     ** Auxiliary functions from Debug Interface module
     ** See Copyright Notice in lua.h
     */
    #[no_mangle]
    fn luaG_typeerror(L: *mut lua_State, o: *const TValue, opname: *const lua_char) -> !;
    #[no_mangle]
    fn luaG_ordererror(L: *mut lua_State, p1: *const TValue, p2: *const TValue) -> !;
    #[no_mangle]
    fn luaG_traceexec(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_precall(L: *mut lua_State, func: StkId, nresults: lua_int) -> lua_int;
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: lua_int) -> ();
    #[no_mangle]
    fn luaD_poscall(
        L: *mut lua_State,
        ci: *mut CallInfo,
        firstResult: StkId,
        nres: lua_int,
    ) -> lua_int;
    #[no_mangle]
    fn luaD_growstack(L: *mut lua_State, n: lua_int) -> ();
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State, nelems: lua_int) -> *mut LClosure;
    #[no_mangle]
    fn luaF_findupval(L: *mut lua_State, level: StkId) -> *mut UpVal;
    #[no_mangle]
    fn luaF_close(L: *mut lua_State, level: StkId) -> ();
    #[no_mangle]
    fn luaC_step(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State, o: *mut Table) -> ();
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal) -> ();
    #[no_mangle]
    fn luaS_eqlngstr(a: *mut TString, b: *mut TString) -> lua_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const lua_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_createlngstrobj(L: *mut lua_State, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State, t: *mut Table, key: lua_Integer, value: *mut TValue) -> ();
    #[no_mangle]
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    #[no_mangle]
    fn luaH_newkey(L: *mut lua_State, t: *mut Table, key: *const TValue) -> *mut TValue;
    #[no_mangle]
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: lua_uint,
        nhsize: lua_uint,
    ) -> ();
    #[no_mangle]
    fn luaH_resizearray(L: *mut lua_State, t: *mut Table, nasize: lua_uint) -> ();
    #[no_mangle]
    fn luaH_getn(t: *mut Table) -> lua_Unsigned;
}
pub type __sig_atomic_t = lua_int;
pub type size_t = lua_ulong;
pub type ptrdiff_t = lua_long;
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
/* unsigned integer type */
pub type lua_Unsigned = lua_ulonglong;
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
pub type OpCode = lua_uint;
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
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
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
** $Id: lvm.h,v 2.41.1.1 2017/04/19 17:20:42 roberto Exp $
** Lua virtual machine
** See Copyright Notice in lua.h
*/
/*
** You can define LUA_FLOORN2I if you want to convert floats to integers
** by flooring them (instead of raising an error if they are not
** integral values)
*/
/*
** fast track for 'gettable': if 't' is a table and 't[k]' is not nil,
** return 1 with 'slot' pointing to 't[k]' (final result).  Otherwise,
** return 0 (meaning it will have to check metamethod) with 'slot'
** pointing to a nil 't[k]' (if 't' is a table) or NULL (otherwise).
** 'f' is the raw get function to use.
*/
/* not a table; 'slot' is NULL and result is 0 */
/* else, do raw access */
/* result not nil? */
/*
** standard implementation for 'gettable'
*/
/*
** Fast track for set table. If 't' is a table and 't[k]' is not nil,
** call GC barrier, do a raw 't[k]=v', and return true; otherwise,
** return false with 'slot' equal to NULL (if 't' is not a table) or
** 'nil'. (This is needed by 'luaV_finishget'.) Note that, if the macro
** returns true, there is no need to 'invalidateTMcache', because the
** call is not creating a new entry.
*/
#[no_mangle]
pub unsafe extern "C" fn luaV_equalobj(
    mut L: *mut lua_State,
    mut t1: *const TValue,
    mut t2: *const TValue,
) -> lua_int {
    let mut tm: *const TValue = 0 as *const TValue;
    if (*t1).tt_ & 0x3fi32 != (*t2).tt_ & 0x3fi32 {
        /* not the same variant? */
        if (*t1).tt_ & 0xfi32 != (*t2).tt_ & 0xfi32 || (*t1).tt_ & 0xfi32 != 3i32 {
            /* only numbers can be equal with different variants */
            return 0i32;
        } else {
            /* two numbers with different variants */
            /* compare them as integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            return (0 != if (*t1).tt_ == 3i32 | 1i32 << 4i32 {
                i1 = (*t1).value_.i;
                1i32
            } else {
                luaV_tointeger(t1, &mut i1, 0i32)
            } && 0 != if (*t2).tt_ == 3i32 | 1i32 << 4i32 {
                i2 = (*t2).value_.i;
                1i32
            } else {
                luaV_tointeger(t2, &mut i2, 0i32)
            } && i1 == i2) as lua_int;
        }
    } else {
        /* values have same type and same variant */
        match (*t1).tt_ & 0x3fi32 {
            0 => return 1i32,
            19 => return ((*t1).value_.i == (*t2).value_.i) as lua_int,
            3 => return ((*t1).value_.n == (*t2).value_.n) as lua_int,
            1 => {
                /* true must be 1 !! */
                return ((*t1).value_.b == (*t2).value_.b) as lua_int;
            }
            2 => return ((*t1).value_.p == (*t2).value_.p) as lua_int,
            22 => return ((*t1).value_.f == (*t2).value_.f) as lua_int,
            4 => {
                return (&mut (*((*t1).value_.gc as *mut GCUnion)).ts as *mut TString
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).ts as *mut TString)
                    as lua_int
            }
            20 => {
                return luaS_eqlngstr(
                    &mut (*((*t1).value_.gc as *mut GCUnion)).ts,
                    &mut (*((*t2).value_.gc as *mut GCUnion)).ts,
                )
            }
            7 => {
                if &mut (*((*t1).value_.gc as *mut GCUnion)).u as *mut Udata
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).u as *mut Udata
                {
                    return 1i32;
                } else if L.is_null() {
                    return 0i32;
                } else {
                    tm = if (*((*t1).value_.gc as *mut GCUnion)).u.metatable.is_null() {
                        0 as *const TValue
                    } else if 0
                        != (*(*((*t1).value_.gc as *mut GCUnion)).u.metatable).flags as lua_uint
                            & 1u32 << TM_EQ as lua_int
                    {
                        0 as *const TValue
                    } else {
                        luaT_gettm(
                            (*((*t1).value_.gc as *mut GCUnion)).u.metatable,
                            TM_EQ,
                            (*(*L).l_G).tmname[TM_EQ as lua_int as usize],
                        )
                    };
                    if tm.is_null() {
                        tm = if (*((*t2).value_.gc as *mut GCUnion)).u.metatable.is_null() {
                            0 as *const TValue
                        } else if 0
                            != (*(*((*t2).value_.gc as *mut GCUnion)).u.metatable).flags
                                as lua_uint
                                & 1u32 << TM_EQ as lua_int
                        {
                            0 as *const TValue
                        } else {
                            luaT_gettm(
                                (*((*t2).value_.gc as *mut GCUnion)).u.metatable,
                                TM_EQ,
                                (*(*L).l_G).tmname[TM_EQ as lua_int as usize],
                            )
                        }
                    }
                }
            }
            5 => {
                /* will try TM */
                if &mut (*((*t1).value_.gc as *mut GCUnion)).h as *mut Table
                    == &mut (*((*t2).value_.gc as *mut GCUnion)).h as *mut Table
                {
                    return 1i32;
                } else if L.is_null() {
                    return 0i32;
                } else {
                    tm = if (*((*t1).value_.gc as *mut GCUnion)).h.metatable.is_null() {
                        0 as *const TValue
                    } else if 0
                        != (*(*((*t1).value_.gc as *mut GCUnion)).h.metatable).flags as lua_uint
                            & 1u32 << TM_EQ as lua_int
                    {
                        0 as *const TValue
                    } else {
                        luaT_gettm(
                            (*((*t1).value_.gc as *mut GCUnion)).h.metatable,
                            TM_EQ,
                            (*(*L).l_G).tmname[TM_EQ as lua_int as usize],
                        )
                    };
                    if tm.is_null() {
                        tm = if (*((*t2).value_.gc as *mut GCUnion)).h.metatable.is_null() {
                            0 as *const TValue
                        } else if 0
                            != (*(*((*t2).value_.gc as *mut GCUnion)).h.metatable).flags
                                as lua_uint
                                & 1u32 << TM_EQ as lua_int
                        {
                            0 as *const TValue
                        } else {
                            luaT_gettm(
                                (*((*t2).value_.gc as *mut GCUnion)).h.metatable,
                                TM_EQ,
                                (*(*L).l_G).tmname[TM_EQ as lua_int as usize],
                            )
                        }
                    }
                }
            }
            _ => {
                /* will try TM */
                return ((*t1).value_.gc == (*t2).value_.gc) as lua_int;
            }
        }
        /* no TM? */
        if tm.is_null() {
            /* objects are different */
            return 0i32;
        } else {
            /* call TM */
            luaT_callTM(L, tm, t1, t2, (*L).top, 1i32);
            return !((*(*L).top).tt_ == 0i32
                || (*(*L).top).tt_ == 1i32 && (*(*L).top).value_.b == 0i32)
                as lua_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tointeger(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut mode: lua_int,
) -> lua_int {
    let mut n: lua_Number = 0.;
    let mut f: lua_Number = 0.;
    let mut current_block: u64;
    let mut v: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    loop {
        if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            n = (*obj).value_.n;
            f = floor(n);
            if n != f {
                current_block = 8258075665625361029;
                break;
            } else {
                current_block = 16559507199688588974;
                break;
            }
        } else if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
            *p = (*obj).value_.i;
            return 1i32;
        } else if (*obj).tt_ & 0xfi32 == 4i32
            && luaO_str2num(
                (&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString
                    as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
                &mut v,
            ) == if (*((*obj).value_.gc as *mut GCUnion)).ts.tt as lua_int
                == 4i32 | 0i32 << 4i32
            {
                (*((*obj).value_.gc as *mut GCUnion)).ts.shrlen as lua_ulong
            } else {
                (*((*obj).value_.gc as *mut GCUnion)).ts.u.lnglen
            }
            .wrapping_add(1i32 as lua_ulong)
        {
            obj = &mut v
        } else {
            return 0i32;
        }
    }
    match current_block {
        8258075665625361029 => {
            /* not an integral value? */
            if mode == 0i32 {
                /* fails if mode demands integral value */
                return 0i32;
            } else if mode > 1i32 {
                /* convert floor to ceil (remember: n != f) */
                f += 1i32 as lua_double
            }
        }
        _ => {}
    }
    return (f >= (-9223372036854775807i64 - 1i64) as lua_double
        && f < -((-9223372036854775807i64 - 1i64) as lua_double)
        && {
            *p = f as lua_longlong;
            0 != 1i32
        }) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessthan(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> lua_int {
    let mut res: lua_int = 0;
    /* both operands are numbers? */
    if (*l).tt_ & 0xfi32 == 3i32 && (*r).tt_ & 0xfi32 == 3i32 {
        return LTnum(l, r);
    } else if (*l).tt_ & 0xfi32 == 4i32 && (*r).tt_ & 0xfi32 == 4i32 {
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) < 0i32) as lua_int;
    } else {
        /* no metamethod? */
        res = luaT_callorderTM(L, l, r, TM_LT);
        if res < 0i32 {
            /* error */
            luaG_ordererror(L, l, r);
        } else {
            return res;
        }
    };
}
/*
** Compare two strings 'ls' x 'rs', returning an integer smaller-equal-
** -larger than zero if 'ls' is smaller-equal-larger than 'rs'.
** The code is a little tricky because it allows '\0' in the strings
** and it uses 'strcoll' (to respect locales) for each segments
** of the strings.
*/
unsafe extern "C" fn l_strcmp(mut ls: *const TString, mut rs: *const TString) -> lua_int {
    let mut l: *const lua_char = (ls as *mut lua_char)
        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
    let mut ll: size_t = if (*ls).tt as lua_int == 4i32 | 0i32 << 4i32 {
        (*ls).shrlen as lua_ulong
    } else {
        (*ls).u.lnglen
    };
    let mut r: *const lua_char = (rs as *mut lua_char)
        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
    let mut lr: size_t = if (*rs).tt as lua_int == 4i32 | 0i32 << 4i32 {
        (*rs).shrlen as lua_ulong
    } else {
        (*rs).u.lnglen
    };
    loop {
        /* for each segment */
        let mut temp: lua_int = strcoll(l, r);
        /* not equal? */
        if temp != 0i32 {
            /* done */
            return temp;
        } else {
            /* strings are equal up to a '\0' */
            /* index of first '\0' in both strings */
            let mut len: size_t = strlen(l);
            /* 'rs' is finished? */
            if len == lr {
                /* check 'ls' */
                return if len == ll { 0i32 } else { 1i32 };
            } else if len == ll {
                /* 'ls' is smaller than 'rs' ('rs' is not finished) */
                return -1i32;
            } else {
                /* both strings longer than 'len'; go on comparing after the '\0' */
                len = len.wrapping_add(1);
                l = l.offset(len as isize);
                ll = (ll as lua_ulong).wrapping_sub(len) as size_t as size_t;
                r = r.offset(len as isize);
                lr = (lr as lua_ulong).wrapping_sub(len) as size_t as size_t
            }
        }
    }
}
/*
** Return 'l < r', for numbers.
*/
unsafe extern "C" fn LTnum(mut l: *const TValue, mut r: *const TValue) -> lua_int {
    if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            /* both are integers */
            return (li < (*r).value_.i) as lua_int;
        } else {
            return LTintfloat(li, (*r).value_.n);
        }
    } else {
        /* 'l' must be float */
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            /* both are float */
            return (lf < (*r).value_.n) as lua_int;
        } else if !(lf == lf) {
            /* NaN < i is always false */
            return 0i32;
        } else {
            return (0 == LEintfloat((*r).value_.i, lf)) as lua_int;
        }
    };
}
/*
** Check whether integer 'i' is less than or equal to float 'f'.
** See comments on previous function.
*/
unsafe extern "C" fn LEintfloat(mut i: lua_Integer, mut f: lua_Number) -> lua_int {
    if !(-((1i32 as lua_Integer) << 53i32) <= i && i <= (1i32 as lua_Integer) << 53i32) {
        /* -minint == maxint + 1 */
        if f >= -((-9223372036854775807i64 - 1i64) as lua_Number) {
            /* f >= maxint + 1 > i */
            return 1i32;
        } else if f >= (-9223372036854775807i64 - 1i64) as lua_Number {
            /* compare them as integers */
            return (i <= f as lua_Integer) as lua_int;
        } else {
            return 0i32;
        }
    } else {
        return (i as lua_Number <= f) as lua_int;
    };
}
/*
** Check whether integer 'i' is less than float 'f'. If 'i' has an
** exact representation as a float ('l_intfitsf'), compare numbers as
** floats. Otherwise, if 'f' is outside the range for integers, result
** is trivial. Otherwise, compare them as integers. (When 'i' has no
** float representation, either 'f' is "far away" from 'i' or 'f' has
** no precision left for a fractional part; either way, how 'f' is
** truncated is irrelevant.) When 'f' is NaN, comparisons must result
** in false.
*/
unsafe extern "C" fn LTintfloat(mut i: lua_Integer, mut f: lua_Number) -> lua_int {
    if !(-((1i32 as lua_Integer) << 53i32) <= i && i <= (1i32 as lua_Integer) << 53i32) {
        /* -minint == maxint + 1 */
        if f >= -((-9223372036854775807i64 - 1i64) as lua_Number) {
            /* f >= maxint + 1 > i */
            return 1i32;
        } else if f > (-9223372036854775807i64 - 1i64) as lua_Number {
            /* compare them as integers */
            return (i < f as lua_Integer) as lua_int;
        } else {
            return 0i32;
        }
    } else {
        return ((i as lua_Number) < f) as lua_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_lessequal(
    mut L: *mut lua_State,
    mut l: *const TValue,
    mut r: *const TValue,
) -> lua_int {
    let mut res: lua_int = 0;
    /* both operands are numbers? */
    if (*l).tt_ & 0xfi32 == 3i32 && (*r).tt_ & 0xfi32 == 3i32 {
        return LEnum(l, r);
    } else if (*l).tt_ & 0xfi32 == 4i32 && (*r).tt_ & 0xfi32 == 4i32 {
        return (l_strcmp(
            &mut (*((*l).value_.gc as *mut GCUnion)).ts,
            &mut (*((*r).value_.gc as *mut GCUnion)).ts,
        ) <= 0i32) as lua_int;
    } else {
        /* try 'le' */
        res = luaT_callorderTM(L, l, r, TM_LE);
        if res >= 0i32 {
            return res;
        } else {
            /* try 'lt': */
            /* mark it is doing 'lt' for 'le' */
            (*(*L).ci).callstatus =
                ((*(*L).ci).callstatus as lua_int | 1i32 << 7i32) as lua_ushort;
            res = luaT_callorderTM(L, r, l, TM_LT);
            /* clear mark */
            (*(*L).ci).callstatus =
                ((*(*L).ci).callstatus as lua_int ^ 1i32 << 7i32) as lua_ushort;
            if res < 0i32 {
                luaG_ordererror(L, l, r);
            } else {
                return (0 == res) as lua_int;
            }
        }
    };
}
/*
** Return 'l <= r', for numbers.
*/
unsafe extern "C" fn LEnum(mut l: *const TValue, mut r: *const TValue) -> lua_int {
    if (*l).tt_ == 3i32 | 1i32 << 4i32 {
        let mut li: lua_Integer = (*l).value_.i;
        if (*r).tt_ == 3i32 | 1i32 << 4i32 {
            /* both are integers */
            return (li <= (*r).value_.i) as lua_int;
        } else {
            return LEintfloat(li, (*r).value_.n);
        }
    } else {
        /* 'l' must be float */
        let mut lf: lua_Number = (*l).value_.n;
        if (*r).tt_ == 3i32 | 0i32 << 4i32 {
            /* both are float */
            return (lf <= (*r).value_.n) as lua_int;
        } else if !(lf == lf) {
            /*  NaN <= i is always false */
            return 0i32;
        } else {
            return (0 == LTintfloat((*r).value_.i, lf)) as lua_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_tonumber_(
    mut obj: *const TValue,
    mut n: *mut lua_Number,
) -> lua_int {
    let mut v: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        *n = (*obj).value_.i as lua_Number;
        return 1i32;
    } else if (*obj).tt_ & 0xfi32 == 4i32
        && luaO_str2num(
            (&mut (*((*obj).value_.gc as *mut GCUnion)).ts as *mut TString as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            &mut v,
        ) == if (*((*obj).value_.gc as *mut GCUnion)).ts.tt as lua_int == 4i32 | 0i32 << 4i32
        {
            (*((*obj).value_.gc as *mut GCUnion)).ts.shrlen as lua_ulong
        } else {
            (*((*obj).value_.gc as *mut GCUnion)).ts.u.lnglen
        }
        .wrapping_add(1i32 as lua_ulong)
    {
        /* convert result of 'luaO_str2num' to a float */
        *n = if v.tt_ == 3i32 | 1i32 << 4i32 {
            v.value_.i as lua_Number
        } else {
            v.value_.n
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishget(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) -> () {
    /* counter to avoid infinite loops */
    let mut loop_0: lua_int = 0;
    /* metamethod */
    let mut tm: *const TValue = 0 as *const TValue;
    loop_0 = 0i32;
    while loop_0 < 2000i32 {
        if slot.is_null() {
            /* 't' is not a table? */
            tm = luaT_gettmbyobj(L, t, TM_INDEX);
            if (*tm).tt_ == 0i32 {
                /* no metamethod */
                luaG_typeerror(L, t, s!(b"index\x00"));
            }
        } else {
            /* else will try the metamethod */
            /* 't' is a table */
            /* table's metamethod */
            tm = if (*((*t).value_.gc as *mut GCUnion)).h.metatable.is_null() {
                0 as *const TValue
            } else if 0
                != (*(*((*t).value_.gc as *mut GCUnion)).h.metatable).flags as lua_uint
                    & 1u32 << TM_INDEX as lua_int
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*((*t).value_.gc as *mut GCUnion)).h.metatable,
                    TM_INDEX,
                    (*(*L).l_G).tmname[TM_INDEX as lua_int as usize],
                )
            };
            if tm.is_null() {
                /* no metamethod? */
                /* result is nil */
                (*val).tt_ = 0i32;
                return;
            }
        }
        /* else will try the metamethod */
        if (*tm).tt_ & 0xfi32 == 6i32 {
            /* is metamethod a function? */
            /* call it */
            luaT_callTM(L, tm, t, key, val, 1i32);
            return;
        } else {
            /* else try to access 'tm[key]' */
            t = tm;
            if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
                slot = 0 as *const TValue;
                0i32
            } else {
                slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
                !((*slot).tt_ == 0i32) as lua_int
            } {
                /* fast track? */
                /* done */
                let mut io1: *mut TValue = val;
                *io1 = *slot;
                return;
            } else {
                loop_0 += 1
            }
        }
    }
    /* else repeat (tail call 'luaV_finishget') */
    luaG_runerror!(L, s!(b"\'__index\' chain too long; possible loop\x00"),);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishset(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut key: *mut TValue,
    mut val: StkId,
    mut slot: *const TValue,
) -> () {
    /* counter to avoid infinite loops */
    let mut loop_0: lua_int = 0;
    loop_0 = 0i32;
    while loop_0 < 2000i32 {
        /* '__newindex' metamethod */
        let mut tm: *const TValue = 0 as *const TValue;
        if !slot.is_null() {
            /* is 't' a table? */
            /* save 't' table */
            let mut h: *mut Table = &mut (*((*t).value_.gc as *mut GCUnion)).h;
            /* old value must be nil */
            /* get metamethod */
            tm = if (*h).metatable.is_null() {
                0 as *const TValue
            } else if 0
                != (*(*h).metatable).flags as lua_uint & 1u32 << TM_NEWINDEX as lua_int
            {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_NEWINDEX,
                    (*(*L).l_G).tmname[TM_NEWINDEX as lua_int as usize],
                )
            };
            if tm.is_null() {
                /* no metamethod? */
                /* no previous entry? */
                if slot == &luaO_nilobject_ as *const TValue {
                    /* create one */
                    slot = luaH_newkey(L, h, key)
                }
                /* no metamethod and (now) there is an entry with given key */
                /* set its new value */
                *(slot as *mut TValue) = *val;
                (*h).flags = 0i32 as lu_byte;
                if 0 != (*val).tt_ & 1i32 << 6i32
                    && 0 != (*h).marked as lua_int & 1i32 << 2i32
                    && 0 != (*(*val).value_.gc).marked as lua_int
                        & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    luaC_barrierback_(L, h);
                } else {
                };
                return;
            }
        } else {
            /* else will try the metamethod */
            /* not a table; check metamethod */
            tm = luaT_gettmbyobj(L, t, TM_NEWINDEX);
            if (*tm).tt_ == 0i32 {
                luaG_typeerror(L, t, s!(b"index\x00"));
            }
        }
        /* try the metamethod */
        if (*tm).tt_ & 0xfi32 == 6i32 {
            luaT_callTM(L, tm, t, key, val, 0i32);
            return;
        } else {
            /* else repeat assignment over 'tm' */
            t = tm;
            if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
                slot = 0 as *const TValue;
                0i32
            } else {
                slot = luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, key);
                if (*slot).tt_ == 0i32 {
                    0i32
                } else {
                    if 0 != (*val).tt_ & 1i32 << 6i32
                        && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as lua_int
                            & 1i32 << 2i32
                        && 0 != (*(*val).value_.gc).marked as lua_int
                            & (1i32 << 0i32 | 1i32 << 1i32)
                    {
                        luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
                    } else {
                    };
                    *(slot as *mut TValue) = *val;
                    1i32
                }
            } {
                /* done */
                return;
            } else {
                loop_0 += 1
            }
        }
    }
    /* else loop */
    luaG_runerror!(L, s!(b"\'__newindex\' chain too long; possible loop\x00"),);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_finishOp(mut L: *mut lua_State) -> () {
    let mut ci: *mut CallInfo = (*L).ci;
    let mut base: StkId = (*ci).u.l.base;
    /* interrupted instruction */
    let mut inst: Instruction = *(*ci).u.l.savedpc.offset(-1isize);
    let mut op: OpCode = (inst >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
    match op as lua_uint {
        13 | 14 | 15 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 16 | 17 | 25 | 26 | 28 | 6 | 7 | 12 => {
            let mut io1: *mut TValue = base.offset(
                (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as lua_int
                    as isize,
            );
            (*L).top = (*L).top.offset(-1isize);
            *io1 = *(*L).top
        }
        33 | 32 | 31 => {
            let mut res: lua_int = !((*(*L).top.offset(-1isize)).tt_ == 0i32
                || (*(*L).top.offset(-1isize)).tt_ == 1i32
                    && (*(*L).top.offset(-1isize)).value_.b == 0i32)
                as lua_int;
            (*L).top = (*L).top.offset(-1isize);
            if 0 != (*ci).callstatus as lua_int & 1i32 << 7i32 {
                /* "<=" using "<" instead? */
                /* clear mark */
                (*ci).callstatus =
                    ((*ci).callstatus as lua_int ^ 1i32 << 7i32) as lua_ushort;
                /* negate result */
                res = (0 == res) as lua_int
            }
            /* condition failed? */
            if res
                != (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                    as lua_int
            {
                /* skip jump instruction */
                (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
            }
        }
        29 => {
            /* top when 'luaT_trybinTM' was called */
            let mut top: StkId = (*L).top.offset(-1isize);
            /* first element to concatenate */
            let mut b: lua_int = (inst >> 0i32 + 6i32 + 8i32 + 9i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                as lua_int;
            /* yet to concatenate */
            let mut total: lua_int = top
                .offset(-1isize)
                .wrapping_offset_from(base.offset(b as isize))
                as lua_long as lua_int;
            /* put TM result in proper position */
            let mut io1_0: *mut TValue = top.offset(-2isize);
            *io1_0 = *top;
            if total > 1i32 {
                /* are there elements to concat? */
                /* top is one after last element (at top-2) */
                (*L).top = top.offset(-1isize);
                /* concat them (may yield again) */
                luaV_concat(L, total);
            }
            /* move final result to final position */
            let mut io1_1: *mut TValue = (*ci).u.l.base.offset(
                (inst >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as lua_int
                    as isize,
            );
            *io1_1 = *(*L).top.offset(-1isize);
            /* restore top */
            (*L).top = (*ci).top
        }
        41 => {
            /* correct top */
            (*L).top = (*ci).top
        }
        36 => {
            /* nresults >= 0? */
            if (inst >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                as lua_int
                - 1i32
                >= 0i32
            {
                /* adjust results */
                (*L).top = (*ci).top
            }
        }
        37 | 8 | 10 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_concat(mut L: *mut lua_State, mut total: lua_int) -> () {
    let mut buff: [lua_char; 40] = [0; 40];
    loop {
        let mut top: StkId = (*L).top;
        /* number of elements handled in this pass (at least 2) */
        let mut n: lua_int = 2i32;
        if !((*top.offset(-2isize)).tt_ & 0xfi32 == 4i32
            || (*top.offset(-2isize)).tt_ & 0xfi32 == 3i32)
            || !((*top.offset(-1isize)).tt_ & 0xfi32 == 4i32
                || (*top.offset(-1isize)).tt_ & 0xfi32 == 3i32 && {
                    luaO_tostring(L, top.offset(-1isize));
                    0 != 1i32
                }) {
            luaT_trybinTM(
                L,
                top.offset(-2isize) as *const TValue,
                top.offset(-1isize) as *const TValue,
                top.offset(-2isize),
                TM_CONCAT,
            );
        } else if (*top.offset(-1isize)).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32
            && (*((*top.offset(-1isize)).value_.gc as *mut GCUnion))
                .ts
                .shrlen as lua_int
                == 0i32
        {
            /* result is first operand */
            ((*top.offset(-2isize)).tt_ & 0xfi32 == 4i32
                || (*top.offset(-2isize)).tt_ & 0xfi32 == 3i32 && {
                    luaO_tostring(L, top.offset(-2isize));
                    0 != 1i32
                }) as lua_int;
        } else if (*top.offset(-2isize)).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32
            && (*((*top.offset(-2isize)).value_.gc as *mut GCUnion))
                .ts
                .shrlen as lua_int
                == 0i32
        {
            /* first operand is an empty string? */
            /* result is second op. */
            let mut io1: *mut TValue = top.offset(-2isize);
            *io1 = *top.offset(-1isize)
        } else {
            /* at least two non-empty string values; get as many as possible */
            let mut tl: size_t = if (*((*top.offset(-1isize)).value_.gc as *mut GCUnion)).ts.tt
                as lua_int
                == 4i32 | 0i32 << 4i32
            {
                (*((*top.offset(-1isize)).value_.gc as *mut GCUnion))
                    .ts
                    .shrlen as lua_ulong
            } else {
                (*((*top.offset(-1isize)).value_.gc as *mut GCUnion))
                    .ts
                    .u
                    .lnglen
            };
            let mut ts: *mut TString = 0 as *mut TString;
            /* collect total length and number of strings */
            n = 1i32;
            while n < total
                && ((*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 4i32
                    || (*top.offset(-(n as isize)).offset(-1isize)).tt_ & 0xfi32 == 3i32 && {
                        luaO_tostring(L, top.offset(-(n as isize)).offset(-1isize));
                        0 != 1i32
                    }) {
                let mut l: size_t = if (*((*top.offset(-(n as isize)).offset(-1isize)).value_.gc
                    as *mut GCUnion))
                    .ts
                    .tt as lua_int
                    == 4i32 | 0i32 << 4i32
                {
                    (*((*top.offset(-(n as isize)).offset(-1isize)).value_.gc as *mut GCUnion))
                        .ts
                        .shrlen as lua_ulong
                } else {
                    (*((*top.offset(-(n as isize)).offset(-1isize)).value_.gc as *mut GCUnion))
                        .ts
                        .u
                        .lnglen
                };
                if l >= if (::std::mem::size_of::<size_t>() as lua_ulong)
                    < ::std::mem::size_of::<lua_Integer>() as lua_ulong
                {
                    !(0i32 as size_t)
                } else {
                    9223372036854775807i64 as size_t
                }
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(tl)
                {
                    luaG_runerror!(L, s!(b"string length overflow\x00"),);
                } else {
                    tl = (tl as lua_ulong).wrapping_add(l) as size_t as size_t;
                    n += 1
                }
            }
            if tl <= 40i32 as lua_ulong {
                /* is result a short string? */
                buff = [0; 40];
                /* copy strings to buffer */
                copy2buff(top, n, buff.as_mut_ptr());
                ts = luaS_newlstr(L, buff.as_mut_ptr(), tl)
            } else {
                /* long string; copy strings directly to final result */
                ts = luaS_createlngstrobj(L, tl);
                copy2buff(
                    top,
                    n,
                    (ts as *mut lua_char)
                        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
                );
            }
            /* create result */
            let mut io: *mut TValue = top.offset(-(n as isize));
            let mut x_: *mut TString = ts;
            (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
            (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32
        }
        /* got 'n' strings to create 1 new */
        total -= n - 1i32;
        /* popped 'n' strings and pushed one */
        (*L).top = (*L).top.offset(-((n - 1i32) as isize));
        if !(total > 1i32) {
            break;
        }
    }
}
/* macro used by 'luaV_concat' to ensure that element at 'o' is a string */
/* copy strings in stack from top - n up to top - 1 to buffer */
unsafe extern "C" fn copy2buff(
    mut top: StkId,
    mut n: lua_int,
    mut buff: *mut lua_char,
) -> () {
    /* size already copied */
    let mut tl: size_t = 0i32 as size_t;
    loop {
        /* length of string being copied */
        let mut l: size_t = if (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion))
            .ts
            .tt as lua_int
            == 4i32 | 0i32 << 4i32
        {
            (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion))
                .ts
                .shrlen as lua_ulong
        } else {
            (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion))
                .ts
                .u
                .lnglen
        };
        memcpy(
            buff.offset(tl as isize) as *mut lua_void,
            (&mut (*((*top.offset(-(n as isize))).value_.gc as *mut GCUnion)).ts as *mut TString
                as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                as *const lua_void,
            l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
        tl = (tl as lua_ulong).wrapping_add(l) as size_t as size_t;
        n -= 1;
        if !(n > 0i32) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaV_execute(mut L: *mut lua_State) -> () {
    let mut io_31: *mut TValue = 0 as *mut TValue;
    let mut x__0: *mut LClosure = 0 as *mut LClosure;
    let mut ci: *mut CallInfo = (*L).ci;
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    let mut k: *mut TValue = 0 as *mut TValue;
    let mut base: StkId = 0 as *mut TValue;
    /* fresh invocation of 'luaV_execute" */
    (*ci).callstatus = ((*ci).callstatus as lua_int | 1i32 << 3i32) as lua_ushort;
    /* reentry point when frame changes (call/return) */
    loop {
        /* local reference to function's closure */
        cl = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.l;
        /* local reference to function's constant table */
        k = (*(*cl).p).k;
        /* local copy of function's base */
        base = (*ci).u.l.base;
        /* main loop of interpreter */
        's_17: loop {
            let mut i: Instruction = 0;
            let mut ra: StkId = 0 as *mut TValue;
            let fresh0 = (*ci).u.l.savedpc;
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
            i = *fresh0;
            if 0 != (*L).hookmask & (1i32 << 2i32 | 1i32 << 3i32) {
                luaG_traceexec(L);
                base = (*ci).u.l.base
            }
            ra = base.offset(
                (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as lua_int
                    as isize,
            );
            match (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode
                as lua_uint
            {
                0 => {
                    let mut io1: *mut TValue = ra;
                    *io1 = *base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    continue;
                }
                1 => {
                    let mut rb: *mut TValue = k.offset(
                        (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut io1_0: *mut TValue = ra;
                    *io1_0 = *rb;
                    continue;
                }
                2 => {
                    let mut rb_0: *mut TValue = 0 as *mut TValue;
                    let fresh1 = (*ci).u.l.savedpc;
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                    rb_0 = k.offset(
                        (*fresh1 >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut io1_1: *mut TValue = ra;
                    *io1_1 = *rb_0;
                    continue;
                }
                3 => {
                    let mut io: *mut TValue = ra;
                    (*io).value_.b = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    (*io).tt_ = 1i32;
                    if !(0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int)
                    {
                        continue;
                    }
                    /* skip next instruction (if C) */
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                    continue;
                }
                4 => {
                    let mut b: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    loop {
                        let fresh2 = ra;
                        ra = ra.offset(1);
                        (*fresh2).tt_ = 0i32;
                        let fresh3 = b;
                        b = b - 1;
                        if !(0 != fresh3) {
                            continue 's_17;
                        }
                    }
                }
                5 => {
                    let mut b_0: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut io1_2: *mut TValue = ra;
                    *io1_2 = *(*(*cl).upvals[b_0 as usize]).v;
                    continue;
                }
                6 => {
                    let mut upval: *mut TValue = (*(*cl).upvals[(i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int
                        as usize])
                        .v;
                    let mut rc: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut slot: *const TValue = 0 as *const TValue;
                    if 0 != if !((*upval).tt_ == 5i32 | 1i32 << 6i32) {
                        slot = 0 as *const TValue;
                        0i32
                    } else {
                        slot = luaH_get(&mut (*((*upval).value_.gc as *mut GCUnion)).h, rc);
                        !((*slot).tt_ == 0i32) as lua_int
                    } {
                        let mut io1_3: *mut TValue = ra;
                        *io1_3 = *slot;
                        continue;
                    } else {
                        luaV_finishget(L, upval, rc, ra, slot);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                7 => {
                    let mut rb_1: StkId = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut rc_0: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut slot_0: *const TValue = 0 as *const TValue;
                    if 0 != if !((*rb_1).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_0 = 0 as *const TValue;
                        0i32
                    } else {
                        slot_0 = luaH_get(&mut (*((*rb_1).value_.gc as *mut GCUnion)).h, rc_0);
                        !((*slot_0).tt_ == 0i32) as lua_int
                    } {
                        let mut io1_4: *mut TValue = ra;
                        *io1_4 = *slot_0;
                        continue;
                    } else {
                        luaV_finishget(L, rb_1 as *const TValue, rc_0, ra, slot_0);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                8 => {
                    let mut upval_0: *mut TValue = (*(*cl).upvals[(i >> 0i32 + 6i32
                        & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as lua_int
                        as usize])
                        .v;
                    let mut rb_2: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_1: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut slot_1: *const TValue = 0 as *const TValue;
                    if !(0 == if !((*upval_0).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_1 = 0 as *const TValue;
                        0i32
                    } else {
                        slot_1 = luaH_get(&mut (*((*upval_0).value_.gc as *mut GCUnion)).h, rb_2);
                        if (*slot_1).tt_ == 0i32 {
                            0i32
                        } else {
                            if 0 != (*rc_1).tt_ & 1i32 << 6i32
                                && 0 != (*((*upval_0).value_.gc as *mut GCUnion)).h.marked
                                    as lua_int
                                    & 1i32 << 2i32
                                && 0 != (*(*rc_1).value_.gc).marked as lua_int
                                    & (1i32 << 0i32 | 1i32 << 1i32)
                            {
                                luaC_barrierback_(
                                    L,
                                    &mut (*((*upval_0).value_.gc as *mut GCUnion)).h,
                                );
                            } else {
                            };
                            *(slot_1 as *mut TValue) = *rc_1;
                            1i32
                        }
                    }) {
                        continue;
                    }
                    luaV_finishset(L, upval_0, rb_2, rc_1, slot_1);
                    base = (*ci).u.l.base;
                    continue;
                }
                9 => {
                    let mut uv: *mut UpVal = (*cl).upvals[(i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int
                        as usize];
                    let mut io1_5: *mut TValue = (*uv).v;
                    *io1_5 = *ra;
                    if 0 != (*(*uv).v).tt_ & 1i32 << 6i32
                        && !((*uv).v != &mut (*uv).u.value as *mut TValue)
                    {
                        luaC_upvalbarrier_(L, uv);
                    } else {
                    };
                    continue;
                }
                10 => {
                    let mut rb_3: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_2: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut slot_2: *const TValue = 0 as *const TValue;
                    if !(0 == if !((*ra).tt_ == 5i32 | 1i32 << 6i32) {
                        slot_2 = 0 as *const TValue;
                        0i32
                    } else {
                        slot_2 = luaH_get(&mut (*((*ra).value_.gc as *mut GCUnion)).h, rb_3);
                        if (*slot_2).tt_ == 0i32 {
                            0i32
                        } else {
                            if 0 != (*rc_2).tt_ & 1i32 << 6i32
                                && 0 != (*((*ra).value_.gc as *mut GCUnion)).h.marked as lua_int
                                    & 1i32 << 2i32
                                && 0 != (*(*rc_2).value_.gc).marked as lua_int
                                    & (1i32 << 0i32 | 1i32 << 1i32)
                            {
                                luaC_barrierback_(L, &mut (*((*ra).value_.gc as *mut GCUnion)).h);
                            } else {
                            };
                            *(slot_2 as *mut TValue) = *rc_2;
                            1i32
                        }
                    }) {
                        continue;
                    }
                    luaV_finishset(L, ra as *const TValue, rb_3, rc_2, slot_2);
                    base = (*ci).u.l.base;
                    continue;
                }
                11 => {
                    let mut b_1: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut c: lua_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut t: *mut Table = luaH_new(L);
                    let mut io_0: *mut TValue = ra;
                    let mut x_: *mut Table = t;
                    (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                    (*io_0).tt_ = 5i32 | 1i32 << 6i32;
                    if b_1 != 0i32 || c != 0i32 {
                        luaH_resize(
                            L,
                            t,
                            luaO_fb2int(b_1) as lua_uint,
                            luaO_fb2int(c) as lua_uint,
                        );
                    }
                    if !((*(*L).l_G).GCdebt > 0i32 as lua_long) {
                        continue;
                    }
                    (*L).top = ra.offset(1isize);
                    luaC_step(L);
                    (*L).top = (*ci).top;
                    base = (*ci).u.l.base;
                    continue;
                }
                12 => {
                    let mut aux: *const TValue = 0 as *const TValue;
                    let mut rb_4: StkId = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut rc_3: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    /* key must be a string */
                    let mut key: *mut TString = &mut (*((*rc_3).value_.gc as *mut GCUnion)).ts;
                    let mut io1_6: *mut TValue = ra.offset(1isize);
                    *io1_6 = *rb_4;
                    if 0 != if !((*rb_4).tt_ == 5i32 | 1i32 << 6i32) {
                        aux = 0 as *const TValue;
                        0i32
                    } else {
                        aux = luaH_getstr(&mut (*((*rb_4).value_.gc as *mut GCUnion)).h, key);
                        !((*aux).tt_ == 0i32) as lua_int
                    } {
                        let mut io1_7: *mut TValue = ra;
                        *io1_7 = *aux;
                        continue;
                    } else {
                        luaV_finishget(L, rb_4 as *const TValue, rc_3, ra, aux);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                13 => {
                    let mut rb_5: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_4: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb: lua_Number = 0.;
                    let mut nc: lua_Number = 0.;
                    if (*rb_5).tt_ == 3i32 | 1i32 << 4i32 && (*rc_4).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib: lua_Integer = (*rb_5).value_.i;
                        let mut ic: lua_Integer = (*rc_4).value_.i;
                        let mut io_1: *mut TValue = ra;
                        (*io_1).value_.i =
                            (ib as lua_Unsigned).wrapping_add(ic as lua_Unsigned) as lua_Integer;
                        (*io_1).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_5).tt_ == 3i32 | 0i32 << 4i32 {
                        nb = (*rb_5).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_5, &mut nb)
                    } && 0 != if (*rc_4).tt_ == 3i32 | 0i32 << 4i32 {
                        nc = (*rc_4).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_4, &mut nc)
                    } {
                        let mut io_2: *mut TValue = ra;
                        (*io_2).value_.n = nb + nc;
                        (*io_2).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_5, rc_4, ra, TM_ADD);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                14 => {
                    let mut rb_6: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_5: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_0: lua_Number = 0.;
                    let mut nc_0: lua_Number = 0.;
                    if (*rb_6).tt_ == 3i32 | 1i32 << 4i32 && (*rc_5).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib_0: lua_Integer = (*rb_6).value_.i;
                        let mut ic_0: lua_Integer = (*rc_5).value_.i;
                        let mut io_3: *mut TValue = ra;
                        (*io_3).value_.i = (ib_0 as lua_Unsigned).wrapping_sub(ic_0 as lua_Unsigned)
                            as lua_Integer;
                        (*io_3).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_6).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_0 = (*rb_6).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_6, &mut nb_0)
                    } && 0 != if (*rc_5).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_0 = (*rc_5).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_5, &mut nc_0)
                    } {
                        let mut io_4: *mut TValue = ra;
                        (*io_4).value_.n = nb_0 - nc_0;
                        (*io_4).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_6, rc_5, ra, TM_SUB);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                15 => {
                    let mut rb_7: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_6: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_1: lua_Number = 0.;
                    let mut nc_1: lua_Number = 0.;
                    if (*rb_7).tt_ == 3i32 | 1i32 << 4i32 && (*rc_6).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib_1: lua_Integer = (*rb_7).value_.i;
                        let mut ic_1: lua_Integer = (*rc_6).value_.i;
                        let mut io_5: *mut TValue = ra;
                        (*io_5).value_.i = (ib_1 as lua_Unsigned).wrapping_mul(ic_1 as lua_Unsigned)
                            as lua_Integer;
                        (*io_5).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_7).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_1 = (*rb_7).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_7, &mut nb_1)
                    } && 0 != if (*rc_6).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_1 = (*rc_6).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_6, &mut nc_1)
                    } {
                        let mut io_6: *mut TValue = ra;
                        (*io_6).value_.n = nb_1 * nc_1;
                        (*io_6).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_7, rc_6, ra, TM_MUL);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                18 => {
                    /* float division (always with floats) */
                    let mut rb_8: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_7: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_2: lua_Number = 0.;
                    let mut nc_2: lua_Number = 0.;
                    if 0 != if (*rb_8).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_2 = (*rb_8).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_8, &mut nb_2)
                    } && 0 != if (*rc_7).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_2 = (*rc_7).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_7, &mut nc_2)
                    } {
                        let mut io_7: *mut TValue = ra;
                        (*io_7).value_.n = nb_2 / nc_2;
                        (*io_7).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_8, rc_7, ra, TM_DIV);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                20 => {
                    let mut rb_9: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_8: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut ib_2: lua_Integer = 0;
                    let mut ic_2: lua_Integer = 0;
                    if 0 != if (*rb_9).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_2 = (*rb_9).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_9, &mut ib_2, 0i32)
                    } && 0 != if (*rc_8).tt_ == 3i32 | 1i32 << 4i32 {
                        ic_2 = (*rc_8).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_8, &mut ic_2, 0i32)
                    } {
                        let mut io_8: *mut TValue = ra;
                        (*io_8).value_.i =
                            (ib_2 as lua_Unsigned & ic_2 as lua_Unsigned) as lua_Integer;
                        (*io_8).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_9, rc_8, ra, TM_BAND);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                21 => {
                    let mut rb_10: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_9: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut ib_3: lua_Integer = 0;
                    let mut ic_3: lua_Integer = 0;
                    if 0 != if (*rb_10).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_3 = (*rb_10).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_10, &mut ib_3, 0i32)
                    } && 0 != if (*rc_9).tt_ == 3i32 | 1i32 << 4i32 {
                        ic_3 = (*rc_9).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_9, &mut ic_3, 0i32)
                    } {
                        let mut io_9: *mut TValue = ra;
                        (*io_9).value_.i =
                            (ib_3 as lua_Unsigned | ic_3 as lua_Unsigned) as lua_Integer;
                        (*io_9).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_10, rc_9, ra, TM_BOR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                22 => {
                    let mut rb_11: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_10: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut ib_4: lua_Integer = 0;
                    let mut ic_4: lua_Integer = 0;
                    if 0 != if (*rb_11).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_4 = (*rb_11).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_11, &mut ib_4, 0i32)
                    } && 0 != if (*rc_10).tt_ == 3i32 | 1i32 << 4i32 {
                        ic_4 = (*rc_10).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_10, &mut ic_4, 0i32)
                    } {
                        let mut io_10: *mut TValue = ra;
                        (*io_10).value_.i =
                            (ib_4 as lua_Unsigned ^ ic_4 as lua_Unsigned) as lua_Integer;
                        (*io_10).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_11, rc_10, ra, TM_BXOR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                23 => {
                    let mut rb_12: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_11: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut ib_5: lua_Integer = 0;
                    let mut ic_5: lua_Integer = 0;
                    if 0 != if (*rb_12).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_5 = (*rb_12).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_12, &mut ib_5, 0i32)
                    } && 0 != if (*rc_11).tt_ == 3i32 | 1i32 << 4i32 {
                        ic_5 = (*rc_11).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_11, &mut ic_5, 0i32)
                    } {
                        let mut io_11: *mut TValue = ra;
                        (*io_11).value_.i = luaV_shiftl(ib_5, ic_5);
                        (*io_11).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_12, rc_11, ra, TM_SHL);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                24 => {
                    let mut rb_13: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_12: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut ib_6: lua_Integer = 0;
                    let mut ic_6: lua_Integer = 0;
                    if 0 != if (*rb_13).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_6 = (*rb_13).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_13, &mut ib_6, 0i32)
                    } && 0 != if (*rc_12).tt_ == 3i32 | 1i32 << 4i32 {
                        ic_6 = (*rc_12).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rc_12, &mut ic_6, 0i32)
                    } {
                        let mut io_12: *mut TValue = ra;
                        (*io_12).value_.i = luaV_shiftl(ib_6, -ic_6);
                        (*io_12).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_13, rc_12, ra, TM_SHR);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                16 => {
                    let mut rb_14: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_13: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_3: lua_Number = 0.;
                    let mut nc_3: lua_Number = 0.;
                    if (*rb_14).tt_ == 3i32 | 1i32 << 4i32 && (*rc_13).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib_7: lua_Integer = (*rb_14).value_.i;
                        let mut ic_7: lua_Integer = (*rc_13).value_.i;
                        let mut io_13: *mut TValue = ra;
                        (*io_13).value_.i = luaV_mod(L, ib_7, ic_7);
                        (*io_13).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_14).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_3 = (*rb_14).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_14, &mut nb_3)
                    } && 0 != if (*rc_13).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_3 = (*rc_13).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_13, &mut nc_3)
                    } {
                        let mut m: lua_Number = 0.;
                        m = fmod(nb_3, nc_3);
                        if m * nc_3 < 0i32 as lua_double {
                            m += nc_3
                        }
                        let mut io_14: *mut TValue = ra;
                        (*io_14).value_.n = m;
                        (*io_14).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_14, rc_13, ra, TM_MOD);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                19 => {
                    /* floor division */
                    let mut rb_15: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_14: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_4: lua_Number = 0.;
                    let mut nc_4: lua_Number = 0.;
                    if (*rb_15).tt_ == 3i32 | 1i32 << 4i32 && (*rc_14).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib_8: lua_Integer = (*rb_15).value_.i;
                        let mut ic_8: lua_Integer = (*rc_14).value_.i;
                        let mut io_15: *mut TValue = ra;
                        (*io_15).value_.i = luaV_div(L, ib_8, ic_8);
                        (*io_15).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_15).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_4 = (*rb_15).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_15, &mut nb_4)
                    } && 0 != if (*rc_14).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_4 = (*rc_14).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_14, &mut nc_4)
                    } {
                        let mut io_16: *mut TValue = ra;
                        (*io_16).value_.n = floor(nb_4 / nc_4);
                        (*io_16).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_15, rc_14, ra, TM_IDIV);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                17 => {
                    let mut rb_16: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_15: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut nb_5: lua_Number = 0.;
                    let mut nc_5: lua_Number = 0.;
                    if 0 != if (*rb_16).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_5 = (*rb_16).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_16, &mut nb_5)
                    } && 0 != if (*rc_15).tt_ == 3i32 | 0i32 << 4i32 {
                        nc_5 = (*rc_15).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rc_15, &mut nc_5)
                    } {
                        let mut io_17: *mut TValue = ra;
                        (*io_17).value_.n = pow(nb_5, nc_5);
                        (*io_17).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_16, rc_15, ra, TM_POW);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                25 => {
                    let mut rb_17: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut nb_6: lua_Number = 0.;
                    if (*rb_17).tt_ == 3i32 | 1i32 << 4i32 {
                        let mut ib_9: lua_Integer = (*rb_17).value_.i;
                        let mut io_18: *mut TValue = ra;
                        (*io_18).value_.i = (0i32 as lua_Unsigned)
                            .wrapping_sub(ib_9 as lua_Unsigned)
                            as lua_Integer;
                        (*io_18).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else if 0 != if (*rb_17).tt_ == 3i32 | 0i32 << 4i32 {
                        nb_6 = (*rb_17).value_.n;
                        1i32
                    } else {
                        luaV_tonumber_(rb_17, &mut nb_6)
                    } {
                        let mut io_19: *mut TValue = ra;
                        (*io_19).value_.n = -nb_6;
                        (*io_19).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_17, rb_17, ra, TM_UNM);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                26 => {
                    let mut rb_18: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    let mut ib_10: lua_Integer = 0;
                    if 0 != if (*rb_18).tt_ == 3i32 | 1i32 << 4i32 {
                        ib_10 = (*rb_18).value_.i;
                        1i32
                    } else {
                        luaV_tointeger(rb_18, &mut ib_10, 0i32)
                    } {
                        let mut io_20: *mut TValue = ra;
                        (*io_20).value_.i =
                            (!(0i32 as lua_Unsigned) ^ ib_10 as lua_Unsigned) as lua_Integer;
                        (*io_20).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        luaT_trybinTM(L, rb_18, rb_18, ra, TM_BNOT);
                        base = (*ci).u.l.base;
                        continue;
                    }
                }
                27 => {
                    let mut rb_19: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    /* next assignment may change this value */
                    let mut res: lua_int = ((*rb_19).tt_ == 0i32
                        || (*rb_19).tt_ == 1i32 && (*rb_19).value_.b == 0i32)
                        as lua_int;
                    let mut io_21: *mut TValue = ra;
                    (*io_21).value_.b = res;
                    (*io_21).tt_ = 1i32;
                    continue;
                }
                28 => {
                    luaV_objlen(
                        L,
                        ra,
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        ) as *const TValue,
                    );
                    base = (*ci).u.l.base;
                    continue;
                }
                29 => {
                    let mut b_2: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut c_0: lua_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut rb_20: StkId = 0 as *mut TValue;
                    /* mark the end of concat operands */
                    (*L).top = base.offset(c_0 as isize).offset(1isize);
                    luaV_concat(L, c_0 - b_2 + 1i32);
                    base = (*ci).u.l.base;
                    /* 'luaV_concat' may invoke TMs and move the stack */
                    ra = base.offset(
                        (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int as isize,
                    );
                    rb_20 = base.offset(b_2 as isize);
                    let mut io1_8: *mut TValue = ra;
                    *io1_8 = *rb_20;
                    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
                        (*L).top = if ra >= rb_20 {
                            ra.offset(1isize)
                        } else {
                            rb_20
                        };
                        luaC_step(L);
                        (*L).top = (*ci).top;
                        base = (*ci).u.l.base
                    }
                    /* restore top */
                    (*L).top = (*ci).top;
                    continue;
                }
                30 => {
                    let mut a: lua_int = (i >> 0i32 + 6i32
                        & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as lua_int;
                    if a != 0i32 {
                        luaF_close(L, (*ci).u.l.base.offset(a as isize).offset(-1isize));
                    }
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                        ((i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as lua_int
                            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                            + 0i32) as isize,
                    );
                    continue;
                }
                31 => {
                    let mut rb_21: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 + 9i32
                                & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    let mut rc_16: *mut TValue = if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                    {
                        k.offset(
                            ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int
                                & !(1i32 << 9i32 - 1i32)) as isize,
                        )
                    } else {
                        base.offset(
                            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                as lua_int as isize,
                        )
                    };
                    if luaV_equalobj(L, rb_21, rc_16)
                        != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_0: lua_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int;
                        if a_0 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_0 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                32 => {
                    if luaV_lessthan(
                        L,
                        if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int as isize,
                            )
                        },
                        if 0 != (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int as isize,
                            )
                        },
                    ) != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as lua_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_1: lua_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int;
                        if a_1 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_1 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                33 => {
                    if luaV_lessequal(
                        L,
                        if 0 != (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32 + 9i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int as isize,
                            )
                        },
                        if 0 != (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                            & 1i32 << 9i32 - 1i32
                        {
                            k.offset(
                                ((i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int
                                    & !(1i32 << 9i32 - 1i32))
                                    as isize,
                            )
                        } else {
                            base.offset(
                                (i >> 0i32 + 6i32 + 8i32
                                    & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                                    as lua_int as isize,
                            )
                        },
                    ) != (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                        as lua_int
                    {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize)
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_2: lua_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int;
                        if a_2 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_2 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        )
                    }
                    base = (*ci).u.l.base;
                    continue;
                }
                34 => {
                    if 0 != if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                    {
                        ((*ra).tt_ == 0i32 || (*ra).tt_ == 1i32 && (*ra).value_.b == 0i32)
                            as lua_int
                    } else {
                        !((*ra).tt_ == 0i32 || (*ra).tt_ == 1i32 && (*ra).value_.b == 0i32)
                            as lua_int
                    } {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                        continue;
                    } else {
                        i = *(*ci).u.l.savedpc;
                        let mut a_3: lua_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int;
                        if a_3 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_3 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        );
                        continue;
                    }
                }
                35 => {
                    let mut rb_22: *mut TValue = base.offset(
                        (i >> 0i32 + 6i32 + 8i32 + 9i32
                            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    if 0 != if 0
                        != (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int
                    {
                        ((*rb_22).tt_ == 0i32 || (*rb_22).tt_ == 1i32 && (*rb_22).value_.b == 0i32)
                            as lua_int
                    } else {
                        !((*rb_22).tt_ == 0i32 || (*rb_22).tt_ == 1i32 && (*rb_22).value_.b == 0i32)
                            as lua_int
                    } {
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1isize);
                        continue;
                    } else {
                        let mut io1_9: *mut TValue = ra;
                        *io1_9 = *rb_22;
                        i = *(*ci).u.l.savedpc;
                        let mut a_4: lua_int = (i >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int;
                        if a_4 != 0i32 {
                            luaF_close(L, (*ci).u.l.base.offset(a_4 as isize).offset(-1isize));
                        }
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)
                                + 1i32) as isize,
                        );
                        continue;
                    }
                }
                36 => {
                    let mut b_3: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut nresults: lua_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int
                        - 1i32;
                    if b_3 != 0i32 {
                        /* else previous instruction set top */
                        (*L).top = ra.offset(b_3 as isize)
                    }
                    if 0 != luaD_precall(L, ra, nresults) {
                        /* C function? */
                        if nresults >= 0i32 {
                            /* adjust results */
                            (*L).top = (*ci).top
                        }
                        base = (*ci).u.l.base;
                        /* update 'base' */
                        continue;
                    } else {
                        /* Lua function */
                        ci = (*L).ci;
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                37 => {
                    let mut b_4: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    if b_4 != 0i32 {
                        /* else previous instruction set top */
                        (*L).top = ra.offset(b_4 as isize)
                    }
                    if 0 != luaD_precall(L, ra, -1i32) {
                        /* C function? */
                        base = (*ci).u.l.base;
                        /* update 'base' */
                        continue;
                    } else {
                        /* tail call: put called frame (n) in place of caller one (o) */
                        /* called frame */
                        let mut nci: *mut CallInfo = (*L).ci;
                        /* caller frame */
                        let mut oci: *mut CallInfo = (*nci).previous;
                        /* called function */
                        let mut nfunc: StkId = (*nci).func;
                        /* caller function */
                        let mut ofunc: StkId = (*oci).func;
                        /* last stack slot filled by 'precall' */
                        let mut lim: StkId = (*nci).u.l.base.offset(
                            (*(*((*nfunc).value_.gc as *mut GCUnion)).cl.l.p).numparams
                                as lua_int as isize,
                        );
                        let mut aux_0: lua_int = 0;
                        /* close all upvalues from previous call */
                        if (*(*cl).p).sizep > 0i32 {
                            luaF_close(L, (*oci).u.l.base);
                        }
                        /* move new frame into old one */
                        aux_0 = 0i32;
                        while nfunc.offset(aux_0 as isize) < lim {
                            let mut io1_10: *mut TValue = ofunc.offset(aux_0 as isize);
                            *io1_10 = *nfunc.offset(aux_0 as isize);
                            aux_0 += 1
                        }
                        /* correct base */
                        (*oci).u.l.base = ofunc
                            .offset((*nci).u.l.base.wrapping_offset_from(nfunc) as lua_long
                                as isize);
                        /* correct top */
                        (*L).top = ofunc
                            .offset((*L).top.wrapping_offset_from(nfunc) as lua_long as isize);
                        (*oci).top = (*L).top;
                        (*oci).u.l.savedpc = (*nci).u.l.savedpc;
                        /* function was tail called */
                        (*oci).callstatus =
                            ((*oci).callstatus as lua_int | 1i32 << 5i32) as lua_ushort;
                        /* remove new frame */
                        (*L).ci = oci;
                        ci = (*L).ci;
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                38 => {
                    let mut b_5: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    if (*(*cl).p).sizep > 0i32 {
                        luaF_close(L, base);
                    }
                    b_5 = luaD_poscall(
                        L,
                        ci,
                        ra,
                        if b_5 != 0i32 {
                            b_5 - 1i32
                        } else {
                            (*L).top.wrapping_offset_from(ra) as lua_long as lua_int
                        },
                    );
                    /* local 'ci' still from callee */
                    if 0 != (*ci).callstatus as lua_int & 1i32 << 3i32 {
                        /* external invocation: return */
                        return;
                    } else {
                        /* invocation via reentry: continue execution */
                        ci = (*L).ci;
                        if 0 != b_5 {
                            (*L).top = (*ci).top
                        }
                        /* restart luaV_execute over new Lua function */
                        break;
                    }
                }
                39 => {
                    if (*ra).tt_ == 3i32 | 1i32 << 4i32 {
                        /* integer loop? */
                        let mut step: lua_Integer = (*ra.offset(2isize)).value_.i;
                        /* increment index */
                        let mut idx: lua_Integer = ((*ra).value_.i as lua_Unsigned)
                            .wrapping_add(step as lua_Unsigned)
                            as lua_Integer;
                        let mut limit: lua_Integer = (*ra.offset(1isize)).value_.i;
                        if !(0 != if (0i32 as lua_longlong) < step {
                            (idx <= limit) as lua_int
                        } else {
                            (limit <= idx) as lua_int
                        }) {
                            continue;
                        }
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                                as isize,
                        );
                        /* update internal index... */
                        let mut io_22: *mut TValue = ra;
                        (*io_22).value_.i = idx;
                        /* ...and external index */
                        let mut io_23: *mut TValue = ra.offset(3isize);
                        (*io_23).value_.i = idx;
                        (*io_23).tt_ = 3i32 | 1i32 << 4i32;
                        continue;
                    } else {
                        /* floating loop */
                        let mut step_0: lua_Number = (*ra.offset(2isize)).value_.n;
                        /* inc. index */
                        let mut idx_0: lua_Number = (*ra).value_.n + step_0;
                        let mut limit_0: lua_Number = (*ra.offset(1isize)).value_.n;
                        if !(0 != if (0i32 as lua_double) < step_0 {
                            (idx_0 <= limit_0) as lua_int
                        } else {
                            (limit_0 <= idx_0) as lua_int
                        }) {
                            continue;
                        }
                        /* jump back */
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                            ((i >> 0i32 + 6i32 + 8i32
                                & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                                as lua_int
                                - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                                as isize,
                        );
                        /* update internal index... */
                        let mut io_24: *mut TValue = ra;
                        (*io_24).value_.n = idx_0;
                        /* ...and external index */
                        let mut io_25: *mut TValue = ra.offset(3isize);
                        (*io_25).value_.n = idx_0;
                        (*io_25).tt_ = 3i32 | 0i32 << 4i32;
                        continue;
                    }
                }
                40 => {
                    let mut init: *mut TValue = ra;
                    let mut plimit: *mut TValue = ra.offset(1isize);
                    let mut pstep: *mut TValue = ra.offset(2isize);
                    let mut ilimit: lua_Integer = 0;
                    let mut stopnow: lua_int = 0;
                    if (*init).tt_ == 3i32 | 1i32 << 4i32
                        && (*pstep).tt_ == 3i32 | 1i32 << 4i32
                        && 0 != forlimit(plimit, &mut ilimit, (*pstep).value_.i, &mut stopnow)
                    {
                        /* all values are integer */
                        let mut initv: lua_Integer = if 0 != stopnow {
                            0i32 as lua_longlong
                        } else {
                            (*init).value_.i
                        };
                        let mut io_26: *mut TValue = plimit;
                        (*io_26).value_.i = ilimit;
                        (*io_26).tt_ = 3i32 | 1i32 << 4i32;
                        let mut io_27: *mut TValue = init;
                        (*io_27).value_.i = (initv as lua_Unsigned)
                            .wrapping_sub((*pstep).value_.i as lua_Unsigned)
                            as lua_Integer;
                        (*io_27).tt_ = 3i32 | 1i32 << 4i32
                    } else {
                        /* try making all values floats */
                        let mut ninit: lua_Number = 0.;
                        let mut nlimit: lua_Number = 0.;
                        let mut nstep: lua_Number = 0.;
                        if 0 == if (*plimit).tt_ == 3i32 | 0i32 << 4i32 {
                            nlimit = (*plimit).value_.n;
                            1i32
                        } else {
                            luaV_tonumber_(plimit, &mut nlimit)
                        } {
                            luaG_runerror!(L, s!(b"\'for\' limit must be a number\x00"),);
                        } else {
                            let mut io_28: *mut TValue = plimit;
                            (*io_28).value_.n = nlimit;
                            (*io_28).tt_ = 3i32 | 0i32 << 4i32;
                            if 0 == if (*pstep).tt_ == 3i32 | 0i32 << 4i32 {
                                nstep = (*pstep).value_.n;
                                1i32
                            } else {
                                luaV_tonumber_(pstep, &mut nstep)
                            } {
                                luaG_runerror!(L, s!(b"\'for\' step must be a number\x00"),);
                            } else {
                                let mut io_29: *mut TValue = pstep;
                                (*io_29).value_.n = nstep;
                                (*io_29).tt_ = 3i32 | 0i32 << 4i32;
                                if 0 == if (*init).tt_ == 3i32 | 0i32 << 4i32 {
                                    ninit = (*init).value_.n;
                                    1i32
                                } else {
                                    luaV_tonumber_(init, &mut ninit)
                                } {
                                    luaG_runerror!(
                                        L,
                                        s!(b"\'for\' initial value must be a number\x00"),
                                    );
                                } else {
                                    let mut io_30: *mut TValue = init;
                                    (*io_30).value_.n = ninit - nstep;
                                    (*io_30).tt_ = 3i32 | 0i32 << 4i32
                                }
                            }
                        }
                    }
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                        ((i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as lua_int
                            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32))
                            as isize,
                    );
                    continue;
                }
                41 => {
                    /* call base */
                    let mut cb: StkId = ra.offset(3isize);
                    let mut io1_11: *mut TValue = cb.offset(2isize);
                    *io1_11 = *ra.offset(2isize);
                    let mut io1_12: *mut TValue = cb.offset(1isize);
                    *io1_12 = *ra.offset(1isize);
                    let mut io1_13: *mut TValue = cb;
                    *io1_13 = *ra;
                    /* func. + 2 args (state and index) */
                    (*L).top = cb.offset(3isize);
                    luaD_call(
                        L,
                        cb,
                        (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                            as lua_int,
                    );
                    base = (*ci).u.l.base;
                    (*L).top = (*ci).top;
                    /* go to next instruction */
                    let fresh4 = (*ci).u.l.savedpc;
                    (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                    i = *fresh4;
                    ra = base.offset(
                        (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                            as lua_int as isize,
                    )
                }
                42 => {}
                43 => {
                    let mut n: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut c_1: lua_int = (i >> 0i32 + 6i32 + 8i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int;
                    let mut last: lua_uint = 0;
                    let mut h: *mut Table = 0 as *mut Table;
                    if n == 0i32 {
                        n = (*L).top.wrapping_offset_from(ra) as lua_long as lua_int - 1i32
                    }
                    if c_1 == 0i32 {
                        let fresh5 = (*ci).u.l.savedpc;
                        (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(1);
                        c_1 = (*fresh5 >> 0i32 + 6i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
                            as lua_int
                    }
                    h = &mut (*((*ra).value_.gc as *mut GCUnion)).h;
                    last = ((c_1 - 1i32) * 50i32 + n) as lua_uint;
                    /* needs more space? */
                    if last > (*h).sizearray {
                        /* preallocate it at once */
                        luaH_resizearray(L, h, last);
                    }
                    while n > 0i32 {
                        let mut val: *mut TValue = ra.offset(n as isize);
                        let fresh6 = last;
                        last = last.wrapping_sub(1);
                        luaH_setint(L, h, fresh6 as lua_Integer, val);
                        if 0 != (*val).tt_ & 1i32 << 6i32
                            && 0 != (*h).marked as lua_int & 1i32 << 2i32
                            && 0 != (*(*val).value_.gc).marked as lua_int
                                & (1i32 << 0i32 | 1i32 << 1i32)
                        {
                            luaC_barrierback_(L, h);
                        } else {
                        };
                        n -= 1
                    }
                    /* correct top (in case of previous open call) */
                    (*L).top = (*ci).top;
                    continue;
                }
                44 => {
                    let mut p: *mut Proto = *(*(*cl).p).p.offset(
                        (i >> 0i32 + 6i32 + 8i32
                            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                            as lua_int as isize,
                    );
                    /* cached closure */
                    let mut ncl: *mut LClosure = getcached(p, (*cl).upvals.as_mut_ptr(), base);
                    /* no match? */
                    if ncl.is_null() {
                        /* create a new one */
                        pushclosure(L, p, (*cl).upvals.as_mut_ptr(), base, ra);
                    } else {
                        io_31 = ra;
                        /* push cashed closure */
                        x__0 = ncl;
                        (*io_31).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
                        (*io_31).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32
                    }
                    if !((*(*L).l_G).GCdebt > 0i32 as lua_long) {
                        continue;
                    }
                    (*L).top = ra.offset(1isize);
                    luaC_step(L);
                    (*L).top = (*ci).top;
                    base = (*ci).u.l.base;
                    continue;
                }
                45 => {
                    /* required results */
                    let mut b_6: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
                        & !((!(0i32 as Instruction)) << 9i32) << 0i32)
                        as lua_int
                        - 1i32;
                    let mut j: lua_int = 0;
                    let mut n_0: lua_int = base.wrapping_offset_from((*ci).func) as lua_long
                        as lua_int
                        - (*(*cl).p).numparams as lua_int
                        - 1i32;
                    /* less arguments than parameters? */
                    if n_0 < 0i32 {
                        /* no vararg arguments */
                        n_0 = 0i32
                    }
                    if b_6 < 0i32 {
                        /* B == 0? */
                        /* get all var. arguments */
                        b_6 = n_0;
                        if (*L).stack_last.wrapping_offset_from((*L).top) as lua_long
                            <= n_0 as lua_long
                        {
                            luaD_growstack(L, n_0);
                        }
                        base = (*ci).u.l.base;
                        /* previous call may change the stack */
                        ra = base.offset(
                            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32)
                                as lua_int as isize,
                        );
                        (*L).top = ra.offset(n_0 as isize)
                    }
                    j = 0i32;
                    while j < b_6 && j < n_0 {
                        let mut io1_15: *mut TValue = ra.offset(j as isize);
                        *io1_15 = *base.offset(-(n_0 as isize)).offset(j as isize);
                        j += 1
                    }
                    /* complete required results with nil */
                    loop {
                        if !(j < b_6) {
                            continue 's_17;
                        }
                        (*ra.offset(j as isize)).tt_ = 0i32;
                        j += 1
                    }
                }
                46 | _ => {
                    continue;
                }
            }
            if (*ra.offset(1isize)).tt_ == 0i32 {
                continue;
            }
            /* continue loop? */
            /* save control variable */
            let mut io1_14: *mut TValue = ra;
            *io1_14 = *ra.offset(1isize);
            /* jump back */
            (*ci).u.l.savedpc = (*ci).u.l.savedpc.offset(
                ((i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
                    as lua_int
                    - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as isize,
            )
        }
    }
}
/*
** check whether cached closure in prototype 'p' may be reused, that is,
** whether there is a cached closure with the same upvalues needed by
** new closure to be created.
*/
unsafe extern "C" fn getcached(
    mut p: *mut Proto,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
) -> *mut LClosure {
    let mut c: *mut LClosure = (*p).cache;
    if !c.is_null() {
        /* is there a cached closure? */
        let mut nup: lua_int = (*p).sizeupvalues;
        let mut uv: *mut Upvaldesc = (*p).upvalues;
        let mut i: lua_int = 0;
        i = 0i32;
        while i < nup {
            /* check whether it has right upvalues */
            let mut v: *mut TValue = if 0 != (*uv.offset(i as isize)).instack as lua_int {
                base.offset((*uv.offset(i as isize)).idx as lua_int as isize)
            } else {
                (**encup.offset((*uv.offset(i as isize)).idx as isize)).v
            };
            if (*(*c).upvals[i as usize]).v != v {
                /* wrong upvalue; cannot reuse closure */
                return 0 as *mut LClosure;
            } else {
                i += 1
            }
        }
    }
    /* return cached closure (or NULL if no cached closure) */
    return c;
}
/*
** create a new Lua closure, push it in the stack, and initialize
** its upvalues. Note that the closure is not cached if prototype is
** already black (which means that 'cache' was already cleared by the
** GC).
*/
unsafe extern "C" fn pushclosure(
    mut L: *mut lua_State,
    mut p: *mut Proto,
    mut encup: *mut *mut UpVal,
    mut base: StkId,
    mut ra: StkId,
) -> () {
    let mut nup: lua_int = (*p).sizeupvalues;
    let mut uv: *mut Upvaldesc = (*p).upvalues;
    let mut i: lua_int = 0;
    let mut ncl: *mut LClosure = luaF_newLclosure(L, nup);
    (*ncl).p = p;
    let mut io: *mut TValue = ra;
    /* anchor new closure in stack */
    let mut x_: *mut LClosure = ncl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    i = 0i32;
    while i < nup {
        /* fill in its upvalues */
        /* upvalue refers to local variable? */
        if 0 != (*uv.offset(i as isize)).instack {
            (*ncl).upvals[i as usize] = luaF_findupval(
                L,
                base.offset((*uv.offset(i as isize)).idx as lua_int as isize),
            )
        } else {
            (*ncl).upvals[i as usize] = *encup.offset((*uv.offset(i as isize)).idx as isize)
        }
        (*(*ncl).upvals[i as usize]).refcount =
            (*(*ncl).upvals[i as usize]).refcount.wrapping_add(1);
        i += 1
    }
    /* new closure is white, so we do not need a barrier here */
    /* cache will not break GC invariant? */
    if 0 == (*p).marked as lua_int & 1i32 << 2i32 {
        /* save it on cache for reuse */
        (*p).cache = ncl
    };
}
/*
** Try to convert a 'for' limit to an integer, preserving the
** semantics of the loop.
** (The following explanation assumes a non-negative step; it is valid
** for negative steps mutatis mutandis.)
** If the limit can be converted to an integer, rounding down, that is
** it.
** Otherwise, check whether the limit can be converted to a number.  If
** the number is too large, it is OK to set the limit as LUA_MAXINTEGER,
** which means no limit.  If the number is too negative, the loop
** should not run, because any initial integer value is larger than the
** limit. So, it sets the limit to LUA_MININTEGER. 'stopnow' corrects
** the extreme case when the initial value is LUA_MININTEGER, in which
** case the LUA_MININTEGER limit would still run the loop once.
*/
unsafe extern "C" fn forlimit(
    mut obj: *const TValue,
    mut p: *mut lua_Integer,
    mut step: lua_Integer,
    mut stopnow: *mut lua_int,
) -> lua_int {
    /* usually, let loops run */
    *stopnow = 0i32;
    if 0 == luaV_tointeger(
        obj,
        p,
        if step < 0i32 as lua_longlong {
            2i32
        } else {
            1i32
        },
    ) {
        /* not fit in integer? */
        /* try to convert to float */
        let mut n: lua_Number = 0.;
        /* cannot convert to float? */
        if 0 == if (*obj).tt_ == 3i32 | 0i32 << 4i32 {
            n = (*obj).value_.n;
            1i32
        } else {
            luaV_tonumber_(obj, &mut n)
        } {
            /* not a number */
            return 0i32;
        } else if (0i32 as lua_double) < n {
            /* if true, float is larger than max integer */
            *p = 9223372036854775807i64;
            if step < 0i32 as lua_longlong {
                *stopnow = 1i32
            }
        } else {
            /* float is smaller than min integer */
            *p = -9223372036854775807i64 - 1i64;
            if step >= 0i32 as lua_longlong {
                *stopnow = 1i32
            }
        }
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn luaV_objlen(
    mut L: *mut lua_State,
    mut ra: StkId,
    mut rb: *const TValue,
) -> () {
    let mut tm: *const TValue = 0 as *const TValue;
    match (*rb).tt_ & 0x3fi32 {
        5 => {
            let mut h: *mut Table = &mut (*((*rb).value_.gc as *mut GCUnion)).h;
            tm = if (*h).metatable.is_null() {
                0 as *const TValue
            } else if 0 != (*(*h).metatable).flags as lua_uint & 1u32 << TM_LEN as lua_int {
                0 as *const TValue
            } else {
                luaT_gettm(
                    (*h).metatable,
                    TM_LEN,
                    (*(*L).l_G).tmname[TM_LEN as lua_int as usize],
                )
            };
            if tm.is_null() {
                /* metamethod? break switch to call it */
                /* else primitive len */
                let mut io: *mut TValue = ra;
                (*io).value_.i = luaH_getn(h) as lua_Integer;
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                return;
            }
        }
        4 => {
            let mut io_0: *mut TValue = ra;
            (*io_0).value_.i = (*((*rb).value_.gc as *mut GCUnion)).ts.shrlen as lua_Integer;
            (*io_0).tt_ = 3i32 | 1i32 << 4i32;
            return;
        }
        20 => {
            let mut io_1: *mut TValue = ra;
            (*io_1).value_.i = (*((*rb).value_.gc as *mut GCUnion)).ts.u.lnglen as lua_Integer;
            (*io_1).tt_ = 3i32 | 1i32 << 4i32;
            return;
        }
        _ => {
            /* try metamethod */
            tm = luaT_gettmbyobj(L, rb, TM_LEN);
            /* no metamethod? */
            if (*tm).tt_ == 0i32 {
                luaG_typeerror(L, rb, s!(b"get length of\x00"));
            }
        }
    }
    luaT_callTM(L, tm, rb, rb, ra, 1i32);
}
#[no_mangle]
pub unsafe extern "C" fn luaV_div(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1u32 as lua_ulonglong) <= 1u32 as lua_ulonglong {
        /* special cases: -1 or 0 */
        if n == 0i32 as lua_longlong {
            luaG_runerror!(L, s!(b"attempt to divide by zero\x00"),);
        } else {
            return (0i32 as lua_Unsigned).wrapping_sub(m as lua_Unsigned) as lua_Integer;
        }
    } else {
        /* perform C division */
        let mut q: lua_Integer = m / n;
        /* 'm/n' would be negative non-integer? */
        if m ^ n < 0i32 as lua_longlong && m % n != 0i32 as lua_longlong {
            /* correct result for different rounding */
            q -= 1i32 as lua_longlong
        }
        return q;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_mod(
    mut L: *mut lua_State,
    mut m: lua_Integer,
    mut n: lua_Integer,
) -> lua_Integer {
    if (n as lua_Unsigned).wrapping_add(1u32 as lua_ulonglong) <= 1u32 as lua_ulonglong {
        /* special cases: -1 or 0 */
        if n == 0i32 as lua_longlong {
            luaG_runerror!(L, s!(b"attempt to perform \'n%%0\'\x00"),);
        } else {
            return 0i32 as lua_Integer;
        }
    } else {
        let mut r: lua_Integer = m % n;
        /* 'm/n' would be non-integer negative? */
        if r != 0i32 as lua_longlong && m ^ n < 0i32 as lua_longlong {
            /* correct result for different rounding */
            r += n
        }
        return r;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaV_shiftl(mut x: lua_Integer, mut y: lua_Integer) -> lua_Integer {
    if y < 0i32 as lua_longlong {
        /* shift right? */
        if y <= -((::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(8i32 as lua_ulong) as lua_int) as lua_longlong
        {
            return 0i32 as lua_Integer;
        } else {
            return (x as lua_Unsigned >> -y as lua_Unsigned) as lua_Integer;
        }
    } else if y
        >= (::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(8i32 as lua_ulong) as lua_int as lua_longlong
    {
        return 0i32 as lua_Integer;
    } else {
        return ((x as lua_Unsigned) << y as lua_Unsigned) as lua_Integer;
    };
}
