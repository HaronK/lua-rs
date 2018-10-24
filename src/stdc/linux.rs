use stdc::prelude::*;
use types::prelude::*;

extern "C" {
    #[no_mangle]
    pub static mut stdin: *mut FILE;
    #[no_mangle]
    pub static mut stderr: *mut FILE;
    #[no_mangle]
    pub static mut stdout: *mut FILE;
    #[no_mangle]
    pub fn __errno_location() -> *mut lua_int;
    #[no_mangle]
    pub fn __ctype_tolower_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn __ctype_b_loc() -> *mut *const lua_ushort;
    #[no_mangle]
    pub fn __uflow(_: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn flockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    pub fn funlockfile(__stream: *mut FILE) -> ();
    #[no_mangle]
    pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    #[no_mangle]
    pub fn random() -> lua_long;
    #[no_mangle]
    pub fn srandom(__seed: lua_uint) -> ();
    #[no_mangle]
    pub fn dlsym(__handle: *mut lua_void, __name: *const lua_char) -> *mut lua_void;
    #[no_mangle]
    pub fn dlerror() -> *mut lua_char;
    #[no_mangle]
    pub fn dlopen(__file: *const lua_char, __mode: lua_int) -> *mut lua_void;
    #[no_mangle]
    pub fn dlclose(__handle: *mut lua_void) -> lua_int;
    #[no_mangle]
    pub fn _longjmp(_: *mut __jmp_buf_tag, _: lua_int) -> !;
}

pub fn errno() -> lua_int {
    unsafe { *__errno_location() }
}

pub unsafe extern "C" fn tolower(mut __c: lua_int) -> lua_int {
    return if __c >= -128i32 && __c < 256i32 {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}

pub unsafe extern "C" fn toupper(mut __c: lua_int) -> lua_int {
    if __c >= -128i32 && __c < 256i32 {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    }
}

pub fn isalpha(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISalpha as lua_int as lua_ushort as lua_int
    }
}

pub fn iscntrl(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _IScntrl as lua_int as lua_ushort as lua_int
    }
}

pub fn isdigit(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISdigit as lua_int as lua_ushort as lua_int
    }
}

pub fn isgraph(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISgraph as lua_int as lua_ushort as lua_int
    }
}

pub fn islower(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISlower as lua_int as lua_ushort as lua_int
    }
}

pub fn ispunct(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISpunct as lua_int as lua_ushort as lua_int
    }
}

pub fn isspace(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISspace as lua_int as lua_ushort as lua_int
    }
}

pub fn isupper(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISupper as lua_int as lua_ushort as lua_int
    }
}

pub fn isalnum(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISalnum as lua_int as lua_ushort as lua_int
    }
}

pub fn isxdigit(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISxdigit as lua_int as lua_ushort as lua_int
    }
}

pub fn isprint(c: lua_int) -> lua_int {
    unsafe {
        *(*__ctype_b_loc()).offset(c as isize) as lua_int
            & _ISprint as lua_int as lua_ushort as lua_int
    }
}

pub unsafe extern "C" fn l_getc(mut __fp: *mut FILE) -> lua_int {
    if 0 != ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as lua_int as lua_long {
        __uflow(__fp)
    } else {
        let fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(fresh0 as *mut lua_uchar) as lua_int
    }
}
