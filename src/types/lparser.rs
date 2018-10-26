use super::prelude::*;

/* dynamic structures used by the parser */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dyndata {
    pub actvar: unnamed_6,
    pub gt: Labellist,
    pub label: Labellist,
}
/* list of labels or gotos */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labellist {
    pub arr: *mut Labeldesc,
    pub n: lua_int,
    pub size: lua_int,
}
/* description of pending goto statements and label statements */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Labeldesc {
    pub name: *mut TString,
    pub pc: lua_int,
    pub line: lua_int,
    pub nactvar: lu_byte,
}
/* dynamic structures used by the parser */
/* list of active local variables */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_6 {
    pub arr: *mut Vardesc,
    pub n: lua_int,
    pub size: lua_int,
}

/* kinds of variables/expressions */
pub type expkind = lua_uint;
/* vararg expression; info = instruction pc */
pub const VVARARG: expkind = 14;
/* expression is a function call; info = instruction pc */
pub const VCALL: expkind = 13;
/* expression can put result in any register;
info = instruction pc */
pub const VRELOCABLE: expkind = 12;
/* expression is a test/comparison;
info = pc of corresponding jump instruction */
pub const VJMP: expkind = 11;
/* indexed variable;
ind.vt = whether 't' is register or upvalue;
ind.t = table register or upvalue;
ind.idx = key's R/K index */
pub const VINDEXED: expkind = 10;
/* upvalue variable; info = index of upvalue in 'upvalues' */
pub const VUPVAL: expkind = 9;
/* local variable; info = local register */
pub const VLOCAL: expkind = 8;
/* expression has its value in a fixed register;
info = result register */
pub const VNONRELOC: expkind = 7;
/* integer constant; nval = numerical integer value */
pub const VKINT: expkind = 6;
/* floating constant; nval = numerical float value */
pub const VKFLT: expkind = 5;
/* constant in 'k'; info = index of constant in 'k' */
pub const VK: expkind = 4;
/* constant false */
pub const VFALSE: expkind = 3;
/* constant true */
pub const VTRUE: expkind = 2;
/* constant nil */
pub const VNIL: expkind = 1;
/* when 'expdesc' describes the last expression a list,
this kind means an empty list (so, no expression) */
pub const VVOID: expkind = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc {
    pub k: expkind,
    pub u: expdesc_1,
    pub t: lua_int,
    pub f: lua_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union expdesc_1 {
    pub ival: lua_Integer,
    pub nval: lua_Number,
    pub info: lua_int,
    pub ind: expdesc_2,
}
/* for indexed variables (VINDEXED) */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct expdesc_2 {
    pub idx: lua_short,
    pub t: lu_byte,
    pub vt: lu_byte,
}

/* description of active local variable */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vardesc {
    pub idx: lua_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FuncState {
    pub f: *mut Proto,
    pub prev: *mut FuncState,
    pub ls: *mut LexState,
    pub bl: *mut BlockCnt,
    pub pc: lua_int,
    pub lasttarget: lua_int,
    pub jpc: lua_int,
    pub nk: lua_int,
    pub np: lua_int,
    pub firstlocal: lua_int,
    pub nlocvars: lua_short,
    pub nactvar: lu_byte,
    pub nups: lu_byte,
    pub freereg: lu_byte,
}
/* control of blocks */
/* defined in lparser.c */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BlockCnt {
    pub previous: *mut BlockCnt,
    pub firstlabel: lua_int,
    pub firstgoto: lua_int,
    pub nactvar: lu_byte,
    pub upval: lu_byte,
    pub isloop: lu_byte,
}
