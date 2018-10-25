use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn luaD_throw(L: *mut lua_State, errcode: lua_int) -> !;
    #[no_mangle]
    fn luaC_fullgc(L: *mut lua_State, isemergency: lua_int) -> ();
}

#[no_mangle]
pub unsafe extern "C" fn luaM_toobig(mut L: *mut lua_State) -> ! {
    luaG_runerror!(L, s!(b"memory allocation error: block too big\x00"),);
}
/* not to be called directly */
#[no_mangle]
pub unsafe extern "C" fn luaM_realloc_(
    mut L: *mut lua_State,
    mut block: *mut lua_void,
    mut osize: size_t,
    mut nsize: size_t,
) -> *mut lua_void {
    let mut newblock: *mut lua_void = 0 as *mut lua_void;
    let mut g: *mut global_State = (*L).l_G;
    let mut realosize: size_t = if !block.is_null() {
        osize
    } else {
        0i32 as lua_ulong
    };
    newblock = (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize);
    if newblock.is_null() && nsize > 0i32 as lua_ulong {
        /* cannot fail when shrinking a block */
        if !(*g).version.is_null() {
            /* is state fully built? */
            /* try to free some memory... */
            luaC_fullgc(L, 1i32);
            /* try again */
            newblock =
                (*g).frealloc.expect("non-null function pointer")((*g).ud, block, osize, nsize)
        }
        if newblock.is_null() {
            luaD_throw(L, 4i32);
        }
    }
    (*g).GCdebt = ((*g).GCdebt as lua_ulong)
        .wrapping_add(nsize)
        .wrapping_sub(realosize) as l_mem;
    return newblock;
}
#[no_mangle]
pub unsafe extern "C" fn luaM_growaux_(
    mut L: *mut lua_State,
    mut block: *mut lua_void,
    mut size: *mut lua_int,
    mut size_elems: size_t,
    mut limit: lua_int,
    mut what: *const lua_char,
) -> *mut lua_void {
    let mut newblock: *mut lua_void = 0 as *mut lua_void;
    let mut newsize: lua_int = 0;
    if *size >= limit / 2i32 {
        /* cannot double it? */
        /* cannot grow even a little? */
        if *size >= limit {
            luaG_runerror!(L, s!(b"too many %s (limit is %d)\x00"), what, limit,);
        } else {
            newsize = limit
        }
    } else {
        newsize = *size * 2i32;
        if newsize < 4i32 {
            /* minimum size */
            newsize = 4i32
        }
    }
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && (newsize as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(size_elems)
    {
        luaM_toobig(L);
    } else {
    };
    newblock = luaM_realloc_(
        L,
        block,
        (*size as lua_ulong).wrapping_mul(size_elems),
        (newsize as lua_ulong).wrapping_mul(size_elems),
    );
    /* update only when everything else is OK */
    *size = newsize;
    return newblock;
}
