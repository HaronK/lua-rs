use std::process::exit;
use types::*;

// TODO: implement

pub static mut stdin: *mut FILE = 0 as *mut FILE;
pub static mut stderr: *mut FILE = 0 as *mut FILE;
pub static mut stdout: *mut FILE = 0 as *mut FILE;

pub fn errno() -> lua_int {
    0
}

pub unsafe extern "C" fn tolower(mut __c: lua_int) -> lua_int {
    0
}

pub unsafe extern "C" fn toupper(mut __c: lua_int) -> lua_int {
    0
}

pub fn isalpha(c: lua_int) -> lua_int {
    0
}

pub fn iscntrl(c: lua_int) -> lua_int {
    0
}

pub fn isdigit(c: lua_int) -> lua_int {
    0
}

pub fn isgraph(c: lua_int) -> lua_int {
    0
}

pub fn islower(c: lua_int) -> lua_int {
    0
}

pub fn ispunct(c: lua_int) -> lua_int {
    0
}

pub fn isspace(c: lua_int) -> lua_int {
    0
}

pub fn isupper(c: lua_int) -> lua_int {
    0
}

pub fn isalnum(c: lua_int) -> lua_int {
    0
}

pub fn isxdigit(c: lua_int) -> lua_int {
    0
}

pub fn isprint(c: lua_int) -> lua_int {
    0
}

pub unsafe extern "C" fn l_getc(mut __fp: *mut FILE) -> lua_int {
    0
}

pub fn flockfile(__stream: *mut FILE) -> () {}

pub fn funlockfile(__stream: *mut FILE) -> () {}

pub fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm {
    0 as *mut tm
}
pub fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm {
    0 as *mut tm
}
pub fn random() -> lua_long {
    0
}
pub fn srandom(__seed: lua_uint) -> () {}
pub fn dlsym(__handle: *mut lua_void, __name: *const lua_char) -> *mut lua_void {
    0 as *mut lua_void
}
pub fn dlerror() -> *mut lua_char {
    0 as *mut lua_char
}
pub fn dlopen(__file: *const lua_char, __mode: lua_int) -> *mut lua_void {
    0 as *mut lua_void
}
pub fn dlclose(__handle: *mut lua_void) -> lua_int {
    0
}

pub fn _longjmp(_: *mut __jmp_buf_tag, _: lua_int) -> ! {
    exit(1)
}
