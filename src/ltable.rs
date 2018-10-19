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
    #[no_mangle]
    fn frexp(_: libc::c_double, _: *mut libc::c_int) -> libc::c_double;
    /*
     ** 'module' operation for hashing (size is always a power of 2)
     */
    /*
     ** (address of) a fixed nil value
     */
    #[no_mangle]
    static luaO_nilobject_: TValue;
    #[no_mangle]
    fn luaO_ceillog2(x: libc::c_uint) -> libc::c_int;
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
    fn luaG_runerror(L: *mut lua_State, fmt: *const libc::c_char, ...) -> !;
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: libc::c_int) -> !;
    #[no_mangle]
    fn luaD_rawrunprotected(L: *mut lua_State, f: Pfunc, ud: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn luaC_newobj(L: *mut lua_State, tt: libc::c_int, sz: size_t) -> *mut GCObject;
    #[no_mangle]
    fn luaC_barrierback_(L: *mut lua_State, o: *mut Table) -> ();
    #[no_mangle]
    fn luaS_hashlongstr(ts: *mut TString) -> libc::c_uint;
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
    fn luaV_tointeger(obj: *const TValue, p: *mut lua_Integer, mode: libc::c_int) -> libc::c_int;
}
pub type __sig_atomic_t = libc::c_int;
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
/* unsigned integer type */
pub type lua_Unsigned = libc::c_ulonglong;
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
pub type Pfunc = Option<unsafe extern "C" fn(_: *mut lua_State, _: *mut libc::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxsetnodeT {
    pub t: *mut Table,
    pub nhsize: libc::c_uint,
}
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
pub unsafe extern "C" fn luaH_getint(mut t: *mut Table, mut key: lua_Integer) -> *const TValue {
    /* (1 <= key && key <= t->sizearray) */
    if (key as lua_Unsigned).wrapping_sub(1i32 as libc::c_ulonglong)
        < (*t).sizearray as libc::c_ulonglong
    {
        return &mut *(*t).array.offset((key - 1i32 as libc::c_longlong) as isize) as *mut TValue;
    } else {
        let mut n: *mut Node = &mut *(*t).node.offset(
            (key & ((1i32 << (*t).lsizenode as libc::c_int) - 1i32) as libc::c_longlong)
                as libc::c_int as isize,
        ) as *mut Node;
        loop {
            /* check whether 'key' is somewhere in the chain */
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 3i32 | 1i32 << 4i32
                && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .i
                    == key
            {
                /* that's it */
                return &mut (*n).i_val;
            } else {
                let mut nx: libc::c_int = (*n).i_key.nk.next;
                if nx == 0i32 {
                    break;
                }
                n = n.offset(nx as isize)
            }
        }
        return &luaO_nilobject_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setint(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: lua_Integer,
    mut value: *mut TValue,
) -> () {
    let mut p: *const TValue = luaH_getint(t, key);
    let mut cell: *mut TValue = 0 as *mut TValue;
    if p != &luaO_nilobject_ as *const TValue {
        cell = p as *mut TValue
    } else {
        let mut k: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut k;
        (*io).value_.i = key;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        cell = luaH_newkey(L, t, &mut k)
    }
    *cell = *value;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_newkey(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut mp: *mut Node = 0 as *mut Node;
    let mut aux: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    if (*key).tt_ == 0i32 {
        luaG_runerror(
            L,
            b"table index is nil\x00" as *const u8 as *const libc::c_char,
        );
    } else {
        if (*key).tt_ == 3i32 | 0i32 << 4i32 {
            let mut k: lua_Integer = 0;
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                /* does index fit in an integer? */
                let mut io: *mut TValue = &mut aux;
                (*io).value_.i = k;
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                /* insert it as an integer */
                key = &mut aux
            } else if !((*key).value_.n == (*key).value_.n) {
                luaG_runerror(
                    L,
                    b"table index is NaN\x00" as *const u8 as *const libc::c_char,
                );
            }
        }
        mp = mainposition(t, key);
        if !((*mp).i_val.tt_ == 0i32) || (*t).lastfree.is_null() {
            /* main position is taken? */
            let mut othern: *mut Node = 0 as *mut Node;
            /* get a free place */
            let mut f: *mut Node = getfreepos(t);
            if f.is_null() {
                /* cannot find a free place? */
                /* grow table */
                rehash(L, t, key);
                /* whatever called 'newkey' takes care of TM cache */
                /* insert key into grown table */
                return luaH_set(L, t, key);
            } else {
                othern = mainposition(t, &mut (*mp).i_key.tvk as *mut TValue as *const TValue);
                if othern != mp {
                    /* is colliding node out of its main position? */
                    /* yes; move colliding node into free position */
                    /* find previous */
                    while othern.offset((*othern).i_key.nk.next as isize) != mp {
                        othern = othern.offset((*othern).i_key.nk.next as isize)
                    }
                    /* rechain to point to 'f' */
                    (*othern).i_key.nk.next =
                        f.wrapping_offset_from(othern) as libc::c_long as libc::c_int;
                    /* copy colliding node into free pos. (mp->next also goes) */
                    *f = *mp;
                    if (*mp).i_key.nk.next != 0i32 {
                        /* correct 'next' */
                        (*f).i_key.nk.next +=
                            mp.wrapping_offset_from(f) as libc::c_long as libc::c_int;
                        /* now 'mp' is free */
                        (*mp).i_key.nk.next = 0i32
                    }
                    (*mp).i_val.tt_ = 0i32
                } else {
                    /* colliding node is in its own main position */
                    /* new node will go into free position */
                    if (*mp).i_key.nk.next != 0i32 {
                        /* chain new position */
                        (*f).i_key.nk.next =
                            mp.offset((*mp).i_key.nk.next as isize)
                                .wrapping_offset_from(f) as libc::c_long
                                as libc::c_int
                    }
                    (*mp).i_key.nk.next = f.wrapping_offset_from(mp) as libc::c_long as libc::c_int;
                    mp = f
                }
            }
        }
        let mut k_: *mut TKey = &mut (*mp).i_key;
        let mut io_: *const TValue = key;
        (*k_).nk.value_ = (*io_).value_;
        (*k_).nk.tt_ = (*io_).tt_;
        if 0 != (*key).tt_ & 1i32 << 6i32
            && 0 != (*t).marked as libc::c_int & 1i32 << 2i32
            && 0 != (*(*key).value_.gc).marked as libc::c_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            luaC_barrierback_(L, t);
        } else {
        };
        return &mut (*mp).i_val;
    };
}
unsafe extern "C" fn getfreepos(mut t: *mut Table) -> *mut Node {
    if !(*t).lastfree.is_null() {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = (*t).lastfree.offset(-1isize);
            if !((*(&mut (*(*t).lastfree).i_key.tvk as *mut TValue as *const TValue)).tt_ == 0i32) {
                continue;
            }
            return (*t).lastfree;
        }
    }
    /* could not find a free place */
    return 0 as *mut Node;
}
/*
** returns the 'main' position of an element in a table (that is, the index
** of its hash value)
*/
unsafe extern "C" fn mainposition(mut t: *const Table, mut key: *const TValue) -> *mut Node {
    match (*key).tt_ & 0x3fi32 {
        19 => {
            return &mut *(*t).node.offset(
                ((*key).value_.i
                    & ((1i32 << (*t).lsizenode as libc::c_int) - 1i32) as libc::c_longlong)
                    as libc::c_int as isize,
            ) as *mut Node
        }
        3 => {
            return &mut *(*t).node.offset(
                (l_hashfloat((*key).value_.n)
                    % ((1i32 << (*t).lsizenode as libc::c_int) - 1i32 | 1i32))
                    as isize,
            ) as *mut Node
        }
        4 => {
            return &mut *(*t).node.offset(
                ((*((*key).value_.gc as *mut GCUnion)).ts.hash
                    & ((1i32 << (*t).lsizenode as libc::c_int) - 1i32) as libc::c_uint)
                    as libc::c_int as isize,
            ) as *mut Node
        }
        20 => {
            return &mut *(*t).node.offset(
                (luaS_hashlongstr(&mut (*((*key).value_.gc as *mut GCUnion)).ts)
                    & ((1i32 << (*t).lsizenode as libc::c_int) - 1i32) as libc::c_uint)
                    as libc::c_int as isize,
            ) as *mut Node
        }
        1 => {
            return &mut *(*t)
                .node
                .offset(((*key).value_.b & (1i32 << (*t).lsizenode as libc::c_int) - 1i32) as isize)
                as *mut Node
        }
        2 => {
            return &mut *(*t).node.offset(
                (((*key).value_.p as size_t
                    & (2147483647i32 as libc::c_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as libc::c_ulong) as libc::c_uint)
                    .wrapping_rem(
                        ((1i32 << (*t).lsizenode as libc::c_int) - 1i32 | 1i32) as libc::c_uint,
                    ) as isize,
            ) as *mut Node
        }
        22 => {
            return &mut *(*t).node.offset(
                // ((::std::mem::transmute::<lua_CFunction, size_t>((*key).value_.f)
                (((*key).value_.f.unwrap() as size_t
                    & (2147483647i32 as libc::c_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as libc::c_ulong) as libc::c_uint)
                    .wrapping_rem(
                        ((1i32 << (*t).lsizenode as libc::c_int) - 1i32 | 1i32) as libc::c_uint,
                    ) as isize,
            ) as *mut Node;
        }
        _ => {
            return &mut *(*t).node.offset(
                (((*key).value_.gc as size_t
                    & (2147483647i32 as libc::c_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as libc::c_ulong) as libc::c_uint)
                    .wrapping_rem(
                        ((1i32 << (*t).lsizenode as libc::c_int) - 1i32 | 1i32) as libc::c_uint,
                    ) as isize,
            ) as *mut Node
        }
    };
}
/* value */
/* key */
/*
** Hash for floating-point numbers.
** The main computation should be just
**     n = frexp(n, &i); return (n * INT_MAX) + i
** but there are some numerical subtleties.
** In a two-complement representation, INT_MAX does not has an exact
** representation as a float, but INT_MIN does; because the absolute
** value of 'frexp' is smaller than 1 (unless 'n' is inf/NaN), the
** absolute value of the product 'frexp * -INT_MIN' is smaller or equal
** to INT_MAX. Next, the use of 'unsigned int' avoids overflows when
** adding 'i'; the use of '~u' (instead of '-u') avoids problems with
** INT_MIN.
*/
unsafe extern "C" fn l_hashfloat(mut n: lua_Number) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut ni: lua_Integer = 0;
    n = frexp(n, &mut i) * -((-2147483647i32 - 1i32) as lua_Number);
    if !(n >= (-9223372036854775807i64 - 1i64) as libc::c_double
        && n < -((-9223372036854775807i64 - 1i64) as libc::c_double)
        && {
            ni = n as libc::c_longlong;
            0 != 1i32
        }) {
        /* is 'n' inf/-inf/NaN? */
        return 0i32;
    } else {
        /* normal case */
        let mut u: libc::c_uint = (i as libc::c_uint).wrapping_add(ni as libc::c_uint);
        return (if u <= 2147483647i32 as libc::c_uint {
            u
        } else {
            !u
        }) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_set(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut p: *const TValue = luaH_get(t, key);
    if p != &luaO_nilobject_ as *const TValue {
        return p as *mut TValue;
    } else {
        return luaH_newkey(L, t, key);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_get(mut t: *mut Table, mut key: *const TValue) -> *const TValue {
    match (*key).tt_ & 0x3fi32 {
        4 => return luaH_getshortstr(t, &mut (*((*key).value_.gc as *mut GCUnion)).ts),
        19 => return luaH_getint(t, (*key).value_.i),
        0 => return &luaO_nilobject_,
        3 => {
            let mut k: lua_Integer = 0;
            /* index is int? */
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                /* use specialized version */
                return luaH_getint(t, k);
            }
        }
        _ => {}
    }
    /* else... */
    /* FALLTHROUGH */
    return getgeneric(t, key);
}
/*
** "Generic" get version. (Not that generic: not valid for integers,
** which may be in array part, nor for floats with integral values.)
*/
unsafe extern "C" fn getgeneric(mut t: *mut Table, mut key: *const TValue) -> *const TValue {
    let mut n: *mut Node = mainposition(t, key);
    loop {
        /* check whether 'key' is somewhere in the chain */
        if 0 != luaV_equalobj(
            0 as *mut lua_State,
            &mut (*n).i_key.tvk as *mut TValue as *const TValue,
            key,
        ) {
            /* that's it */
            return &mut (*n).i_val;
        } else {
            let mut nx: libc::c_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                /* not found */
                return &luaO_nilobject_;
            } else {
                n = n.offset(nx as isize)
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getshortstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    let mut n: *mut Node = &mut *(*t).node.offset(
        ((*key).hash & ((1i32 << (*t).lsizenode as libc::c_int) - 1i32) as libc::c_uint)
            as libc::c_int as isize,
    ) as *mut Node;
    loop {
        /* check whether 'key' is somewhere in the chain */
        let mut k: *const TValue = &mut (*n).i_key.tvk as *mut TValue as *const TValue;
        if (*k).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32
            && &mut (*((*k).value_.gc as *mut GCUnion)).ts as *mut TString == key
        {
            /* that's it */
            return &mut (*n).i_val;
        } else {
            let mut nx: libc::c_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                /* not found */
                return &luaO_nilobject_;
            } else {
                n = n.offset(nx as isize)
            }
        }
    }
}
/*
** nums[i] = number of keys 'k' where 2^(i - 1) < k <= 2^i
*/
unsafe extern "C" fn rehash(mut L: *mut lua_State, mut t: *mut Table, mut ek: *const TValue) -> () {
    /* optimal size for array part */
    let mut asize: libc::c_uint = 0;
    /* number of keys in the array part */
    let mut na: libc::c_uint = 0;
    let mut nums: [libc::c_uint; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    let mut totaluse: libc::c_int = 0;
    i = 0i32;
    while i
        <= (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
    {
        /* reset counts */
        nums[i as usize] = 0i32 as libc::c_uint;
        i += 1
    }
    /* count keys in array part */
    na = numusearray(t, nums.as_mut_ptr());
    /* all those keys are integer keys */
    totaluse = na as libc::c_int;
    /* count keys in hash part */
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    /* count extra key */
    na = na.wrapping_add(countint(ek, nums.as_mut_ptr()) as libc::c_uint);
    totaluse += 1;
    /* compute new size for array part */
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    /* resize the table to new computed sizes */
    luaH_resize(L, t, asize, (totaluse as libc::c_uint).wrapping_sub(na));
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resize(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: libc::c_uint,
    mut nhsize: libc::c_uint,
) -> () {
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_int = 0;
    let mut asn: AuxsetnodeT = AuxsetnodeT {
        t: 0 as *mut Table,
        nhsize: 0,
    };
    let mut oldasize: libc::c_uint = (*t).sizearray;
    let mut oldhsize: libc::c_int = if (*t).lastfree.is_null() {
        0i32
    } else {
        1i32 << (*t).lsizenode as libc::c_int
    };
    /* save old hash ... */
    let mut nold: *mut Node = (*t).node;
    /* array part must grow? */
    if nasize > oldasize {
        setarrayvector(L, t, nasize);
    }
    /* create new hash part with appropriate size */
    asn.t = t;
    asn.nhsize = nhsize;
    if luaD_rawrunprotected(
        L,
        Some(auxsetnode),
        &mut asn as *mut AuxsetnodeT as *mut libc::c_void,
    ) != 0i32
    {
        /* mem. error? */
        /* array back to its original size */
        setarrayvector(L, t, oldasize);
        /* rethrow memory error */
        luaD_throw(L, 4i32);
    } else {
        if nasize < oldasize {
            /* array part must shrink? */
            (*t).sizearray = nasize;
            /* re-insert elements from vanishing slice */
            i = nasize;
            while i < oldasize {
                if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
                    luaH_setint(
                        L,
                        t,
                        i.wrapping_add(1i32 as libc::c_uint) as lua_Integer,
                        &mut *(*t).array.offset(i as isize),
                    );
                }
                i = i.wrapping_add(1)
            }
            /* shrink array */
            if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                >= ::std::mem::size_of::<size_t>() as libc::c_ulong
                && (nasize as size_t).wrapping_add(1i32 as libc::c_ulong)
                    > (!(0i32 as size_t))
                        .wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
            {
                luaM_toobig(L);
            } else {
            };
            (*t).array = luaM_realloc_(
                L,
                (*t).array as *mut libc::c_void,
                (oldasize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
                (nasize as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
            ) as *mut TValue
        }
        /* re-insert elements from hash part */
        j = oldhsize - 1i32;
        while j >= 0i32 {
            let mut old: *mut Node = nold.offset(j as isize);
            if !((*old).i_val.tt_ == 0i32) {
                /* doesn't need barrier/invalidate cache, as entry was
                already present in the table */
                let mut io1: *mut TValue =
                    luaH_set(L, t, &mut (*old).i_key.tvk as *mut TValue as *const TValue);
                *io1 = (*old).i_val
            }
            j -= 1
        }
        /* not the dummy node? */
        if oldhsize > 0i32 {
            /* free old hash */
            luaM_realloc_(
                L,
                nold as *mut libc::c_void,
                (oldhsize as size_t).wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
                0i32 as size_t,
            );
        }
        return;
    };
}
unsafe extern "C" fn setarrayvector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: libc::c_uint,
) -> () {
    let mut i: libc::c_uint = 0;
    if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
        >= ::std::mem::size_of::<size_t>() as libc::c_ulong
        && (size as size_t).wrapping_add(1i32 as libc::c_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as libc::c_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*t).array = luaM_realloc_(
        L,
        (*t).array as *mut libc::c_void,
        ((*t).sizearray as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        (size as libc::c_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
    ) as *mut TValue;
    i = (*t).sizearray;
    while i < size {
        (*(*t).array.offset(i as isize)).tt_ = 0i32;
        i = i.wrapping_add(1)
    }
    (*t).sizearray = size;
}
unsafe extern "C" fn auxsetnode(mut L: *mut lua_State, mut ud: *mut libc::c_void) -> () {
    let mut asn: *mut AuxsetnodeT = ud as *mut AuxsetnodeT;
    setnodevector(L, (*asn).t, (*asn).nhsize);
}
unsafe extern "C" fn setnodevector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: libc::c_uint,
) -> () {
    if size == 0i32 as libc::c_uint {
        /* no elements to hash part? */
        /* use common 'dummynode' */
        (*t).node = &dummynode_ as *const Node as *mut Node;
        (*t).lsizenode = 0i32 as lu_byte;
        /* signal that it is using dummy node */
        (*t).lastfree = 0 as *mut Node
    } else {
        let mut i: libc::c_int = 0;
        let mut lsize: libc::c_int = luaO_ceillog2(size);
        if lsize
            > (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(8i32 as libc::c_ulong)
                .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
                - 1i32
        {
            luaG_runerror(L, b"table overflow\x00" as *const u8 as *const libc::c_char);
        } else {
            size = (1i32 << lsize) as libc::c_uint;
            if ::std::mem::size_of::<libc::c_uint>() as libc::c_ulong
                >= ::std::mem::size_of::<size_t>() as libc::c_ulong
                && (size as size_t).wrapping_add(1i32 as libc::c_ulong)
                    > (!(0i32 as size_t))
                        .wrapping_div(::std::mem::size_of::<Node>() as libc::c_ulong)
            {
                luaM_toobig(L);
            } else {
            };
            (*t).node = luaM_realloc_(
                L,
                0 as *mut libc::c_void,
                (0i32 as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
                (size as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
            ) as *mut Node;
            i = 0i32;
            while i < size as libc::c_int {
                let mut n: *mut Node = &mut *(*t).node.offset(i as isize) as *mut Node;
                (*n).i_key.nk.next = 0i32;
                (*n).i_key.nk.tt_ = 0i32;
                (*n).i_val.tt_ = 0i32;
                i += 1
            }
            (*t).lsizenode = lsize as lu_byte;
            /* all positions are free */
            (*t).lastfree = &mut *(*t).node.offset(size as isize) as *mut Node
        }
    };
}
/*
** $Id: ltable.c,v 2.118.1.4 2018/06/08 16:22:51 roberto Exp $
** Lua tables (hash)
** See Copyright Notice in lua.h
*/
/*
** Implementation of tables (aka arrays, objects, or hash tables).
** Tables keep its elements in two parts: an array part and a hash part.
** Non-negative integer keys are all candidates to be kept in the array
** part. The actual size of the array is the largest 'n' such that
** more than half the slots between 1 and n are in use.
** Hash uses a mix of chained scatter table with Brent's variation.
** A main invariant of these tables is that, if an element is not
** in its main position (i.e. the 'original' position that its hash gives
** to it), then the colliding element is in its own main position.
** Hence even when the load factor reaches 100%, performance remains good.
*/
/*
** Maximum size of array part (MAXASIZE) is 2^MAXABITS. MAXABITS is
** the largest integer such that MAXASIZE fits in an unsigned int.
*/
/*
** Maximum size of hash part is 2^MAXHBITS. MAXHBITS is the largest
** integer such that 2^MAXHBITS fits in a signed int. (Note that the
** maximum number of elements in a table, 2^MAXABITS + 2^MAXHBITS, still
** fits comfortably in an unsigned int.)
*/
/*
** for some types, it is better to avoid modulus by power of 2, as
** they tend to have many 2 factors.
*/
static mut dummynode_: Node = Node {
    i_val: lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0i32,
    },
    i_key: TKey {
        nk: unnamed_3 {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0i32,
            next: 0i32,
        },
    },
};
/*
** {=============================================================
** Rehash
** ==============================================================
*/
/*
** Compute the optimal size for the array part of table 't'. 'nums' is a
** "count array" where 'nums[i]' is the number of integers in the table
** between 2^(i - 1) + 1 and 2^i. 'pna' enters with the total number of
** integer keys in the table and leaves with the number of keys that
** will go to the array part; return the optimal size.
*/
unsafe extern "C" fn computesizes(
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_uint {
    let mut i: libc::c_int = 0;
    /* 2^i (candidate for optimal size) */
    let mut twotoi: libc::c_uint = 0;
    /* number of elements smaller than 2^i */
    let mut a: libc::c_uint = 0i32 as libc::c_uint;
    /* number of elements to go to array part */
    let mut na: libc::c_uint = 0i32 as libc::c_uint;
    /* optimal size for array part */
    let mut optimal: libc::c_uint = 0i32 as libc::c_uint;
    /* loop while keys can fill more than half of total size */
    i = 0i32;
    twotoi = 1i32 as libc::c_uint;
    while twotoi > 0i32 as libc::c_uint && *pna > twotoi.wrapping_div(2i32 as libc::c_uint) {
        if *nums.offset(i as isize) > 0i32 as libc::c_uint {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi.wrapping_div(2i32 as libc::c_uint) {
                /* more than half elements present? */
                /* optimal size (till now) */
                optimal = twotoi;
                /* all elements up to 'optimal' will go to array part */
                na = a
            }
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2i32 as libc::c_uint)
    }
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(mut key: *const TValue, mut nums: *mut libc::c_uint) -> libc::c_int {
    let mut k: libc::c_uint = arrayindex(key);
    if k != 0i32 as libc::c_uint {
        /* is 'key' an appropriate array index? */
        /* count as such */
        let ref mut fresh0 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        return 1i32;
    } else {
        return 0i32;
    };
}
/*
** returns the index for 'key' if 'key' is an appropriate key to live in
** the array part of the table, 0 otherwise.
*/
unsafe extern "C" fn arrayindex(mut key: *const TValue) -> libc::c_uint {
    if (*key).tt_ == 3i32 | 1i32 << 4i32 {
        let mut k: lua_Integer = (*key).value_.i;
        if (0i32 as libc::c_longlong) < k
            && k as lua_Unsigned
                <= (1u32
                    << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                        .wrapping_mul(8i32 as libc::c_ulong)
                        .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int)
                    as libc::c_ulonglong
        {
            /* 'key' is an appropriate array index */
            return k as libc::c_uint;
        }
    }
    /* 'key' did not match some condition */
    return 0i32 as libc::c_uint;
}
unsafe extern "C" fn numusehash(
    mut t: *const Table,
    mut nums: *mut libc::c_uint,
    mut pna: *mut libc::c_uint,
) -> libc::c_int {
    /* total number of elements */
    let mut totaluse: libc::c_int = 0i32;
    /* elements added to 'nums' (can go to array part) */
    let mut ause: libc::c_int = 0i32;
    let mut i: libc::c_int = 1i32 << (*t).lsizenode as libc::c_int;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(0 != fresh1) {
            break;
        }
        let mut n: *mut Node = &mut *(*t).node.offset(i as isize) as *mut Node;
        if (*n).i_val.tt_ == 0i32 {
            continue;
        }
        ause += countint(&mut (*n).i_key.tvk as *mut TValue as *const TValue, nums);
        totaluse += 1
    }
    *pna = (*pna).wrapping_add(ause as libc::c_uint);
    return totaluse;
}
/*
** Count keys in array part of table 't': Fill 'nums[i]' with
** number of keys that will go into corresponding slice and return
** total number of non-nil keys.
*/
unsafe extern "C" fn numusearray(mut t: *const Table, mut nums: *mut libc::c_uint) -> libc::c_uint {
    let mut lg: libc::c_int = 0;
    /* 2^lg */
    let mut ttlg: libc::c_uint = 0;
    /* summation of 'nums' */
    let mut ause: libc::c_uint = 0i32 as libc::c_uint;
    /* count to traverse all array keys */
    let mut i: libc::c_uint = 1i32 as libc::c_uint;
    /* traverse each slice */
    lg = 0i32;
    ttlg = 1i32 as libc::c_uint;
    while lg
        <= (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(8i32 as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong) as libc::c_int
    {
        /* counter */
        let mut lc: libc::c_uint = 0i32 as libc::c_uint;
        let mut lim: libc::c_uint = ttlg;
        if lim > (*t).sizearray {
            /* adjust upper limit */
            lim = (*t).sizearray;
            if i > lim {
                /* no more elements to count */
                break;
            }
        }
        /* count elements in range (2^(lg - 1), 2^lg] */
        while i <= lim {
            if !((*(*t)
                .array
                .offset(i.wrapping_sub(1i32 as libc::c_uint) as isize))
            .tt_ == 0i32)
            {
                lc = lc.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        let ref mut fresh2 = *nums.offset(lg as isize);
        *fresh2 = (*fresh2).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        ttlg = ttlg.wrapping_mul(2i32 as libc::c_uint)
    }
    return ause;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(mut t: *mut Table, mut key: *mut TString) -> *const TValue {
    if (*key).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
        return luaH_getshortstr(t, key);
    } else {
        /* for long strings, use generic case */
        let mut ko: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut ko;
        let mut x_: *mut TString = key;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as libc::c_int | 1i32 << 6i32;
        return getgeneric(t, &mut ko);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State) -> *mut Table {
    let mut o: *mut GCObject =
        luaC_newobj(L, 5i32, ::std::mem::size_of::<Table>() as libc::c_ulong);
    let mut t: *mut Table = &mut (*(o as *mut GCUnion)).h;
    (*t).metatable = 0 as *mut Table;
    (*t).flags = !0i32 as lu_byte;
    (*t).array = 0 as *mut TValue;
    (*t).sizearray = 0i32 as libc::c_uint;
    setnodevector(L, t, 0i32 as libc::c_uint);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resizearray(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: libc::c_uint,
) -> () {
    let mut nsize: libc::c_int = if (*t).lastfree.is_null() {
        0i32
    } else {
        1i32 << (*t).lsizenode as libc::c_int
    };
    luaH_resize(L, t, nasize, nsize as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State, mut t: *mut Table) -> () {
    if !(*t).lastfree.is_null() {
        luaM_realloc_(
            L,
            (*t).node as *mut libc::c_void,
            ((1i32 << (*t).lsizenode as libc::c_int) as size_t)
                .wrapping_mul(::std::mem::size_of::<Node>() as libc::c_ulong),
            0i32 as size_t,
        );
    }
    luaM_realloc_(
        L,
        (*t).array as *mut libc::c_void,
        ((*t).sizearray as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<TValue>() as libc::c_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        t as *mut libc::c_void,
        ::std::mem::size_of::<Table>() as libc::c_ulong,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaH_next(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> libc::c_int {
    /* find original element */
    let mut i: libc::c_uint = findindex(L, t, key);
    while i < (*t).sizearray {
        /* try first array part */
        if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
            /* a non-nil value? */
            let mut io: *mut TValue = key;
            (*io).value_.i = i.wrapping_add(1i32 as libc::c_uint) as lua_Integer;
            (*io).tt_ = 3i32 | 1i32 << 4i32;
            let mut io1: *mut TValue = key.offset(1isize);
            *io1 = *(*t).array.offset(i as isize);
            return 1i32;
        } else {
            i = i.wrapping_add(1)
        }
    }
    i = i.wrapping_sub((*t).sizearray);
    while (i as libc::c_int) < 1i32 << (*t).lsizenode as libc::c_int {
        /* hash part */
        if !((*(*t).node.offset(i as isize)).i_val.tt_ == 0i32) {
            /* a non-nil value? */
            let mut io1_0: *mut TValue = key;
            *io1_0 =
                *(&mut (*(*t).node.offset(i as isize)).i_key.tvk as *mut TValue as *const TValue);
            let mut io1_1: *mut TValue = key.offset(1isize);
            *io1_1 = (*(*t).node.offset(i as isize)).i_val;
            return 1i32;
        } else {
            i = i.wrapping_add(1)
        }
    }
    /* no more elements */
    return 0i32;
}
/*
** returns the index of a 'key' for table traversals. First goes all
** elements in the array part, then elements in the hash part. The
** beginning of a traversal is signaled by 0.
*/
unsafe extern "C" fn findindex(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    if (*key).tt_ == 0i32 {
        /* first iteration */
        return 0i32 as libc::c_uint;
    } else {
        i = arrayindex(key as *const TValue);
        /* is 'key' inside array part? */
        if i != 0i32 as libc::c_uint && i <= (*t).sizearray {
            /* yes; that's the index */
            return i;
        } else {
            let mut nx: libc::c_int = 0;
            let mut n: *mut Node = mainposition(t, key as *const TValue);
            loop {
                /* check whether 'key' is somewhere in the chain */
                /* key may be dead already, but it is ok to use it in 'next' */
                if 0 != luaV_equalobj(
                    0 as *mut lua_State,
                    &mut (*n).i_key.tvk as *mut TValue as *const TValue,
                    key as *const TValue,
                ) || (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 9i32 + 1i32
                    && 0 != (*key).tt_ & 1i32 << 6i32
                    && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc as *mut libc::c_void
                        == (*key).value_.gc as *mut libc::c_void
                {
                    /* key index in hash table */
                    i = n.wrapping_offset_from(&mut *(*t).node.offset(0isize) as *mut Node)
                        as libc::c_long as libc::c_int as libc::c_uint;
                    /* hash elements are numbered after array ones */
                    return i
                        .wrapping_add(1i32 as libc::c_uint)
                        .wrapping_add((*t).sizearray);
                } else {
                    nx = (*n).i_key.nk.next;
                    if nx == 0i32 {
                        /* key not found */
                        luaG_runerror(
                            L,
                            b"invalid key to \'next\'\x00" as *const u8 as *const libc::c_char,
                        );
                    } else {
                        n = n.offset(nx as isize)
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> lua_Unsigned {
    let mut j: libc::c_uint = (*t).sizearray;
    if j > 0i32 as libc::c_uint
        && (*(*t)
            .array
            .offset(j.wrapping_sub(1i32 as libc::c_uint) as isize))
        .tt_ == 0i32
    {
        /* there is a boundary in the array part: (binary) search for it */
        let mut i: libc::c_uint = 0i32 as libc::c_uint;
        while j.wrapping_sub(i) > 1i32 as libc::c_uint {
            let mut m: libc::c_uint = i.wrapping_add(j).wrapping_div(2i32 as libc::c_uint);
            if (*(*t)
                .array
                .offset(m.wrapping_sub(1i32 as libc::c_uint) as isize))
            .tt_ == 0i32
            {
                j = m
            } else {
                i = m
            }
        }
        return i as lua_Unsigned;
    } else if (*t).lastfree.is_null() {
        /* that is easy... */
        return j as lua_Unsigned;
    } else {
        return unbound_search(t, j as lua_Unsigned);
    };
}
unsafe extern "C" fn unbound_search(mut t: *mut Table, mut j: lua_Unsigned) -> lua_Unsigned {
    /* i is zero or a present index */
    let mut i: lua_Unsigned = j;
    j = j.wrapping_add(1);
    /* find 'i' and 'j' such that i is present and j is not */
    while !((*luaH_getint(t, j as lua_Integer)).tt_ == 0i32) {
        i = j;
        if j > (9223372036854775807i64 as lua_Unsigned).wrapping_div(2i32 as libc::c_ulonglong) {
            /* overflow? */
            /* table was built with bad purposes: resort to linear search */
            i = 1i32 as lua_Unsigned;
            while !((*luaH_getint(t, i as lua_Integer)).tt_ == 0i32) {
                i = i.wrapping_add(1)
            }
            return i.wrapping_sub(1i32 as libc::c_ulonglong);
        } else {
            j = (j as libc::c_ulonglong).wrapping_mul(2i32 as libc::c_ulonglong) as lua_Unsigned
                as lua_Unsigned
        }
    }
    /* now do a binary search between them */
    while j.wrapping_sub(i) > 1i32 as libc::c_ulonglong {
        let mut m: lua_Unsigned = i.wrapping_add(j).wrapping_div(2i32 as libc::c_ulonglong);
        if (*luaH_getint(t, m as lua_Integer)).tt_ == 0i32 {
            j = m
        } else {
            i = m
        }
    }
    return i;
}
