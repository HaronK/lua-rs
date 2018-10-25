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
