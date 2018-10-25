use types::prelude::*;

extern "C" {
    #[no_mangle]
    fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
}

/* only for Lua functions */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_1 {
    pub base: StkId,
    pub savedpc: *const Instruction,
}

#[no_mangle]
pub unsafe extern "C" fn luaZ_init(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut reader: lua_Reader,
    mut data: *mut lua_void,
) -> () {
    (*z).L = L;
    (*z).reader = reader;
    (*z).data = data;
    (*z).n = 0i32 as size_t;
    (*z).p = 0 as *const lua_char;
}
/* read next n bytes */
#[no_mangle]
pub unsafe extern "C" fn luaZ_read(mut z: *mut ZIO, mut b: *mut lua_void, mut n: size_t) -> size_t {
    while 0 != n {
        let mut m: size_t = 0;
        if (*z).n == 0i32 as lua_ulong {
            /* no bytes in buffer? */
            /* try to read more */
            if luaZ_fill(z) == -1i32 {
                /* no more input; return number of missing bytes */
                return n;
            } else {
                /* luaZ_fill consumed first byte; put it back */
                (*z).n = (*z).n.wrapping_add(1);
                (*z).p = (*z).p.offset(-1isize)
            }
        }
        /* min. between n and z->n */
        m = if n <= (*z).n { n } else { (*z).n };
        memcpy(b, (*z).p as *const lua_void, m);
        (*z).n = ((*z).n as lua_ulong).wrapping_sub(m) as size_t as size_t;
        (*z).p = (*z).p.offset(m as isize);
        b = (b as *mut lua_char).offset(m as isize) as *mut lua_void;
        n = (n as lua_ulong).wrapping_sub(m) as size_t as size_t
    }
    return 0i32 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn luaZ_fill(mut z: *mut ZIO) -> lua_int {
    let mut size: size_t = 0;
    let mut L: *mut lua_State = (*z).L;
    let mut buff: *const lua_char = 0 as *const lua_char;
    buff = (*z).reader.expect("non-null function pointer")(L, (*z).data, &mut size);
    if buff.is_null() || size == 0i32 as lua_ulong {
        return -1i32;
    } else {
        /* discount char being returned */
        (*z).n = size.wrapping_sub(1i32 as lua_ulong);
        (*z).p = buff;
        let fresh0 = (*z).p;
        (*z).p = (*z).p.offset(1);
        return *fresh0 as lua_uchar as lua_int;
    };
}
