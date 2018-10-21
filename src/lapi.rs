use libc;
use lobject::*;
use lua::*;

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
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaC_barrier_(L: *mut lua_State, o: *mut GCObject, v: *mut GCObject) -> ();
    #[no_mangle]
    fn luaD_growstack(L: *mut lua_State, n: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State, f: Pfunc, ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn luaV_tonumber_(obj: *const TValue, n: *mut lua_Number) -> libc::c_int;
    #[no_mangle]
    static luaT_typenames_: [*const libc::c_char; 11];
    #[no_mangle]
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn luaC_step(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaO_tostring(L: *mut lua_State, obj: StkId) -> ();
    #[no_mangle]
    fn luaH_getn(t: *mut Table) -> lua_Unsigned;
    #[no_mangle]
    fn luaO_arith(
        L: *mut lua_State,
        op: libc::c_int,
        p1: *const TValue,
        p2: *const TValue,
        res: *mut TValue,
    ) -> ();
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
    fn luaV_equalobj(L: *mut lua_State, t1: *const TValue, t2: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_lessequal(L: *mut lua_State, l: *const TValue, r: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaV_lessthan(L: *mut lua_State, l: *const TValue, r: *const TValue) -> libc::c_int;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_new(L: *mut lua_State, str: *const libc::c_char) -> *mut TString;
    #[no_mangle]
    fn luaO_pushvfstring(
        L: *mut lua_State,
        fmt: *const libc::c_char,
        argp: *mut __va_list_tag,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn luaF_newCclosure(L: *mut lua_State, nelems: libc::c_int) -> *mut CClosure;
    /*
     ** $Id: ltable.h,v 2.23.1.2 2018/05/24 19:39:05 roberto Exp $
     ** Lua tables (hash)
     ** See Copyright Notice in lua.h
     */
    /* 'const' to avoid wrong writings that can mess up field 'next' */
    /*
     ** writable version of 'gkey'; allows updates to individual fields,
     ** but not to the whole (which has incompatible type)
     */
    /* true when 't' is using 'dummynode' as its hash part */
    /* allocated size for hash nodes */
    /* returns the key, given the value of a table entry */
    #[no_mangle]
    fn luaH_getint(t: *mut Table, key: lua_Integer) -> *const TValue;
    #[no_mangle]
    fn luaV_finishget(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    ) -> ();
    #[no_mangle]
    fn luaH_getstr(t: *mut Table, key: *mut TString) -> *const TValue;
    #[no_mangle]
    fn luaH_get(t: *mut Table, key: *const TValue) -> *const TValue;
    #[no_mangle]
    fn luaH_resize(
        L: *mut lua_State,
        t: *mut Table,
        nasize: libc::c_uint,
        nhsize: libc::c_uint,
    ) -> ();
    #[no_mangle]
    fn luaH_new(L: *mut lua_State) -> *mut Table;
    #[no_mangle]
    fn luaS_newudata(L: *mut lua_State, s: size_t) -> *mut Udata;
    #[no_mangle]
    fn luaV_finishset(
        L: *mut lua_State,
        t: *const TValue,
        key: *mut TValue,
        val: StkId,
        slot: *const TValue,
    ) -> ();
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State, o: *mut Table) -> ();
    #[no_mangle]
    fn luaH_set(L: *mut lua_State, t: *mut Table, key: *const TValue) -> *mut TValue;
    #[no_mangle]
    fn luaH_setint(L: *mut lua_State, t: *mut Table, key: lua_Integer, value: *mut TValue) -> ();
    #[no_mangle]
    fn luaC_checkfinalizer(L: *mut lua_State, o: *mut GCObject, mt: *mut Table) -> ();
    #[no_mangle]
    fn luaD_callnoyield(L: *mut lua_State, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_call(L: *mut lua_State, func: StkId, nResults: libc::c_int) -> ();
    #[no_mangle]
    fn luaD_pcall(
        L: *mut lua_State,
        func: Pfunc,
        u: *mut libc::c_void,
        oldtop: ptrdiff_t,
        ef: ptrdiff_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaC_upvalbarrier_(L: *mut lua_State, uv: *mut UpVal) -> ();
    #[no_mangle]
    fn luaD_protectedparser(
        L: *mut lua_State,
        z: *mut ZIO,
        name: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaZ_init(L: *mut lua_State, z: *mut ZIO, reader: lua_Reader, data: *mut libc::c_void)
        -> ();
    /* dump one chunk; from ldump.c */
    #[no_mangle]
    fn luaU_dump(
        L: *mut lua_State,
        f: *const Proto,
        w: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    /* macros to convert a GCObject into a specific value */
    /* macro to convert a Lua object into a GCObject */
    /* actual number of total bytes allocated */
    #[no_mangle]
    fn luaE_setdebt(g: *mut global_State, debt: l_mem) -> ();
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State, isemergency: libc::c_int) -> ();
    #[no_mangle]
    fn luaG_errormsg(L: *mut lua_State) -> !;
    #[no_mangle]
    fn luaH_next(L: *mut lua_State, t: *mut Table, key: StkId) -> libc::c_int;
    #[no_mangle]
    fn luaV_concat(L: *mut lua_State, total: libc::c_int) -> ();
    #[no_mangle]
    fn luaV_objlen(L: *mut lua_State, ra: StkId, rb: *const TValue) -> ();
    #[no_mangle]
    fn luaO_str2num(s: *const libc::c_char, o: *mut TValue) -> size_t;
    #[no_mangle]
    fn luaC_upvdeccount(L: *mut lua_State, uv: *mut UpVal) -> ();
}

pub type __builtin_va_list = [__va_list_tag; 1];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}

pub type va_list = __builtin_va_list;
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
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
/*
** Type for functions that read/write blocks when loading/dumping Lua chunks
*/
pub type lua_Reader = Option<
    unsafe extern "C" fn(_: *mut lua_State, _: *mut libc::c_void, _: *mut size_t)
        -> *const libc::c_char,
>;
pub type lua_Writer = Option<
    unsafe extern "C" fn(
        _: *mut lua_State,
        _: *const libc::c_void,
        _: size_t,
        _: *mut libc::c_void,
    ) -> libc::c_int,
>;
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
** Ensures that address after this type is always fully aligned.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub union UTString {
    pub dummy: L_Umaxalign,
    pub tsv: TString,
}
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
    pub u: libc::c_double,
    pub s: *mut libc::c_void,
    pub i: lua_Integer,
    pub l: libc::c_long,
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
** Execute a protected call.
*/
/* data to 'f_call' */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallS {
    pub func: StkId,
    pub nresults: libc::c_int,
}
pub type ZIO = Zio;
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
/*
** generic extra include file
*/
/*
** RCS ident string
*/

/*
** $Id: lapi.c,v 2.259.1.2 2017/12/06 18:35:12 roberto Exp $
** Lua API
** See Copyright Notice in lua.h
*/

/* value at a non-valid index */
//#define NONVALIDVALUE		cast(TValue *, luaO_nilobject)
macro_rules! NONVALIDVALUE {
    () => {
        &luaO_nilobject_ as *const TValue as *mut TValue
    };
}

/* corresponding test */
//#define isvalid(o)	((o) != luaO_nilobject)
macro_rules! isvalid {
    ($o:expr) => {
        ($o != &luaO_nilobject_ as *const TValue as StkId)
    };
}

/* test for pseudo index */
//#define ispseudo(i)		((i) <= LUA_REGISTRYINDEX)
macro_rules! ispseudo {
    ($i:expr) => {
        ($i <= LUA_REGISTRYINDEX)
    };
}

/* test for upvalue */
//#define isupvalue(i)		((i) < LUA_REGISTRYINDEX)
macro_rules! isupvalue {
    ($i:expr) => {
        ($i < LUA_REGISTRYINDEX)
    };
}

/* test for valid but not pseudo index */
//#define isstackindex(i, o)	(isvalid(o) && !ispseudo(i))
macro_rules! isstackindex {
    ($i:expr, $o:expr) => {
        (isvalid!(o) && !ispseudo!(i))
    };
}

// #define api_checkvalidindex(l,o)  api_check(l, isvalid(o), "invalid index")

// #define api_checkstackindex(l, i, o)  \
// 	api_check(l, isstackindex(i, o), "index not in the stack")

#[no_mangle]
pub static mut lua_ident: [libc::c_char; 129] = [
    36, 76, 117, 97, 86, 101, 114, 115, 105, 111, 110, 58, 32, 76, 117, 97, 32, 53, 46, 51, 46, 53,
    32, 32, 67, 111, 112, 121, 114, 105, 103, 104, 116, 32, 40, 67, 41, 32, 49, 57, 57, 52, 45, 50,
    48, 49, 56, 32, 76, 117, 97, 46, 111, 114, 103, 44, 32, 80, 85, 67, 45, 82, 105, 111, 32, 36,
    36, 76, 117, 97, 65, 117, 116, 104, 111, 114, 115, 58, 32, 82, 46, 32, 73, 101, 114, 117, 115,
    97, 108, 105, 109, 115, 99, 104, 121, 44, 32, 76, 46, 32, 72, 46, 32, 100, 101, 32, 70, 105,
    103, 117, 101, 105, 114, 101, 100, 111, 44, 32, 87, 46, 32, 67, 101, 108, 101, 115, 32, 36, 0,
];
#[no_mangle]
pub unsafe extern "C" fn lua_atpanic(
    mut L: *mut lua_State,
    mut panicf: lua_CFunction,
) -> lua_CFunction {
    let mut old: lua_CFunction = None;
    old = (*(*L).l_G).panic;
    (*(*L).l_G).panic = panicf;
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn lua_version(mut L: *mut lua_State) -> *const lua_Number {
    static mut version: lua_Number = { 503i32 as lua_Number };
    if L.is_null() {
        return &version;
    } else {
        return (*(*L).l_G).version;
    };
}
/*
** basic stack manipulation
*/
#[no_mangle]
pub unsafe extern "C" fn lua_absindex(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    return if idx > 0i32 || ispseudo!(idx) {
        idx
    } else {
        (*L).top.wrapping_offset_from((*(*L).ci).func) as libc::c_long as libc::c_int + idx
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettop(mut L: *mut lua_State) -> libc::c_int {
    return (*L)
        .top
        .wrapping_offset_from((*(*L).ci).func.offset(1isize)) as libc::c_long
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_settop(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut func: StkId = (*(*L).ci).func;
    if idx >= 0i32 {
        while (*L).top < func.offset(1isize).offset(idx as isize) {
            let fresh0 = (*L).top;
            (*L).top = (*L).top.offset(1);
            (*fresh0).tt_ = 0i32
        }
        (*L).top = func.offset(1isize).offset(idx as isize)
    } else {
        (*L).top = (*L).top.offset((idx + 1i32) as isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvalue(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut io1: *mut TValue = (*L).top;
    *io1 = *index2addr(L, idx);
    (*L).top = (*L).top.offset(1isize);
}

unsafe extern "C" fn index2addr(mut L: *mut lua_State, mut idx: libc::c_int) -> *mut TValue {
    let mut ci: *mut CallInfo = (*L).ci;
    if idx > 0i32 {
        let mut o: *mut TValue = (*ci).func.offset(idx as isize);
        // api_check(L, idx <= ci->top - (ci->func + 1), "unacceptable index");
        if o >= (*L).top {
            return NONVALIDVALUE!();
        } else {
            return o;
        }
    } else if !ispseudo!(idx) {
        /* negative index */
        // api_check(L, idx != 0 && -idx <= L->top - (ci->func + 1), "invalid index");
        return (*L).top.offset(idx as isize);
    } else if idx == LUA_REGISTRYINDEX {
        return &mut (*(*L).l_G).l_registry;
    } else {
        /* upvalues */
        idx = LUA_REGISTRYINDEX - idx;
        // api_check(L, idx <= MAXUPVAL + 1, "upvalue index too large");
        /* light C function? */
        if ttislcf!(*(*ci).func) {
            /* it has no upvalues */
            return NONVALIDVALUE!();
        } else {
            let mut func: *mut CClosure = &mut (*((*(*ci).func).value_.gc as *mut GCUnion)).cl.c;
            return if idx <= (*func).nupvalues as libc::c_int {
                &mut (*func).upvalue[(idx - 1i32) as usize] as *mut TValue
            } else {
                NONVALIDVALUE!()
            };
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rotate(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: libc::c_int,
) -> () {
    let mut p: StkId = 0 as *mut TValue;
    let mut t: StkId = 0 as *mut TValue;
    let mut m: StkId = 0 as *mut TValue;
    /* end of stack segment being rotated */
    t = (*L).top.offset(-1isize);
    /* start of segment */
    p = index2addr(L, idx);
    /* end of prefix */
    m = if n >= 0i32 {
        t.offset(-(n as isize))
    } else {
        p.offset(-(n as isize)).offset(-1isize)
    };
    /* reverse the prefix with length 'n' */
    reverse(L, p, m);
    /* reverse the suffix */
    reverse(L, m.offset(1isize), t);
    /* reverse the entire segment */
    reverse(L, p, t);
}
/*
** Reverse the stack segment from 'from' to 'to'
** (auxiliary to 'lua_rotate')
*/
unsafe extern "C" fn reverse(mut _L: *mut lua_State, mut from: StkId, mut to: StkId) -> () {
    while from < to {
        let mut temp: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io1: *mut TValue = &mut temp;
        *io1 = *from;
        let mut io1_0: *mut TValue = from;
        *io1_0 = *to;
        let mut io1_1: *mut TValue = to;
        *io1_1 = temp;
        from = from.offset(1isize);
        to = to.offset(-1isize)
    }
}
#[no_mangle]
pub unsafe extern "C" fn lua_copy(
    mut L: *mut lua_State,
    mut fromidx: libc::c_int,
    mut toidx: libc::c_int,
) -> () {
    let mut fr: *mut TValue = 0 as *mut TValue;
    let mut to: *mut TValue = 0 as *mut TValue;
    fr = index2addr(L, fromidx);
    to = index2addr(L, toidx);
    let mut io1: *mut TValue = to;
    *io1 = *fr;
    /* function upvalue? */
    if isupvalue!(toidx) {
        if 0 != (*fr).tt_ & 1i32 << 6i32
            && 0 != (*((*(*(*L).ci).func).value_.gc as *mut GCUnion))
                .cl
                .c
                .marked as libc::c_int
                & 1i32 << 2i32
            && 0 != (*(*fr).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            luaC_barrier_(
                L,
                &mut (*(&mut (*((*(*(*L).ci).func).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                    as *mut GCUnion))
                    .gc,
                (*fr).value_.gc,
            );
        } else {
        };
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_checkstack(mut L: *mut lua_State, mut n: libc::c_int) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut ci: *mut CallInfo = (*L).ci;
    /* stack large enough? */
    if (*L).stack_last.wrapping_offset_from((*L).top) as libc::c_long > n as libc::c_long {
        /* yes; check is OK */
        res = 1i32
    } else {
        /* no; need to grow stack */
        let mut inuse: libc::c_int =
            (*L).top.wrapping_offset_from((*L).stack) as libc::c_long as libc::c_int + 5i32;
        /* can grow without overflow? */
        if inuse > 1000000i32 - n {
            /* no */
            res = 0i32
        } else {
            res = (luaD_rawrunprotected(
                L,
                Some(growstack),
                &mut n as *mut libc::c_int as *mut libc::c_void,
            ) == LUA_OK) as libc::c_int
        }
    }
    if 0 != res && (*ci).top < (*L).top.offset(n as isize) {
        /* adjust frame top */
        (*ci).top = (*L).top.offset(n as isize)
    }
    return res;
}
/*
** to be called by 'lua_checkstack' in protected mode, to grow stack
** capturing memory errors
*/
unsafe extern "C" fn growstack(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    let mut size: libc::c_int = *(ud as *mut libc::c_int);
    luaD_growstack(L, size);
}
#[no_mangle]
pub unsafe extern "C" fn lua_xmove(
    mut from: *mut lua_State,
    mut to: *mut lua_State,
    mut n: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0;
    if from == to {
        return;
    } else {
        (*from).top = (*from).top.offset(-(n as isize));
        i = 0i32;
        while i < n {
            let mut io1: *mut TValue = (*to).top;
            *io1 = *(*from).top.offset(i as isize);
            /* stack already checked by previous 'api_check' */
            (*to).top = (*to).top.offset(1isize);
            i += 1
        }
        return;
    };
}
/*
** access functions (stack -> C)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_isnumber(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    return if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_isstring(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ & 0xfi32 == 4i32 || (*o).tt_ & 0xfi32 == 3i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_iscfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return (ttislcf!(*o) || (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isinteger(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return ((*o).tt_ == 3i32 | 1i32 << 4i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_isuserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return ((*o).tt_ == 7i32 | 1i32 << 6i32 || (*o).tt_ == 2i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_type(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut o: StkId = index2addr(L, idx);
    return if isvalid!(o) {
        (*o).tt_ & 0xfi32
    } else {
        -1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_typename(
    mut _L: *mut lua_State,
    mut t: libc::c_int,
) -> *const libc::c_char {
    return luaT_typenames_[(t + 1i32) as usize];
}
#[no_mangle]
pub unsafe extern "C" fn lua_tonumberx(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Number {
    let mut n: lua_Number = 0.;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_ == 3i32 | 0i32 << 4i32 {
        n = (*o).value_.n;
        1i32
    } else {
        luaV_tonumber_(o, &mut n)
    };
    if 0 == isnum {
        /* call to 'tonumber' may change 'n' even if it fails */
        n = 0i32 as lua_Number
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tointegerx(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut pisnum: *mut libc::c_int,
) -> lua_Integer {
    let mut res: lua_Integer = 0;
    let mut o: *const TValue = index2addr(L, idx);
    let mut isnum: libc::c_int = if (*o).tt_ == 3i32 | 1i32 << 4i32 {
        res = (*o).value_.i;
        1i32
    } else {
        luaV_tointeger(o, &mut res, 0i32)
    };
    if 0 == isnum {
        /* call to 'tointeger' may change 'n' even if it fails */
        res = 0i32 as lua_Integer
    }
    if !pisnum.is_null() {
        *pisnum = isnum
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_toboolean(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut o: *const TValue = index2addr(L, idx);
    return !((*o).tt_ == 0i32 || (*o).tt_ == 1i32 && (*o).value_.b == 0i32) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lua_tolstring(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut o: StkId = index2addr(L, idx);
    if !((*o).tt_ & 0xfi32 == 4i32) {
        if !((*o).tt_ & 0xfi32 == 3i32) {
            /* not convertible? */
            if !len.is_null() {
                *len = 0i32 as size_t
            }
            return 0 as *const libc::c_char;
        } else {
            /* 'luaO_tostring' may create a new string */
            luaO_tostring(L, o);
            if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                luaC_step(L);
            }
            /* previous call may reallocate the stack */
            o = index2addr(L, idx)
        }
    }
    if !len.is_null() {
        *len = if (*((*o).value_.gc as *mut GCUnion)).ts.tt as libc::c_int == 4i32 | 0i32 << 4i32 {
            (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as libc::c_ulong
        } else {
            (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen
        }
    }
    return (&mut (*((*o).value_.gc as *mut GCUnion)).ts as *mut TString as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawlen(mut L: *mut lua_State, mut idx: libc::c_int) -> size_t {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        4 => return (*((*o).value_.gc as *mut GCUnion)).ts.shrlen as size_t,
        20 => return (*((*o).value_.gc as *mut GCUnion)).ts.u.lnglen,
        7 => return (*((*o).value_.gc as *mut GCUnion)).u.len,
        5 => return luaH_getn(&mut (*((*o).value_.gc as *mut GCUnion)).h) as size_t,
        _ => return 0i32 as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tocfunction(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> lua_CFunction {
    let mut o: StkId = index2addr(L, idx);
    if ttislcf!(*o) {
        return (*o).value_.f;
    } else if (*o).tt_ == 6i32 | 2i32 << 4i32 | 1i32 << 6i32 {
        return (*((*o).value_.gc as *mut GCUnion)).cl.c.f;
    } else {
        return None;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_touserdata(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0xfi32 {
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *mut libc::c_void
        }
        2 => return (*o).value_.p,
        _ => return 0 as *mut libc::c_void,
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_tothread(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *mut lua_State {
    let mut o: StkId = index2addr(L, idx);
    return if !((*o).tt_ == 8i32 | 1i32 << 6i32) {
        0 as *mut lua_State
    } else {
        &mut (*((*o).value_.gc as *mut GCUnion)).th
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_topointer(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> *const libc::c_void {
    let mut o: StkId = index2addr(L, idx);
    match (*o).tt_ & 0x3fi32 {
        5 => return &mut (*((*o).value_.gc as *mut GCUnion)).h as *mut Table as *const libc::c_void,
        6 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.l as *mut LClosure
                as *const libc::c_void
        }
        38 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).cl.c as *mut CClosure
                as *const libc::c_void
        }
        22 => {
            return ::std::mem::transmute::<lua_CFunction, size_t>((*o).value_.f)
                as *mut libc::c_void
        }
        8 => {
            return &mut (*((*o).value_.gc as *mut GCUnion)).th as *mut lua_State
                as *const libc::c_void
        }
        7 => {
            return (&mut (*((*o).value_.gc as *mut GCUnion)).u as *mut Udata as *mut libc::c_char)
                .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
                as *const libc::c_void
        }
        2 => return (*o).value_.p,
        _ => return 0 as *const libc::c_void,
    };
}
/*
** Comparison and arithmetic functions
*/
/* ORDER TM, ORDER OP */
#[no_mangle]
pub unsafe extern "C" fn lua_arith(mut L: *mut lua_State, mut op: libc::c_int) -> () {
    if !(op != 12i32 && op != 13i32) {
        /* all other operations expect two operands */
        /* for unary operations, add fake 2nd operand */
        let mut io1: *mut TValue = (*L).top;
        *io1 = *(*L).top.offset(-1isize);
        (*L).top = (*L).top.offset(1isize)
    }
    /* first operand at top - 2, second at top - 1; result go to top - 2 */
    luaO_arith(
        L,
        op,
        (*L).top.offset(-2isize) as *const TValue,
        (*L).top.offset(-1isize) as *const TValue,
        (*L).top.offset(-2isize),
    );
    /* remove second operand */
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawequal(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
) -> libc::c_int {
    let mut o1: StkId = index2addr(L, index1);
    let mut o2: StkId = index2addr(L, index2);
    return if isvalid!(o1) && isvalid!(o2) {
        luaV_equalobj(
            0 as *mut lua_State,
            o1 as *const TValue,
            o2 as *const TValue,
        )
    } else {
        0i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_compare(
    mut L: *mut lua_State,
    mut index1: libc::c_int,
    mut index2: libc::c_int,
    mut op: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut o1: StkId = 0 as *mut TValue;
    let mut o2: StkId = 0 as *mut TValue;
    let mut i: libc::c_int = 0i32;
    /* may call tag method */
    o1 = index2addr(L, index1);
    o2 = index2addr(L, index2);
    if isvalid!(o1) && isvalid!(o2) {
        match op {
            0 => {
                current_block = 9754527956383385389;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            1 => {
                current_block = 17152279037612840043;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            2 => {
                current_block = 8492507194373517126;
                match current_block {
                    8492507194373517126 => {
                        i = luaV_lessequal(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    17152279037612840043 => {
                        i = luaV_lessthan(L, o1 as *const TValue, o2 as *const TValue)
                    }
                    _ => i = luaV_equalobj(L, o1 as *const TValue, o2 as *const TValue),
                }
            }
            _ => {}
        }
    }
    return i;
}
/*
** push functions (C -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_pushnil(mut L: *mut lua_State) -> () {
    (*(*L).top).tt_ = 0i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushnumber(mut L: *mut lua_State, mut n: lua_Number) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.n = n;
    (*io).tt_ = 3i32 | 0i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushinteger(mut L: *mut lua_State, mut n: lua_Integer) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.i = n;
    (*io).tt_ = 3i32 | 1i32 << 4i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> *const libc::c_char {
    let mut ts: *mut TString = 0 as *mut TString;
    ts = if len == 0i32 as libc::c_ulong {
        luaS_new(L, s!(b"\x00"))
    } else {
        luaS_newlstr(L, s, len)
    };
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut TString = ts;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    return (ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushstring(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> *const libc::c_char {
    if s.is_null() {
        (*(*L).top).tt_ = 0i32
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        ts = luaS_new(L, s);
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = ts;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        /* internal copy's address */
        s = (ts as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
    }
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushvfstring(
    mut L: *mut lua_State,
    mut fmt: *const libc::c_char,
    mut argp: *mut __va_list_tag,
) -> *const libc::c_char {
    let mut ret: *const libc::c_char = 0 as *const libc::c_char;
    ret = luaO_pushvfstring(L, fmt, argp);
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushcclosure(
    mut L: *mut lua_State,
    mut fn_0: lua_CFunction,
    mut n: libc::c_int,
) -> () {
    if n == 0i32 {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.f = fn_0;
        (*io).tt_ = 6i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut cl: *mut CClosure = 0 as *mut CClosure;
        cl = luaF_newCclosure(L, n);
        (*cl).f = fn_0;
        (*L).top = (*L).top.offset(-(n as isize));
        loop {
            let fresh1 = n;
            n = n - 1;
            if !(0 != fresh1) {
                break;
            }
            let mut io1: *mut TValue = &mut (*cl).upvalue[n as usize] as *mut TValue;
            *io1 = *(*L).top.offset(n as isize)
        }
        /* does not need barrier because closure is white */
        let mut io_0: *mut TValue = (*L).top;
        let mut x_: *mut CClosure = cl;
        (*io_0).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io_0).tt_ = 6i32 | 2i32 << 4i32 | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
            luaC_step(L);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushboolean(mut L: *mut lua_State, mut b: libc::c_int) -> () {
    /* ensure that true is 1 */
    let mut io: *mut TValue = (*L).top;
    (*io).value_.b = (b != 0i32) as libc::c_int;
    (*io).tt_ = 1i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushlightuserdata(
    mut L: *mut lua_State,
    mut p: *mut libc::c_void,
) -> () {
    let mut io: *mut TValue = (*L).top;
    (*io).value_.p = p;
    (*io).tt_ = 2i32;
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_pushthread(mut L: *mut lua_State) -> libc::c_int {
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut lua_State = L;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 8i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    return ((*(*L).l_G).mainthread == L) as libc::c_int;
}
/*
** get functions (Lua -> stack)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_getglobal(
    mut L: *mut lua_State,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    return auxgetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** get functions (Lua -> stack)
*/
unsafe extern "C" fn auxgetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> libc::c_int {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishget(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_gettable(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-1isize) as *const TValue,
        );
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top.offset(-1isize);
        *io1 = *slot
    } else {
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getfield(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) -> libc::c_int {
    return auxgetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_geti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        !((*slot).tt_ == 0i32) as libc::c_int
    } {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *slot;
        (*L).top = (*L).top.offset(1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishget(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawget(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = (*L).top.offset(-1isize);
    *io1 = *luaH_get(
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize) as *const TValue,
    );
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgeti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawgetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    t = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2i32;
    let mut io1: *mut TValue = (*L).top;
    *io1 = *luaH_get(&mut (*((*t).value_.gc as *mut GCUnion)).h, &mut k);
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_createtable(
    mut L: *mut lua_State,
    mut narray: libc::c_int,
    mut nrec: libc::c_int,
) -> () {
    let mut t: *mut Table = 0 as *mut Table;
    t = luaH_new(L);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Table = t;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 5i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if narray > 0i32 || nrec > 0i32 {
        luaH_resize(L, t, narray as libc::c_uint, nrec as libc::c_uint);
    }
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_newuserdata(
    mut L: *mut lua_State,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut u: *mut Udata = 0 as *mut Udata;
    u = luaS_newudata(L, size);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut Udata = u;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 7i32 | 1i32 << 6i32;
    (*L).top = (*L).top.offset(1isize);
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    }
    return (u as *mut libc::c_char)
        .offset(::std::mem::size_of::<UUdata>() as libc::c_ulong as isize)
        as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *const TValue = 0 as *const TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    let mut res: libc::c_int = 0i32;
    obj = index2addr(L, objindex);
    match (*obj).tt_ & 0xfi32 {
        5 => mt = (*((*obj).value_.gc as *mut GCUnion)).h.metatable,
        7 => mt = (*((*obj).value_.gc as *mut GCUnion)).u.metatable,
        _ => mt = (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize],
    }
    if !mt.is_null() {
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut Table = mt;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = 5i32 | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        res = 1i32
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getuservalue(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
) -> libc::c_int {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = (*L).top;
    let mut iu: *const Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*io).value_ = (*iu).user_;
    (*io).tt_ = (*iu).ttuv_ as libc::c_int;
    (*L).top = (*L).top.offset(1isize);
    return (*(*L).top.offset(-1isize)).tt_ & 0xfi32;
}
/*
** set functions (stack -> Lua)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_setglobal(mut L: *mut lua_State, mut name: *const libc::c_char) -> () {
    let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, luaH_getint(reg, 2i32 as lua_Integer), name);
}
/*
** set functions (stack -> Lua)
*/
/*
** t[k] = value at the top of the stack (where 'k' is a string)
*/
unsafe extern "C" fn auxsetstr(
    mut L: *mut lua_State,
    mut t: *const TValue,
    mut k: *const libc::c_char,
) -> () {
    let mut slot: *const TValue = 0 as *const TValue;
    let mut str: *mut TString = luaS_new(L, k);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getstr(&mut (*((*t).value_.gc as *mut GCUnion)).h, str);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        /* push 'str' (to make it a TValue) */
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = str;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishset(
            L,
            t,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_settable(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    let mut slot: *const TValue = 0 as *const TValue;
    if 0 == if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_get(
            &mut (*((*t).value_.gc as *mut GCUnion)).h,
            (*L).top.offset(-2isize) as *const TValue,
        );
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-2isize),
            (*L).top.offset(-1isize),
            slot,
        );
    }
    /* pop index and value */
    (*L).top = (*L).top.offset(-2isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setfield(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut k: *const libc::c_char,
) -> () {
    /* unlock done in 'auxsetstr' */
    auxsetstr(L, index2addr(L, idx), k);
}
#[no_mangle]
pub unsafe extern "C" fn lua_seti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> () {
    let mut t: StkId = 0 as *mut TValue;
    let mut slot: *const TValue = 0 as *const TValue;
    t = index2addr(L, idx);
    if 0 != if !((*t).tt_ == 5i32 | 1i32 << 6i32) {
        slot = 0 as *const TValue;
        0i32
    } else {
        slot = luaH_getint(&mut (*((*t).value_.gc as *mut GCUnion)).h, n);
        if (*slot).tt_ == 0i32 {
            0i32
        } else {
            if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
                && 0 != (*((*t).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
                && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrierback_(L, &mut (*((*t).value_.gc as *mut GCUnion)).h);
            } else {
            };
            *(slot as *mut TValue) = *(*L).top.offset(-1isize);
            1i32
        }
    } {
        /* pop value */
        (*L).top = (*L).top.offset(-1isize)
    } else {
        let mut io: *mut TValue = (*L).top;
        (*io).value_.i = n;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        (*L).top = (*L).top.offset(1isize);
        luaV_finishset(
            L,
            t as *const TValue,
            (*L).top.offset(-1isize),
            (*L).top.offset(-2isize),
            slot,
        );
        /* pop value and key */
        (*L).top = (*L).top.offset(-2isize)
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawset(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    slot = luaH_set(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-2isize) as *const TValue,
    );
    *slot = *(*L).top.offset(-1isize);
    (*((*o).value_.gc as *mut GCUnion)).h.flags = 0i32 as lu_byte;
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-2isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawseti(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut n: lua_Integer,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    luaH_setint(
        L,
        &mut (*((*o).value_.gc as *mut GCUnion)).h,
        n,
        (*L).top.offset(-1isize),
    );
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_rawsetp(
    mut L: *mut lua_State,
    mut idx: libc::c_int,
    mut p: *const libc::c_void,
) -> () {
    let mut o: StkId = 0 as *mut TValue;
    let mut k: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    let mut slot: *mut TValue = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *mut TValue = &mut k;
    (*io).value_.p = p as *mut libc::c_void;
    (*io).tt_ = 2i32;
    slot = luaH_set(L, &mut (*((*o).value_.gc as *mut GCUnion)).h, &mut k);
    *slot = *(*L).top.offset(-1isize);
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*((*o).value_.gc as *mut GCUnion)).h.marked as libc::c_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrierback_(L, &mut (*((*o).value_.gc as *mut GCUnion)).h);
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_setmetatable(
    mut L: *mut lua_State,
    mut objindex: libc::c_int,
) -> libc::c_int {
    let mut obj: *mut TValue = 0 as *mut TValue;
    let mut mt: *mut Table = 0 as *mut Table;
    obj = index2addr(L, objindex);
    if (*(*L).top.offset(-1isize)).tt_ == 0i32 {
        mt = 0 as *mut Table
    } else {
        mt = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion)).h
    }
    match (*obj).tt_ & 0xfi32 {
        5 => {
            let ref mut fresh2 = (*((*obj).value_.gc as *mut GCUnion)).h.metatable;
            *fresh2 = mt;
            if !mt.is_null() {
                if 0 != (*(*obj).value_.gc).marked as libc::c_int & 1i32 << 2i32
                    && 0 != (*mt).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    luaC_barrier_(
                        L,
                        &mut (*((*obj).value_.gc as *mut GCUnion)).gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        7 => {
            let ref mut fresh3 = (*((*obj).value_.gc as *mut GCUnion)).u.metatable;
            *fresh3 = mt;
            if !mt.is_null() {
                if 0 != (*((*obj).value_.gc as *mut GCUnion)).u.marked as libc::c_int & 1i32 << 2i32
                    && 0 != (*mt).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
                {
                    luaC_barrier_(
                        L,
                        &mut (*(&mut (*((*obj).value_.gc as *mut GCUnion)).u as *mut Udata
                            as *mut GCUnion))
                            .gc,
                        &mut (*(mt as *mut GCUnion)).gc,
                    );
                } else {
                };
                luaC_checkfinalizer(L, (*obj).value_.gc, mt);
            }
        }
        _ => (*(*L).l_G).mt[((*obj).tt_ & 0xfi32) as usize] = mt,
    }
    (*L).top = (*L).top.offset(-1isize);
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setuservalue(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut o: StkId = 0 as *mut TValue;
    o = index2addr(L, idx);
    let mut io: *const TValue = (*L).top.offset(-1isize) as *const TValue;
    let mut iu: *mut Udata = &mut (*((*o).value_.gc as *mut GCUnion)).u;
    (*iu).user_ = (*io).value_;
    (*iu).ttuv_ = (*io).tt_ as lu_byte;
    if 0 != (*(*L).top.offset(-1isize)).tt_ & 1i32 << 6i32
        && 0 != (*(*o).value_.gc).marked as libc::c_int & 1i32 << 2i32
        && 0 != (*(*(*L).top.offset(-1isize)).value_.gc).marked as libc::c_int
            & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrier_(
            L,
            &mut (*((*o).value_.gc as *mut GCUnion)).gc,
            (*(*L).top.offset(-1isize)).value_.gc,
        );
    } else {
    };
    (*L).top = (*L).top.offset(-1isize);
}
/*
** 'load' and 'call' functions (load and run Lua code)
*/
#[no_mangle]
pub unsafe extern "C" fn lua_callk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> () {
    let mut func: StkId = 0 as *mut TValue;
    func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_some() && (*L).nny as libc::c_int == 0i32 {
        /* need to prepare continuation? */
        /* save continuation */
        (*(*L).ci).u.c.k = k;
        /* save context */
        (*(*L).ci).u.c.ctx = ctx;
        /* do the call */
        luaD_call(L, func, nresults);
    } else {
        /* no continuation or no yieldable */
        /* just do the call */
        luaD_callnoyield(L, func, nresults);
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_pcallk(
    mut L: *mut lua_State,
    mut nargs: libc::c_int,
    mut nresults: libc::c_int,
    mut errfunc: libc::c_int,
    mut ctx: lua_KContext,
    mut k: lua_KFunction,
) -> libc::c_int {
    let mut o: StkId = 0 as *mut TValue;
    let mut ci: *mut CallInfo = 0 as *mut CallInfo;
    let mut c: CallS = CallS {
        func: 0 as *mut TValue,
        nresults: 0,
    };
    let mut status: libc::c_int = 0;
    let mut func: ptrdiff_t = 0;
    if errfunc == 0i32 {
        func = 0i32 as ptrdiff_t
    } else {
        o = index2addr(L, errfunc);
        func = (o as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long
    }
    /* function to be called */
    c.func = (*L).top.offset(-((nargs + 1i32) as isize));
    if k.is_none() || (*L).nny as libc::c_int > 0i32 {
        /* no continuation or no yieldable? */
        /* do a 'conventional' protected call */
        c.nresults = nresults;
        status = luaD_pcall(
            L,
            Some(f_call),
            &mut c as *mut CallS as *mut libc::c_void,
            (c.func as *mut libc::c_char).wrapping_offset_from((*L).stack as *mut libc::c_char)
                as libc::c_long,
            func,
        )
    } else {
        /* prepare continuation (call is already protected by 'resume') */
        ci = (*L).ci;
        /* save continuation */
        (*ci).u.c.k = k;
        /* save context */
        (*ci).u.c.ctx = ctx;
        /* save information for error recovery */
        (*ci).extra = (c.func as *mut libc::c_char)
            .wrapping_offset_from((*L).stack as *mut libc::c_char)
            as libc::c_long;
        (*ci).u.c.old_errfunc = (*L).errfunc;
        (*L).errfunc = func;
        /* save value of 'allowhook' */
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 0i32)
            | (*L).allowhook as libc::c_int) as libc::c_ushort;
        /* function can do error recovery */
        (*ci).callstatus = ((*ci).callstatus as libc::c_int | 1i32 << 4i32) as libc::c_ushort;
        /* do the call */
        luaD_call(L, c.func, nresults);
        (*ci).callstatus = ((*ci).callstatus as libc::c_int & !(1i32 << 4i32)) as libc::c_ushort;
        (*L).errfunc = (*ci).u.c.old_errfunc;
        /* if it is here, there were no errors */
        status = LUA_OK
    }
    if nresults == -1i32 && (*(*L).ci).top < (*L).top {
        (*(*L).ci).top = (*L).top
    }
    return status;
}
unsafe extern "C" fn f_call(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    let mut c: *mut CallS = ud as *mut CallS;
    luaD_callnoyield(L, (*c).func, (*c).nresults);
}
#[no_mangle]
pub unsafe extern "C" fn lua_load(
    mut L: *mut lua_State,
    mut reader: lua_Reader,
    mut data: *mut libc::c_void,
    mut chunkname: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut z: ZIO = Zio {
        n: 0,
        p: 0 as *const libc::c_char,
        reader: None,
        data: 0 as *mut libc::c_void,
        L: 0 as *mut lua_State,
    };
    let mut status: libc::c_int = 0;
    if chunkname.is_null() {
        chunkname = s!(b"?\x00")
    }
    luaZ_init(L, &mut z, reader, data);
    status = luaD_protectedparser(L, &mut z, chunkname, mode);
    if status == LUA_OK {
        /* no errors? */
        /* get newly created function */
        let mut f: *mut LClosure = &mut (*((*(*L).top.offset(-1isize)).value_.gc as *mut GCUnion))
            .cl
            .l;
        if (*f).nupvalues as libc::c_int >= 1i32 {
            /* does it have an upvalue? */
            /* get global table from registry */
            let mut reg: *mut Table = &mut (*((*(*L).l_G).l_registry.value_.gc as *mut GCUnion)).h;
            let mut gt: *const TValue = luaH_getint(reg, 2i32 as lua_Integer);
            /* set global table as 1st upvalue of 'f' (may be LUA_ENV) */
            let mut io1: *mut TValue = (*(*f).upvals[0usize]).v;
            *io1 = *gt;
            if 0 != (*(*(*f).upvals[0usize]).v).tt_ & 1i32 << 6i32
                && !((*(*f).upvals[0usize]).v != &mut (*(*f).upvals[0usize]).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, (*f).upvals[0usize]);
            } else {
            };
        }
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_dump(
    mut L: *mut lua_State,
    mut writer: lua_Writer,
    mut data: *mut libc::c_void,
    mut strip: libc::c_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut o: *mut TValue = 0 as *mut TValue;
    o = (*L).top.offset(-1isize);
    if (*o).tt_ == 6i32 | 0i32 << 4i32 | 1i32 << 6i32 {
        status = luaU_dump(
            L,
            (*((*o).value_.gc as *mut GCUnion)).cl.l.p,
            writer,
            data,
            strip,
        )
    } else {
        status = 1i32
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn lua_status(mut L: *mut lua_State) -> libc::c_int {
    return (*L).status as libc::c_int;
}
/*
** garbage-collection function and options
*/
#[no_mangle]
pub unsafe extern "C" fn lua_gc(
    mut L: *mut lua_State,
    mut what: libc::c_int,
    mut data: libc::c_int,
) -> libc::c_int {
    let mut res: libc::c_int = 0i32;
    let mut g: *mut global_State = 0 as *mut global_State;
    g = (*L).l_G;
    match what {
        0 => (*g).gcrunning = 0i32 as lu_byte,
        1 => {
            luaE_setdebt(g, 0i32 as l_mem);
            (*g).gcrunning = 1i32 as lu_byte
        }
        2 => {
            luaC_fullgc(L, 0i32);
        }
        3 => {
            /* GC values are expressed in Kbytes: #bytes/2^10 */
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem >> 10i32) as libc::c_int
        }
        4 => {
            res = (((*g).totalbytes + (*g).GCdebt) as lu_mem & 0x3ffi32 as libc::c_ulong)
                as libc::c_int
        }
        5 => {
            /* =1 to signal that it did an actual step */
            let mut debt: l_mem = 1i32 as l_mem;
            let mut oldrunning: lu_byte = (*g).gcrunning;
            /* allow GC to run */
            (*g).gcrunning = 1i32 as lu_byte;
            if data == 0i32 {
                /* to do a "small" step */
                luaE_setdebt(
                    g,
                    -((100i32 as libc::c_ulong)
                        .wrapping_mul(::std::mem::size_of::<TString>() as libc::c_ulong)
                        as libc::c_int) as l_mem,
                );
                luaC_step(L);
            } else {
                /* add 'data' to total debt */
                debt = data as l_mem * 1024i32 as libc::c_long + (*g).GCdebt;
                luaE_setdebt(g, debt);
                if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
                    luaC_step(L);
                }
            }
            /* restore previous state */
            (*g).gcrunning = oldrunning;
            /* end of cycle? */
            if debt > 0i32 as libc::c_long && (*g).gcstate as libc::c_int == 7i32 {
                /* signal it */
                res = 1i32
            }
        }
        6 => {
            res = (*g).gcpause;
            (*g).gcpause = data
        }
        7 => {
            res = (*g).gcstepmul;
            if data < 40i32 {
                /* avoid ridiculous low values (and 0) */
                data = 40i32
            }
            (*g).gcstepmul = data
        }
        9 => res = (*g).gcrunning as libc::c_int,
        _ => {
            /* invalid option */
            res = -1i32
        }
    }
    return res;
}
/*
** miscellaneous functions
*/
#[no_mangle]
pub unsafe extern "C" fn lua_error(mut L: *mut lua_State) -> libc::c_int {
    luaG_errormsg(L);
}
#[no_mangle]
pub unsafe extern "C" fn lua_next(mut L: *mut lua_State, mut idx: libc::c_int) -> libc::c_int {
    let mut t: StkId = 0 as *mut TValue;
    let mut more: libc::c_int = 0;
    t = index2addr(L, idx);
    more = luaH_next(
        L,
        &mut (*((*t).value_.gc as *mut GCUnion)).h,
        (*L).top.offset(-1isize),
    );
    if 0 != more {
        (*L).top = (*L).top.offset(1isize)
    } else {
        (*L).top = (*L).top.offset(-1isize)
    }
    return more;
}
#[no_mangle]
pub unsafe extern "C" fn lua_concat(mut L: *mut lua_State, mut n: libc::c_int) -> () {
    if n >= 2i32 {
        luaV_concat(L, n);
    } else if n == 0i32 {
        /* push empty string */
        let mut io: *mut TValue = (*L).top;
        let mut x_: *mut TString = luaS_newlstr(L, s!(b"\x00"), 0i32 as size_t);
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        (*L).top = (*L).top.offset(1isize)
    }
    /* else n == 1; nothing to do */
    if (*(*L).l_G).GCdebt > 0i32 as libc::c_long {
        luaC_step(L);
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_len(mut L: *mut lua_State, mut idx: libc::c_int) -> () {
    let mut t: StkId = 0 as *mut TValue;
    t = index2addr(L, idx);
    luaV_objlen(L, (*L).top, t as *const TValue);
    (*L).top = (*L).top.offset(1isize);
}
#[no_mangle]
pub unsafe extern "C" fn lua_stringtonumber(
    mut L: *mut lua_State,
    mut s: *const libc::c_char,
) -> size_t {
    let mut sz: size_t = luaO_str2num(s, (*L).top);
    if sz != 0i32 as libc::c_ulong {
        (*L).top = (*L).top.offset(1isize)
    }
    return sz;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getallocf(
    mut L: *mut lua_State,
    mut ud: *mut *mut libc::c_void,
) -> lua_Alloc {
    let mut f: lua_Alloc = None;
    if !ud.is_null() {
        *ud = (*(*L).l_G).ud
    }
    f = (*(*L).l_G).frealloc;
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_setallocf(
    mut L: *mut lua_State,
    mut f: lua_Alloc,
    mut ud: *mut libc::c_void,
) -> () {
    (*(*L).l_G).ud = ud;
    (*(*L).l_G).frealloc = f;
}
#[no_mangle]
pub unsafe extern "C" fn lua_getupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    name = aux_upvalue(
        index2addr(L, funcindex),
        n,
        &mut val,
        0 as *mut *mut CClosure,
        0 as *mut *mut UpVal,
    );
    if !name.is_null() {
        let mut io1: *mut TValue = (*L).top;
        *io1 = *val;
        (*L).top = (*L).top.offset(1isize)
    }
    return name;
}
unsafe extern "C" fn aux_upvalue(
    mut fi: StkId,
    mut n: libc::c_int,
    mut val: *mut *mut TValue,
    mut owner: *mut *mut CClosure,
    mut uv: *mut *mut UpVal,
) -> *const libc::c_char {
    match (*fi).tt_ & 0x3fi32 {
        38 => {
            /* C closure */
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            if !(1i32 <= n && n <= (*f).nupvalues as libc::c_int) {
                return 0 as *const libc::c_char;
            } else {
                *val = &mut (*f).upvalue[(n - 1i32) as usize] as *mut TValue;
                if !owner.is_null() {
                    *owner = f
                }
                return s!(b"\x00");
            }
        }
        6 => {
            /* Lua closure */
            let mut f_0: *mut LClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
            let mut name: *mut TString = 0 as *mut TString;
            let mut p: *mut Proto = (*f_0).p;
            if !(1i32 <= n && n <= (*p).sizeupvalues) {
                return 0 as *const libc::c_char;
            } else {
                *val = (*(*f_0).upvals[(n - 1i32) as usize]).v;
                if !uv.is_null() {
                    *uv = (*f_0).upvals[(n - 1i32) as usize]
                }
                name = (*(*p).upvalues.offset((n - 1i32) as isize)).name;
                return if name.is_null() {
                    s!(b"(*no name)\x00")
                } else {
                    (name as *mut libc::c_char)
                        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                };
            }
        }
        _ => {
            /* not a closure */
            return 0 as *const libc::c_char;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn lua_setupvalue(
    mut L: *mut lua_State,
    mut funcindex: libc::c_int,
    mut n: libc::c_int,
) -> *const libc::c_char {
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    /* to avoid warnings */
    let mut val: *mut TValue = 0 as *mut TValue;
    let mut owner: *mut CClosure = 0 as *mut CClosure;
    let mut uv: *mut UpVal = 0 as *mut UpVal;
    let mut fi: StkId = 0 as *mut TValue;
    fi = index2addr(L, funcindex);
    name = aux_upvalue(fi, n, &mut val, &mut owner, &mut uv);
    if !name.is_null() {
        (*L).top = (*L).top.offset(-1isize);
        let mut io1: *mut TValue = val;
        *io1 = *(*L).top;
        if !owner.is_null() {
            if 0 != (*(*L).top).tt_ & 1i32 << 6i32
                && 0 != (*owner).marked as libc::c_int & 1i32 << 2i32
                && 0 != (*(*(*L).top).value_.gc).marked as libc::c_int
                    & (1i32 << 0i32 | 1i32 << 1i32)
            {
                luaC_barrier_(L, &mut (*(owner as *mut GCUnion)).gc, (*(*L).top).value_.gc);
            } else {
            };
        } else if !uv.is_null() {
            if 0 != (*(*uv).v).tt_ & 1i32 << 6i32 && !((*uv).v != &mut (*uv).u.value as *mut TValue)
            {
                luaC_upvalbarrier_(L, uv);
            } else {
            };
        }
    }
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvalueid(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut fi: StkId = index2addr(L, fidx);
    match (*fi).tt_ & 0x3fi32 {
        6 => {
            /* lua closure */
            return *getupvalref(L, fidx, n, 0 as *mut *mut LClosure) as *mut libc::c_void;
        }
        38 => {
            /* C closure */
            let mut f: *mut CClosure = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.c;
            return &mut (*f).upvalue[(n - 1i32) as usize] as *mut TValue as *mut libc::c_void;
        }
        _ => return 0 as *mut libc::c_void,
    };
}
unsafe extern "C" fn getupvalref(
    mut L: *mut lua_State,
    mut fidx: libc::c_int,
    mut n: libc::c_int,
    mut pf: *mut *mut LClosure,
) -> *mut *mut UpVal {
    let mut f: *mut LClosure = 0 as *mut LClosure;
    let mut fi: StkId = index2addr(L, fidx);
    f = &mut (*((*fi).value_.gc as *mut GCUnion)).cl.l;
    if !pf.is_null() {
        *pf = f
    }
    /* get its upvalue pointer */
    return &mut (*f).upvals[(n - 1i32) as usize] as *mut *mut UpVal;
}
#[no_mangle]
pub unsafe extern "C" fn lua_upvaluejoin(
    mut L: *mut lua_State,
    mut fidx1: libc::c_int,
    mut n1: libc::c_int,
    mut fidx2: libc::c_int,
    mut n2: libc::c_int,
) -> () {
    let mut f1: *mut LClosure = 0 as *mut LClosure;
    let mut up1: *mut *mut UpVal = getupvalref(L, fidx1, n1, &mut f1);
    let mut up2: *mut *mut UpVal = getupvalref(L, fidx2, n2, 0 as *mut *mut LClosure);
    luaC_upvdeccount(L, *up1);
    *up1 = *up2;
    (**up1).refcount = (**up1).refcount.wrapping_add(1);
    if (**up1).v != &mut (**up1).u.value as *mut TValue {
        (**up1).u.open.touched = 1i32
    }
    if 0 != (*(**up1).v).tt_ & 1i32 << 6i32 && !((**up1).v != &mut (**up1).u.value as *mut TValue) {
        luaC_upvalbarrier_(L, *up1);
    } else {
    };
}
