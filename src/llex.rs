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
    /*
     ** Lua Upvalues
     */
    pub type UpVal;
    /* control of blocks */
    /* defined in lparser.c */
    pub type BlockCnt;
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
    fn luaO_utf8esc(buff: *mut libc::c_char, x: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaO_hexavalue(c: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
    /* not to be called directly */
    #[no_mangle]
    fn luaM_realloc_(
        L: *mut lua_State,
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaZ_fill(z: *mut ZIO) -> libc::c_int;
    #[no_mangle]
    fn luaG_addinfo(
        L: *mut lua_State,
        msg: *const libc::c_char,
        src: *mut TString,
        line: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
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
    fn luaC_step(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaH_set(L: *mut lua_State, t: *mut Table, key: *const TValue) -> *mut TValue;
}
pub type size_t = libc::c_ulong;
pub type ptrdiff_t = libc::c_long;
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
    pub nk: unnamed_3,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_3 {
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
    pub startpc: libc::c_int,
    pub endpc: libc::c_int,
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
** $Id: llex.h,v 1.79.1.1 2017/04/19 17:20:42 roberto Exp $
** Lexical Analyzer
** See Copyright Notice in lua.h
*/
/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER RESERVED"
*/
pub type RESERVED = libc::c_uint;
pub const TK_STRING: RESERVED = 293;
pub const TK_NAME: RESERVED = 292;
pub const TK_INT: RESERVED = 291;
pub const TK_FLT: RESERVED = 290;
pub const TK_EOS: RESERVED = 289;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_SHR: RESERVED = 287;
pub const TK_SHL: RESERVED = 286;
pub const TK_NE: RESERVED = 285;
pub const TK_LE: RESERVED = 284;
pub const TK_GE: RESERVED = 283;
pub const TK_EQ: RESERVED = 282;
pub const TK_DOTS: RESERVED = 281;
pub const TK_CONCAT: RESERVED = 280;
/* other terminal symbols */
pub const TK_IDIV: RESERVED = 279;
pub const TK_WHILE: RESERVED = 278;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_TRUE: RESERVED = 276;
pub const TK_THEN: RESERVED = 275;
pub const TK_RETURN: RESERVED = 274;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_OR: RESERVED = 272;
pub const TK_NOT: RESERVED = 271;
pub const TK_NIL: RESERVED = 270;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_IN: RESERVED = 268;
pub const TK_IF: RESERVED = 267;
pub const TK_GOTO: RESERVED = 266;
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_FOR: RESERVED = 264;
pub const TK_FALSE: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
/* terminal symbols denoted by reserved words */
pub const TK_AND: RESERVED = 257;
/* number of reserved words */
#[derive(Copy, Clone)]
#[repr(C)]
pub union SemInfo {
    pub r: lua_Number,
    pub i: lua_Integer,
    pub ts: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Token {
    pub token: libc::c_int,
    pub seminfo: SemInfo,
}
/* state of the lexer plus state of the parser when shared by all
functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: libc::c_int,
    pub linenumber: libc::c_int,
    pub lastline: libc::c_int,
    pub t: Token,
    pub lookahead: Token,
    pub fs: *mut FuncState,
    pub L: *mut lua_State,
    pub z: *mut ZIO,
    pub buff: *mut Mbuffer,
    pub h: *mut Table,
    pub dyd: *mut Dyndata,
    pub source: *mut TString,
    pub envn: *mut TString,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_4,
    pub gt: Labellist,
    pub label: Labellist,
}
/* list of labels or gotos */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: libc::c_int,
    pub size: libc::c_int,
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
/* dynamic structures used by the parser */
/* list of active local variables */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_4 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: libc::c_int,
    pub lasttarget: libc::c_int,
    pub jpc: libc::c_int,
    pub nk: libc::c_int,
    pub np: libc::c_int,
    pub firstlocal: libc::c_int,
    pub nlocvars: libc::c_short,
    pub nactvar: lu_byte,
    pub nups: lu_byte,
    pub freereg: lu_byte,
}
#[no_mangle]
pub unsafe extern "C" fn luaX_init(mut L: *mut lua_State) -> () {
    let mut i: libc::c_int = 0;
    /* create env name */
    let mut e: *mut TString = luaS_newlstr(
        L,
        b"_ENV\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    /* never collect this name */
    luaC_fix(L, &mut (*(e as *mut GCUnion)).gc);
    i = 0i32;
    while i < TK_WHILE as libc::c_int - 257i32 + 1i32 {
        let mut ts: *mut TString = luaS_new(L, luaX_tokens[i as usize]);
        /* reserved words are never collected */
        luaC_fix(L, &mut (*(ts as *mut GCUnion)).gc);
        /* reserved word */
        (*ts).extra = (i + 1i32) as lu_byte;
        i += 1
    }
}
/*
** $Id: llex.c,v 2.96.1.1 2017/04/19 17:20:42 roberto Exp $
** Lexical Analyzer
** See Copyright Notice in lua.h
*/
/* ORDER RESERVED */
static mut luaX_tokens: [*const libc::c_char; 37] = [
    b"and\x00" as *const u8 as *const libc::c_char,
    b"break\x00" as *const u8 as *const libc::c_char,
    b"do\x00" as *const u8 as *const libc::c_char,
    b"else\x00" as *const u8 as *const libc::c_char,
    b"elseif\x00" as *const u8 as *const libc::c_char,
    b"end\x00" as *const u8 as *const libc::c_char,
    b"false\x00" as *const u8 as *const libc::c_char,
    b"for\x00" as *const u8 as *const libc::c_char,
    b"function\x00" as *const u8 as *const libc::c_char,
    b"goto\x00" as *const u8 as *const libc::c_char,
    b"if\x00" as *const u8 as *const libc::c_char,
    b"in\x00" as *const u8 as *const libc::c_char,
    b"local\x00" as *const u8 as *const libc::c_char,
    b"nil\x00" as *const u8 as *const libc::c_char,
    b"not\x00" as *const u8 as *const libc::c_char,
    b"or\x00" as *const u8 as *const libc::c_char,
    b"repeat\x00" as *const u8 as *const libc::c_char,
    b"return\x00" as *const u8 as *const libc::c_char,
    b"then\x00" as *const u8 as *const libc::c_char,
    b"true\x00" as *const u8 as *const libc::c_char,
    b"until\x00" as *const u8 as *const libc::c_char,
    b"while\x00" as *const u8 as *const libc::c_char,
    b"//\x00" as *const u8 as *const libc::c_char,
    b"..\x00" as *const u8 as *const libc::c_char,
    b"...\x00" as *const u8 as *const libc::c_char,
    b"==\x00" as *const u8 as *const libc::c_char,
    b">=\x00" as *const u8 as *const libc::c_char,
    b"<=\x00" as *const u8 as *const libc::c_char,
    b"~=\x00" as *const u8 as *const libc::c_char,
    b"<<\x00" as *const u8 as *const libc::c_char,
    b">>\x00" as *const u8 as *const libc::c_char,
    b"::\x00" as *const u8 as *const libc::c_char,
    b"<eof>\x00" as *const u8 as *const libc::c_char,
    b"<number>\x00" as *const u8 as *const libc::c_char,
    b"<integer>\x00" as *const u8 as *const libc::c_char,
    b"<name>\x00" as *const u8 as *const libc::c_char,
    b"<string>\x00" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn luaX_setinput(
    mut L: *mut lua_State,
    mut ls: *mut LexState,
    mut z: *mut ZIO,
    mut source: *mut TString,
    mut firstchar: libc::c_int,
) -> () {
    (*ls).t.token = 0i32;
    (*ls).L = L;
    (*ls).current = firstchar;
    /* no look-ahead token */
    (*ls).lookahead.token = TK_EOS as libc::c_int;
    (*ls).z = z;
    (*ls).fs = 0 as *mut FuncState;
    (*ls).linenumber = 1i32;
    (*ls).lastline = 1i32;
    (*ls).source = source;
    /* get env name */
    (*ls).envn = luaS_newlstr(
        L,
        b"_ENV\x00" as *const u8 as *const libc::c_char,
        (::std::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong),
    );
    /* initialize buffer */
    (*(*ls).buff).buffer = luaM_realloc_(
        (*ls).L,
        (*(*ls).buff).buffer as *mut libc::c_void,
        (*(*ls).buff)
            .buffsize
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
        (32i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    ) as *mut libc::c_char;
    (*(*ls).buff).buffsize = 32i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_newstring(
    mut ls: *mut LexState,
    mut str: *const libc::c_char,
    mut l: size_t,
) -> *mut TString {
    let mut L: *mut lua_State = (*ls).L;
    /* entry for 'str' */
    let mut o: *mut TValue = 0 as *mut TValue;
    /* create new string */
    let mut ts: *mut TString = luaS_newlstr(L, str, l);
    /* temporarily anchor it in stack */
    let fresh0 = (*L).top;
    (*L).top = (*L).top.offset(1);
    let mut io: *mut TValue = fresh0;
    let mut x_: *mut TString = ts;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    o = luaH_set(L, (*ls).h, (*L).top.offset(-1isize) as *const TValue);
    if (*o).tt_ == 0i32 {
        /* not in use yet? */
        /* boolean value does not need GC barrier;
        table has no metatable, so it does not need to invalidate cache */
        /* t[string] = true */
        let mut io_0: *mut TValue = o;
        (*io_0).value_.b = 1i32;
        (*io_0).tt_ = 1i32;
        if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
            luaC_step(L);
        }
    } else {
        ts = &mut (*((*(&mut (*((o as *mut libc::c_char).offset(-0isize) as *mut Node))
            .i_key
            .tvk as *mut TValue as *const TValue))
            .value_
            .gc as *mut GCUnion))
            .ts
    }
    /* remove string from stack */
    (*L).top = (*L).top.offset(-1isize);
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_next(mut ls: *mut LexState) -> () {
    (*ls).lastline = (*ls).linenumber;
    if (*ls).lookahead.token != TK_EOS as libc::c_int {
        /* is there a look-ahead token? */
        /* use this one */
        (*ls).t = (*ls).lookahead;
        /* and discharge it */
        (*ls).lookahead.token = TK_EOS as libc::c_int
    } else {
        (*ls).t.token = llex(ls, &mut (*ls).t.seminfo)
    };
}
unsafe extern "C" fn llex(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> libc::c_int {
    (*(*ls).buff).n = 0i32 as size_t;
    loop {
        match (*ls).current {
            10 | 13 => {
                /* line breaks */
                inclinenumber(ls);
            }
            32 | 12 | 9 | 11 => {
                /* spaces */
                let fresh1 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh1 > 0i32 as libc::c_ulong {
                    let fresh2 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh2 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                }
            }
            45 => {
                /* '-' or '--' (comment) */
                let fresh3 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh3 > 0i32 as libc::c_ulong {
                    let fresh4 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh4 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if (*ls).current != '-' as i32 {
                    return '-' as i32;
                } else {
                    /* else is a comment */
                    let fresh5 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh5 > 0i32 as libc::c_ulong {
                        let fresh6 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh6 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    if (*ls).current == '[' as i32 {
                        /* long comment? */
                        let mut sep: libc::c_int = skip_sep(ls);
                        /* 'skip_sep' may dirty the buffer */
                        (*(*ls).buff).n = 0i32 as size_t;
                        if sep >= 0i32 {
                            /* skip long comment */
                            read_long_string(ls, 0 as *mut SemInfo, sep);
                            /* previous call may dirty the buff. */
                            (*(*ls).buff).n = 0i32 as size_t;
                            continue;
                        }
                    }
                    /* else short comment */
                    while !((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32)
                        && (*ls).current != -1i32
                    {
                        /* skip until end of line (or end of file) */
                        let fresh7 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh7 > 0i32 as libc::c_ulong {
                            let fresh8 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh8 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        }
                    }
                }
            }
            91 => {
                /* long string or simply '[' */
                let mut sep_0: libc::c_int = skip_sep(ls);
                if sep_0 >= 0i32 {
                    read_long_string(ls, seminfo, sep_0);
                    return TK_STRING as libc::c_int;
                } else if sep_0 != -1i32 {
                    lexerror(
                        ls,
                        b"invalid long string delimiter\x00" as *const u8 as *const libc::c_char,
                        TK_STRING as libc::c_int,
                    );
                } else {
                    return '[' as i32;
                }
            }
            61 => {
                let fresh9 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh9 > 0i32 as libc::c_ulong {
                    let fresh10 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh10 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_EQ as libc::c_int;
                } else {
                    return '=' as i32;
                }
            }
            60 => {
                let fresh11 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh11 > 0i32 as libc::c_ulong {
                    let fresh12 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh12 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_LE as libc::c_int;
                } else if 0 != check_next1(ls, '<' as i32) {
                    return TK_SHL as libc::c_int;
                } else {
                    return '<' as i32;
                }
            }
            62 => {
                let fresh13 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh13 > 0i32 as libc::c_ulong {
                    let fresh14 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh14 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_GE as libc::c_int;
                } else if 0 != check_next1(ls, '>' as i32) {
                    return TK_SHR as libc::c_int;
                } else {
                    return '>' as i32;
                }
            }
            47 => {
                let fresh15 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh15 > 0i32 as libc::c_ulong {
                    let fresh16 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh16 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '/' as i32) {
                    return TK_IDIV as libc::c_int;
                } else {
                    return '/' as i32;
                }
            }
            126 => {
                let fresh17 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh17 > 0i32 as libc::c_ulong {
                    let fresh18 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh18 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '=' as i32) {
                    return TK_NE as libc::c_int;
                } else {
                    return '~' as i32;
                }
            }
            58 => {
                let fresh19 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh19 > 0i32 as libc::c_ulong {
                    let fresh20 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh20 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, ':' as i32) {
                    return TK_DBCOLON as libc::c_int;
                } else {
                    return ':' as i32;
                }
            }
            34 | 39 => {
                /* short literal strings */
                read_string(ls, (*ls).current, seminfo);
                return TK_STRING as libc::c_int;
            }
            46 => {
                /* '.', '..', '...', or number */
                save(ls, (*ls).current);
                let fresh21 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh21 > 0i32 as libc::c_ulong {
                    let fresh22 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh22 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                if 0 != check_next1(ls, '.' as i32) {
                    if 0 != check_next1(ls, '.' as i32) {
                        /* '...' */
                        return TK_DOTS as libc::c_int;
                    } else {
                        return TK_CONCAT as libc::c_int;
                    }
                } else if 0
                    == luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 1i32
                {
                    return '.' as i32;
                } else {
                    return read_numeral(ls, seminfo);
                }
            }
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return read_numeral(ls, seminfo),
            -1 => return TK_EOS as libc::c_int,
            _ => {
                if 0 != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 0i32 {
                    /* identifier or reserved word? */
                    let mut ts: *mut TString = 0 as *mut TString;
                    loop {
                        save(ls, (*ls).current);
                        let fresh23 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh23 > 0i32 as libc::c_ulong {
                            let fresh24 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh24 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        if !(0
                            != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int
                                & (1i32 << 0i32 | 1i32 << 1i32))
                        {
                            break;
                        }
                    }
                    ts = luaX_newstring(ls, (*(*ls).buff).buffer, (*(*ls).buff).n);
                    (*seminfo).ts = ts;
                    /* reserved word? */
                    if (*ts).tt as libc::c_int == 4i32 | 0i32 << 4i32
                        && (*ts).extra as libc::c_int > 0i32
                    {
                        return (*ts).extra as libc::c_int - 1i32 + 257i32;
                    } else {
                        return TK_NAME as libc::c_int;
                    }
                } else {
                    /* single-char tokens (+ - / ...) */
                    let mut c: libc::c_int = (*ls).current;
                    let fresh25 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh25 > 0i32 as libc::c_ulong {
                        let fresh26 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh26 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    };
                    return c;
                }
            }
        }
    }
}
unsafe extern "C" fn save(mut ls: *mut LexState, mut c: libc::c_int) -> () {
    let mut b: *mut Mbuffer = (*ls).buff;
    if (*b).n.wrapping_add(1i32 as libc::c_ulong) > (*b).buffsize {
        let mut newsize: size_t = 0;
        if (*b).buffsize >= if (::std::mem::size_of::<size_t>() as libc::c_ulong)
            < ::std::mem::size_of::<lua_Integer>() as libc::c_ulong
        {
            !(0i32 as size_t)
        } else {
            9223372036854775807i64 as size_t
        }
        .wrapping_div(2i32 as libc::c_ulong)
        {
            lexerror(
                ls,
                b"lexical element too long\x00" as *const u8 as *const libc::c_char,
                0i32,
            );
        } else {
            newsize = (*b).buffsize.wrapping_mul(2i32 as libc::c_ulong);
            (*b).buffer = luaM_realloc_(
                (*ls).L,
                (*b).buffer as *mut libc::c_void,
                (*b).buffsize
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
                newsize.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            ) as *mut libc::c_char;
            (*b).buffsize = newsize
        }
    }
    let fresh27 = (*b).n;
    (*b).n = (*b).n.wrapping_add(1);
    *(*b).buffer.offset(fresh27 as isize) = c as libc::c_char;
}
unsafe extern "C" fn lexerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
    mut token: libc::c_int,
) -> ! {
    msg = luaG_addinfo((*ls).L, msg, (*ls).source, (*ls).linenumber);
    if 0 != token {
        luaO_pushfstring(
            (*ls).L,
            b"%s near %s\x00" as *const u8 as *const libc::c_char,
            msg,
            txtToken(ls, token),
        );
    }
    luaD_throw((*ls).L, 3i32);
}
unsafe extern "C" fn txtToken(
    mut ls: *mut LexState,
    mut token: libc::c_int,
) -> *const libc::c_char {
    match token {
        292 | 293 | 290 | 291 => {
            save(ls, '\u{0}' as i32);
            return luaO_pushfstring(
                (*ls).L,
                b"\'%s\'\x00" as *const u8 as *const libc::c_char,
                (*(*ls).buff).buffer,
            );
        }
        _ => return luaX_token2str(ls, token),
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_token2str(
    mut ls: *mut LexState,
    mut token: libc::c_int,
) -> *const libc::c_char {
    if token < 257i32 {
        /* single-byte symbols? */
        return luaO_pushfstring(
            (*ls).L,
            b"\'%c\'\x00" as *const u8 as *const libc::c_char,
            token,
        );
    } else {
        let mut s: *const libc::c_char = luaX_tokens[(token - 257i32) as usize];
        /* fixed format (symbols and reserved words)? */
        if token < TK_EOS as libc::c_int {
            return luaO_pushfstring(
                (*ls).L,
                b"\'%s\'\x00" as *const u8 as *const libc::c_char,
                s,
            );
        } else {
            return s;
        }
    };
}
/* LUA_NUMBER */
/*
** this function is quite liberal in what it accepts, as 'luaO_str2num'
** will reject ill-formed numerals.
*/
unsafe extern "C" fn read_numeral(mut ls: *mut LexState, mut seminfo: *mut SemInfo) -> libc::c_int {
    let mut obj: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *mut GCObject,
        },
        tt_: 0,
    };
    let mut expo: *const libc::c_char = b"Ee\x00" as *const u8 as *const libc::c_char;
    let mut first: libc::c_int = (*ls).current;
    save(ls, (*ls).current);
    let fresh28 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh28 > 0i32 as libc::c_ulong {
        let fresh29 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh29 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* hexadecimal? */
    if first == '0' as i32 && 0 != check_next2(ls, b"xX\x00" as *const u8 as *const libc::c_char) {
        expo = b"Pp\x00" as *const u8 as *const libc::c_char
    }
    loop {
        /* exponent part? */
        if 0 != check_next2(ls, expo) {
            /* optional exponent sign */
            check_next2(ls, b"-+\x00" as *const u8 as *const libc::c_char);
        }
        if 0 != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 4i32 {
            save(ls, (*ls).current);
            let fresh30 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh30 > 0i32 as libc::c_ulong {
                let fresh31 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh31 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            }
        } else {
            if !((*ls).current == '.' as i32) {
                break;
            }
            save(ls, (*ls).current);
            let fresh32 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh32 > 0i32 as libc::c_ulong {
                let fresh33 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh33 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            }
        }
    }
    save(ls, '\u{0}' as i32);
    /* format error? */
    if luaO_str2num((*(*ls).buff).buffer, &mut obj) == 0i32 as libc::c_ulong {
        lexerror(
            ls,
            b"malformed number\x00" as *const u8 as *const libc::c_char,
            TK_FLT as libc::c_int,
        );
    } else if obj.tt_ == 3i32 | 1i32 << 4i32 {
        (*seminfo).i = obj.value_.i;
        return TK_INT as libc::c_int;
    } else {
        (*seminfo).r = obj.value_.n;
        return TK_FLT as libc::c_int;
    };
}
/*
** Check whether current char is in set 'set' (with two chars) and
** saves it
*/
unsafe extern "C" fn check_next2(
    mut ls: *mut LexState,
    mut set: *const libc::c_char,
) -> libc::c_int {
    if (*ls).current == *set.offset(0isize) as libc::c_int
        || (*ls).current == *set.offset(1isize) as libc::c_int
    {
        save(ls, (*ls).current);
        let fresh34 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh34 > 0i32 as libc::c_ulong {
            let fresh35 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh35 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
/*
** =======================================================
** LEXICAL ANALYZER
** =======================================================
*/
unsafe extern "C" fn check_next1(mut ls: *mut LexState, mut c: libc::c_int) -> libc::c_int {
    if (*ls).current == c {
        let fresh36 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh36 > 0i32 as libc::c_ulong {
            let fresh37 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh37 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn read_string(
    mut ls: *mut LexState,
    mut del: libc::c_int,
    mut seminfo: *mut SemInfo,
) -> () {
    let mut current_block: u64;
    /* keep delimiter (for error messages) */
    save(ls, (*ls).current);
    let fresh38 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh38 > 0i32 as libc::c_ulong {
        let fresh39 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh39 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    's_4: while (*ls).current != del {
        match (*ls).current {
            -1 => {
                lexerror(
                    ls,
                    b"unfinished string\x00" as *const u8 as *const libc::c_char,
                    TK_EOS as libc::c_int,
                );
            }
            10 | 13 => {
                lexerror(
                    ls,
                    b"unfinished string\x00" as *const u8 as *const libc::c_char,
                    TK_STRING as libc::c_int,
                );
            }
            92 => {
                /* escape sequences */
                /* final character to be saved */
                let mut c: libc::c_int = 0;
                /* keep '\\' for error messages */
                save(ls, (*ls).current);
                let fresh40 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh40 > 0i32 as libc::c_ulong {
                    let fresh41 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh41 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                match (*ls).current {
                    97 => {
                        c = '\u{7}' as i32;
                        current_block = 12992004776645797882;
                    }
                    98 => {
                        c = '\u{8}' as i32;
                        current_block = 12992004776645797882;
                    }
                    102 => {
                        c = '\u{c}' as i32;
                        current_block = 12992004776645797882;
                    }
                    110 => {
                        c = '\n' as i32;
                        current_block = 12992004776645797882;
                    }
                    114 => {
                        c = '\r' as i32;
                        current_block = 12992004776645797882;
                    }
                    116 => {
                        c = '\t' as i32;
                        current_block = 12992004776645797882;
                    }
                    118 => {
                        c = '\u{b}' as i32;
                        current_block = 12992004776645797882;
                    }
                    120 => {
                        c = readhexaesc(ls);
                        current_block = 12992004776645797882;
                    }
                    117 => {
                        utf8esc(ls);
                        continue;
                    }
                    10 | 13 => {
                        inclinenumber(ls);
                        c = '\n' as i32;
                        current_block = 8303449541411860306;
                    }
                    92 | 34 | 39 => {
                        c = (*ls).current;
                        current_block = 12992004776645797882;
                    }
                    -1 => {
                        /* will raise an error next loop */
                        continue;
                    }
                    122 => {
                        /* zap following span of spaces */
                        /* remove '\\' */
                        (*(*ls).buff).n = ((*(*ls).buff).n as libc::c_ulong)
                            .wrapping_sub(1i32 as libc::c_ulong)
                            as size_t as size_t;
                        /* skip the 'z' */
                        let fresh42 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh42 > 0i32 as libc::c_ulong {
                            let fresh43 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh43 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        };
                        loop {
                            if !(0
                                != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int
                                    & 1i32 << 3i32)
                            {
                                continue 's_4;
                            }
                            if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
                                inclinenumber(ls);
                            } else {
                                let fresh44 = (*(*ls).z).n;
                                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                                (*ls).current = if fresh44 > 0i32 as libc::c_ulong {
                                    let fresh45 = (*(*ls).z).p;
                                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                                    *fresh45 as libc::c_uchar as libc::c_int
                                } else {
                                    luaZ_fill((*ls).z)
                                }
                            }
                        }
                    }
                    _ => {
                        esccheck(
                            ls,
                            luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int
                                & 1i32 << 1i32,
                            b"invalid escape sequence\x00" as *const u8 as *const libc::c_char,
                        );
                        /* digital escape '\ddd' */
                        c = readdecesc(ls);
                        current_block = 8303449541411860306;
                    }
                }
                match current_block {
                    12992004776645797882 => {
                        let fresh46 = (*(*ls).z).n;
                        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                        (*ls).current = if fresh46 > 0i32 as libc::c_ulong {
                            let fresh47 = (*(*ls).z).p;
                            (*(*ls).z).p = (*(*ls).z).p.offset(1);
                            *fresh47 as libc::c_uchar as libc::c_int
                        } else {
                            luaZ_fill((*ls).z)
                        }
                    }
                    _ => {}
                }
                /* go through */
                /* remove '\\' */
                (*(*ls).buff).n = ((*(*ls).buff).n as libc::c_ulong)
                    .wrapping_sub(1i32 as libc::c_ulong) as size_t
                    as size_t;
                save(ls, c);
            }
            _ => {
                /* go through */
                save(ls, (*ls).current);
                let fresh48 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh48 > 0i32 as libc::c_ulong {
                    let fresh49 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh49 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                }
            }
        }
    }
    /* skip delimiter */
    save(ls, (*ls).current);
    let fresh50 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh50 > 0i32 as libc::c_ulong {
        let fresh51 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh51 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    (*seminfo).ts = luaX_newstring(
        ls,
        (*(*ls).buff).buffer.offset(1isize),
        (*(*ls).buff).n.wrapping_sub(2i32 as libc::c_ulong),
    );
}
unsafe extern "C" fn readdecesc(mut ls: *mut LexState) -> libc::c_int {
    let mut i: libc::c_int = 0;
    /* result accumulator */
    let mut r: libc::c_int = 0i32;
    i = 0i32;
    while i < 3i32
        && 0 != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 1i32
    {
        /* read up to 3 digits */
        r = 10i32 * r + (*ls).current - '0' as i32;
        save(ls, (*ls).current);
        let fresh52 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh52 > 0i32 as libc::c_ulong {
            let fresh53 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh53 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        i += 1
    }
    esccheck(
        ls,
        (r <= 127i32 * 2i32 + 1i32) as libc::c_int,
        b"decimal escape too large\x00" as *const u8 as *const libc::c_char,
    );
    /* remove read digits from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as size_t as size_t;
    return r;
}
unsafe extern "C" fn esccheck(
    mut ls: *mut LexState,
    mut c: libc::c_int,
    mut msg: *const libc::c_char,
) -> () {
    if 0 == c {
        if (*ls).current != -1i32 {
            /* add current to buffer for error message */
            save(ls, (*ls).current);
            let fresh54 = (*(*ls).z).n;
            (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
            (*ls).current = if fresh54 > 0i32 as libc::c_ulong {
                let fresh55 = (*(*ls).z).p;
                (*(*ls).z).p = (*(*ls).z).p.offset(1);
                *fresh55 as libc::c_uchar as libc::c_int
            } else {
                luaZ_fill((*ls).z)
            }
        }
        lexerror(ls, msg, TK_STRING as libc::c_int);
    } else {
        return;
    };
}
/*
** increment line number and skips newline sequence (any of
** \n, \r, \n\r, or \r\n)
*/
unsafe extern "C" fn inclinenumber(mut ls: *mut LexState) -> () {
    let mut old: libc::c_int = (*ls).current;
    /* skip '\n' or '\r' */
    let fresh56 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh56 > 0i32 as libc::c_ulong {
        let fresh57 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh57 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    if ((*ls).current == '\n' as i32 || (*ls).current == '\r' as i32) && (*ls).current != old {
        /* skip '\n\r' or '\r\n' */
        let fresh58 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh58 > 0i32 as libc::c_ulong {
            let fresh59 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh59 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        }
    }
    (*ls).linenumber += 1;
    if (*ls).linenumber >= 2147483647i32 {
        lexerror(
            ls,
            b"chunk has too many lines\x00" as *const u8 as *const libc::c_char,
            0i32,
        );
    } else {
        return;
    };
}
unsafe extern "C" fn utf8esc(mut ls: *mut LexState) -> () {
    let mut buff: [libc::c_char; 8] = [0; 8];
    let mut n: libc::c_int = luaO_utf8esc(buff.as_mut_ptr(), readutf8esc(ls));
    /* add 'buff' to string */
    while n > 0i32 {
        save(ls, buff[(8i32 - n) as usize] as libc::c_int);
        n -= 1
    }
}
unsafe extern "C" fn readutf8esc(mut ls: *mut LexState) -> libc::c_ulong {
    let mut r: libc::c_ulong = 0;
    /* chars to be removed: '\', 'u', '{', and first digit */
    let mut i: libc::c_int = 4i32;
    /* skip 'u' */
    save(ls, (*ls).current);
    let fresh60 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh60 > 0i32 as libc::c_ulong {
        let fresh61 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh61 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        ((*ls).current == '{' as i32) as libc::c_int,
        b"missing \'{\'\x00" as *const u8 as *const libc::c_char,
    );
    /* must have at least one digit */
    r = gethexa(ls) as libc::c_ulong;
    loop {
        save(ls, (*ls).current);
        let fresh62 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh62 > 0i32 as libc::c_ulong {
            let fresh63 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh63 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        if !(0 != luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 4i32) {
            break;
        }
        i += 1;
        r = (r << 4i32).wrapping_add(luaO_hexavalue((*ls).current) as libc::c_ulong);
        esccheck(
            ls,
            (r <= 0x10ffffi32 as libc::c_ulong) as libc::c_int,
            b"UTF-8 value too large\x00" as *const u8 as *const libc::c_char,
        );
    }
    esccheck(
        ls,
        ((*ls).current == '}' as i32) as libc::c_int,
        b"missing \'}\'\x00" as *const u8 as *const libc::c_char,
    );
    /* skip '}' */
    let fresh64 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh64 > 0i32 as libc::c_ulong {
        let fresh65 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh65 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* remove saved chars from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(i as libc::c_ulong) as size_t as size_t;
    return r;
}
unsafe extern "C" fn gethexa(mut ls: *mut LexState) -> libc::c_int {
    save(ls, (*ls).current);
    let fresh66 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh66 > 0i32 as libc::c_ulong {
        let fresh67 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh67 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    esccheck(
        ls,
        luai_ctype_[((*ls).current + 1i32) as usize] as libc::c_int & 1i32 << 4i32,
        b"hexadecimal digit expected\x00" as *const u8 as *const libc::c_char,
    );
    return luaO_hexavalue((*ls).current);
}
unsafe extern "C" fn readhexaesc(mut ls: *mut LexState) -> libc::c_int {
    let mut r: libc::c_int = gethexa(ls);
    r = (r << 4i32) + gethexa(ls);
    /* remove saved chars from buffer */
    (*(*ls).buff).n =
        ((*(*ls).buff).n as libc::c_ulong).wrapping_sub(2i32 as libc::c_ulong) as size_t as size_t;
    return r;
}
/*
** skip a sequence '[=*[' or ']=*]'; if sequence is well formed, return
** its number of '='s; otherwise, return a negative number (-1 iff there
** are no '='s after initial bracket)
*/
unsafe extern "C" fn skip_sep(mut ls: *mut LexState) -> libc::c_int {
    let mut count: libc::c_int = 0i32;
    let mut s: libc::c_int = (*ls).current;
    save(ls, (*ls).current);
    let fresh68 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh68 > 0i32 as libc::c_ulong {
        let fresh69 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh69 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    while (*ls).current == '=' as i32 {
        save(ls, (*ls).current);
        let fresh70 = (*(*ls).z).n;
        (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
        (*ls).current = if fresh70 > 0i32 as libc::c_ulong {
            let fresh71 = (*(*ls).z).p;
            (*(*ls).z).p = (*(*ls).z).p.offset(1);
            *fresh71 as libc::c_uchar as libc::c_int
        } else {
            luaZ_fill((*ls).z)
        };
        count += 1
    }
    return if (*ls).current == s {
        count
    } else {
        -count - 1i32
    };
}
unsafe extern "C" fn read_long_string(
    mut ls: *mut LexState,
    mut seminfo: *mut SemInfo,
    mut sep: libc::c_int,
) -> () {
    /* initial line (for error message) */
    let mut line: libc::c_int = (*ls).linenumber;
    /* skip 2nd '[' */
    save(ls, (*ls).current);
    let fresh72 = (*(*ls).z).n;
    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
    (*ls).current = if fresh72 > 0i32 as libc::c_ulong {
        let fresh73 = (*(*ls).z).p;
        (*(*ls).z).p = (*(*ls).z).p.offset(1);
        *fresh73 as libc::c_uchar as libc::c_int
    } else {
        luaZ_fill((*ls).z)
    };
    /* string starts with a newline? */
    if (*ls).current == '\n' as i32 || (*ls).current == '\r' as i32 {
        /* skip it */
        inclinenumber(ls);
    }
    loop {
        match (*ls).current {
            -1 => {
                /* error */
                let mut what: *const libc::c_char = if !seminfo.is_null() {
                    b"string\x00" as *const u8 as *const libc::c_char
                } else {
                    b"comment\x00" as *const u8 as *const libc::c_char
                };
                let mut msg: *const libc::c_char = luaO_pushfstring(
                    (*ls).L,
                    b"unfinished long %s (starting at line %d)\x00" as *const u8
                        as *const libc::c_char,
                    what,
                    line,
                );
                lexerror(ls, msg, TK_EOS as libc::c_int);
            }
            93 => {
                if !(skip_sep(ls) == sep) {
                    continue;
                }
                /* skip 2nd ']' */
                save(ls, (*ls).current);
                let fresh74 = (*(*ls).z).n;
                (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                (*ls).current = if fresh74 > 0i32 as libc::c_ulong {
                    let fresh75 = (*(*ls).z).p;
                    (*(*ls).z).p = (*(*ls).z).p.offset(1);
                    *fresh75 as libc::c_uchar as libc::c_int
                } else {
                    luaZ_fill((*ls).z)
                };
                break;
            }
            10 | 13 => {
                save(ls, '\n' as i32);
                inclinenumber(ls);
                if !seminfo.is_null() {
                    continue;
                }
                /* avoid wasting space */
                (*(*ls).buff).n = 0i32 as size_t
            }
            _ => {
                if !seminfo.is_null() {
                    save(ls, (*ls).current);
                    let fresh76 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh76 > 0i32 as libc::c_ulong {
                        let fresh77 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh77 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    }
                } else {
                    let fresh78 = (*(*ls).z).n;
                    (*(*ls).z).n = (*(*ls).z).n.wrapping_sub(1);
                    (*ls).current = if fresh78 > 0i32 as libc::c_ulong {
                        let fresh79 = (*(*ls).z).p;
                        (*(*ls).z).p = (*(*ls).z).p.offset(1);
                        *fresh79 as libc::c_uchar as libc::c_int
                    } else {
                        luaZ_fill((*ls).z)
                    }
                }
            }
        }
    }
    if !seminfo.is_null() {
        (*seminfo).ts = luaX_newstring(
            ls,
            (*(*ls).buff).buffer.offset((2i32 + sep) as isize),
            (*(*ls).buff)
                .n
                .wrapping_sub((2i32 * (2i32 + sep)) as libc::c_ulong),
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaX_lookahead(mut ls: *mut LexState) -> libc::c_int {
    (*ls).lookahead.token = llex(ls, &mut (*ls).lookahead.seminfo);
    return (*ls).lookahead.token;
}
#[no_mangle]
pub unsafe extern "C" fn luaX_syntaxerror(
    mut ls: *mut LexState,
    mut msg: *const libc::c_char,
) -> ! {
    lexerror(ls, msg, (*ls).t.token);
}
