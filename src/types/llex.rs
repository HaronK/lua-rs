use super::prelude::*;

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
    pub token: lua_int,
    pub seminfo: SemInfo,
}
/* state of the lexer plus state of the parser when shared by all
functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LexState {
    pub current: lua_int,
    pub linenumber: lua_int,
    pub lastline: lua_int,
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
