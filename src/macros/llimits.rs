/* internal assertions for in-house debugging */
// #if defined(lua_assert)
// #define check_exp(c,e)		(lua_assert(c), (e))
/* to avoid problems with conditions too long */
// #define lua_longassert(c)	((c) ? (void)0 : lua_assert(0))
// #else
// #define lua_assert(c)		((void)0)
// #define check_exp(c,e)		(e)
// #define lua_longassert(c)	((void)0)
// #endif
macro_rules! lua_assert {
    ($c:expr) => {};
}
macro_rules! check_exp {
    ($c:expr, $e:expr) => {
        ($e)
    };
}
macro_rules! lua_assert {
    ($c:expr) => {};
}
