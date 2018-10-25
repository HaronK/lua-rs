use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn lua_resume(L: *mut lua_State, from: *mut lua_State, narg: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_isyieldable(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_yieldk(
        L: *mut lua_State,
        nresults: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> lua_int;
}
