use libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut stdout: *mut FILE;
    #[no_mangle]
    static mut stderr: *mut FILE;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    #[no_mangle]
    fn lua_close(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: libc::c_int, isnum: *mut libc::c_int) -> lua_Integer;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: libc::c_int, len: *mut size_t) -> *const libc::c_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: libc::c_int) -> *mut libc::c_void;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut libc::c_void) -> ();
    #[no_mangle]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> libc::c_int;
    #[no_mangle]
    fn lua_load(
        L: *mut lua_State,
        reader_0: lua_Reader,
        dt: *mut libc::c_void,
        chunkname: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const libc::c_char,
        mode: *const libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn luaL_newstate() -> *mut lua_State;
    /* dump one chunk; from ldump.c */
    #[no_mangle]
    fn luaU_dump(
        L: *mut lua_State,
        f: *const Proto,
        w: lua_Writer,
        data: *mut libc::c_void,
        strip: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    /* opcode names */
    #[no_mangle]
    static luaP_opnames: [*const libc::c_char; 48];
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type unnamed = libc::c_uint;
pub const _ISalnum: unnamed = 8;
pub const _ISpunct: unnamed = 4;
pub const _IScntrl: unnamed = 2;
pub const _ISblank: unnamed = 1;
pub const _ISgraph: unnamed = 32768;
pub const _ISprint: unnamed = 16384;
pub const _ISspace: unnamed = 8192;
pub const _ISxdigit: unnamed = 4096;
pub const _ISdigit: unnamed = 2048;
pub const _ISalpha: unnamed = 1024;
pub const _ISlower: unnamed = 512;
pub const _ISupper: unnamed = 256;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
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
    pub u: unnamed_0,
    pub extra: ptrdiff_t,
    pub nresults: libc::c_short,
    pub callstatus: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_0 {
    pub l: unnamed_2,
    pub c: unnamed_1,
}
/* only for C functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
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
pub struct unnamed_2 {
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
    pub u: unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_3 {
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
    pub nk: unnamed_4,
    pub tvk: TValue,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_4 {
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
pub type lua_Writer = Option<
    unsafe extern "C" fn(
        _: *mut lua_State,
        _: *const libc::c_void,
        _: size_t,
        _: *mut libc::c_void,
    ) -> libc::c_int,
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
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
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
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
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
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
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
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
/* basic instruction format */
pub const iAx: OpMode = 3;
pub const iAsBx: OpMode = 2;
/* argument is used */
pub const OpArgU: OpArgMask = 1;
/*===========================================================================
  Notes:
  (*) In OP_CALL, if (B == 0) then B = top. If (C == 0), then 'top' is
  set to last_result+1, so next open instruction (OP_CALL, OP_RETURN,
  OP_SETLIST) may use 'top'.

  (*) In OP_VARARG, if (B == 0) then use actual number of varargs and
  set top (like in OP_CALL with C == 0).

  (*) In OP_RETURN, if (B == 0) then return up to 'top'.

  (*) In OP_SETLIST, if (B == 0) then B = 'top'; if (C == 0) then next
  'instruction' is EXTRAARG(real C).

  (*) In OP_LOADKX, the next 'instruction' is always EXTRAARG.

  (*) For comparisons, A specifies what condition the test should accept
  (true or false).

  (*) All 'skips' (pc++) assume that next instruction is a jump.

===========================================================================*/
/*
** masks for instruction properties. The format is:
** bits 0-1: op mode
** bits 2-3: C arg mode
** bits 4-5: B arg mode
** bit 6: instruction set register A
** bit 7: operator is a test (next instruction must be a jump)
*/
pub type OpArgMask = libc::c_uint;
/* argument is a constant or register/constant */
pub const OpArgK: OpArgMask = 3;
/* argument is a register or a jump offset */
pub const OpArgR: OpArgMask = 2;
/* argument is not used */
pub const OpArgN: OpArgMask = 0;
pub const iABx: OpMode = 1;
pub const iABC: OpMode = 0;
/*
** $Id: lopcodes.h,v 1.149.1.1 2017/04/19 17:20:42 roberto Exp $
** Opcodes for Lua virtual machine
** See Copyright Notice in lua.h
*/
/*===========================================================================
  We assume that instructions are unsigned numbers.
  All instructions have an opcode in the first 6 bits.
  Instructions can have the following fields:
	'A' : 8 bits
	'B' : 9 bits
	'C' : 9 bits
	'Ax' : 26 bits ('A', 'B', and 'C' together)
	'Bx' : 18 bits ('B' and 'C' together)
	'sBx' : signed Bx

  A signed argument is represented in excess K; that is, the number
  value is the unsigned value minus K. K is exactly the maximum value
  for that argument (so that -max is represented by 0, and +max is
  represented by 2*max), which is half the maximum for the corresponding
  unsigned argument.
===========================================================================*/
pub type OpMode = libc::c_uint;
/*
** $Id: luac.c,v 1.76 2018/06/19 01:32:02 lhf Exp $
** Lua compiler (saves bytecodes to files; also lists bytecodes)
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn PrintFunction(mut f: *const Proto, mut full: libc::c_int) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizep;
    PrintHeader(f);
    PrintCode(f);
    if 0 != full {
        PrintDebug(f);
    }
    i = 0i32;
    while i < n {
        PrintFunction(*(*f).p.offset(i as isize), full);
        i += 1
    }
}
unsafe extern "C" fn PrintDebug(mut f: *const Proto) -> () {
    let mut i: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    n = (*f).sizek;
    printf(
        b"constants (%d) for %p:\n\x00" as *const u8 as *const libc::c_char,
        n,
        f as *const libc::c_void,
    );
    i = 0i32;
    while i < n {
        printf(b"\t%d\t\x00" as *const u8 as *const libc::c_char, i + 1i32);
        PrintConstant(f, i);
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        i += 1
    }
    n = (*f).sizelocvars;
    printf(
        b"locals (%d) for %p:\n\x00" as *const u8 as *const libc::c_char,
        n,
        f as *const libc::c_void,
    );
    i = 0i32;
    while i < n {
        printf(
            b"\t%d\t%s\t%d\t%d\n\x00" as *const u8 as *const libc::c_char,
            i,
            ((*(*f).locvars.offset(i as isize)).varname as *mut libc::c_char)
                .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize),
            (*(*f).locvars.offset(i as isize)).startpc + 1i32,
            (*(*f).locvars.offset(i as isize)).endpc + 1i32,
        );
        i += 1
    }
    n = (*f).sizeupvalues;
    printf(
        b"upvalues (%d) for %p:\n\x00" as *const u8 as *const libc::c_char,
        n,
        f as *const libc::c_void,
    );
    i = 0i32;
    while i < n {
        printf(
            b"\t%d\t%s\t%d\t%d\n\x00" as *const u8 as *const libc::c_char,
            i,
            if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
                ((*(*f).upvalues.offset(i as isize)).name as *mut libc::c_char)
                    .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
            } else {
                b"-\x00" as *const u8 as *const libc::c_char
            },
            (*(*f).upvalues.offset(i as isize)).instack as libc::c_int,
            (*(*f).upvalues.offset(i as isize)).idx as libc::c_int,
        );
        i += 1
    }
}
unsafe extern "C" fn PrintConstant(mut f: *const Proto, mut i: libc::c_int) -> () {
    let mut o: *const TValue = &mut *(*f).k.offset(i as isize) as *mut TValue;
    match (*o).tt_ & 0x3fi32 {
        0 => {
            printf(b"nil\x00" as *const u8 as *const libc::c_char);
        }
        1 => {
            printf(if 0 != (*o).value_.b {
                b"true\x00" as *const u8 as *const libc::c_char
            } else {
                b"false\x00" as *const u8 as *const libc::c_char
            });
        }
        3 => {
            let mut buff: [libc::c_char; 100] = [0; 100];
            sprintf(
                buff.as_mut_ptr(),
                b"%.14g\x00" as *const u8 as *const libc::c_char,
                (*o).value_.n,
            );
            printf(
                b"%s\x00" as *const u8 as *const libc::c_char,
                buff.as_mut_ptr(),
            );
            if buff[strspn(
                buff.as_mut_ptr(),
                b"-0123456789\x00" as *const u8 as *const libc::c_char,
            ) as usize] as libc::c_int
                == '\u{0}' as i32
            {
                printf(b".0\x00" as *const u8 as *const libc::c_char);
            }
        }
        19 => {
            printf(
                b"%lld\x00" as *const u8 as *const libc::c_char,
                (*o).value_.i,
            );
        }
        4 | 20 => {
            PrintString(&mut (*((*o).value_.gc as *mut GCUnion)).ts);
        }
        _ => {
            printf(
                b"? type=%d\x00" as *const u8 as *const libc::c_char,
                (*o).tt_ & 0x3fi32,
            );
        }
    };
}
/*
** $Id: luac.c,v 1.76 2018/06/19 01:32:02 lhf Exp $
** print bytecodes
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn PrintString(mut ts: *const TString) -> () {
    let mut s: *const libc::c_char = (ts as *mut libc::c_char)
        .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize);
    let mut i: size_t = 0;
    let mut n: size_t = if (*ts).tt as libc::c_int == 4i32 | 0i32 << 4i32 {
        (*ts).shrlen as libc::c_ulong
    } else {
        (*ts).u.lnglen
    };
    printf(b"%c\x00" as *const u8 as *const libc::c_char, '\"' as i32);
    i = 0i32 as size_t;
    while i < n {
        let mut c: libc::c_int = *s.offset(i as isize) as libc::c_uchar as libc::c_int;
        match c {
            34 => {
                printf(b"\\\"\x00" as *const u8 as *const libc::c_char);
            }
            92 => {
                printf(b"\\\\\x00" as *const u8 as *const libc::c_char);
            }
            7 => {
                printf(b"\\a\x00" as *const u8 as *const libc::c_char);
            }
            8 => {
                printf(b"\\b\x00" as *const u8 as *const libc::c_char);
            }
            12 => {
                printf(b"\\f\x00" as *const u8 as *const libc::c_char);
            }
            10 => {
                printf(b"\\n\x00" as *const u8 as *const libc::c_char);
            }
            13 => {
                printf(b"\\r\x00" as *const u8 as *const libc::c_char);
            }
            9 => {
                printf(b"\\t\x00" as *const u8 as *const libc::c_char);
            }
            11 => {
                printf(b"\\v\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                if 0 != *(*__ctype_b_loc()).offset(c as isize) as libc::c_int
                    & _ISprint as libc::c_int as libc::c_ushort as libc::c_int
                {
                    printf(b"%c\x00" as *const u8 as *const libc::c_char, c);
                } else {
                    printf(b"\\%03d\x00" as *const u8 as *const libc::c_char, c);
                }
            }
        }
        i = i.wrapping_add(1)
    }
    printf(b"%c\x00" as *const u8 as *const libc::c_char, '\"' as i32);
}
unsafe extern "C" fn PrintCode(mut f: *const Proto) -> () {
    let mut code: *const Instruction = (*f).code;
    let mut pc: libc::c_int = 0;
    let mut n: libc::c_int = (*f).sizecode;
    pc = 0i32;
    while pc < n {
        let mut i: Instruction = *code.offset(pc as isize);
        let mut o: OpCode = (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
        let mut a: libc::c_int =
            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as libc::c_int;
        let mut b: libc::c_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
            & !((!(0i32 as Instruction)) << 9i32) << 0i32)
            as libc::c_int;
        let mut c: libc::c_int =
            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32) as libc::c_int;
        let mut ax: libc::c_int = (i >> 0i32 + 6i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
            as libc::c_int;
        let mut bx: libc::c_int = (i >> 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
            as libc::c_int;
        let mut sbx: libc::c_int = (i >> 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
            as libc::c_int
            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
        let mut line: libc::c_int = if !(*f).lineinfo.is_null() {
            *(*f).lineinfo.offset(pc as isize)
        } else {
            -1i32
        };
        printf(b"\t%d\t\x00" as *const u8 as *const libc::c_char, pc + 1i32);
        if line > 0i32 {
            printf(b"[%d]\t\x00" as *const u8 as *const libc::c_char, line);
        } else {
            printf(b"[-]\t\x00" as *const u8 as *const libc::c_char);
        }
        printf(
            b"%-9s\t\x00" as *const u8 as *const libc::c_char,
            luaP_opnames[o as usize],
        );
        match (luaP_opmodes[o as usize] as libc::c_int & 3i32) as OpMode as libc::c_uint {
            0 => {
                printf(b"%d\x00" as *const u8 as *const libc::c_char, a);
                if (luaP_opmodes[o as usize] as libc::c_int >> 4i32 & 3i32) as OpArgMask
                    as libc::c_uint
                    != OpArgN as libc::c_int as libc::c_uint
                {
                    printf(
                        b" %d\x00" as *const u8 as *const libc::c_char,
                        if 0 != b & 1i32 << 9i32 - 1i32 {
                            -1i32 - (b & !(1i32 << 9i32 - 1i32))
                        } else {
                            b
                        },
                    );
                }
                if (luaP_opmodes[o as usize] as libc::c_int >> 2i32 & 3i32) as OpArgMask
                    as libc::c_uint
                    != OpArgN as libc::c_int as libc::c_uint
                {
                    printf(
                        b" %d\x00" as *const u8 as *const libc::c_char,
                        if 0 != c & 1i32 << 9i32 - 1i32 {
                            -1i32 - (c & !(1i32 << 9i32 - 1i32))
                        } else {
                            c
                        },
                    );
                }
            }
            1 => {
                printf(b"%d\x00" as *const u8 as *const libc::c_char, a);
                if (luaP_opmodes[o as usize] as libc::c_int >> 4i32 & 3i32) as OpArgMask
                    as libc::c_uint
                    == OpArgK as libc::c_int as libc::c_uint
                {
                    printf(b" %d\x00" as *const u8 as *const libc::c_char, -1i32 - bx);
                }
                if (luaP_opmodes[o as usize] as libc::c_int >> 4i32 & 3i32) as OpArgMask
                    as libc::c_uint
                    == OpArgU as libc::c_int as libc::c_uint
                {
                    printf(b" %d\x00" as *const u8 as *const libc::c_char, bx);
                }
            }
            2 => {
                printf(b"%d %d\x00" as *const u8 as *const libc::c_char, a, sbx);
            }
            3 => {
                printf(b"%d\x00" as *const u8 as *const libc::c_char, -1i32 - ax);
            }
            _ => {}
        }
        match o as libc::c_uint {
            1 => {
                printf(b"\t; \x00" as *const u8 as *const libc::c_char);
                PrintConstant(f, bx);
            }
            5 | 9 => {
                printf(
                    b"\t; %s\x00" as *const u8 as *const libc::c_char,
                    if !(*(*f).upvalues.offset(b as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(b as isize)).name as *mut libc::c_char)
                            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    } else {
                        b"-\x00" as *const u8 as *const libc::c_char
                    },
                );
            }
            6 => {
                printf(
                    b"\t; %s\x00" as *const u8 as *const libc::c_char,
                    if !(*(*f).upvalues.offset(b as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(b as isize)).name as *mut libc::c_char)
                            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    } else {
                        b"-\x00" as *const u8 as *const libc::c_char
                    },
                );
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(b" \x00" as *const u8 as *const libc::c_char);
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            8 => {
                printf(
                    b"\t; %s\x00" as *const u8 as *const libc::c_char,
                    if !(*(*f).upvalues.offset(a as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(a as isize)).name as *mut libc::c_char)
                            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
                    } else {
                        b"-\x00" as *const u8 as *const libc::c_char
                    },
                );
                if 0 != b & 1i32 << 9i32 - 1i32 {
                    printf(b" \x00" as *const u8 as *const libc::c_char);
                    PrintConstant(f, b & !(1i32 << 9i32 - 1i32));
                }
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(b" \x00" as *const u8 as *const libc::c_char);
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            7 | 12 => {
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(b"\t; \x00" as *const u8 as *const libc::c_char);
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            10 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 31 | 32 | 33 => {
                if 0 != b & 1i32 << 9i32 - 1i32 || 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(b"\t; \x00" as *const u8 as *const libc::c_char);
                    if 0 != b & 1i32 << 9i32 - 1i32 {
                        PrintConstant(f, b & !(1i32 << 9i32 - 1i32));
                    } else {
                        printf(b"-\x00" as *const u8 as *const libc::c_char);
                    }
                    printf(b" \x00" as *const u8 as *const libc::c_char);
                    if 0 != c & 1i32 << 9i32 - 1i32 {
                        PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                    } else {
                        printf(b"-\x00" as *const u8 as *const libc::c_char);
                    }
                }
            }
            30 | 39 | 40 | 42 => {
                printf(
                    b"\t; to %d\x00" as *const u8 as *const libc::c_char,
                    sbx + pc + 2i32,
                );
            }
            44 => {
                printf(
                    b"\t; %p\x00" as *const u8 as *const libc::c_char,
                    *(*f).p.offset(bx as isize) as *const libc::c_void,
                );
            }
            43 => {
                if c == 0i32 {
                    pc += 1;
                    printf(
                        b"\t; %d\x00" as *const u8 as *const libc::c_char,
                        *code.offset(pc as isize) as libc::c_int,
                    );
                } else {
                    printf(b"\t; %d\x00" as *const u8 as *const libc::c_char, c);
                }
            }
            46 => {
                printf(b"\t; \x00" as *const u8 as *const libc::c_char);
                PrintConstant(f, ax);
            }
            _ => {}
        }
        printf(b"\n\x00" as *const u8 as *const libc::c_char);
        pc += 1
    }
}
unsafe extern "C" fn PrintHeader(mut f: *const Proto) -> () {
    let mut s: *const libc::c_char = if !(*f).source.is_null() {
        ((*f).source as *mut libc::c_char)
            .offset(::std::mem::size_of::<UTString>() as libc::c_ulong as isize)
    } else {
        b"=?\x00" as *const u8 as *const libc::c_char
    };
    if *s as libc::c_int == '@' as i32 || *s as libc::c_int == '=' as i32 {
        s = s.offset(1isize)
    } else if *s as libc::c_int
        == (*::std::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b"\x1bLua\x00"))[0usize]
            as libc::c_int
    {
        s = b"(bstring)\x00" as *const u8 as *const libc::c_char
    } else {
        s = b"(string)\x00" as *const u8 as *const libc::c_char
    }
    printf(
        b"\n%s <%s:%d,%d> (%d instruction%s at %p)\n\x00" as *const u8 as *const libc::c_char,
        if (*f).linedefined == 0i32 {
            b"main\x00" as *const u8 as *const libc::c_char
        } else {
            b"function\x00" as *const u8 as *const libc::c_char
        },
        s,
        (*f).linedefined,
        (*f).lastlinedefined,
        (*f).sizecode,
        if (*f).sizecode == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
        f as *const libc::c_void,
    );
    printf(
        b"%d%s param%s, %d slot%s, %d upvalue%s, \x00" as *const u8 as *const libc::c_char,
        (*f).numparams as libc::c_int,
        if 0 != (*f).is_vararg as libc::c_int {
            b"+\x00" as *const u8 as *const libc::c_char
        } else {
            b"\x00" as *const u8 as *const libc::c_char
        },
        if (*f).numparams as libc::c_int == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
        (*f).maxstacksize as libc::c_int,
        if (*f).maxstacksize as libc::c_int == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
        (*f).sizeupvalues,
        if (*f).sizeupvalues == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
    );
    printf(
        b"%d local%s, %d constant%s, %d function%s\n\x00" as *const u8 as *const libc::c_char,
        (*f).sizelocvars,
        if (*f).sizelocvars == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
        (*f).sizek,
        if (*f).sizek == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
        (*f).sizep,
        if (*f).sizep == 1i32 {
            b"\x00" as *const u8 as *const libc::c_char
        } else {
            b"s\x00" as *const u8 as *const libc::c_char
        },
    );
}
/* default program name */
/* default output file */
/* list bytecodes? */
static mut listing: libc::c_int = 0i32;
/* dump bytecodes? */
static mut dumping: libc::c_int = 1i32;
/* strip debug information? */
static mut stripping: libc::c_int = 0i32;
/* default output file name */
static mut Output: [libc::c_char; 9] = [108, 117, 97, 99, 46, 111, 117, 116, 0];
/* actual output file name */
static mut output: *const libc::c_char = unsafe { Output.as_ptr() as *mut _ };
/* actual program name */
static mut progname: *const libc::c_char = b"luac\x00" as *const u8 as *const libc::c_char;
unsafe extern "C" fn fatal(mut message: *const libc::c_char) -> () {
    fprintf(
        stderr,
        b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
        progname,
        message,
    );
    exit(1i32);
}
unsafe extern "C" fn cannot(mut what: *const libc::c_char) -> () {
    fprintf(
        stderr,
        b"%s: cannot %s %s: %s\n\x00" as *const u8 as *const libc::c_char,
        progname,
        what,
        output,
        strerror(*__errno_location()),
    );
    exit(1i32);
}
unsafe extern "C" fn usage(mut message: *const libc::c_char) -> () {
    if *message as libc::c_int == '-' as i32 {
        fprintf(
            stderr,
            b"%s: unrecognized option \'%s\'\n\x00" as *const u8 as *const libc::c_char,
            progname,
            message,
        );
    } else {
        fprintf(
            stderr,
            b"%s: %s\n\x00" as *const u8 as *const libc::c_char,
            progname,
            message,
        );
    }
    fprintf(stderr,
            b"usage: %s [options] [filenames]\nAvailable options are:\n  -l       list (use -l -l for full listing)\n  -o name  output to file \'name\' (default is \"%s\")\n  -p       parse only\n  -s       strip debug information\n  -v       show version information\n  --       stop handling options\n  -        stop handling options and process stdin\n\x00"
                as *const u8 as *const libc::c_char, progname,
            Output.as_mut_ptr());
    exit(1i32);
}
unsafe extern "C" fn doargs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut version: libc::c_int = 0i32;
    if !(*argv.offset(0isize)).is_null() && **argv.offset(0isize) as libc::c_int != 0i32 {
        progname = *argv.offset(0isize)
    }
    i = 1i32;
    while i < argc {
        /* end of options; keep it */
        if **argv.offset(i as isize) as libc::c_int != '-' as i32 {
            break;
        }
        /* end of options; skip it */
        if strcmp(
            *argv.offset(i as isize),
            b"--\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            i += 1;
            if !(0 != version) {
                break;
            }
            version += 1;
            break;
        } else {
            /* end of options; use stdin */
            if strcmp(
                *argv.offset(i as isize),
                b"-\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                break;
            }
            /* list */
            if strcmp(
                *argv.offset(i as isize),
                b"-l\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                listing += 1
            } else if strcmp(
                *argv.offset(i as isize),
                b"-o\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                i += 1;
                output = *argv.offset(i as isize);
                if output.is_null()
                    || *output as libc::c_int == 0i32
                    || *output as libc::c_int == '-' as i32
                        && *output.offset(1isize) as libc::c_int != 0i32
                {
                    usage(b"\'-o\' needs argument\x00" as *const u8 as *const libc::c_char);
                }
                if strcmp(
                    *argv.offset(i as isize),
                    b"-\x00" as *const u8 as *const libc::c_char,
                ) == 0i32
                {
                    output = 0 as *const libc::c_char
                }
            } else if strcmp(
                *argv.offset(i as isize),
                b"-p\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                dumping = 0i32
            } else if strcmp(
                *argv.offset(i as isize),
                b"-s\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                stripping = 1i32
            } else if strcmp(
                *argv.offset(i as isize),
                b"-v\x00" as *const u8 as *const libc::c_char,
            ) == 0i32
            {
                version += 1
            } else {
                /* unknown option */
                usage(*argv.offset(i as isize));
            }
            i += 1
        }
    }
    if i == argc && (0 != listing || 0 == dumping) {
        dumping = 0i32;
        i -= 1;
        let ref mut fresh0 = *argv.offset(i as isize);
        *fresh0 = Output.as_mut_ptr()
    }
    if 0 != version {
        printf(
            b"%s\n\x00" as *const u8 as *const libc::c_char,
            b"Lua 5.3.5  Copyright (C) 1994-2018 Lua.org, PUC-Rio\x00" as *const u8
                as *const libc::c_char,
        );
        if version == argc - 1i32 {
            exit(0i32);
        }
    }
    return i;
}
unsafe extern "C" fn reader(
    mut _L: *mut lua_State,
    mut ud: *mut libc::c_void,
    mut size: *mut size_t,
) -> *const libc::c_char {
    let ref mut fresh1 = *(ud as *mut libc::c_int);
    let fresh2 = *fresh1;
    *fresh1 = *fresh1 - 1;
    if 0 != fresh2 {
        *size = (::std::mem::size_of::<[libc::c_char; 19]>() as libc::c_ulong)
            .wrapping_sub(1i32 as libc::c_ulong);
        return b"(function()end)();\x00" as *const u8 as *const libc::c_char;
    } else {
        *size = 0i32 as size_t;
        return 0 as *const libc::c_char;
    };
}
unsafe extern "C" fn combine(mut L: *mut lua_State, mut n: libc::c_int) -> *const Proto {
    if n == 1i32 {
        return (*((*(*L).top.offset(-1i32 as isize)).value_.gc as *mut GCUnion))
            .cl
            .l
            .p;
    } else {
        let mut f: *mut Proto = 0 as *mut Proto;
        let mut i: libc::c_int = n;
        if lua_load(
            L,
            Some(reader),
            &mut i as *mut libc::c_int as *mut libc::c_void,
            b"=(luac)\x00" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ) != 0i32
        {
            fatal(lua_tolstring(L, -1i32, 0 as *mut size_t));
        }
        f = (*((*(*L).top.offset(-1i32 as isize)).value_.gc as *mut GCUnion))
            .cl
            .l
            .p;
        i = 0i32;
        while i < n {
            let ref mut fresh3 = *(*f).p.offset(i as isize);
            *fresh3 = (*((*(*L).top.offset((i - n - 1i32) as isize)).value_.gc as *mut GCUnion))
                .cl
                .l
                .p;
            if (**(*f).p.offset(i as isize)).sizeupvalues > 0i32 {
                (*(**(*f).p.offset(i as isize)).upvalues.offset(0isize)).instack = 0i32 as lu_byte
            }
            i += 1
        }
        (*f).sizelineinfo = 0i32;
        return f;
    };
}
unsafe extern "C" fn writer(
    mut _L: *mut lua_State,
    mut p: *const libc::c_void,
    mut size: size_t,
    mut u: *mut libc::c_void,
) -> libc::c_int {
    return (fwrite(p, size, 1i32 as size_t, u as *mut FILE) != 1i32 as libc::c_ulong
        && size != 0i32 as libc::c_ulong) as libc::c_int;
}
unsafe extern "C" fn pmain(mut L: *mut lua_State) -> libc::c_int {
    let mut argc: libc::c_int = lua_tointegerx(L, 1i32, 0 as *mut libc::c_int) as libc::c_int;
    let mut argv: *mut *mut libc::c_char = lua_touserdata(L, 2i32) as *mut *mut libc::c_char;
    let mut f: *const Proto = 0 as *const Proto;
    let mut i: libc::c_int = 0;
    if 0 == lua_checkstack(L, argc) {
        fatal(b"too many input files\x00" as *const u8 as *const libc::c_char);
    }
    i = 0i32;
    while i < argc {
        let mut filename: *const libc::c_char = if strcmp(
            *argv.offset(i as isize),
            b"-\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            0 as *mut libc::c_char
        } else {
            *argv.offset(i as isize)
        };
        if luaL_loadfilex(L, filename, 0 as *const libc::c_char) != 0i32 {
            fatal(lua_tolstring(L, -1i32, 0 as *mut size_t));
        }
        i += 1
    }
    f = combine(L, argc);
    if 0 != listing {
        PrintFunction(f, (listing > 1i32) as libc::c_int);
    }
    if 0 != dumping {
        let mut D: *mut FILE = if output.is_null() {
            stdout
        } else {
            fopen(output, b"wb\x00" as *const u8 as *const libc::c_char)
        };
        if D.is_null() {
            cannot(b"open\x00" as *const u8 as *const libc::c_char);
        }
        luaU_dump(L, f, Some(writer), D as *mut libc::c_void, stripping);
        if 0 != ferror(D) {
            cannot(b"write\x00" as *const u8 as *const libc::c_char);
        }
        if 0 != fclose(D) {
            cannot(b"close\x00" as *const u8 as *const libc::c_char);
        }
    }
    return 0i32;
}
pub(crate) unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut L: *mut lua_State = 0 as *mut lua_State;
    let mut i: libc::c_int = doargs(argc, argv);
    argc -= i;
    argv = argv.offset(i as isize);
    if argc <= 0i32 {
        usage(b"no input files given\x00" as *const u8 as *const libc::c_char);
    }
    L = luaL_newstate();
    if L.is_null() {
        fatal(b"cannot create state: not enough memory\x00" as *const u8 as *const libc::c_char);
    }
    lua_pushcclosure(L, Some(pmain), 0i32);
    lua_pushinteger(L, argc as lua_Integer);
    lua_pushlightuserdata(L, argv as *mut libc::c_void);
    if lua_pcallk(L, 2i32, 0i32, 0i32, 0i32 as lua_KContext, None) != 0i32 {
        fatal(lua_tolstring(L, -1i32, 0 as *mut size_t));
    }
    lua_close(L);
    return 0i32;
}
