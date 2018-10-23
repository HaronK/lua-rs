/* macro defining a nil value */
// #define NILCONSTANT	{NULL}, LUA_TNIL

// #define val_(o)		((o)->value_)

/* raw type tag of a TValue */
// #define rttype(o)	((o)->tt_)
macro_rules! rttype {
    ($o:expr) => {
        (($o).tt_)
    };
}

/* tag with no variants (bits 0-3) */
// #define novariant(x)	((x) & 0x0F)

/* type tag of a TValue (bits 0-3 for tags + variant bits 4-5) */
// #define ttype(o)	(rttype(o) & 0x3F)

/* type tag of a TValue with no variants (bits 0-3) */
// #define ttnov(o)	(novariant(rttype(o)))

/* Macros to test type */
// #define checktag(o,t)		(rttype(o) == (t))
macro_rules! checktag {
    ($o:expr, $t:expr) => {
        rttype!($o) == ($t)
    };
}
// #define checktype(o,t)		(ttnov(o) == (t))
// #define ttisnumber(o)		checktype((o), LUA_TNUMBER)
// #define ttisfloat(o)		checktag((o), LUA_TNUMFLT)
// #define ttisinteger(o)		checktag((o), LUA_TNUMINT)
// #define ttisnil(o)		checktag((o), LUA_TNIL)
// #define ttisboolean(o)		checktag((o), LUA_TBOOLEAN)
// #define ttislightuserdata(o)	checktag((o), LUA_TLIGHTUSERDATA)
// #define ttisstring(o)		checktype((o), LUA_TSTRING)
// #define ttisshrstring(o)	checktag((o), ctb(LUA_TSHRSTR))
// #define ttislngstring(o)	checktag((o), ctb(LUA_TLNGSTR))
// #define ttistable(o)		checktag((o), ctb(LUA_TTABLE))
// #define ttisfunction(o)		checktype(o, LUA_TFUNCTION)
// #define ttisclosure(o)		((rttype(o) & 0x1F) == LUA_TFUNCTION)
// #define ttisCclosure(o)		checktag((o), ctb(LUA_TCCL))
macro_rules! ttisCclosure {
    ($o:expr) => {
        checktag(($o), ctb!(LUA_TCCL))
    };
}
// #define ttisLclosure(o)		checktag((o), ctb(LUA_TLCL))
// #define ttislcf(o)		checktag((o), LUA_TLCF)
macro_rules! ttislcf {
    ($o:expr) => {
        checktag!(($o), LUA_TLCF)
    };
}
// #define ttisfulluserdata(o)	checktag((o), ctb(LUA_TUSERDATA))
// #define ttisthread(o)		checktag((o), ctb(LUA_TTHREAD))
// #define ttisdeadkey(o)		checktag((o), LUA_TDEADKEY)

/* Macros to access values */
// #define ivalue(o)	check_exp(ttisinteger(o), val_(o).i)
// #define fltvalue(o)	check_exp(ttisfloat(o), val_(o).n)
// #define nvalue(o)	check_exp(ttisnumber(o), \
// 	(ttisinteger(o) ? cast_num(ivalue(o)) : fltvalue(o)))
// #define gcvalue(o)	check_exp(iscollectable(o), val_(o).gc)
// #define pvalue(o)	check_exp(ttislightuserdata(o), val_(o).p)
// #define tsvalue(o)	check_exp(ttisstring(o), gco2ts(val_(o).gc))
// #define uvalue(o)	check_exp(ttisfulluserdata(o), gco2u(val_(o).gc))
// #define clvalue(o)	check_exp(ttisclosure(o), gco2cl(val_(o).gc))
// #define clLvalue(o)	check_exp(ttisLclosure(o), gco2lcl(val_(o).gc))
// #define clCvalue(o)	check_exp(ttisCclosure(o), gco2ccl(val_(o).gc))
// #define fvalue(o)	check_exp(ttislcf(o), val_(o).f)
// #define hvalue(o)	check_exp(ttistable(o), gco2t(val_(o).gc))
// #define bvalue(o)	check_exp(ttisboolean(o), val_(o).b)
// #define thvalue(o)	check_exp(ttisthread(o), gco2th(val_(o).gc))

/* a dead value may get the 'gc' field, but cannot access its contents */
// #define deadvalue(o)	check_exp(ttisdeadkey(o), cast(void *, val_(o).gc))

// #define l_isfalse(o)	(ttisnil(o) || (ttisboolean(o) && bvalue(o) == 0))

// #define iscollectable(o)	(rttype(o) & BIT_ISCOLLECTABLE)

/* Macros for internal tests */
// #define righttt(obj)		(ttype(obj) == gcvalue(obj)->tt)

// #define checkliveness(L,obj) \
// 	lua_longassert(!iscollectable(obj) || \
// 		(righttt(obj) && (L == NULL || !isdead(G(L),gcvalue(obj)))))

/* Macros to set values */
// #define settt_(o,t)	((o)->tt_=(t))

// #define setfltvalue(obj,x) \
//   { TValue *io=(obj); val_(io).n=(x); settt_(io, LUA_TNUMFLT); }

// #define chgfltvalue(obj,x) \
//   { TValue *io=(obj); lua_assert(ttisfloat(io)); val_(io).n=(x); }

// #define setivalue(obj,x) \
//   { TValue *io=(obj); val_(io).i=(x); settt_(io, LUA_TNUMINT); }

// #define chgivalue(obj,x) \
//   { TValue *io=(obj); lua_assert(ttisinteger(io)); val_(io).i=(x); }

// #define setnilvalue(obj) settt_(obj, LUA_TNIL)

// #define setfvalue(obj,x) \
//   { TValue *io=(obj); val_(io).f=(x); settt_(io, LUA_TLCF); }

// #define setpvalue(obj,x) \
//   { TValue *io=(obj); val_(io).p=(x); settt_(io, LUA_TLIGHTUSERDATA); }

// #define setbvalue(obj,x) \
//   { TValue *io=(obj); val_(io).b=(x); settt_(io, LUA_TBOOLEAN); }

// #define setgcovalue(L,obj,x) \
//   { TValue *io = (obj); GCObject *i_g=(x); \
//     val_(io).gc = i_g; settt_(io, ctb(i_g->tt)); }

// #define setsvalue(L,obj,x) \
//   { TValue *io = (obj); TString *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(x_->tt)); \
//     checkliveness(L,io); }

// #define setuvalue(L,obj,x) \
//   { TValue *io = (obj); Udata *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(LUA_TUSERDATA)); \
//     checkliveness(L,io); }

// #define setthvalue(L,obj,x) \
//   { TValue *io = (obj); lua_State *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(LUA_TTHREAD)); \
//     checkliveness(L,io); }

// #define setclLvalue(L,obj,x) \
//   { TValue *io = (obj); LClosure *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(LUA_TLCL)); \
//     checkliveness(L,io); }

// #define setclCvalue(L,obj,x) \
//   { TValue *io = (obj); CClosure *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(LUA_TCCL)); \
//     checkliveness(L,io); }

// #define sethvalue(L,obj,x) \
//   { TValue *io = (obj); Table *x_ = (x); \
//     val_(io).gc = obj2gco(x_); settt_(io, ctb(LUA_TTABLE)); \
//     checkliveness(L,io); }

// #define setdeadvalue(obj)	settt_(obj, LUA_TDEADKEY)

// #define setobj(L,obj1,obj2) \
// 	{ TValue *io1=(obj1); *io1 = *(obj2); \
// 	  (void)L; checkliveness(L,io1); }

/*
** different types of assignments, according to destination
*/

/* from stack to (same) stack */
// #define setobjs2s	setobj
/* to stack (not from same stack) */
// #define setobj2s	setobj
// #define setsvalue2s	setsvalue
// #define sethvalue2s	sethvalue
// #define setptvalue2s	setptvalue

/* from table to same table */
// #define setobjt2t	setobj

/* to new object */
// #define setobj2n	setobj
// #define setsvalue2n	setsvalue

/* to table (define it as an expression to be used in macros) */
// #define setobj2t(L,o1,o2)  ((void)L, *(o1)=*(o2), checkliveness(L,(o1)))

/* mark a tag as collectable */
// #define ctb(t)			((t) | BIT_ISCOLLECTABLE)
macro_rules! ctb {
    ($t:expr) => {
        (($t) | BIT_ISCOLLECTABLE)
    };
}

// ------------------------------------------------
// variadic macros

// TODO: implement!
#[macro_export]
macro_rules! luaO_pushfstring {
    ($lua_State:expr, $($args:tt)*) => {{
        let mut endptr: *const lua_char = 0 as *const lua_char;
        endptr
    }};
}
