use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn lua_checkstack(L: *mut lua_State, n: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_xmove(from: *mut lua_State, to: *mut lua_State, n: lua_int) -> ();

    #[no_mangle]
    fn lua_atpanic(L: *mut lua_State, panicf: lua_CFunction) -> lua_CFunction;

    #[no_mangle]
    fn lua_version(L: *mut lua_State) -> *const lua_Number;

    #[no_mangle]
    fn lua_absindex(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_gettop(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_settop(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_rotate(L: *mut lua_State, idx: lua_int, n: lua_int) -> ();

    #[no_mangle]
    fn lua_copy(L: *mut lua_State, fromidx: lua_int, toidx: lua_int) -> ();

    #[no_mangle]
    fn lua_pushvalue(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_type(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_typename(L: *mut lua_State, tp: lua_int) -> *const lua_char;

    #[no_mangle]
    fn lua_iscfunction(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_isinteger(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_isnumber(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_isstring(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_rawequal(L: *mut lua_State, idx1: lua_int, idx2: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_compare(L: *mut lua_State, idx1: lua_int, idx2: lua_int, op: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_stringtonumber(L: *mut lua_State, s: *const lua_char) -> size_t;

    #[no_mangle]
    fn lua_tonumberx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Number;

    #[no_mangle]
    fn lua_tointegerx(L: *mut lua_State, idx: lua_int, isnum: *mut lua_int) -> lua_Integer;

    #[no_mangle]
    fn lua_toboolean(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_tolstring(L: *mut lua_State, idx: lua_int, len: *mut size_t) -> *const lua_char;

    #[no_mangle]
    fn lua_rawlen(L: *mut lua_State, idx: lua_int) -> size_t;

    #[no_mangle]
    fn lua_touserdata(L: *mut lua_State, idx: lua_int) -> *mut lua_void;

    #[no_mangle]
    fn lua_tothread(L: *mut lua_State, idx: lua_int) -> *mut lua_State;

    #[no_mangle]
    fn lua_topointer(L: *mut lua_State, idx: lua_int) -> *const lua_void;

    #[no_mangle]
    fn lua_pushnil(L: *mut lua_State) -> ();

    #[no_mangle]
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number) -> ();

    #[no_mangle]
    fn lua_pushinteger(L: *mut lua_State, n: lua_Integer) -> ();

    #[no_mangle]
    fn lua_pushlstring(L: *mut lua_State, s: *const lua_char, len: size_t) -> *const lua_char;

    #[no_mangle]
    fn lua_pushstring(L: *mut lua_State, s: *const lua_char) -> *const lua_char;

    #[no_mangle]
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: lua_int) -> ();

    #[no_mangle]
    fn lua_pushboolean(L: *mut lua_State, b: lua_int) -> ();

    #[no_mangle]
    fn lua_pushlightuserdata(L: *mut lua_State, p: *mut lua_void) -> ();

    #[no_mangle]
    fn lua_pushthread(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_getglobal(L: *mut lua_State, name: *const lua_char) -> lua_int;

    #[no_mangle]
    fn lua_gettable(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_getfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> lua_int;

    #[no_mangle]
    fn lua_geti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;

    #[no_mangle]
    fn lua_rawget(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_rawgeti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> lua_int;

    #[no_mangle]
    fn lua_rawgetp(L: *mut lua_State, idx: lua_int, p: *const lua_void) -> lua_int;

    #[no_mangle]
    fn lua_createtable(L: *mut lua_State, narr: lua_int, nrec: lua_int) -> ();

    #[no_mangle]
    fn lua_getmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_getuservalue(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_setglobal(L: *mut lua_State, name: *const lua_char) -> ();

    #[no_mangle]
    fn lua_setfield(L: *mut lua_State, idx: lua_int, k: *const lua_char) -> ();

    #[no_mangle]
    fn lua_seti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> ();

    #[no_mangle]
    fn lua_rawset(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_rawseti(L: *mut lua_State, idx: lua_int, n: lua_Integer) -> ();

    #[no_mangle]
    fn lua_rawsetp(L: *mut lua_State, idx: lua_int, p: *const lua_void) -> ();

    #[no_mangle]
    fn lua_setmetatable(L: *mut lua_State, objindex: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_setuservalue(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_status(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_gc(L: *mut lua_State, what: lua_int, data: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_error(L: *mut lua_State) -> lua_int;

    #[no_mangle]
    fn lua_next(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_concat(L: *mut lua_State, n: lua_int) -> ();

    #[no_mangle]
    fn lua_len(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_getallocf(L: *mut lua_State, ud: *mut *mut lua_void) -> lua_Alloc;

    #[no_mangle]
    fn lua_newuserdata(L: *mut lua_State, sz: size_t) -> *mut lua_void;

    #[no_mangle]
    fn lua_getupvalue(L: *mut lua_State, funcindex: lua_int, n: lua_int) -> *const lua_char;

    #[no_mangle]
    fn lua_setupvalue(L: *mut lua_State, funcindex: lua_int, n: lua_int) -> *const lua_char;

    #[no_mangle]
    fn lua_upvalueid(L: *mut lua_State, fidx: lua_int, n: lua_int) -> *mut lua_void;

    #[no_mangle]
    fn lua_isuserdata(L: *mut lua_State, idx: lua_int) -> lua_int;

    #[no_mangle]
    fn lua_arith(L: *mut lua_State, op: lua_int) -> ();

    #[no_mangle]
    fn lua_tocfunction(L: *mut lua_State, idx: lua_int) -> lua_CFunction;

    #[no_mangle]
    fn lua_pushvfstring(
        L: *mut lua_State,
        fmt: *const lua_char,
        argp: *mut __va_list_tag,
    ) -> *const lua_char;

    // Macro: lua_pushfstring

    #[no_mangle]
    fn lua_settable(L: *mut lua_State, idx: lua_int) -> ();

    #[no_mangle]
    fn lua_callk(
        L: *mut lua_State,
        nargs: lua_int,
        nresults: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> ();

    #[no_mangle]
    fn lua_pcallk(
        L: *mut lua_State,
        nargs: lua_int,
        nresults: lua_int,
        errfunc: lua_int,
        ctx: lua_KContext,
        k: lua_KFunction,
    ) -> lua_int;

    #[no_mangle]
    fn lua_load(
        L: *mut lua_State,
        reader: lua_Reader,
        data: *mut lua_void,
        chunkname: *const lua_char,
        mode: *const lua_char,
    ) -> lua_int;

    #[no_mangle]
    fn lua_dump(
        L: *mut lua_State,
        writer: lua_Writer,
        data: *mut lua_void,
        strip: lua_int,
    ) -> lua_int;

    #[no_mangle]
    fn lua_setallocf(L: *mut lua_State, f: lua_Alloc, ud: *mut lua_void) -> ();

    #[no_mangle]
    fn lua_upvaluejoin(
        L: *mut lua_State,
        fidx1: lua_int,
        n1: lua_int,
        fidx2: lua_int,
        n2: lua_int,
    ) -> ();
}
