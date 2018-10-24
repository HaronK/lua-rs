pub mod ldo;
pub mod lfunc;
pub mod llimits;
pub mod lobject;
pub mod lstate;
pub mod lua;

pub mod prelude {
    pub use super::{ldo::*, lfunc::*, llimits::*, lobject::*, lstate::*, lua::*, *};
    pub use stdc::common::*;
}

// copy of libc::c_void
#[repr(u8)]
pub enum lua_void {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type lua_char = i8;
pub type lua_schar = i8;
pub type lua_uchar = u8;
pub type lua_short = i16;
pub type lua_ushort = u16;
pub type lua_int = i32;
pub type lua_uint = u32;
pub type lua_float = f32;
pub type lua_double = f64;
pub type lua_long = i64;
pub type lua_ulong = u64;
pub type lua_longlong = i64;
pub type lua_ulonglong = u64;
