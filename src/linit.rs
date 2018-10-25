use types::prelude::*;
extern "C" {
    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();
    #[no_mangle]
    fn luaopen_base(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_coroutine(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_table(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_io(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_os(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_string(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_utf8(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_bit32(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_math(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_debug(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaopen_package(L: *mut lua_State) -> lua_int;
    #[no_mangle]
    fn luaL_requiref(
        L: *mut lua_State,
        modname: *const lua_char,
        openf: lua_CFunction,
        glb: lua_int,
    ) -> ();
}

/* open all previous libraries */
#[no_mangle]
pub unsafe extern "C" fn luaL_openlibs(mut L: *mut lua_State) -> () {
    let mut lib: *const luaL_Reg = 0 as *const luaL_Reg;
    /* "require" functions from 'loadedlibs' and set results to global table */
    lib = loadedlibs.as_ptr();
    while (*lib).func.is_some() {
        luaL_requiref(L, (*lib).name, (*lib).func, 1i32);
        /* remove lib */
        lua_settop(L, -1i32 - 1i32);
        lib = lib.offset(1isize)
    }
}

/*
** these libs are loaded by lua.c and are readily available to any Lua
** program
*/
static mut loadedlibs: [luaL_Reg; 12] = [
    lua_reg!(b"_G\x00", luaopen_base),
    lua_reg!(b"package\x00", luaopen_package),
    lua_reg!(b"coroutine\x00", luaopen_coroutine),
    lua_reg!(b"table\x00", luaopen_table),
    lua_reg!(b"io\x00", luaopen_io),
    lua_reg!(b"os\x00", luaopen_os),
    lua_reg!(b"string\x00", luaopen_string),
    lua_reg!(b"math\x00", luaopen_math),
    lua_reg!(b"utf8\x00", luaopen_utf8),
    lua_reg!(b"debug\x00", luaopen_debug),
    lua_reg!(b"bit32\x00", luaopen_bit32),
    lua_reg_none!(0),
];
