use super::prelude::*;

/*
** equality for short strings, which are always internalized
*/
#[no_mangle]
pub unsafe extern "C" fn luaS_hash(
    mut str: *const lua_char,
    mut l: size_t,
    mut seed: lua_uint,
) -> lua_uint {
    let mut h: lua_uint = seed ^ l as lua_uint;
    let mut step: size_t = (l >> 5i32).wrapping_add(1i32 as lua_ulong);
    while l >= step {
        h ^= (h << 5i32).wrapping_add(h >> 2i32).wrapping_add(
            *str.offset(l.wrapping_sub(1i32 as lua_ulong) as isize) as lu_byte as lua_uint,
        );
        l = (l as lua_ulong).wrapping_sub(step) as size_t as size_t
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_hashlongstr(mut ts: *mut TString) -> lua_uint {
    if (*ts).extra as lua_int == 0i32 {
        /* no hash? */
        (*ts).hash = luaS_hash(
            (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            (*ts).u.lnglen,
            (*ts).hash,
        );
        /* now it has its hash */
        (*ts).extra = 1i32 as lu_byte
    }
    return (*ts).hash;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_eqlngstr(mut a: *mut TString, mut b: *mut TString) -> lua_int {
    let mut len: size_t = (*a).u.lnglen;
    /* same instance or... */
    return (a == b
        || len == (*b).u.lnglen
            && memcmp(
                (a as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                (b as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                len,
            ) == 0i32) as lua_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_resize(mut L: *mut lua_State, mut newsize: lua_int) -> () {
    let mut i: lua_int = 0;
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    if newsize > (*tb).size {
        /* grow table if needed */
        if ::std::mem::size_of::<lua_int>() as lua_ulong
            >= ::std::mem::size_of::<size_t>() as lua_ulong
            && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut lua_void,
            ((*tb).size as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
            (newsize as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
        ) as *mut *mut TString;
        i = (*tb).size;
        while i < newsize {
            let ref mut fresh0 = *(*tb).hash.offset(i as isize);
            *fresh0 = 0 as *mut TString;
            i += 1
        }
    }
    i = 0i32;
    while i < (*tb).size {
        /* rehash */
        let mut p: *mut TString = *(*tb).hash.offset(i as isize);
        let ref mut fresh1 = *(*tb).hash.offset(i as isize);
        *fresh1 = 0 as *mut TString;
        while !p.is_null() {
            /* for each node in the list */
            /* save next */
            let mut hnext: *mut TString = (*p).u.hnext;
            /* new position */
            let mut h: lua_uint = ((*p).hash & (newsize - 1i32) as lua_uint) as lua_int as lua_uint;
            /* chain it */
            (*p).u.hnext = *(*tb).hash.offset(h as isize);
            let ref mut fresh2 = *(*tb).hash.offset(h as isize);
            *fresh2 = p;
            p = hnext
        }
        i += 1
    }
    if newsize < (*tb).size {
        /* shrink table if needed */
        /* vanishing slice should be empty */
        if ::std::mem::size_of::<lua_int>() as lua_ulong
            >= ::std::mem::size_of::<size_t>() as lua_ulong
            && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
                > (!(0i32 as size_t))
                    .wrapping_div(::std::mem::size_of::<*mut TString>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
        };
        (*tb).hash = luaM_realloc_(
            L,
            (*tb).hash as *mut lua_void,
            ((*tb).size as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
            (newsize as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut TString>() as lua_ulong),
        ) as *mut *mut TString
    }
    (*tb).size = newsize;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_clearcache(mut g: *mut global_State) -> () {
    let mut i: lua_int = 0;
    let mut j: lua_int = 0;
    i = 0i32;
    while i < 53i32 {
        j = 0i32;
        while j < 2i32 {
            /* will entry be collected? */
            if 0 != (*(*g).strcache[i as usize][j as usize]).marked as lua_int
                & (1i32 << 0i32 | 1i32 << 1i32)
            {
                /* replace it with something fixed */
                (*g).strcache[i as usize][j as usize] = (*g).memerrmsg
            }
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_init(mut L: *mut lua_State) -> () {
    let mut g: *mut global_State = (*L).l_G;
    let mut i: lua_int = 0;
    let mut j: lua_int = 0;
    /* initial size of string table */
    luaS_resize(L, 128i32);
    /* pre-create memory-error message */
    (*g).memerrmsg = luaS_newlstr(
        L,
        s!(b"not enough memory\x00"),
        (::std::mem::size_of::<[lua_char; 18]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    /* it should never be collected */
    luaC_fix(L, &mut (*((*g).memerrmsg as *mut GCUnion)).gc);
    /* fill cache with valid strings */
    i = 0i32;
    while i < 53i32 {
        j = 0i32;
        while j < 2i32 {
            (*g).strcache[i as usize][j as usize] = (*g).memerrmsg;
            j += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newlstr(
    mut L: *mut lua_State,
    mut str: *const lua_char,
    mut l: size_t,
) -> *mut TString {
    /* short string? */
    if l <= 40i32 as lua_ulong {
        return internshrstr(L, str, l);
    } else {
        let mut ts: *mut TString = 0 as *mut TString;
        if l >= if (::std::mem::size_of::<size_t>() as lua_ulong)
            < ::std::mem::size_of::<lua_Integer>() as lua_ulong
        {
            !(0i32 as size_t)
        } else {
            9223372036854775807i64 as size_t
        }
        .wrapping_sub(::std::mem::size_of::<TString>() as lua_ulong)
        .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
        {
            luaM_toobig(L);
        } else {
            ts = luaS_createlngstrobj(L, l);
            memcpy(
                (ts as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *mut lua_void,
                str as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            );
            return ts;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_createlngstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = createstrobj(L, l, 4i32 | 1i32 << 4i32, (*(*L).l_G).seed);
    (*ts).u.lnglen = l;
    return ts;
}
/*
** creates a new string object
*/
unsafe extern "C" fn createstrobj(
    mut L: *mut lua_State,
    mut l: size_t,
    mut tag: lua_int,
    mut h: lua_uint,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    /* total size of TString object */
    let mut totalsize: size_t = 0;
    totalsize = (::std::mem::size_of::<UTString>() as lua_ulong).wrapping_add(
        l.wrapping_add(1i32 as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    o = luaC_newobj(L, tag, totalsize);
    ts = &mut (*(o as *mut GCUnion)).ts;
    (*ts).hash = h;
    (*ts).extra = 0i32 as lu_byte;
    /* ending 0 */
    *(ts as *mut lua_char)
        .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
        .offset(l as isize) = '\u{0}' as i32 as lua_char;
    return ts;
}
/*
** checks whether short string exists and reuses it or creates a new one
*/
unsafe extern "C" fn internshrstr(
    mut L: *mut lua_State,
    mut str: *const lua_char,
    mut l: size_t,
) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    let mut g: *mut global_State = (*L).l_G;
    let mut h: lua_uint = luaS_hash(str, l, (*g).seed);
    let mut list: *mut *mut TString = &mut *(*g)
        .strt
        .hash
        .offset((h & ((*g).strt.size - 1i32) as lua_uint) as lua_int as isize)
        as *mut *mut TString;
    /* otherwise 'memcmp'/'memcpy' are undefined */
    ts = *list;
    while !ts.is_null() {
        if l == (*ts).shrlen as lua_ulong
            && memcmp(
                str as *const lua_void,
                (ts as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
                    as *const lua_void,
                l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
            ) == 0i32
        {
            /* found! */
            /* dead (but not collected yet)? */
            if 0 == ((*ts).marked as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32))
                & ((*g).currentwhite as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32))
            {
                /* resurrect it */
                (*ts).marked = ((*ts).marked as lua_int ^ (1i32 << 0i32 | 1i32 << 1i32)) as lu_byte
            }
            return ts;
        } else {
            ts = (*ts).u.hnext
        }
    }
    if (*g).strt.nuse >= (*g).strt.size && (*g).strt.size <= 2147483647i32 / 2i32 {
        luaS_resize(L, (*g).strt.size * 2i32);
        /* recompute with new size */
        list = &mut *(*g)
            .strt
            .hash
            .offset((h & ((*g).strt.size - 1i32) as lua_uint) as lua_int as isize)
            as *mut *mut TString
    }
    ts = createstrobj(L, l, 4i32 | 0i32 << 4i32, h);
    memcpy(
        (ts as *mut lua_char).offset(::std::mem::size_of::<UTString>() as lua_ulong as isize)
            as *mut lua_void,
        str as *const lua_void,
        l.wrapping_mul(::std::mem::size_of::<lua_char>() as lua_ulong),
    );
    (*ts).shrlen = l as lu_byte;
    (*ts).u.hnext = *list;
    *list = ts;
    (*g).strt.nuse += 1;
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_remove(mut L: *mut lua_State, mut ts: *mut TString) -> () {
    let mut tb: *mut stringtable = &mut (*(*L).l_G).strt;
    let mut p: *mut *mut TString = &mut *(*tb)
        .hash
        .offset(((*ts).hash & ((*tb).size - 1i32) as lua_uint) as lua_int as isize)
        as *mut *mut TString;
    /* find previous element */
    while *p != ts {
        p = &mut (**p).u.hnext
    }
    /* remove element from its list */
    *p = (**p).u.hnext;
    (*tb).nuse -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn luaS_newudata(mut L: *mut lua_State, mut s: size_t) -> *mut Udata {
    let mut u: *mut Udata = 0 as *mut Udata;
    let mut o: *mut GCObject = 0 as *mut GCObject;
    if s > if (::std::mem::size_of::<size_t>() as lua_ulong)
        < ::std::mem::size_of::<lua_Integer>() as lua_ulong
    {
        !(0i32 as size_t)
    } else {
        9223372036854775807i64 as size_t
    }
    .wrapping_sub(::std::mem::size_of::<Udata>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
        o = luaC_newobj(
            L,
            7i32,
            (::std::mem::size_of::<UUdata>() as lua_ulong).wrapping_add(s),
        );
        u = &mut (*(o as *mut GCUnion)).u;
        (*u).len = s;
        (*u).metatable = 0 as *mut Table;
        let mut io: *const TValue = &luaO_nilobject_;
        let mut iu: *mut Udata = u;
        (*iu).user_ = (*io).value_;
        (*iu).ttuv_ = (*io).tt_ as lu_byte;
        return u;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaS_new(mut L: *mut lua_State, mut str: *const lua_char) -> *mut TString {
    /* hash */
    let mut i: lua_uint = ((str as size_t
        & (2147483647i32 as lua_uint)
            .wrapping_mul(2u32)
            .wrapping_add(1u32) as lua_ulong) as lua_uint)
        .wrapping_rem(53i32 as lua_uint);
    let mut j: lua_int = 0;
    let mut p: *mut *mut TString = (*(*L).l_G).strcache[i as usize].as_mut_ptr();
    j = 0i32;
    while j < 2i32 {
        /* hit? */
        if strcmp(
            str,
            (*p.offset(j as isize) as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
        ) == 0i32
        {
            /* that is it */
            return *p.offset(j as isize);
        } else {
            j += 1
        }
    }
    /* normal route */
    j = 2i32 - 1i32;
    while j > 0i32 {
        /* move out last element */
        let ref mut fresh3 = *p.offset(j as isize);
        *fresh3 = *p.offset((j - 1i32) as isize);
        j -= 1
    }
    /* new element is first in the list */
    let ref mut fresh4 = *p.offset(0isize);
    *fresh4 = luaS_newlstr(L, str, strlen(str));
    return *p.offset(0isize);
}
