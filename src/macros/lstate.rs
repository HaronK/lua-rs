
// #define isLua(ci)	((ci)->callstatus & CIST_LUA)

/* assume that CIST_OAH has offset 0 and that 'v' is strictly 0/1 */
// #define setoah(st,v)	((st) = ((st) & ~CIST_OAH) | (v))
// #define getoah(st)	((st) & CIST_OAH)

// #define G(L)	(L->l_G)

// #define cast_u(o)	cast(union GCUnion *, (o))

/* macros to convert a GCObject into a specific value */
// #define gco2ts(o)  \
// 	check_exp(novariant((o)->tt) == LUA_TSTRING, &((cast_u(o))->ts))
// #define gco2u(o)  check_exp((o)->tt == LUA_TUSERDATA, &((cast_u(o))->u))
// #define gco2lcl(o)  check_exp((o)->tt == LUA_TLCL, &((cast_u(o))->cl.l))
// #define gco2ccl(o)  check_exp((o)->tt == LUA_TCCL, &((cast_u(o))->cl.c))
// macro_rules! gco2ccl {
//     ($o:expr) => {
//         check_exp!((o).tt == LUA_TCCL, &((cast_u!(o)).cl.c))
//     };
// }
// #define gco2cl(o)  \
// 	check_exp(novariant((o)->tt) == LUA_TFUNCTION, &((cast_u(o))->cl))
// #define gco2t(o)  check_exp((o)->tt == LUA_TTABLE, &((cast_u(o))->h))
// #define gco2p(o)  check_exp((o)->tt == LUA_TPROTO, &((cast_u(o))->p))
// #define gco2th(o)  check_exp((o)->tt == LUA_TTHREAD, &((cast_u(o))->th))


/* macro to convert a Lua object into a GCObject */
// #define obj2gco(v) \
// 	check_exp(novariant((v)->tt) < LUA_TDEADKEY, (&(cast_u(v)->gc)))


/* actual number of total bytes allocated */
// #define gettotalbytes(g)	cast(lu_mem, (g)->totalbytes + (g)->GCdebt)
