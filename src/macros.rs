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
