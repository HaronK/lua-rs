// ------------------------------------------------
// Common macros

#[macro_export]
macro_rules! s {
    ($str:expr) => {
        $str as *const u8 as *const lua_char
    };
}

#[macro_export]
macro_rules! const_c_str {
    ($name:ident, $str:expr) => {
        const $name: *const lua_char = s!($str);
    };
}

#[macro_export]
macro_rules! pub_const_c_str {
    ($name:ident, $str:expr) => {
        pub const $name: *const lua_char = s!($str);
    };
}

#[macro_export]
macro_rules! lua_reg {
    ($name:expr, $func:ident) => {
        luaL_Reg {
            name: s!($name),
            func: Some($func),
        }
    };
}

#[macro_export]
macro_rules! lua_reg_none {
    ($name:expr) => {
        luaL_Reg {
            name: s!($name),
            func: None,
        }
    };
}

// ------------------------------------------------
// lapi macros

// TODO: implement!
#[macro_export]
macro_rules! lua_pushfstring {
    ($lua_State:expr, $($args:tt)*) => {{
        let mut endptr: *const lua_char = 0 as *const lua_char;
        endptr
    }};
}

// ------------------------------------------------
// lauxlib macros

// TODO: implement!
#[macro_export]
macro_rules! luaL_error {
    ($lua_State:expr, $($args:tt)*) => {{
        0
    }};
}

// ------------------------------------------------
// ldebug macros

// TODO: implement!
#[macro_export]
macro_rules! luaG_runerror {
    ($lua_State:expr, $($args:tt)*) => {
        ::std::process::exit(1)
    };
}
