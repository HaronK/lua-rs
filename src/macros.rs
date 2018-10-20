#[macro_export]
macro_rules! s {
    ($str:expr) => {
        $str as *const u8 as *const libc::c_char
    };
}

#[macro_export]
macro_rules! const_c_str {
    ($name:ident, $str:expr) => {
        const $name: *const libc::c_char = s!($str);
    };
}

#[macro_export]
macro_rules! pub_const_c_str {
    ($name:ident, $str:expr) => {
        pub const $name: *const libc::c_char = s!($str);
    };
}

// TODO: implement!
#[macro_export]
macro_rules! luaO_pushfstring {
    ($lua_State:expr, $($args:tt)*) => {{
        let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
        endptr
    }};
}

// TODO: implement!
#[macro_export]
macro_rules! lua_pushfstring {
    ($lua_State:expr, $($args:tt)*) => {{
        let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
        endptr
    }};
}

// TODO: implement!
#[macro_export]
macro_rules! luaG_runerror {
    ($lua_State:expr, $($args:tt)*) => {
        ::std::process::exit(1)
    };
}

// TODO: implement!
#[macro_export]
macro_rules! luaL_error {
    ($lua_State:expr, $fmt:expr, $($args:tt)*) => {{
        0
    }};
}
