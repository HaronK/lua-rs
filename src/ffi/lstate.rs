use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn lua_newthread(L: *mut lua_State) -> *mut lua_State;

    #[no_mangle]
    fn lua_newstate(f: lua_Alloc, ud: *mut lua_void) -> *mut lua_State;

    #[no_mangle]
    fn lua_close(L: *mut lua_State) -> ();
}
