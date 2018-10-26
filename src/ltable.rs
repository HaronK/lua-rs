use super::prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AuxsetnodeT {
    pub t: *mut Table,
    pub nhsize: lua_uint,
}

/* true when 't' is using 'dummynode' as its hash part */
/* allocated size for hash nodes */
/* returns the key, given the value of a table entry */
#[no_mangle]
pub unsafe extern "C" fn luaH_getint(mut t: *mut Table, mut key: lua_Integer) -> *const TValue {
    /* (1 <= key && key <= t->sizearray) */
    if (key as lua_Unsigned).wrapping_sub(1i32 as lua_ulonglong) < (*t).sizearray as lua_ulonglong {
        return &mut *(*t).array.offset((key - 1i32 as lua_longlong) as isize) as *mut TValue;
    } else {
        let mut n: *mut Node = &mut *(*t).node.offset(
            (key & ((1i32 << (*t).lsizenode as lua_int) - 1i32) as lua_longlong) as lua_int
                as isize,
        ) as *mut Node;
        loop {
            /* check whether 'key' is somewhere in the chain */
            if (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 3i32 | 1i32 << 4i32
                && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                    .value_
                    .i
                    == key
            {
                /* that's it */
                return &mut (*n).i_val;
            } else {
                let mut nx: lua_int = (*n).i_key.nk.next;
                if nx == 0i32 {
                    break;
                }
                n = n.offset(nx as isize)
            }
        }
        return &luaO_nilobject_;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_setint(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: lua_Integer,
    mut value: *mut TValue,
) -> () {
    let mut p: *const TValue = luaH_getint(t, key);
    let mut cell: *mut TValue = 0 as *mut TValue;
    if p != &luaO_nilobject_ as *const TValue {
        cell = p as *mut TValue
    } else {
        let mut k: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut k;
        (*io).value_.i = key;
        (*io).tt_ = 3i32 | 1i32 << 4i32;
        cell = luaH_newkey(L, t, &mut k)
    }
    *cell = *value;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_newkey(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut mp: *mut Node = 0 as *mut Node;
    let mut aux: TValue = lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0,
    };
    if (*key).tt_ == 0i32 {
        luaG_runerror!(L, s!(b"table index is nil\x00"),);
    } else {
        if (*key).tt_ == 3i32 | 0i32 << 4i32 {
            let mut k: lua_Integer = 0;
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                /* does index fit in an integer? */
                let mut io: *mut TValue = &mut aux;
                (*io).value_.i = k;
                (*io).tt_ = 3i32 | 1i32 << 4i32;
                /* insert it as an integer */
                key = &mut aux
            } else if !((*key).value_.n == (*key).value_.n) {
                luaG_runerror!(L, s!(b"table index is NaN\x00"),);
            }
        }
        mp = mainposition(t, key);
        if !((*mp).i_val.tt_ == 0i32) || (*t).lastfree.is_null() {
            /* main position is taken? */
            let mut othern: *mut Node = 0 as *mut Node;
            /* get a free place */
            let mut f: *mut Node = getfreepos(t);
            if f.is_null() {
                /* cannot find a free place? */
                /* grow table */
                rehash(L, t, key);
                /* whatever called 'newkey' takes care of TM cache */
                /* insert key into grown table */
                return luaH_set(L, t, key);
            } else {
                othern = mainposition(t, &mut (*mp).i_key.tvk as *mut TValue as *const TValue);
                if othern != mp {
                    /* is colliding node out of its main position? */
                    /* yes; move colliding node into free position */
                    /* find previous */
                    while othern.offset((*othern).i_key.nk.next as isize) != mp {
                        othern = othern.offset((*othern).i_key.nk.next as isize)
                    }
                    /* rechain to point to 'f' */
                    (*othern).i_key.nk.next = f.wrapping_offset_from(othern) as lua_long as lua_int;
                    /* copy colliding node into free pos. (mp->next also goes) */
                    *f = *mp;
                    if (*mp).i_key.nk.next != 0i32 {
                        /* correct 'next' */
                        (*f).i_key.nk.next += mp.wrapping_offset_from(f) as lua_long as lua_int;
                        /* now 'mp' is free */
                        (*mp).i_key.nk.next = 0i32
                    }
                    (*mp).i_val.tt_ = 0i32
                } else {
                    /* colliding node is in its own main position */
                    /* new node will go into free position */
                    if (*mp).i_key.nk.next != 0i32 {
                        /* chain new position */
                        (*f).i_key.nk.next = mp
                            .offset((*mp).i_key.nk.next as isize)
                            .wrapping_offset_from(f)
                            as lua_long as lua_int
                    }
                    (*mp).i_key.nk.next = f.wrapping_offset_from(mp) as lua_long as lua_int;
                    mp = f
                }
            }
        }
        let mut k_: *mut TKey = &mut (*mp).i_key;
        let mut io_: *const TValue = key;
        (*k_).nk.value_ = (*io_).value_;
        (*k_).nk.tt_ = (*io_).tt_;
        if 0 != (*key).tt_ & 1i32 << 6i32
            && 0 != (*t).marked as lua_int & 1i32 << 2i32
            && 0 != (*(*key).value_.gc).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
        {
            luaC_barrierback_(L, t);
        } else {
        };
        return &mut (*mp).i_val;
    };
}
unsafe extern "C" fn getfreepos(mut t: *mut Table) -> *mut Node {
    if !(*t).lastfree.is_null() {
        while (*t).lastfree > (*t).node {
            (*t).lastfree = (*t).lastfree.offset(-1isize);
            if !((*(&mut (*(*t).lastfree).i_key.tvk as *mut TValue as *const TValue)).tt_ == 0i32) {
                continue;
            }
            return (*t).lastfree;
        }
    }
    /* could not find a free place */
    return 0 as *mut Node;
}
/*
** returns the 'main' position of an element in a table (that is, the index
** of its hash value)
*/
unsafe extern "C" fn mainposition(mut t: *const Table, mut key: *const TValue) -> *mut Node {
    match (*key).tt_ & 0x3fi32 {
        19 => {
            return &mut *(*t).node.offset(
                ((*key).value_.i & ((1i32 << (*t).lsizenode as lua_int) - 1i32) as lua_longlong)
                    as lua_int as isize,
            ) as *mut Node
        }
        3 => {
            return &mut *(*t).node.offset(
                (l_hashfloat((*key).value_.n) % ((1i32 << (*t).lsizenode as lua_int) - 1i32 | 1i32))
                    as isize,
            ) as *mut Node
        }
        4 => {
            return &mut *(*t).node.offset(
                ((*((*key).value_.gc as *mut GCUnion)).ts.hash
                    & ((1i32 << (*t).lsizenode as lua_int) - 1i32) as lua_uint)
                    as lua_int as isize,
            ) as *mut Node
        }
        20 => {
            return &mut *(*t).node.offset(
                (luaS_hashlongstr(&mut (*((*key).value_.gc as *mut GCUnion)).ts)
                    & ((1i32 << (*t).lsizenode as lua_int) - 1i32) as lua_uint)
                    as lua_int as isize,
            ) as *mut Node
        }
        1 => {
            return &mut *(*t)
                .node
                .offset(((*key).value_.b & (1i32 << (*t).lsizenode as lua_int) - 1i32) as isize)
                as *mut Node
        }
        2 => {
            return &mut *(*t).node.offset(
                (((*key).value_.p as size_t
                    & (2147483647i32 as lua_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as lua_ulong) as lua_uint)
                    .wrapping_rem(((1i32 << (*t).lsizenode as lua_int) - 1i32 | 1i32) as lua_uint)
                    as isize,
            ) as *mut Node
        }
        22 => {
            return &mut *(*t).node.offset(
                ((::std::mem::transmute::<lua_CFunction, size_t>((*key).value_.f)
                    & (2147483647i32 as lua_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as lua_ulong) as lua_uint)
                    .wrapping_rem(((1i32 << (*t).lsizenode as lua_int) - 1i32 | 1i32) as lua_uint)
                    as isize,
            ) as *mut Node;
        }
        _ => {
            return &mut *(*t).node.offset(
                (((*key).value_.gc as size_t
                    & (2147483647i32 as lua_uint)
                        .wrapping_mul(2u32)
                        .wrapping_add(1u32) as lua_ulong) as lua_uint)
                    .wrapping_rem(((1i32 << (*t).lsizenode as lua_int) - 1i32 | 1i32) as lua_uint)
                    as isize,
            ) as *mut Node
        }
    };
}
/* value */
/* key */
/*
** Hash for floating-point numbers.
** The main computation should be just
**     n = frexp(n, &i); return (n * INT_MAX) + i
** but there are some numerical subtleties.
** In a two-complement representation, INT_MAX does not has an exact
** representation as a float, but INT_MIN does; because the absolute
** value of 'frexp' is smaller than 1 (unless 'n' is inf/NaN), the
** absolute value of the product 'frexp * -INT_MIN' is smaller or equal
** to INT_MAX. Next, the use of 'unsigned int' avoids overflows when
** adding 'i'; the use of '~u' (instead of '-u') avoids problems with
** INT_MIN.
*/
unsafe extern "C" fn l_hashfloat(mut n: lua_Number) -> lua_int {
    let mut i: lua_int = 0;
    let mut ni: lua_Integer = 0;
    n = frexp(n, &mut i) * -((-2147483647i32 - 1i32) as lua_Number);
    if !(n >= (-9223372036854775807i64 - 1i64) as lua_double
        && n < -((-9223372036854775807i64 - 1i64) as lua_double)
        && {
            ni = n as lua_longlong;
            0 != 1i32
        }) {
        /* is 'n' inf/-inf/NaN? */
        return 0i32;
    } else {
        /* normal case */
        let mut u: lua_uint = (i as lua_uint).wrapping_add(ni as lua_uint);
        return (if u <= 2147483647i32 as lua_uint {
            u
        } else {
            !u
        }) as lua_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_set(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: *const TValue,
) -> *mut TValue {
    let mut p: *const TValue = luaH_get(t, key);
    if p != &luaO_nilobject_ as *const TValue {
        return p as *mut TValue;
    } else {
        return luaH_newkey(L, t, key);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_get(mut t: *mut Table, mut key: *const TValue) -> *const TValue {
    match (*key).tt_ & 0x3fi32 {
        4 => return luaH_getshortstr(t, &mut (*((*key).value_.gc as *mut GCUnion)).ts),
        19 => return luaH_getint(t, (*key).value_.i),
        0 => return &luaO_nilobject_,
        3 => {
            let mut k: lua_Integer = 0;
            /* index is int? */
            if 0 != luaV_tointeger(key, &mut k, 0i32) {
                /* use specialized version */
                return luaH_getint(t, k);
            }
        }
        _ => {}
    }
    /* else... */
    /* FALLTHROUGH */
    return getgeneric(t, key);
}
/*
** "Generic" get version. (Not that generic: not valid for integers,
** which may be in array part, nor for floats with integral values.)
*/
unsafe extern "C" fn getgeneric(mut t: *mut Table, mut key: *const TValue) -> *const TValue {
    let mut n: *mut Node = mainposition(t, key);
    loop {
        /* check whether 'key' is somewhere in the chain */
        if 0 != luaV_equalobj(
            0 as *mut lua_State,
            &mut (*n).i_key.tvk as *mut TValue as *const TValue,
            key,
        ) {
            /* that's it */
            return &mut (*n).i_val;
        } else {
            let mut nx: lua_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                /* not found */
                return &luaO_nilobject_;
            } else {
                n = n.offset(nx as isize)
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getshortstr(
    mut t: *mut Table,
    mut key: *mut TString,
) -> *const TValue {
    let mut n: *mut Node = &mut *(*t).node.offset(
        ((*key).hash & ((1i32 << (*t).lsizenode as lua_int) - 1i32) as lua_uint) as lua_int
            as isize,
    ) as *mut Node;
    loop {
        /* check whether 'key' is somewhere in the chain */
        let mut k: *const TValue = &mut (*n).i_key.tvk as *mut TValue as *const TValue;
        if (*k).tt_ == 4i32 | 0i32 << 4i32 | 1i32 << 6i32
            && &mut (*((*k).value_.gc as *mut GCUnion)).ts as *mut TString == key
        {
            /* that's it */
            return &mut (*n).i_val;
        } else {
            let mut nx: lua_int = (*n).i_key.nk.next;
            if nx == 0i32 {
                /* not found */
                return &luaO_nilobject_;
            } else {
                n = n.offset(nx as isize)
            }
        }
    }
}
/*
** nums[i] = number of keys 'k' where 2^(i - 1) < k <= 2^i
*/
unsafe extern "C" fn rehash(mut L: *mut lua_State, mut t: *mut Table, mut ek: *const TValue) -> () {
    /* optimal size for array part */
    let mut asize: lua_uint = 0;
    /* number of keys in the array part */
    let mut na: lua_uint = 0;
    let mut nums: [lua_uint; 32] = [0; 32];
    let mut i: lua_int = 0;
    let mut totaluse: lua_int = 0;
    i = 0i32;
    while i
        <= (::std::mem::size_of::<lua_int>() as lua_ulong)
            .wrapping_mul(8i32 as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int
    {
        /* reset counts */
        nums[i as usize] = 0i32 as lua_uint;
        i += 1
    }
    /* count keys in array part */
    na = numusearray(t, nums.as_mut_ptr());
    /* all those keys are integer keys */
    totaluse = na as lua_int;
    /* count keys in hash part */
    totaluse += numusehash(t, nums.as_mut_ptr(), &mut na);
    /* count extra key */
    na = na.wrapping_add(countint(ek, nums.as_mut_ptr()) as lua_uint);
    totaluse += 1;
    /* compute new size for array part */
    asize = computesizes(nums.as_mut_ptr(), &mut na);
    /* resize the table to new computed sizes */
    luaH_resize(L, t, asize, (totaluse as lua_uint).wrapping_sub(na));
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resize(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: lua_uint,
    mut nhsize: lua_uint,
) -> () {
    let mut i: lua_uint = 0;
    let mut j: lua_int = 0;
    let mut asn: AuxsetnodeT = AuxsetnodeT {
        t: 0 as *mut Table,
        nhsize: 0,
    };
    let mut oldasize: lua_uint = (*t).sizearray;
    let mut oldhsize: lua_int = if (*t).lastfree.is_null() {
        0i32
    } else {
        1i32 << (*t).lsizenode as lua_int
    };
    /* save old hash ... */
    let mut nold: *mut Node = (*t).node;
    /* array part must grow? */
    if nasize > oldasize {
        setarrayvector(L, t, nasize);
    }
    /* create new hash part with appropriate size */
    asn.t = t;
    asn.nhsize = nhsize;
    if luaD_rawrunprotected(
        L,
        Some(auxsetnode),
        &mut asn as *mut AuxsetnodeT as *mut lua_void,
    ) != 0i32
    {
        /* mem. error? */
        /* array back to its original size */
        setarrayvector(L, t, oldasize);
        /* rethrow memory error */
        luaD_throw(L, 4i32);
    } else {
        if nasize < oldasize {
            /* array part must shrink? */
            (*t).sizearray = nasize;
            /* re-insert elements from vanishing slice */
            i = nasize;
            while i < oldasize {
                if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
                    luaH_setint(
                        L,
                        t,
                        i.wrapping_add(1i32 as lua_uint) as lua_Integer,
                        &mut *(*t).array.offset(i as isize),
                    );
                }
                i = i.wrapping_add(1)
            }
            /* shrink array */
            if ::std::mem::size_of::<lua_uint>() as lua_ulong
                >= ::std::mem::size_of::<size_t>() as lua_ulong
                && (nasize as size_t).wrapping_add(1i32 as lua_ulong)
                    > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as lua_ulong)
            {
                luaM_toobig(L);
            } else {
            };
            (*t).array = luaM_realloc_(
                L,
                (*t).array as *mut lua_void,
                (oldasize as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
                (nasize as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
            ) as *mut TValue
        }
        /* re-insert elements from hash part */
        j = oldhsize - 1i32;
        while j >= 0i32 {
            let mut old: *mut Node = nold.offset(j as isize);
            if !((*old).i_val.tt_ == 0i32) {
                /* doesn't need barrier/invalidate cache, as entry was
                already present in the table */
                let mut io1: *mut TValue =
                    luaH_set(L, t, &mut (*old).i_key.tvk as *mut TValue as *const TValue);
                *io1 = (*old).i_val
            }
            j -= 1
        }
        /* not the dummy node? */
        if oldhsize > 0i32 {
            /* free old hash */
            luaM_realloc_(
                L,
                nold as *mut lua_void,
                (oldhsize as size_t).wrapping_mul(::std::mem::size_of::<Node>() as lua_ulong),
                0i32 as size_t,
            );
        }
        return;
    };
}
unsafe extern "C" fn setarrayvector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: lua_uint,
) -> () {
    let mut i: lua_uint = 0;
    if ::std::mem::size_of::<lua_uint>() as lua_ulong
        >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (size as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*t).array = luaM_realloc_(
        L,
        (*t).array as *mut lua_void,
        ((*t).sizearray as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
        (size as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
    ) as *mut TValue;
    i = (*t).sizearray;
    while i < size {
        (*(*t).array.offset(i as isize)).tt_ = 0i32;
        i = i.wrapping_add(1)
    }
    (*t).sizearray = size;
}
unsafe extern "C" fn auxsetnode(mut L: *mut lua_State, mut ud: *mut lua_void) -> () {
    let mut asn: *mut AuxsetnodeT = ud as *mut AuxsetnodeT;
    setnodevector(L, (*asn).t, (*asn).nhsize);
}
unsafe extern "C" fn setnodevector(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut size: lua_uint,
) -> () {
    if size == 0i32 as lua_uint {
        /* no elements to hash part? */
        /* use common 'dummynode' */
        (*t).node = &dummynode_ as *const Node as *mut Node;
        (*t).lsizenode = 0i32 as lu_byte;
        /* signal that it is using dummy node */
        (*t).lastfree = 0 as *mut Node
    } else {
        let mut i: lua_int = 0;
        let mut lsize: lua_int = luaO_ceillog2(size);
        if lsize
            > (::std::mem::size_of::<lua_int>() as lua_ulong)
                .wrapping_mul(8i32 as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong) as lua_int
                - 1i32
        {
            luaG_runerror!(L, s!(b"table overflow\x00"));
        } else {
            size = (1i32 << lsize) as lua_uint;
            if ::std::mem::size_of::<lua_uint>() as lua_ulong
                >= ::std::mem::size_of::<size_t>() as lua_ulong
                && (size as size_t).wrapping_add(1i32 as lua_ulong)
                    > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Node>() as lua_ulong)
            {
                luaM_toobig(L);
            } else {
            };
            (*t).node = luaM_realloc_(
                L,
                0 as *mut lua_void,
                (0i32 as lua_ulong).wrapping_mul(::std::mem::size_of::<Node>() as lua_ulong),
                (size as lua_ulong).wrapping_mul(::std::mem::size_of::<Node>() as lua_ulong),
            ) as *mut Node;
            i = 0i32;
            while i < size as lua_int {
                let mut n: *mut Node = &mut *(*t).node.offset(i as isize) as *mut Node;
                (*n).i_key.nk.next = 0i32;
                (*n).i_key.nk.tt_ = 0i32;
                (*n).i_val.tt_ = 0i32;
                i += 1
            }
            (*t).lsizenode = lsize as lu_byte;
            /* all positions are free */
            (*t).lastfree = &mut *(*t).node.offset(size as isize) as *mut Node
        }
    };
}
/*
** $Id: ltable.c,v 2.118.1.4 2018/06/08 16:22:51 roberto Exp $
** Lua tables (hash)
** See Copyright Notice in lua.h
*/
/*
** Implementation of tables (aka arrays, objects, or hash tables).
** Tables keep its elements in two parts: an array part and a hash part.
** Non-negative integer keys are all candidates to be kept in the array
** part. The actual size of the array is the largest 'n' such that
** more than half the slots between 1 and n are in use.
** Hash uses a mix of chained scatter table with Brent's variation.
** A main invariant of these tables is that, if an element is not
** in its main position (i.e. the 'original' position that its hash gives
** to it), then the colliding element is in its own main position.
** Hence even when the load factor reaches 100%, performance remains good.
*/
/*
** Maximum size of array part (MAXASIZE) is 2^MAXABITS. MAXABITS is
** the largest integer such that MAXASIZE fits in an unsigned int.
*/
/*
** Maximum size of hash part is 2^MAXHBITS. MAXHBITS is the largest
** integer such that 2^MAXHBITS fits in a signed int. (Note that the
** maximum number of elements in a table, 2^MAXABITS + 2^MAXHBITS, still
** fits comfortably in an unsigned int.)
*/
/*
** for some types, it is better to avoid modulus by power of 2, as
** they tend to have many 2 factors.
*/
static mut dummynode_: Node = Node {
    i_val: lua_TValue {
        value_: Value {
            gc: 0 as *const GCObject as *mut GCObject,
        },
        tt_: 0i32,
    },
    i_key: TKey {
        nk: unnamed_5 {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0i32,
            next: 0i32,
        },
    },
};
/*
** {=============================================================
** Rehash
** ==============================================================
*/
/*
** Compute the optimal size for the array part of table 't'. 'nums' is a
** "count array" where 'nums[i]' is the number of integers in the table
** between 2^(i - 1) + 1 and 2^i. 'pna' enters with the total number of
** integer keys in the table and leaves with the number of keys that
** will go to the array part; return the optimal size.
*/
unsafe extern "C" fn computesizes(mut nums: *mut lua_uint, mut pna: *mut lua_uint) -> lua_uint {
    let mut i: lua_int = 0;
    /* 2^i (candidate for optimal size) */
    let mut twotoi: lua_uint = 0;
    /* number of elements smaller than 2^i */
    let mut a: lua_uint = 0i32 as lua_uint;
    /* number of elements to go to array part */
    let mut na: lua_uint = 0i32 as lua_uint;
    /* optimal size for array part */
    let mut optimal: lua_uint = 0i32 as lua_uint;
    /* loop while keys can fill more than half of total size */
    i = 0i32;
    twotoi = 1i32 as lua_uint;
    while twotoi > 0i32 as lua_uint && *pna > twotoi.wrapping_div(2i32 as lua_uint) {
        if *nums.offset(i as isize) > 0i32 as lua_uint {
            a = a.wrapping_add(*nums.offset(i as isize));
            if a > twotoi.wrapping_div(2i32 as lua_uint) {
                /* more than half elements present? */
                /* optimal size (till now) */
                optimal = twotoi;
                /* all elements up to 'optimal' will go to array part */
                na = a
            }
        }
        i += 1;
        twotoi = twotoi.wrapping_mul(2i32 as lua_uint)
    }
    *pna = na;
    return optimal;
}
unsafe extern "C" fn countint(mut key: *const TValue, mut nums: *mut lua_uint) -> lua_int {
    let mut k: lua_uint = arrayindex(key);
    if k != 0i32 as lua_uint {
        /* is 'key' an appropriate array index? */
        /* count as such */
        let ref mut fresh0 = *nums.offset(luaO_ceillog2(k) as isize);
        *fresh0 = (*fresh0).wrapping_add(1);
        return 1i32;
    } else {
        return 0i32;
    };
}
/*
** returns the index for 'key' if 'key' is an appropriate key to live in
** the array part of the table, 0 otherwise.
*/
unsafe extern "C" fn arrayindex(mut key: *const TValue) -> lua_uint {
    if (*key).tt_ == 3i32 | 1i32 << 4i32 {
        let mut k: lua_Integer = (*key).value_.i;
        if (0i32 as lua_longlong) < k
            && k as lua_Unsigned
                <= (1u32
                    << (::std::mem::size_of::<lua_int>() as lua_ulong)
                        .wrapping_mul(8i32 as lua_ulong)
                        .wrapping_sub(1i32 as lua_ulong) as lua_int)
                    as lua_ulonglong
        {
            /* 'key' is an appropriate array index */
            return k as lua_uint;
        }
    }
    /* 'key' did not match some condition */
    return 0i32 as lua_uint;
}
unsafe extern "C" fn numusehash(
    mut t: *const Table,
    mut nums: *mut lua_uint,
    mut pna: *mut lua_uint,
) -> lua_int {
    /* total number of elements */
    let mut totaluse: lua_int = 0i32;
    /* elements added to 'nums' (can go to array part) */
    let mut ause: lua_int = 0i32;
    let mut i: lua_int = 1i32 << (*t).lsizenode as lua_int;
    loop {
        let fresh1 = i;
        i = i - 1;
        if !(0 != fresh1) {
            break;
        }
        let mut n: *mut Node = &mut *(*t).node.offset(i as isize) as *mut Node;
        if (*n).i_val.tt_ == 0i32 {
            continue;
        }
        ause += countint(&mut (*n).i_key.tvk as *mut TValue as *const TValue, nums);
        totaluse += 1
    }
    *pna = (*pna).wrapping_add(ause as lua_uint);
    return totaluse;
}
/*
** Count keys in array part of table 't': Fill 'nums[i]' with
** number of keys that will go into corresponding slice and return
** total number of non-nil keys.
*/
unsafe extern "C" fn numusearray(mut t: *const Table, mut nums: *mut lua_uint) -> lua_uint {
    let mut lg: lua_int = 0;
    /* 2^lg */
    let mut ttlg: lua_uint = 0;
    /* summation of 'nums' */
    let mut ause: lua_uint = 0i32 as lua_uint;
    /* count to traverse all array keys */
    let mut i: lua_uint = 1i32 as lua_uint;
    /* traverse each slice */
    lg = 0i32;
    ttlg = 1i32 as lua_uint;
    while lg
        <= (::std::mem::size_of::<lua_int>() as lua_ulong)
            .wrapping_mul(8i32 as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int
    {
        /* counter */
        let mut lc: lua_uint = 0i32 as lua_uint;
        let mut lim: lua_uint = ttlg;
        if lim > (*t).sizearray {
            /* adjust upper limit */
            lim = (*t).sizearray;
            if i > lim {
                /* no more elements to count */
                break;
            }
        }
        /* count elements in range (2^(lg - 1), 2^lg] */
        while i <= lim {
            if !((*(*t).array.offset(i.wrapping_sub(1i32 as lua_uint) as isize)).tt_ == 0i32) {
                lc = lc.wrapping_add(1)
            }
            i = i.wrapping_add(1)
        }
        let ref mut fresh2 = *nums.offset(lg as isize);
        *fresh2 = (*fresh2).wrapping_add(lc);
        ause = ause.wrapping_add(lc);
        lg += 1;
        ttlg = ttlg.wrapping_mul(2i32 as lua_uint)
    }
    return ause;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getstr(mut t: *mut Table, mut key: *mut TString) -> *const TValue {
    if (*key).tt as lua_int == 4i32 | 0i32 << 4i32 {
        return luaH_getshortstr(t, key);
    } else {
        /* for long strings, use generic case */
        let mut ko: TValue = lua_TValue {
            value_: Value {
                gc: 0 as *const GCObject as *mut GCObject,
            },
            tt_: 0,
        };
        let mut io: *mut TValue = &mut ko;
        let mut x_: *mut TString = key;
        (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
        (*io).tt_ = (*x_).tt as lua_int | 1i32 << 6i32;
        return getgeneric(t, &mut ko);
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_new(mut L: *mut lua_State) -> *mut Table {
    let mut o: *mut GCObject = luaC_newobj(L, 5i32, ::std::mem::size_of::<Table>() as lua_ulong);
    let mut t: *mut Table = &mut (*(o as *mut GCUnion)).h;
    (*t).metatable = 0 as *mut Table;
    (*t).flags = !0i32 as lu_byte;
    (*t).array = 0 as *mut TValue;
    (*t).sizearray = 0i32 as lua_uint;
    setnodevector(L, t, 0i32 as lua_uint);
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn luaH_resizearray(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut nasize: lua_uint,
) -> () {
    let mut nsize: lua_int = if (*t).lastfree.is_null() {
        0i32
    } else {
        1i32 << (*t).lsizenode as lua_int
    };
    luaH_resize(L, t, nasize, nsize as lua_uint);
}
#[no_mangle]
pub unsafe extern "C" fn luaH_free(mut L: *mut lua_State, mut t: *mut Table) -> () {
    if !(*t).lastfree.is_null() {
        luaM_realloc_(
            L,
            (*t).node as *mut lua_void,
            ((1i32 << (*t).lsizenode as lua_int) as size_t)
                .wrapping_mul(::std::mem::size_of::<Node>() as lua_ulong),
            0i32 as size_t,
        );
    }
    luaM_realloc_(
        L,
        (*t).array as *mut lua_void,
        ((*t).sizearray as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
        0i32 as size_t,
    );
    luaM_realloc_(
        L,
        t as *mut lua_void,
        ::std::mem::size_of::<Table>() as lua_ulong,
        0i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn luaH_next(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> lua_int {
    /* find original element */
    let mut i: lua_uint = findindex(L, t, key);
    while i < (*t).sizearray {
        /* try first array part */
        if !((*(*t).array.offset(i as isize)).tt_ == 0i32) {
            /* a non-nil value? */
            let mut io: *mut TValue = key;
            (*io).value_.i = i.wrapping_add(1i32 as lua_uint) as lua_Integer;
            (*io).tt_ = 3i32 | 1i32 << 4i32;
            let mut io1: *mut TValue = key.offset(1isize);
            *io1 = *(*t).array.offset(i as isize);
            return 1i32;
        } else {
            i = i.wrapping_add(1)
        }
    }
    i = i.wrapping_sub((*t).sizearray);
    while (i as lua_int) < 1i32 << (*t).lsizenode as lua_int {
        /* hash part */
        if !((*(*t).node.offset(i as isize)).i_val.tt_ == 0i32) {
            /* a non-nil value? */
            let mut io1_0: *mut TValue = key;
            *io1_0 =
                *(&mut (*(*t).node.offset(i as isize)).i_key.tvk as *mut TValue as *const TValue);
            let mut io1_1: *mut TValue = key.offset(1isize);
            *io1_1 = (*(*t).node.offset(i as isize)).i_val;
            return 1i32;
        } else {
            i = i.wrapping_add(1)
        }
    }
    /* no more elements */
    return 0i32;
}
/*
** returns the index of a 'key' for table traversals. First goes all
** elements in the array part, then elements in the hash part. The
** beginning of a traversal is signaled by 0.
*/
unsafe extern "C" fn findindex(
    mut L: *mut lua_State,
    mut t: *mut Table,
    mut key: StkId,
) -> lua_uint {
    let mut i: lua_uint = 0;
    if (*key).tt_ == 0i32 {
        /* first iteration */
        return 0i32 as lua_uint;
    } else {
        i = arrayindex(key as *const TValue);
        /* is 'key' inside array part? */
        if i != 0i32 as lua_uint && i <= (*t).sizearray {
            /* yes; that's the index */
            return i;
        } else {
            let mut nx: lua_int = 0;
            let mut n: *mut Node = mainposition(t, key as *const TValue);
            loop {
                /* check whether 'key' is somewhere in the chain */
                /* key may be dead already, but it is ok to use it in 'next' */
                if 0 != luaV_equalobj(
                    0 as *mut lua_State,
                    &mut (*n).i_key.tvk as *mut TValue as *const TValue,
                    key as *const TValue,
                ) || (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue)).tt_ == 9i32 + 1i32
                    && 0 != (*key).tt_ & 1i32 << 6i32
                    && (*(&mut (*n).i_key.tvk as *mut TValue as *const TValue))
                        .value_
                        .gc as *mut lua_void
                        == (*key).value_.gc as *mut lua_void
                {
                    /* key index in hash table */
                    i = n.wrapping_offset_from(&mut *(*t).node.offset(0isize) as *mut Node)
                        as lua_long as lua_int as lua_uint;
                    /* hash elements are numbered after array ones */
                    return i
                        .wrapping_add(1i32 as lua_uint)
                        .wrapping_add((*t).sizearray);
                } else {
                    nx = (*n).i_key.nk.next;
                    if nx == 0i32 {
                        /* key not found */
                        luaG_runerror!(L, s!(b"invalid key to \'next\'\x00"),);
                    } else {
                        n = n.offset(nx as isize)
                    }
                }
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn luaH_getn(mut t: *mut Table) -> lua_Unsigned {
    let mut j: lua_uint = (*t).sizearray;
    if j > 0i32 as lua_uint
        && (*(*t).array.offset(j.wrapping_sub(1i32 as lua_uint) as isize)).tt_ == 0i32
    {
        /* there is a boundary in the array part: (binary) search for it */
        let mut i: lua_uint = 0i32 as lua_uint;
        while j.wrapping_sub(i) > 1i32 as lua_uint {
            let mut m: lua_uint = i.wrapping_add(j).wrapping_div(2i32 as lua_uint);
            if (*(*t).array.offset(m.wrapping_sub(1i32 as lua_uint) as isize)).tt_ == 0i32 {
                j = m
            } else {
                i = m
            }
        }
        return i as lua_Unsigned;
    } else if (*t).lastfree.is_null() {
        /* that is easy... */
        return j as lua_Unsigned;
    } else {
        return unbound_search(t, j as lua_Unsigned);
    };
}
unsafe extern "C" fn unbound_search(mut t: *mut Table, mut j: lua_Unsigned) -> lua_Unsigned {
    /* i is zero or a present index */
    let mut i: lua_Unsigned = j;
    j = j.wrapping_add(1);
    /* find 'i' and 'j' such that i is present and j is not */
    while !((*luaH_getint(t, j as lua_Integer)).tt_ == 0i32) {
        i = j;
        if j > (9223372036854775807i64 as lua_Unsigned).wrapping_div(2i32 as lua_ulonglong) {
            /* overflow? */
            /* table was built with bad purposes: resort to linear search */
            i = 1i32 as lua_Unsigned;
            while !((*luaH_getint(t, i as lua_Integer)).tt_ == 0i32) {
                i = i.wrapping_add(1)
            }
            return i.wrapping_sub(1i32 as lua_ulonglong);
        } else {
            j = (j as lua_ulonglong).wrapping_mul(2i32 as lua_ulonglong) as lua_Unsigned
                as lua_Unsigned
        }
    }
    /* now do a binary search between them */
    while j.wrapping_sub(i) > 1i32 as lua_ulonglong {
        let mut m: lua_Unsigned = i.wrapping_add(j).wrapping_div(2i32 as lua_ulonglong);
        if (*luaH_getint(t, m as lua_Integer)).tt_ == 0i32 {
            j = m
        } else {
            i = m
        }
    }
    return i;
}
