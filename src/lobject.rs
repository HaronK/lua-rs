use lua::*;
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
    fn localeconv() -> *mut lconv;
    #[no_mangle]
    fn pow(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn floor(_: lua_double) -> lua_double;
    #[no_mangle]
    fn fmod(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    fn snprintf(_: *mut lua_char, _: lua_ulong, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn strtod(__nptr: *const lua_char, __endptr: *mut *mut lua_char) -> lua_double;
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    fn strcpy(_: *mut lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    fn strchr(_: *const lua_char, _: lua_int) -> *mut lua_char;
    #[no_mangle]
    fn strspn(_: *const lua_char, _: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn strpbrk(_: *const lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    fn strlen(_: *const lua_char) -> lua_ulong;
    /*
     ** $Id: lctype.h,v 1.12.1.1 2013/04/12 18:48:47 roberto Exp $
     ** 'ctype' functions for Lua
     ** See Copyright Notice in lua.h
     */
    /*
     ** WARNING: the functions defined here do not necessarily correspond
     ** to the similar functions in the standard C ctype.h. They are
     ** optimized for the specific needs of Lua
     */
    /* ASCII case: can use its own tables; faster and fixed */
    /* { */
    /*
     ** add 1 to char to allow index -1 (EOZ)
     */
    /*
     ** 'lalpha' (Lua alphabetic) and 'lalnum' (Lua alphanumeric) both include '_'
     */
    /*
     ** this 'ltolower' only works for alphabetic characters
     */
    /* two more entries for 0 and -1 (EOZ) */
    #[no_mangle]
    static luai_ctype_: [lu_byte; 257];
    #[no_mangle]
    fn luaT_trybinTM(
        L: *mut lua_State,
        p1: *const TValue,
        p2: *const TValue,
        res: StkId,
        event: TMS,
    ) -> ();
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> lua_int;
    #[no_mangle]
    fn luaV_shiftl(x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_div(L: *mut lua_State, x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_mod(L: *mut lua_State, x: lua_Integer, y: lua_Integer) -> lua_Integer;
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: lua_int) -> lua_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const lua_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaV_concat(L: *mut lua_State, total: lua_int) -> ();
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_growstack(L: *mut lua_State, n: lua_int) -> ();
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: lua_uint,
    pub fp_offset: lua_uint,
    pub overflow_arg_area: *mut lua_void,
    pub reg_save_area: *mut lua_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut lua_char,
    pub thousands_sep: *mut lua_char,
    pub grouping: *mut lua_char,
    pub int_curr_symbol: *mut lua_char,
    pub currency_symbol: *mut lua_char,
    pub mon_decimal_point: *mut lua_char,
    pub mon_thousands_sep: *mut lua_char,
    pub mon_grouping: *mut lua_char,
    pub positive_sign: *mut lua_char,
    pub negative_sign: *mut lua_char,
    pub int_frac_digits: lua_char,
    pub frac_digits: lua_char,
    pub p_cs_precedes: lua_char,
    pub p_sep_by_space: lua_char,
    pub n_cs_precedes: lua_char,
    pub n_sep_by_space: lua_char,
    pub p_sign_posn: lua_char,
    pub n_sign_posn: lua_char,
    pub int_p_cs_precedes: lua_char,
    pub int_p_sep_by_space: lua_char,
    pub int_n_cs_precedes: lua_char,
    pub int_n_sep_by_space: lua_char,
    pub int_p_sign_posn: lua_char,
    pub int_n_sign_posn: lua_char,
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
pub const LUA_TPROTO: i32 = LUA_NUMTAGS; /* function prototypes */
pub const LUA_TDEADKEY: i32 = LUA_NUMTAGS + 1; /* removed keys in tables */

/*
** number of all possible tags (including LUA_TNONE but excluding DEADKEY)
*/
pub const LUA_TOTALTAGS: i32 = LUA_TPROTO + 2;

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
pub const LUA_TLCL: i32 = (LUA_TFUNCTION | (0 << 4)); /* Lua closure */
pub const LUA_TLCF: i32 = (LUA_TFUNCTION | (1 << 4)); /* light C function */
pub const LUA_TCCL: i32 = (LUA_TFUNCTION | (2 << 4)); /* C closure */

/* Variant tags for strings */
pub const LUA_TSHRSTR: i32 = (LUA_TSTRING | (0 << 4)); /* short strings */
pub const LUA_TLNGSTR: i32 = (LUA_TSTRING | (1 << 4)); /* long strings */

/* Variant tags for numbers */
pub const LUA_TNUMFLT: i32 = (LUA_TNUMBER | (0 << 4)); /* float numbers */
pub const LUA_TNUMINT: i32 = (LUA_TNUMBER | (1 << 4)); /* integer numbers */

/* Bit mark for collectable types */
pub const BIT_ISCOLLECTABLE: i32 = (1 << 6);

/* mark a tag as collectable */

/* function prototypes */

/* removed keys in tables */

/* Lua closure */
/* light C function */
/* C closure */
/* short strings */
/* long strings */
/* float numbers */
/* integer numbers */
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
/* types of 'usual argument conversions' for lua_Number and lua_Integer */
pub type l_uacNumber = lua_double;
pub type l_uacInt = lua_longlong;
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
** 'module' operation for hashing (size is always a power of 2)
*/
/*
** (address of) a fixed nil value
*/
#[no_mangle]
pub static mut luaO_nilobject_: TValue = lua_TValue {
    value_: Value {
        gc: 0 as *const GCObject as *mut GCObject,
    },
    tt_: 0i32,
};
/* size of buffer for 'luaO_utf8esc' function */
#[no_mangle]
pub unsafe extern "C" fn luaO_int2fb(mut x: lua_uint) -> lua_int {
    /* exponent */
    let mut e: lua_int = 0i32;
    if x < 8i32 as lua_uint {
        return x as lua_int;
    } else {
        while x >= (8i32 << 4i32) as lua_uint {
            /* coarse steps */
            /* x = ceil(x / 16) */
            x = x.wrapping_add(0xfi32 as lua_uint) >> 4i32;
            e += 4i32
        }
        while x >= (8i32 << 1i32) as lua_uint {
            /* fine steps */
            /* x = ceil(x / 2) */
            x = x.wrapping_add(1i32 as lua_uint) >> 1i32;
            e += 1
        }
        return e + 1i32 << 3i32 | x as lua_int - 8i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_fb2int(mut x: lua_int) -> lua_int {
    return if x < 8i32 {
        x
    } else {
        (x & 7i32) + 8i32 << (x >> 3i32) - 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_utf8esc(mut buff: *mut lua_char, mut x: lua_ulong) -> lua_int {
    /* number of bytes put in buffer (backwards) */
    let mut n: lua_int = 1i32;
    /* ascii? */
    if x < 0x80i32 as lua_ulong {
        *buff.offset((8i32 - 1i32) as isize) = x as lua_char
    } else {
        /* need continuation bytes */
        /* maximum that fits in first byte */
        let mut mfb: lua_uint = 0x3fi32 as lua_uint;
        loop {
            /* add continuation bytes */
            let fresh0 = n;
            n = n + 1;
            *buff.offset((8i32 - fresh0) as isize) =
                (0x80i32 as lua_ulong | x & 0x3fi32 as lua_ulong) as lua_char;
            /* remove added bits */
            x >>= 6i32;
            /* now there is one less bit available in first byte */
            mfb >>= 1i32;
            if !(x > mfb as lua_ulong) {
                break;
            }
        }
        /* still needs continuation byte? */
        /* add first byte */
        *buff.offset((8i32 - n) as isize) = ((!mfb << 1i32) as lua_ulong | x) as lua_char
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_ceillog2(mut x: lua_uint) -> lua_int {
    /* log_2[i] = ceil(log2(i - 1)) */
    static mut log_2: [lu_byte; 256] = [
        0i32 as lu_byte,
        1i32 as lu_byte,
        2i32 as lu_byte,
        2i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        3i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        4i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        5i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        6i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        7i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
        8i32 as lu_byte,
    ];
    let mut l: lua_int = 0i32;
    x = x.wrapping_sub(1);
    while x >= 256i32 as lua_uint {
        l += 8i32;
        x >>= 8i32
    }
    return l + log_2[x as usize] as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaO_arith(
    mut L: *mut lua_State,
    mut op: lua_int,
    mut p1: *const TValue,
    mut p2: *const TValue,
    mut res: *mut TValue,
) -> () {
    match op {
        7 | 8 | 9 | 10 | 11 | 13 => {
            /* operate only on integers */
            let mut i1: lua_Integer = 0;
            let mut i2: lua_Integer = 0;
            if 0 != if (*p1).tt_ == 3i32 | 1i32 << 4i32 {
                i1 = (*p1).value_.i;
                1i32
            } else {
                luaV_tointeger(p1, &mut i1, 0i32)
            } && 0
                != if (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                    i2 = (*p2).value_.i;
                    1i32
                } else {
                    luaV_tointeger(p2, &mut i2, 0i32)
                } {
                let mut io: *mut TValue = res;
                (*io).value_.i = intarith(L, op, i1, i2);
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                return;
            }
        }
        5 | 4 => {
            /* go to the end */
            /* operate only on floats */
            let mut n1: lua_Number = 0.;
            let mut n2: lua_Number = 0.;
            if 0 != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                n1 = (*p1).value_.n;
                1i32
            } else {
                luaV_tonumber_(p1, &mut n1)
            } && 0
                != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    n2 = (*p2).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p2, &mut n2)
                } {
                let mut io_0: *mut TValue = res;
                (*io_0).value_.n = numarith(L, op, n1, n2);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
        _ => {
            /* go to the end */
            /* other operations */
            let mut n1_0: lua_Number = 0.;
            let mut n2_0: lua_Number = 0.;
            if (*p1).tt_ == 3i32 | 1i32 << 4i32 && (*p2).tt_ == 3i32 | 1i32 << 4i32 {
                let mut io_1: *mut TValue = res;
                (*io_1).value_.i = intarith(L, op, (*p1).value_.i, (*p2).value_.i);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32;
                return;
            } else if 0
                != if (*p1).tt_ == 3i32 | 0i32 << 4i32 {
                    n1_0 = (*p1).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p1, &mut n1_0)
                }
                && 0 != if (*p2).tt_ == 3i32 | 0i32 << 4i32 {
                    n2_0 = (*p2).value_.n;
                    1i32
                } else {
                    luaV_tonumber_(p2, &mut n2_0)
                } {
                let mut io_2: *mut TValue = res;
                (*io_2).value_.n = numarith(L, op, n1_0, n2_0);
                (*io_2).tt_ = 3i32 | 0i32 << 4i32;
                return;
            }
        }
    }
    /* go to the end */
    /* could not perform raw operation; try metamethod */
    /* should not fail when folding (compile time) */
    luaT_trybinTM(L, p1, p2, res, (op - 0i32 + TM_ADD as lua_int) as TMS);
}
unsafe extern "C" fn numarith(
    mut _L: *mut lua_State,
    mut op: lua_int,
    mut v1: lua_Number,
    mut v2: lua_Number,
) -> lua_Number {
    match op {
        0 => return v1 + v2,
        1 => return v1 - v2,
        2 => return v1 * v2,
        5 => return v1 / v2,
        4 => return pow(v1, v2),
        6 => return floor(v1 / v2),
        12 => return -v1,
        3 => {
            let mut m: lua_Number = 0.;
            m = fmod(v1, v2);
            if m * v2 < 0i32 as lua_double {
                m += v2
            }
            return m;
        }
        _ => return 0i32 as lua_Number,
    };
}
unsafe extern "C" fn intarith(
    mut L: *mut lua_State,
    mut op: lua_int,
    mut v1: lua_Integer,
    mut v2: lua_Integer,
) -> lua_Integer {
    match op {
        0 => return (v1 as lua_Unsigned).wrapping_add(v2 as lua_Unsigned) as lua_Integer,
        1 => return (v1 as lua_Unsigned).wrapping_sub(v2 as lua_Unsigned) as lua_Integer,
        2 => return (v1 as lua_Unsigned).wrapping_mul(v2 as lua_Unsigned) as lua_Integer,
        3 => return luaV_mod(L, v1, v2),
        6 => return luaV_div(L, v1, v2),
        7 => return (v1 as lua_Unsigned & v2 as lua_Unsigned) as lua_Integer,
        8 => return (v1 as lua_Unsigned | v2 as lua_Unsigned) as lua_Integer,
        9 => return (v1 as lua_Unsigned ^ v2 as lua_Unsigned) as lua_Integer,
        10 => return luaV_shiftl(v1, v2),
        11 => return luaV_shiftl(v1, -v2),
        12 => return (0i32 as lua_Unsigned).wrapping_sub(v1 as lua_Unsigned) as lua_Integer,
        13 => return (!(0i32 as lua_Unsigned) ^ v1 as lua_Unsigned) as lua_Integer,
        _ => return 0i32 as lua_Integer,
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_str2num(mut s: *const lua_char, mut o: *mut TValue) -> size_t {
    let mut i: lua_Integer = 0;
    let mut n: lua_Number = 0.;
    let mut e: *const lua_char = 0 as *const lua_char;
    e = l_str2int(s, &mut i);
    if !e.is_null() {
        /* try as an integer */
        let mut io: *mut TValue = o;
        (*io).value_.i = i;
        (*io).tt_ = 3i32 | 1i32 << 4i32
    } else {
        e = l_str2d(s, &mut n);
        if !e.is_null() {
            /* else try as a float */
            let mut io_0: *mut TValue = o;
            (*io_0).value_.n = n;
            (*io_0).tt_ = 3i32 | 0i32 << 4i32
        } else {
            return 0i32 as size_t;
        }
    }
    /* success; return string size */
    return (e.wrapping_offset_from(s) as lua_long + 1i32 as lua_long) as size_t;
}
/*
** Convert string 's' to a Lua number (put in 'result'). Return NULL
** on fail or the address of the ending '\0' on success.
** 'pmode' points to (and 'mode' contains) special things in the string:
** - 'x'/'X' means an hexadecimal numeral
** - 'n'/'N' means 'inf' or 'nan' (which should be rejected)
** - '.' just optimizes the search for the common case (nothing special)
** This function accepts both the current locale or a dot as the radix
** mark. If the convertion fails, it may mean number has a dot but
** locale accepts something else. In that case, the code copies 's'
** to a buffer (because 's' is read-only), changes the dot to the
** current locale radix mark, and tries to convert again.
*/
unsafe extern "C" fn l_str2d(
    mut s: *const lua_char,
    mut result: *mut lua_Number,
) -> *const lua_char {
    let mut endptr: *const lua_char = 0 as *const lua_char;
    let mut pmode: *const lua_char = strpbrk(s, s!(b".xXnN\x00"));
    let mut mode: lua_int = if !pmode.is_null() {
        *pmode as lua_uchar as lua_int | 'A' as i32 ^ 'a' as i32
    } else {
        0i32
    };
    /* reject 'inf' and 'nan' */
    if mode == 'n' as i32 {
        return 0 as *const lua_char;
    } else {
        /* try to convert */
        endptr = l_str2dloc(s, result, mode);
        if endptr.is_null() {
            /* failed? may be a different locale */
            let mut buff: [lua_char; 201] = [0; 201];
            let mut pdot: *const lua_char = strchr(s, '.' as i32);
            if strlen(s) > 200i32 as lua_ulong || pdot.is_null() {
                /* string too long or no dot; fail */
                return 0 as *const lua_char;
            } else {
                /* copy string to buffer */
                strcpy(buff.as_mut_ptr(), s);
                /* correct decimal point */
                buff[pdot.wrapping_offset_from(s) as lua_long as usize] =
                    *(*localeconv()).decimal_point.offset(0isize);
                /* try again */
                endptr = l_str2dloc(buff.as_mut_ptr(), result, mode);
                if !endptr.is_null() {
                    /* make relative to 's' */
                    endptr = s
                        .offset(endptr.wrapping_offset_from(buff.as_mut_ptr()) as lua_long as isize)
                }
            }
        }
        return endptr;
    };
}
/*
** {==================================================================
** Lua's implementation for 'lua_strx2number'
** ===================================================================
*/
/* }====================================================== */
/* maximum length of a numeral */
unsafe extern "C" fn l_str2dloc(
    mut s: *const lua_char,
    mut result: *mut lua_Number,
    mut mode: lua_int,
) -> *const lua_char {
    let mut endptr: *mut lua_char = 0 as *mut lua_char;
    /* try to convert */
    *result = if mode == 'x' as i32 {
        strtod(s, &mut endptr)
    } else {
        strtod(s, &mut endptr)
    };
    if endptr == s as *mut lua_char {
        /* nothing recognized? */
        return 0 as *const lua_char;
    } else {
        while 0
            != luai_ctype_[(*endptr as lua_uchar as lua_int + 1i32) as usize] as lua_int
                & 1i32 << 3i32
        {
            /* skip trailing spaces */
            endptr = endptr.offset(1isize)
        }
        /* OK if no trailing characters */
        return if *endptr as lua_int == '\u{0}' as i32 {
            endptr
        } else {
            0 as *mut lua_char
        };
    };
}
unsafe extern "C" fn l_str2int(
    mut s: *const lua_char,
    mut result: *mut lua_Integer,
) -> *const lua_char {
    let mut a: lua_Unsigned = 0i32 as lua_Unsigned;
    let mut empty: lua_int = 1i32;
    let mut neg: lua_int = 0;
    while 0 != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 3i32 {
        /* skip initial spaces */
        s = s.offset(1isize)
    }
    neg = isneg(&mut s);
    if *s.offset(0isize) as lua_int == '0' as i32
        && (*s.offset(1isize) as lua_int == 'x' as i32
            || *s.offset(1isize) as lua_int == 'X' as i32)
    {
        /* hex? */
        /* skip '0x' */
        s = s.offset(2isize);
        while 0
            != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 4i32
        {
            a = a
                .wrapping_mul(16i32 as lua_ulonglong)
                .wrapping_add(luaO_hexavalue(*s as lua_int) as lua_ulonglong);
            empty = 0i32;
            s = s.offset(1isize)
        }
    } else {
        while 0
            != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 1i32
        {
            let mut d: lua_int = *s as lua_int - '0' as i32;
            /* overflow? */
            if a >= (9223372036854775807i64 / 10i32 as lua_longlong) as lua_Unsigned
                && (a > (9223372036854775807i64 / 10i32 as lua_longlong) as lua_Unsigned
                    || d > (9223372036854775807i64 % 10i32 as lua_longlong) as lua_int + neg)
            {
                /* do not accept it (as integer) */
                return 0 as *const lua_char;
            } else {
                a = a
                    .wrapping_mul(10i32 as lua_ulonglong)
                    .wrapping_add(d as lua_ulonglong);
                empty = 0i32;
                s = s.offset(1isize)
            }
        }
    }
    while 0 != luai_ctype_[(*s as lua_uchar as lua_int + 1i32) as usize] as lua_int & 1i32 << 3i32 {
        /* skip trailing spaces */
        s = s.offset(1isize)
    }
    if 0 != empty || *s as lua_int != '\u{0}' as i32 {
        /* something wrong in the numeral */
        return 0 as *const lua_char;
    } else {
        *result = (if 0 != neg {
            (0u32 as lua_ulonglong).wrapping_sub(a)
        } else {
            a
        }) as lua_Integer;
        return s;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_hexavalue(mut c: lua_int) -> lua_int {
    if 0 != luai_ctype_[(c + 1i32) as usize] as lua_int & 1i32 << 1i32 {
        return c - '0' as i32;
    } else {
        return (c | 'A' as i32 ^ 'a' as i32) - 'a' as i32 + 10i32;
    };
}
unsafe extern "C" fn isneg(mut s: *mut *const lua_char) -> lua_int {
    if **s as lua_int == '-' as i32 {
        *s = (*s).offset(1isize);
        return 1i32;
    } else {
        if **s as lua_int == '+' as i32 {
            *s = (*s).offset(1isize)
        }
        return 0i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaO_tostring(mut L: *mut lua_State, mut obj: StkId) -> () {
    let mut buff: [lua_char; 50] = [0; 50];
    let mut len: size_t = 0;
    if (*obj).tt_ == 3i32 | 1i32 << 4i32 {
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 50]>() as lua_ulong,
            s!(b"%lld\x00"),
            (*obj).value_.i,
        ) as size_t
    } else {
        len = snprintf(
            buff.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 50]>() as lua_ulong,
            s!(b"%.14g\x00"),
            (*obj).value_.n,
        ) as size_t;
        if buff[strspn(buff.as_mut_ptr(), s!(b"-0123456789\x00")) as usize] as lua_int
            == '\u{0}' as i32
        {
            /* looks like an int? */
            let fresh1 = len;
            len = len.wrapping_add(1);
            buff[fresh1 as usize] = *(*localeconv()).decimal_point.offset(0isize);
            /* adds '.0' to result */
            let fresh2 = len;
            len = len.wrapping_add(1);
            buff[fresh2 as usize] = '0' as i32 as lua_char
        }
    }
    let mut io: *mut TValue = obj;
    let mut x_: *mut TString = luaS_newlstr(L, buff.as_mut_ptr(), len);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
}
unsafe extern "C" fn pushstr(mut L: *mut lua_State, mut str: *const lua_char, mut l: size_t) -> () {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = luaS_newlstr(L, str, l);
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
    luaD_inctop(L);
}
#[no_mangle]
pub unsafe extern "C" fn luaO_chunkid(
    mut out: *mut lua_char,
    mut source: *const lua_char,
    mut bufflen: size_t,
) -> () {
    let mut l: size_t = strlen(source);
    if *source as lua_int == '=' as i32 {
        /* 'literal' source */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        } else {
            /* truncate it */
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                bufflen
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(bufflen.wrapping_sub(1i32 as lua_ulong) as isize);
            *out = '\u{0}' as i32 as lua_char
        }
    } else if *source as lua_int == '@' as i32 {
        /* file name */
        /* small enough? */
        if l <= bufflen {
            memcpy(
                out as *mut lua_void,
                source.offset(1isize) as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        } else {
            /* add '...' before rest of name */
            memcpy(
                out as *mut lua_void,
                s!(b"...\x00") as *const lua_void,
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong) as isize,
            );
            bufflen = (bufflen as lua_ulong).wrapping_sub(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong),
            ) as size_t as size_t;
            memcpy(
                out as *mut lua_void,
                source
                    .offset(1isize)
                    .offset(l as isize)
                    .offset(-(bufflen as isize)) as *const lua_void,
                bufflen.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
        }
    } else {
        /* string; format as [string "source"] */
        /* find first new line (if any) */
        let mut nl: *const lua_char = strchr(source, '\n' as i32);
        /* add prefix */
        memcpy(
            out as *mut lua_void,
            s!(b"[string \"\x00") as *const lua_void,
            (::std::mem::size_of::<[lua_char; 10]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
        out = out.offset(
            (::std::mem::size_of::<[lua_char; 10]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong) as isize,
        );
        /* save space for prefix+suffix+'\0' */
        bufflen = (bufflen as lua_ulong).wrapping_sub(
            (::std::mem::size_of::<[lua_char; 15]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_add(1i32 as lua_ulong),
        ) as size_t as size_t;
        if l < bufflen && nl.is_null() {
            /* small one-line source? */
            /* keep it */
            memcpy(
                out as *mut lua_void,
                source as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(l as isize)
        } else {
            if !nl.is_null() {
                /* stop at first newline */
                l = nl.wrapping_offset_from(source) as lua_long as size_t
            }
            if l > bufflen {
                l = bufflen
            }
            memcpy(
                out as *mut lua_void,
                source as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(l as isize);
            memcpy(
                out as *mut lua_void,
                s!(b"...\x00") as *const lua_void,
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong)
                    .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            out = out.offset(
                (::std::mem::size_of::<[lua_char; 4]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong) as isize,
            )
        }
        memcpy(
            out as *mut lua_void,
            s!(b"\"]\x00") as *const lua_void,
            (::std::mem::size_of::<[lua_char; 3]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong)
                .wrapping_add(1i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
        );
    };
}
