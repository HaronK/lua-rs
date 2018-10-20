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
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn luaO_pushfstring(L: *mut lua_State, fmt: *const libc::c_char, ...) -> *const libc::c_char;
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
        block: *mut libc::c_void,
        oldsize: size_t,
        size: size_t,
    ) -> *mut libc::c_void;
    #[no_mangle]
    fn luaZ_read(z: *mut ZIO, b: *mut libc::c_void, n: size_t) -> size_t;
    #[no_mangle]
    fn luaD_inctop(L: *mut lua_State) -> ();
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaF_newproto(L: *mut lua_State) -> *mut Proto;
    #[no_mangle]
    fn luaF_newLclosure(L: *mut lua_State, nelems: libc::c_int) -> *mut LClosure;
    #[no_mangle]
    fn luaS_newlstr(L: *mut lua_State, str: *const libc::c_char, l: size_t) -> *mut TString;
    #[no_mangle]
    fn luaS_createlngstrobj(L: *mut lua_State, l: size_t) -> *mut TString;
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
** $Id: lundump.c,v 2.44.1.1 2017/04/19 17:20:42 roberto Exp $
** load precompiled Lua chunks
** See Copyright Notice in lua.h
*/
/* empty */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LoadState {
    pub L: *mut lua_State,
    pub Z: *mut ZIO,
    pub name: *const libc::c_char,
}
/*
** $Id: lundump.h,v 1.45.1.1 2017/04/19 17:20:42 roberto Exp $
** load precompiled Lua chunks
** See Copyright Notice in lua.h
*/
/* data to catch conversion errors */
/* this is the official format */
/* load one chunk; from lundump.c */
#[no_mangle]
pub unsafe extern "C" fn luaU_undump(
    mut L: *mut lua_State,
    mut Z: *mut ZIO,
    mut name: *const libc::c_char,
) -> *mut LClosure {
    let mut S: LoadState = LoadState {
        L: 0 as *mut lua_State,
        Z: 0 as *mut ZIO,
        name: 0 as *const libc::c_char,
    };
    let mut cl: *mut LClosure = 0 as *mut LClosure;
    if *name as libc::c_int == '@' as i32 || *name as libc::c_int == '=' as i32 {
        S.name = name.offset(1isize)
    } else if *name as libc::c_int
        == (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
            as libc::c_int
    {
        S.name = b"binary string\x00" as *const u8 as *const libc::c_char
    } else {
        S.name = name
    }
    S.L = L;
    S.Z = Z;
    checkHeader(&mut S);
    cl = luaF_newLclosure(L, LoadByte(&mut S) as libc::c_int);
    let mut io: *mut TValue = (*L).top;
    let mut x_: *mut LClosure = cl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    LoadFunction(&mut S, (*cl).p, 0 as *mut TString);
    return cl;
}
unsafe extern "C" fn LoadFunction(
    mut S: *mut LoadState,
    mut f: *mut Proto,
    mut psource: *mut TString,
) -> () {
    (*f).source = LoadString(S);
    /* no source in dump? */
    if (*f).source.is_null() {
        /* reuse parent's source */
        (*f).source = psource
    }
    (*f).linedefined = LoadInt(S);
    (*f).lastlinedefined = LoadInt(S);
    (*f).numparams = LoadByte(S);
    (*f).is_vararg = LoadByte(S);
    (*f).maxstacksize = LoadByte(S);
    LoadCode(S, f);
    LoadConstants(S, f);
    LoadUpvalues(S, f);
    LoadProtos(S, f);
    LoadDebug(S, f);
}
unsafe extern "C" fn LoadDebug(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).lineinfo = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*f).sizelineinfo = n;
    LoadBlock(
        S,
        (*f).lineinfo as *mut libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<LocVar>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).locvars = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as libc::c_ulong),
    ) as *mut LocVar;
    (*f).sizelocvars = n;
    i = 0i32;
    while i < n {
        let ref mut fresh0 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh0 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh1 = (*(*f).locvars.offset(i as isize)).varname;
        *fresh1 = LoadString(S);
        (*(*f).locvars.offset(i as isize)).startpc = LoadInt(S);
        (*(*f).locvars.offset(i as isize)).endpc = LoadInt(S);
        i += 1
    }
    n = LoadInt(S);
    i = 0i32;
    while i < n {
        let ref mut fresh2 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh2 = LoadString(S);
        i += 1
    }
}
unsafe extern "C" fn LoadString(mut S: *mut LoadState) -> *mut TString {
    let mut size: size_t = LoadByte(S) as size_t;
    if size == 0xffi32 as libc::c_ulong {
        LoadBlock(
            S,
            &mut size as *mut size_t as *mut libc::c_void,
            (1i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<size_t>() as libc::c_ulong),
        );
    }
    if size == 0i32 as libc::c_ulong {
        return 0 as *mut TString;
    } else {
        size = size.wrapping_sub(1);
        if size <= 40i32 as libc::c_ulong {
            /* short string? */
            let mut buff: [libc::c_char; 40] = [0; 40];
            LoadBlock(
                S,
                buff.as_mut_ptr() as *mut libc::c_void,
                size.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            return luaS_newlstr((*S).L, buff.as_mut_ptr(), size);
        } else {
            /* long string */
            let mut ts: *mut TString = luaS_createlngstrobj((*S).L, size);
            /* load directly in final place */
            LoadBlock(
                S,
                (ts as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    as *mut libc::c_void,
                size.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
            );
            return ts;
        }
    };
}
unsafe extern "C" fn LoadByte(mut S: *mut LoadState) -> lu_byte {
    let mut x: lu_byte = 0;
    LoadBlock(
        S,
        &mut x as *mut lu_byte as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<lu_byte>() as libc::c_ulong),
    );
    return x;
}
/*
** All high-level loads go through LoadVector; you can change it to
** adapt to the endianness of the input
*/
unsafe extern "C" fn LoadBlock(
    mut S: *mut LoadState,
    mut b: *mut libc::c_void,
    mut size: size_t,
) -> () {
    if luaZ_read((*S).Z, b, size) != 0i32 as libc::c_ulong {
        error(S, b"truncated\x00" as *const u8 as *const libc::c_char);
    } else {
        return;
    };
}
unsafe extern "C" fn error(mut S: *mut LoadState, mut why: *const libc::c_char) -> ! {
    luaO_pushfstring!(
        (*S).L,
        b"%s: %s precompiled chunk\x00" as *const u8 as *const libc::c_char,
        (*S).name,
        why,
    );
    luaD_throw((*S).L, 3i32);
}
unsafe extern "C" fn LoadInt(mut S: *mut LoadState) -> libc::c_int {
    let mut x: libc::c_int = 0;
    LoadBlock(
        S,
        &mut x as *mut libc::c_int as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadProtos(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<*mut Proto>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).p = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as libc::c_ulong),
    ) as *mut *mut Proto;
    (*f).sizep = n;
    i = 0i32;
    while i < n {
        let ref mut fresh3 = *(*f).p.offset(i as isize);
        *fresh3 = 0 as *mut Proto;
        i += 1
    }
    i = 0i32;
    while i < n {
        let ref mut fresh4 = *(*f).p.offset(i as isize);
        *fresh4 = luaF_newproto((*S).L);
        LoadFunction(S, *(*f).p.offset(i as isize), (*f).source);
        i += 1
    }
}
unsafe extern "C" fn LoadUpvalues(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Upvaldesc>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).upvalues = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc>() as libc::c_ulong),
    ) as *mut Upvaldesc;
    (*f).sizeupvalues = n;
    i = 0i32;
    while i < n {
        let ref mut fresh5 = (*(*f).upvalues.offset(i as isize)).name;
        *fresh5 = 0 as *mut TString;
        i += 1
    }
    i = 0i32;
    while i < n {
        (*(*f).upvalues.offset(i as isize)).instack = LoadByte(S);
        (*(*f).upvalues.offset(i as isize)).idx = LoadByte(S);
        i += 1
    }
}
unsafe extern "C" fn LoadConstants(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut io: *mut TValue = 0 as *mut TValue;
    let mut io_0: *mut TValue = 0 as *mut TValue;
    let mut io_1: *mut TValue = 0 as *mut TValue;
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).k = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    (*f).sizek = n;
    i = 0i32;
    while i < n {
        (*(*f).k.offset(i as isize)).tt_ = 0i32;
        i += 1
    }
    i = 0i32;
    while i < n {
        let mut o: *mut TValue = &mut *(*f).k.offset(i as isize) as *mut TValue;
        let mut t: libc::c_int = LoadByte(S) as libc::c_int;
        match t {
            0 => (*o).tt_ = 0i32,
            1 => {
                io = o;
                (*io).value_.b = LoadByte(S) as libc::c_int;
                (*io).tt_ = 1i32
            }
            3 => {
                io_0 = o;
                (*io_0).value_.n = LoadNumber(S);
                (*io_0).tt_ = 3i32 | 0i32 << 4i32
            }
            19 => {
                io_1 = o;
                (*io_1).value_.i = LoadInteger(S);
                (*io_1).tt_ = 3i32 | 1i32 << 4i32
            }
            4 | 20 => {
                let mut io_2: *mut TValue = o;
                let mut x_: *mut TString = LoadString(S);
                (*io_2).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
                (*io_2).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32
            }
            _ => {}
        }
        i += 1
    }
}
unsafe extern "C" fn LoadInteger(mut S: *mut LoadState) -> lua_Integer {
    let mut x: lua_Integer = 0;
    LoadBlock(
        S,
        &mut x as *mut lua_Integer as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Integer>() as libc::c_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadNumber(mut S: *mut LoadState) -> lua_Number {
    let mut x: lua_Number = 0.;
    LoadBlock(
        S,
        &mut x as *mut lua_Number as *mut libc::c_void,
        (1i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<lua_Number>() as libc::c_ulong),
    );
    return x;
}
unsafe extern "C" fn LoadCode(mut S: *mut LoadState, mut f: *mut Proto) -> () {
    let mut n: libc::c_int = LoadInt(S);
    if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (n as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t))
                .wrapping_div(::std::mem::size_of::<Instruction>() as libc::c_ulong)
    {
        luaM_toobig((*S).L);
    } else {
    };
    (*f).code = luaM_realloc_(
        (*S).L,
        0 as *mut libc::c_void,
        (0i32 as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
    ) as *mut Instruction;
    (*f).sizecode = n;
    LoadBlock(
        S,
        (*f).code as *mut libc::c_void,
        (n as libc::c_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as libc::c_ulong),
    );
}
unsafe extern "C" fn checkHeader(mut S: *mut LoadState) -> () {
    /* 1st char already checked */
    checkliteral(
        S,
        (b"\x1bLua\x00" as *const u8 as *const libc::c_char).offset(1isize),
        b"not a\x00" as *const u8 as *const libc::c_char,
    );
    if LoadByte(S) as libc::c_int
        != ((*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"5\x00"))[0usize]
            as libc::c_int
            - '0' as i32)
            * 16i32
            + ((*::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"3\x00"))[0usize]
                as libc::c_int
                - '0' as i32)
    {
        error(
            S,
            b"version mismatch in\x00" as *const u8 as *const libc::c_char,
        );
    } else if LoadByte(S) as libc::c_int != 0i32 {
        error(
            S,
            b"format mismatch in\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        checkliteral(
            S,
            b"\x19\x93\r\n\x1a\n\x00" as *const u8 as *const libc::c_char,
            b"corrupted\x00" as *const u8 as *const libc::c_char,
        );
        fchecksize(
            S,
            ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
            b"int\x00" as *const u8 as *const libc::c_char,
        );
        fchecksize(
            S,
            ::std::mem::size_of::<size_t>() as libc::c_ulong,
            b"size_t\x00" as *const u8 as *const libc::c_char,
        );
        fchecksize(
            S,
            ::std::mem::size_of::<Instruction>() as libc::c_ulong,
            b"Instruction\x00" as *const u8 as *const libc::c_char,
        );
        fchecksize(
            S,
            ::std::mem::size_of::<lua_Integer>() as libc::c_ulong,
            b"lua_Integer\x00" as *const u8 as *const libc::c_char,
        );
        fchecksize(
            S,
            ::std::mem::size_of::<lua_Number>() as libc::c_ulong,
            b"lua_Number\x00" as *const u8 as *const libc::c_char,
        );
        if LoadInteger(S) != 0x5678i32 as libc::c_longlong {
            error(
                S,
                b"endianness mismatch in\x00" as *const u8 as *const libc::c_char,
            );
        } else if LoadNumber(S) != 370.5f64 {
            error(
                S,
                b"float format mismatch in\x00" as *const u8 as *const libc::c_char,
            );
        } else {
            return;
        }
    };
}
unsafe extern "C" fn fchecksize(
    mut S: *mut LoadState,
    mut size: size_t,
    mut tname: *const libc::c_char,
) -> () {
    if LoadByte(S) as libc::c_ulong != size {
        error(
            S,
            luaO_pushfstring!(
                (*S).L,
                b"%s size mismatch in\x00" as *const u8 as *const libc::c_char,
                tname,
            ),
        );
    } else {
        return;
    };
}
unsafe extern "C" fn checkliteral(
    mut S: *mut LoadState,
    mut s: *const libc::c_char,
    mut msg: *const libc::c_char,
) -> () {
    /* larger than both */
    let mut buff: [libc::c_char; 12] = [0; 12];
    let mut len: size_t = strlen(s);
    LoadBlock(
        S,
        buff.as_mut_ptr() as *mut libc::c_void,
        len.wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong),
    );
    if memcmp(
        s as *const libc::c_void,
        buff.as_mut_ptr() as *const libc::c_void,
        len,
    ) != 0i32
    {
        error(S, msg);
    } else {
        return;
    };
}
