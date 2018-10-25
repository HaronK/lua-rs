use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn lua_sethook(L: *mut lua_State, func: lua_Hook, mask: lua_int, count: lua_int) -> ();

    #[no_mangle]
    fn lua_gethook(L: *mut lua_State) -> lua_Hook;

    #[no_mangle]
    fn lua_gethookmask(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_gethookcount(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_getstack(L: *mut lua_State, level: lua_int, ar: *mut lua_Debug) -> lua_int;

    #[no_mangle]
    fn lua_getlocal(L: *mut lua_State, ar: *const lua_Debug, n: lua_int) -> *const lua_char;

    #[no_mangle]
    fn lua_setlocal(L: *mut lua_State, ar: *const lua_Debug, n: lua_int) -> *const lua_char;

    #[no_mangle]
    fn lua_getinfo(L: *mut lua_State, what: *const lua_char, ar: *mut lua_Debug) -> lua_int;
}
