use lua::*;
use luaconf::*;
use stdc::prelude::*;
use types::prelude::*;

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

    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn fopen(__filename: *const lua_char, __modes: *const lua_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn printf(_: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn sprintf(_: *mut lua_char, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    fn fwrite(__ptr: *const lua_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn ferror(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    fn exit(_: lua_int) -> !;
    #[no_mangle]
    fn strcmp(_: *const lua_char, _: *const lua_char) -> lua_int;
    #[no_mangle]
    fn strspn(_: *const lua_char, _: *const lua_char) -> lua_ulong;
    #[no_mangle]
    fn strerror(_: lua_int) -> *mut lua_char;
    #[no_mangle]
    fn lua_close(L: *mut lua_State) -> ();
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: lua_int) -> lua_int;
    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Integer;
    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;
    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: lua_int) -> *mut lua_void;
    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();
    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();
    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut lua_void) -> ();
    #[no_mangle]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: lua_int,
        nresults: lua_int,
        errfunc: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> lua_int;
    #[no_mangle]
    fn lua_load(
        L: *mut lua_State,
        reader_0: lua_Reader,
        dt: *mut lua_void,
        chunkname: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_loadfilex(
        L: *mut lua_State,
        filename: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;
    #[no_mangle]
    fn luaL_newstate() -> *mut lua_State;
    /* dump one chunk; from ldump.c */
    #[no_mangle]
    fn luaU_dump(
        L: *mut lua_State,
        f: *const Proto,
        w: lua_Writer,
        data: *mut lua_void,
        strip: lua_int,
    ) -> lua_int;
    #[no_mangle]
    static luaP_opmodes: [lu_byte; 47];
    /* opcode names */
    #[no_mangle]
    static luaP_opnames: [*const lua_char; 48];
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
** $Id: llimits.h,v 1.141.1.1 2017/04/19 17:20:42 roberto Exp $
** Limits, basic types, and some other 'installation-dependent' definitions
** See Copyright Notice in lua.h
*/

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
pub type OpCode = lua_uint;
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
pub type OpArgMask = lua_uint;
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
pub type OpMode = lua_uint;
/*
** $Id: luac.c,v 1.76 2018/06/19 01:32:02 lhf Exp $
** Lua compiler (saves bytecodes to files; also lists bytecodes)
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn PrintFunction(mut f: *const Proto, mut full: lua_int) -> () {
    let mut i: lua_int = 0;
    let mut n: lua_int = (*f).sizep;
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
    let mut i: lua_int = 0;
    let mut n: lua_int = 0;
    n = (*f).sizek;
    printf(s!(b"constants (%d) for %p:\n\x00"), n, f as *const lua_void);
    i = 0i32;
    while i < n {
        printf(s!(b"\t%d\t\x00"), i + 1i32);
        PrintConstant(f, i);
        printf(s!(b"\n\x00"));
        i += 1
    }
    n = (*f).sizelocvars;
    printf(s!(b"locals (%d) for %p:\n\x00"), n, f as *const lua_void);
    i = 0i32;
    while i < n {
        printf(
            s!(b"\t%d\t%s\t%d\t%d\n\x00"),
            i,
            ((*(*f).locvars.offset(i as isize)).varname as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            (*(*f).locvars.offset(i as isize)).startpc + 1i32,
            (*(*f).locvars.offset(i as isize)).endpc + 1i32,
        );
        i += 1
    }
    n = (*f).sizeupvalues;
    printf(s!(b"upvalues (%d) for %p:\n\x00"), n, f as *const lua_void);
    i = 0i32;
    while i < n {
        printf(
            s!(b"\t%d\t%s\t%d\t%d\n\x00"),
            i,
            if !(*(*f).upvalues.offset(i as isize)).name.is_null() {
                ((*(*f).upvalues.offset(i as isize)).name as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
            } else {
                s!(b"-\x00")
            },
            (*(*f).upvalues.offset(i as isize)).instack as lua_int,
            (*(*f).upvalues.offset(i as isize)).idx as lua_int,
        );
        i += 1
    }
}
unsafe extern "C" fn PrintConstant(mut f: *const Proto, mut i: lua_int) -> () {
    let mut o: *const TValue = &mut *(*f).k.offset(i as isize) as *mut TValue;
    match (*o).tt_ & 0x3fi32 {
        0 => {
            printf(s!(b"nil\x00"));
        }
        1 => {
            printf(if 0 != (*o).value_.b {
                s!(b"true\x00")
            } else {
                s!(b"false\x00")
            });
        }
        3 => {
            let mut buff: [lua_char; 100] = [0; 100];
            sprintf(buff.as_mut_ptr(), s!(b"%.14g\x00"), (*o).value_.n);
            printf(s!(b"%s\x00"), buff.as_mut_ptr());
            if buff[strspn(buff.as_mut_ptr(), s!(b"-0123456789\x00")) as usize] as lua_int
                == '\u{0}' as i32
            {
                printf(s!(b".0\x00"));
            }
        }
        19 => {
            printf(s!(b"%lld\x00"), (*o).value_.i);
        }
        4 | 20 => {
            PrintString(&mut (*((*o).value_.gc as *mut GCUnion)).ts);
        }
        _ => {
            printf(s!(b"? type=%d\x00"), (*o).tt_ & 0x3fi32);
        }
    };
}
/*
** $Id: luac.c,v 1.76 2018/06/19 01:32:02 lhf Exp $
** print bytecodes
** See Copyright Notice in lua.h
*/
unsafe extern "C" fn PrintString(mut ts: *const TString) -> () {
    let mut s: *const lua_char =
        (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize);
    let mut i: size_t = 0;
    let mut n: size_t = if (*ts).tt as lua_int == 4i32 | 0i32 << 4i32 {
        (*ts).shrlen as lua_ulong
    } else {
        (*ts).u.lnglen
    };
    printf(s!(b"%c\x00"), '\"' as i32);
    i = 0i32 as size_t;
    while i < n {
        let mut c: lua_int = *s.offset(i as isize) as lua_uchar as lua_int;
        match c {
            34 => {
                printf(s!(b"\\\"\x00"));
            }
            92 => {
                printf(s!(b"\\\\\x00"));
            }
            7 => {
                printf(s!(b"\\a\x00"));
            }
            8 => {
                printf(s!(b"\\b\x00"));
            }
            12 => {
                printf(s!(b"\\f\x00"));
            }
            10 => {
                printf(s!(b"\\n\x00"));
            }
            13 => {
                printf(s!(b"\\r\x00"));
            }
            9 => {
                printf(s!(b"\\t\x00"));
            }
            11 => {
                printf(s!(b"\\v\x00"));
            }
            _ => {
                if 0 != isprint(c) {
                    printf(s!(b"%c\x00"), c);
                } else {
                    printf(s!(b"\\%03d\x00"), c);
                }
            }
        }
        i = i.wrapping_add(1)
    }
    printf(s!(b"%c\x00"), '\"' as i32);
}
unsafe extern "C" fn PrintCode(mut f: *const Proto) -> () {
    let mut code: *const Instruction = (*f).code;
    let mut pc: lua_int = 0;
    let mut n: lua_int = (*f).sizecode;
    pc = 0i32;
    while pc < n {
        let mut i: Instruction = *code.offset(pc as isize);
        let mut o: OpCode = (i >> 0i32 & !((!(0i32 as Instruction)) << 6i32) << 0i32) as OpCode;
        let mut a: lua_int =
            (i >> 0i32 + 6i32 & !((!(0i32 as Instruction)) << 8i32) << 0i32) as lua_int;
        let mut b: lua_int = (i >> 0i32 + 6i32 + 8i32 + 9i32
            & !((!(0i32 as Instruction)) << 9i32) << 0i32) as lua_int;
        let mut c: lua_int =
            (i >> 0i32 + 6i32 + 8i32 & !((!(0i32 as Instruction)) << 9i32) << 0i32) as lua_int;
        let mut ax: lua_int = (i >> 0i32 + 6i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32 + 8i32) << 0i32)
            as lua_int;
        let mut bx: lua_int = (i >> 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
            as lua_int;
        let mut sbx: lua_int = (i >> 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32 + 9i32) << 0i32)
            as lua_int
            - ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32);
        let mut line: lua_int = if !(*f).lineinfo.is_null() {
            *(*f).lineinfo.offset(pc as isize)
        } else {
            -1i32
        };
        printf(s!(b"\t%d\t\x00"), pc + 1i32);
        if line > 0i32 {
            printf(s!(b"[%d]\t\x00"), line);
        } else {
            printf(s!(b"[-]\t\x00"));
        }
        printf(s!(b"%-9s\t\x00"), luaP_opnames[o as usize]);
        match (luaP_opmodes[o as usize] as lua_int & 3i32) as OpMode as lua_uint {
            0 => {
                printf(s!(b"%d\x00"), a);
                if (luaP_opmodes[o as usize] as lua_int >> 4i32 & 3i32) as OpArgMask as lua_uint
                    != OpArgN as lua_int as lua_uint
                {
                    printf(
                        s!(b" %d\x00"),
                        if 0 != b & 1i32 << 9i32 - 1i32 {
                            -1i32 - (b & !(1i32 << 9i32 - 1i32))
                        } else {
                            b
                        },
                    );
                }
                if (luaP_opmodes[o as usize] as lua_int >> 2i32 & 3i32) as OpArgMask as lua_uint
                    != OpArgN as lua_int as lua_uint
                {
                    printf(
                        s!(b" %d\x00"),
                        if 0 != c & 1i32 << 9i32 - 1i32 {
                            -1i32 - (c & !(1i32 << 9i32 - 1i32))
                        } else {
                            c
                        },
                    );
                }
            }
            1 => {
                printf(s!(b"%d\x00"), a);
                if (luaP_opmodes[o as usize] as lua_int >> 4i32 & 3i32) as OpArgMask as lua_uint
                    == OpArgK as lua_int as lua_uint
                {
                    printf(s!(b" %d\x00"), -1i32 - bx);
                }
                if (luaP_opmodes[o as usize] as lua_int >> 4i32 & 3i32) as OpArgMask as lua_uint
                    == OpArgU as lua_int as lua_uint
                {
                    printf(s!(b" %d\x00"), bx);
                }
            }
            2 => {
                printf(s!(b"%d %d\x00"), a, sbx);
            }
            3 => {
                printf(s!(b"%d\x00"), -1i32 - ax);
            }
            _ => {}
        }
        match o as lua_uint {
            1 => {
                printf(s!(b"\t; \x00"));
                PrintConstant(f, bx);
            }
            5 | 9 => {
                printf(
                    s!(b"\t; %s\x00"),
                    if !(*(*f).upvalues.offset(b as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(b as isize)).name as *mut lua_char)
                            .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    } else {
                        s!(b"-\x00")
                    },
                );
            }
            6 => {
                printf(
                    s!(b"\t; %s\x00"),
                    if !(*(*f).upvalues.offset(b as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(b as isize)).name as *mut lua_char)
                            .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    } else {
                        s!(b"-\x00")
                    },
                );
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(s!(b" \x00"));
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            8 => {
                printf(
                    s!(b"\t; %s\x00"),
                    if !(*(*f).upvalues.offset(a as isize)).name.is_null() {
                        ((*(*f).upvalues.offset(a as isize)).name as *mut lua_char)
                            .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    } else {
                        s!(b"-\x00")
                    },
                );
                if 0 != b & 1i32 << 9i32 - 1i32 {
                    printf(s!(b" \x00"));
                    PrintConstant(f, b & !(1i32 << 9i32 - 1i32));
                }
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(s!(b" \x00"));
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            7 | 12 => {
                if 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(s!(b"\t; \x00"));
                    PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                }
            }
            10 | 13 | 14 | 15 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 | 24 | 31 | 32 | 33 => {
                if 0 != b & 1i32 << 9i32 - 1i32 || 0 != c & 1i32 << 9i32 - 1i32 {
                    printf(s!(b"\t; \x00"));
                    if 0 != b & 1i32 << 9i32 - 1i32 {
                        PrintConstant(f, b & !(1i32 << 9i32 - 1i32));
                    } else {
                        printf(s!(b"-\x00"));
                    }
                    printf(s!(b" \x00"));
                    if 0 != c & 1i32 << 9i32 - 1i32 {
                        PrintConstant(f, c & !(1i32 << 9i32 - 1i32));
                    } else {
                        printf(s!(b"-\x00"));
                    }
                }
            }
            30 | 39 | 40 | 42 => {
                printf(s!(b"\t; to %d\x00"), sbx + pc + 2i32);
            }
            44 => {
                printf(
                    s!(b"\t; %p\x00"),
                    *(*f).p.offset(bx as isize) as *const lua_void,
                );
            }
            43 => {
                if c == 0i32 {
                    pc += 1;
                    printf(s!(b"\t; %d\x00"), *code.offset(pc as isize) as lua_int);
                } else {
                    printf(s!(b"\t; %d\x00"), c);
                }
            }
            46 => {
                printf(s!(b"\t; \x00"));
                PrintConstant(f, ax);
            }
            _ => {}
        }
        printf(s!(b"\n\x00"));
        pc += 1
    }
}
unsafe extern "C" fn PrintHeader(mut f: *const Proto) -> () {
    let mut s: *const lua_char = if !(*f).source.is_null() {
        ((*f).source as *mut lua_char)
            .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
    } else {
        s!(b"=?\x00")
    };
    if *s as lua_int == '@' as i32 || *s as lua_int == '=' as i32 {
        s = s.offset(1isize)
    } else if *s as lua_int
        == (*::std::mem::transmute::<&[u8; 5], &[lua_char; 5]>(b"\x1bLua\x00"))[0usize] as lua_int
    {
        s = s!(b"(bstring)\x00")
    } else {
        s = s!(b"(string)\x00")
    }
    printf(
        s!(b"\n%s <%s:%d,%d> (%d instruction%s at %p)\n\x00"),
        if (*f).linedefined == 0i32 {
            s!(b"main\x00")
        } else {
            s!(b"function\x00")
        },
        s,
        (*f).linedefined,
        (*f).lastlinedefined,
        (*f).sizecode,
        if (*f).sizecode == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
        f as *const lua_void,
    );
    printf(
        s!(b"%d%s param%s, %d slot%s, %d upvalue%s, \x00"),
        (*f).numparams as lua_int,
        if 0 != (*f).is_vararg as lua_int {
            s!(b"+\x00")
        } else {
            s!(b"\x00")
        },
        if (*f).numparams as lua_int == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
        (*f).maxstacksize as lua_int,
        if (*f).maxstacksize as lua_int == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
        (*f).sizeupvalues,
        if (*f).sizeupvalues == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
    );
    printf(
        s!(b"%d local%s, %d constant%s, %d function%s\n\x00"),
        (*f).sizelocvars,
        if (*f).sizelocvars == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
        (*f).sizek,
        if (*f).sizek == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
        (*f).sizep,
        if (*f).sizep == 1i32 {
            s!(b"\x00")
        } else {
            s!(b"s\x00")
        },
    );
}
/* default program name */
/* default output file */
/* list bytecodes? */
static mut listing: lua_int = 0i32;
/* dump bytecodes? */
static mut dumping: lua_int = 1i32;
/* strip debug information? */
static mut stripping: lua_int = 0i32;
/* default output file name */
static mut Output: [lua_char; 9] = [108, 117, 97, 99, 46, 111, 117, 116, 0];
/* actual output file name */
static mut output: *const lua_char = unsafe { Output.as_ptr() as *mut _ };
/* actual program name */
static mut progname: *const lua_char = s!(b"luac\x00");
unsafe extern "C" fn fatal(mut message: *const lua_char) -> () {
    fprintf(stderr, s!(b"%s: %s\n\x00"), progname, message);
    exit(1i32);
}
unsafe extern "C" fn cannot(mut what: *const lua_char) -> () {
    fprintf(
        stderr,
        s!(b"%s: cannot %s %s: %s\n\x00"),
        progname,
        what,
        output,
        strerror(errno()),
    );
    exit(1i32);
}
unsafe extern "C" fn usage(mut message: *const lua_char) -> () {
    if *message as lua_int == '-' as i32 {
        fprintf(
            stderr,
            s!(b"%s: unrecognized option \'%s\'\n\x00"),
            progname,
            message,
        );
    } else {
        fprintf(stderr, s!(b"%s: %s\n\x00"), progname, message);
    }
    fprintf(stderr,
            s!(b"usage: %s [options] [filenames]\nAvailable options are:\n  -l       list (use -l -l for full listing)\n  -o name  output to file \'name\' (default is \"%s\")\n  -p       parse only\n  -s       strip debug information\n  -v       show version information\n  --       stop handling options\n  -        stop handling options and process stdin\n\x00"), progname,
            Output.as_mut_ptr());
    exit(1i32);
}
unsafe extern "C" fn doargs(mut argc: lua_int, mut argv: *mut *mut lua_char) -> lua_int {
    let mut i: lua_int = 0;
    let mut version: lua_int = 0i32;
    if !(*argv.offset(0isize)).is_null() && **argv.offset(0isize) as lua_int != 0i32 {
        progname = *argv.offset(0isize)
    }
    i = 1i32;
    while i < argc {
        /* end of options; keep it */
        if **argv.offset(i as isize) as lua_int != '-' as i32 {
            break;
        }
        /* end of options; skip it */
        if strcmp(*argv.offset(i as isize), s!(b"--\x00")) == 0i32 {
            i += 1;
            if !(0 != version) {
                break;
            }
            version += 1;
            break;
        } else {
            /* end of options; use stdin */
            if strcmp(*argv.offset(i as isize), s!(b"-\x00")) == 0i32 {
                break;
            }
            /* list */
            if strcmp(*argv.offset(i as isize), s!(b"-l\x00")) == 0i32 {
                listing += 1
            } else if strcmp(*argv.offset(i as isize), s!(b"-o\x00")) == 0i32 {
                i += 1;
                output = *argv.offset(i as isize);
                if output.is_null()
                    || *output as lua_int == 0i32
                    || *output as lua_int == '-' as i32 && *output.offset(1isize) as lua_int != 0i32
                {
                    usage(s!(b"\'-o\' needs argument\x00"));
                }
                if strcmp(*argv.offset(i as isize), s!(b"-\x00")) == 0i32 {
                    output = 0 as *const lua_char
                }
            } else if strcmp(*argv.offset(i as isize), s!(b"-p\x00")) == 0i32 {
                dumping = 0i32
            } else if strcmp(*argv.offset(i as isize), s!(b"-s\x00")) == 0i32 {
                stripping = 1i32
            } else if strcmp(*argv.offset(i as isize), s!(b"-v\x00")) == 0i32 {
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
            s!(b"%s\n\x00"),
            s!(b"Lua 5.3.5  Copyright (C) 1994-2018 Lua.org, PUC-Rio\x00"),
        );
        if version == argc - 1i32 {
            exit(0i32);
        }
    }
    return i;
}
unsafe extern "C" fn reader(
    mut _L: *mut lua_State,
    mut ud: *mut lua_void,
    mut size: *mut size_t,
) -> *const lua_char {
    let ref mut fresh1 = *(ud as *mut lua_int);
    let fresh2 = *fresh1;
    *fresh1 = *fresh1 - 1;
    if 0 != fresh2 {
        *size =
            (::std::mem::size_of::<[lua_char; 19]>() as lua_ulong).wrapping_sub(1i32 as lua_ulong);
        return s!(b"(function()end)();\x00");
    } else {
        *size = 0i32 as size_t;
        return 0 as *const lua_char;
    };
}
unsafe extern "C" fn combine(mut L: *mut lua_State, mut n: lua_int) -> *const Proto {
    if n == 1i32 {
        return (*((*(*L).top.offset(-1i32 as isize)).value_.gc as *mut GCUnion))
            .cl
            .l
            .p;
    } else {
        let mut f: *mut Proto = 0 as *mut Proto;
        let mut i: lua_int = n;
        if lua_load(
            L,
            Some(reader),
            &mut i as *mut lua_int as *mut lua_void,
            s!(b"=(luac)\x00"),
            0 as *const lua_char,
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
    mut p: *const lua_void,
    mut size: size_t,
    mut u: *mut lua_void,
) -> lua_int {
    return (fwrite(p, size, 1i32 as size_t, u as *mut FILE) != 1i32 as lua_ulong
        && size != 0i32 as lua_ulong) as lua_int;
}
unsafe extern "C" fn pmain(mut L: *mut lua_State) -> lua_int {
    let mut argc: lua_int = lua_tointegerx(L, 1i32, 0 as *mut lua_int) as lua_int;
    let mut argv: *mut *mut lua_char = lua_touserdata(L, 2i32) as *mut *mut lua_char;
    let mut f: *const Proto = 0 as *const Proto;
    let mut i: lua_int = 0;
    if 0 == lua_checkstack(L, argc) {
        fatal(s!(b"too many input files\x00"));
    }
    i = 0i32;
    while i < argc {
        let mut filename: *const lua_char =
            if strcmp(*argv.offset(i as isize), s!(b"-\x00")) == 0i32 {
                0 as *mut lua_char
            } else {
                *argv.offset(i as isize)
            };
        if luaL_loadfilex(L, filename, 0 as *const lua_char) != LUA_OK {
            fatal(lua_tolstring(L, -1i32, 0 as *mut size_t));
        }
        i += 1
    }
    f = combine(L, argc);
    if 0 != listing {
        PrintFunction(f, (listing > 1i32) as lua_int);
    }
    if 0 != dumping {
        let mut D: *mut FILE = if output.is_null() {
            stdout
        } else {
            fopen(output, s!(b"wb\x00"))
        };
        if D.is_null() {
            cannot(s!(b"open\x00"));
        }
        luaU_dump(L, f, Some(writer), D as *mut lua_void, stripping);
        if 0 != ferror(D) {
            cannot(s!(b"write\x00"));
        }
        if 0 != fclose(D) {
            cannot(s!(b"close\x00"));
        }
    }
    return 0i32;
}
pub(crate) unsafe fn main_0(mut argc: lua_int, mut argv: *mut *mut lua_char) -> lua_int {
    let mut L = 0 as *mut lua_State;
    let mut i = doargs(argc, argv);
    argc -= i;
    argv = argv.offset(i as isize);
    if argc <= 0i32 {
        usage(s!(b"no input files given\x00"));
    }
    L = luaL_newstate();
    if L.is_null() {
        fatal(s!(b"cannot create state: not enough memory\x00"));
    }
    lua_pushcclosure(L, Some(pmain), 0i32);
    lua_pushinteger(L, argc as lua_Integer);
    lua_pushlightuserdata(L, argv as *mut lua_void);
    if lua_pcallk(L, 2i32, 0i32, 0i32, 0i32 as lua_KContext, None) != LUA_OK {
        fatal(lua_tolstring(L, -1i32, 0 as *mut size_t));
    }
    lua_close(L);
    return EXIT_SUCCESS;
}
