use types::prelude::*;

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
/* basic instruction format */
pub const iAx: OpMode = 3;
pub const iAsBx: OpMode = 2;
pub const iABx: OpMode = 1;
pub const iABC: OpMode = 0;
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
/* argument is used */
pub const OpArgU: OpArgMask = 1;
/* argument is not used */
pub const OpArgN: OpArgMask = 0;
#[no_mangle]
pub static mut luaP_opmodes: [lu_byte; 47] = [
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgN as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgR as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iAsBx as lua_int) as lu_byte,
    (1i32 << 7i32
        | 0i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (1i32 << 7i32
        | 0i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (1i32 << 7i32
        | 0i32 << 6i32
        | (OpArgK as lua_int) << 4i32
        | (OpArgK as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (1i32 << 7i32
        | 0i32 << 6i32
        | (OpArgN as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (1i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iAsBx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iAsBx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgN as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgR as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iAsBx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABx as lua_int) as lu_byte,
    (0i32 << 7i32
        | 1i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgN as lua_int) << 2i32
        | iABC as lua_int) as lu_byte,
    (0i32 << 7i32
        | 0i32 << 6i32
        | (OpArgU as lua_int) << 4i32
        | (OpArgU as lua_int) << 2i32
        | iAx as lua_int) as lu_byte,
];
/* opcode names */
#[no_mangle]
pub static mut luaP_opnames: [*const lua_char; 48] = [
    s!(b"MOVE\x00"),
    s!(b"LOADK\x00"),
    s!(b"LOADKX\x00"),
    s!(b"LOADBOOL\x00"),
    s!(b"LOADNIL\x00"),
    s!(b"GETUPVAL\x00"),
    s!(b"GETTABUP\x00"),
    s!(b"GETTABLE\x00"),
    s!(b"SETTABUP\x00"),
    s!(b"SETUPVAL\x00"),
    s!(b"SETTABLE\x00"),
    s!(b"NEWTABLE\x00"),
    s!(b"SELF\x00"),
    s!(b"ADD\x00"),
    s!(b"SUB\x00"),
    s!(b"MUL\x00"),
    s!(b"MOD\x00"),
    s!(b"POW\x00"),
    s!(b"DIV\x00"),
    s!(b"IDIV\x00"),
    s!(b"BAND\x00"),
    s!(b"BOR\x00"),
    s!(b"BXOR\x00"),
    s!(b"SHL\x00"),
    s!(b"SHR\x00"),
    s!(b"UNM\x00"),
    s!(b"BNOT\x00"),
    s!(b"NOT\x00"),
    s!(b"LEN\x00"),
    s!(b"CONCAT\x00"),
    s!(b"JMP\x00"),
    s!(b"EQ\x00"),
    s!(b"LT\x00"),
    s!(b"LE\x00"),
    s!(b"TEST\x00"),
    s!(b"TESTSET\x00"),
    s!(b"CALL\x00"),
    s!(b"TAILCALL\x00"),
    s!(b"RETURN\x00"),
    s!(b"FORLOOP\x00"),
    s!(b"FORPREP\x00"),
    s!(b"TFORCALL\x00"),
    s!(b"TFORLOOP\x00"),
    s!(b"SETLIST\x00"),
    s!(b"CLOSURE\x00"),
    s!(b"VARARG\x00"),
    s!(b"EXTRAARG\x00"),
    0 as *const lua_char,
];
