use types::prelude::*;

extern "C" {
    #[no_mangle]
    pub fn snprintf(_: *mut lua_char, _: lua_ulong, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    pub fn strtod(__nptr: *const lua_char, __endptr: *mut *mut lua_char) -> lua_double;
    #[no_mangle]
    pub fn memcpy(_: *mut lua_void, _: *const lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    pub fn strcpy(_: *mut lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn strpbrk(_: *const lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn fgets(__s: *mut lua_char, __n: lua_int, __stream: *mut FILE) -> *mut lua_char;
    #[no_mangle]
    pub fn strcmp(_: *const lua_char, _: *const lua_char) -> lua_int;

    #[no_mangle]
    pub fn __ctype_toupper_loc() -> *mut *const __int32_t;
    #[no_mangle]
    pub fn realloc(_: *mut lua_void, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    pub fn free(__ptr: *mut lua_void) -> ();
    #[no_mangle]
    pub fn strncmp(_: *const lua_char, _: *const lua_char, _: lua_ulong) -> lua_int;
    #[no_mangle]
    pub fn strstr(_: *const lua_char, _: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn memcmp(_: *const lua_void, _: *const lua_void, _: lua_ulong) -> lua_int;
    #[no_mangle]
    pub fn strcoll(__s1: *const lua_char, __s2: *const lua_char) -> lua_int;
    #[no_mangle]
    pub fn printf(_: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    pub fn sprintf(_: *mut lua_char, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    pub fn isatty(__fd: lua_int) -> lua_int;
    #[no_mangle]
    pub fn readline(_: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn add_history(_: *const lua_char) -> ();
    #[no_mangle]
    pub fn signal(__sig: lua_int, __handler: __sighandler_t) -> __sighandler_t;
    #[no_mangle]
    pub fn getenv(__name: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    pub fn clock() -> clock_t;
    #[no_mangle]
    pub fn memchr(_: *const lua_void, _: lua_int, _: lua_ulong) -> *mut lua_void;
    #[no_mangle]
    pub fn setlocale(__category: lua_int, __locale: *const lua_char) -> *mut lua_char;
    #[no_mangle]
    pub fn exit(_: lua_int) -> !;
    #[no_mangle]
    pub fn mkstemp(__template: *mut lua_char) -> lua_int;
    #[no_mangle]
    pub fn system(__command: *const lua_char) -> lua_int;
    #[no_mangle]
    pub fn difftime(__time1: time_t, __time0: time_t) -> lua_double;
    #[no_mangle]
    pub fn mktime(__tp: *mut tm) -> time_t;
    #[no_mangle]
    pub fn strftime(
        __s: *mut lua_char,
        __maxsize: size_t,
        __format: *const lua_char,
        __tp: *const tm,
    ) -> size_t;
    #[no_mangle]
    pub fn close(__fd: lua_int) -> lua_int;
    #[no_mangle]
    pub fn remove(__filename: *const lua_char) -> lua_int;
    #[no_mangle]
    pub fn rename(__old: *const lua_char, __new: *const lua_char) -> lua_int;
    #[no_mangle]
    pub fn _setjmp(_: *mut __jmp_buf_tag) -> lua_int;
    #[no_mangle]
    pub fn abort() -> !;

    // Math
    #[no_mangle]
    pub fn acos(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn asin(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn atan2(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    pub fn cos(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn sin(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn tan(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn cosh(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn sinh(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn tanh(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn exp(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn frexp(_: lua_double, _: *mut lua_int) -> lua_double;
    #[no_mangle]
    pub fn ldexp(_: lua_double, _: lua_int) -> lua_double;
    #[no_mangle]
    pub fn log(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn log10(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn log2(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn pow(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    pub fn sqrt(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn ceil(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn fabs(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn floor(_: lua_double) -> lua_double;
    #[no_mangle]
    pub fn fmod(_: lua_double, _: lua_double) -> lua_double;
    #[no_mangle]
    pub fn abs(_: lua_int) -> lua_int;

    // IO
    #[no_mangle]
    pub fn localeconv() -> *mut lconv;
    #[no_mangle]
    pub fn tmpfile() -> *mut FILE;
    #[no_mangle]
    pub fn fclose(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn fflush(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn fopen(__filename: *const lua_char, __modes: *const lua_char) -> *mut FILE;
    #[no_mangle]
    pub fn setvbuf(__stream: *mut FILE, __buf: *mut lua_char, __modes: lua_int, __n: size_t)
        -> lua_int;
    #[no_mangle]
    pub fn fprintf(_: *mut FILE, _: *const lua_char, ...) -> lua_int;
    #[no_mangle]
    pub fn getc(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn ungetc(__c: lua_int, __stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn fread(__ptr: *mut lua_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn fwrite(__ptr: *const lua_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    pub fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: lua_int) -> lua_int;
    #[no_mangle]
    pub fn ftello(__stream: *mut FILE) -> __off64_t;
    #[no_mangle]
    pub fn clearerr(__stream: *mut FILE) -> ();
    #[no_mangle]
    pub fn ferror(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn popen(__command: *const lua_char, __modes: *const lua_char) -> *mut FILE;
    #[no_mangle]
    pub fn pclose(__stream: *mut FILE) -> lua_int;
    #[no_mangle]
    pub fn strchr(_: *const lua_char, _: lua_int) -> *mut lua_char;
    #[no_mangle]
    pub fn strspn(_: *const lua_char, _: *const lua_char) -> lua_ulong;
    #[no_mangle]
    pub fn strlen(_: *const lua_char) -> lua_ulong;
    #[no_mangle]
    pub fn strerror(_: lua_int) -> *mut lua_char;
    #[no_mangle]
    pub fn freopen(
        __filename: *const lua_char,
        __modes: *const lua_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    #[no_mangle]
    pub fn feof(__stream: *mut FILE) -> lua_int;
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: lua_int,
    pub tm_min: lua_int,
    pub tm_hour: lua_int,
    pub tm_mday: lua_int,
    pub tm_mon: lua_int,
    pub tm_year: lua_int,
    pub tm_wday: lua_int,
    pub tm_yday: lua_int,
    pub tm_isdst: lua_int,
    pub __tm_gmtoff: lua_long,
    pub __tm_zone: *const lua_char,
}

pub type __builtin_va_list = [__va_list_tag; 1];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: lua_uint,
    pub fp_offset: lua_uint,
    pub overflow_arg_area: *mut lua_void,
    pub reg_save_area: *mut lua_void,
}

pub type va_list = __builtin_va_list;
pub type size_t = lua_ulong;
pub type __sig_atomic_t = lua_int;
pub type __off_t = lua_long;
pub type __off64_t = lua_long;
pub type __sighandler_t = Option<unsafe extern "C" fn(_: lua_int) -> ()>;
pub type off_t = __off64_t;

pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ptrdiff_t = lua_long;
pub type intptr_t = lua_long;
pub type __int32_t = lua_int;
pub const _ISalnum: lua_int = 8;
pub const _ISpunct: lua_int = 4;
pub const _IScntrl: lua_int = 2;
pub const _ISblank: lua_int = 1;
pub const _ISgraph: lua_int = 32768;
pub const _ISprint: lua_int = 16384;
pub const _ISspace: lua_int = 8192;
pub const _ISxdigit: lua_int = 4096;
pub const _ISdigit: lua_int = 2048;
pub const _ISalpha: lua_int = 1024;
pub const _ISlower: lua_int = 512;
pub const _ISupper: lua_int = 256;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}

pub type __clock_t = lua_long;
pub type __time_t = lua_long;
pub type clock_t = __clock_t;
pub type time_t = __time_t;

pub type __jmp_buf = [lua_long; 8];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [lua_ulong; 16],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: lua_int,
    pub __saved_mask: __sigset_t,
}

pub type jmp_buf = [__jmp_buf_tag; 1];

#[derive(Copy, Clone)]
#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut lua_char,
    pub thousands_sep: *mut lua_char,
    pub grouping: *mut lua_char,
    pub int_curr_symbol: *mut lua_char,
    pub currency_symbol: *mut lua_char,
    pub mon_decimal_point: *mut lua_char,
    pub mon_thousands_sep: *mut lua_char,
    pub mon_grouping: *mut lua_char,
    pub positive_sign: *mut lua_char,
    pub negative_sign: *mut lua_char,
    pub int_frac_digits: lua_char,
    pub frac_digits: lua_char,
    pub p_cs_precedes: lua_char,
    pub p_sep_by_space: lua_char,
    pub n_cs_precedes: lua_char,
    pub n_sep_by_space: lua_char,
    pub p_sign_posn: lua_char,
    pub n_sign_posn: lua_char,
    pub int_p_cs_precedes: lua_char,
    pub int_p_sep_by_space: lua_char,
    pub int_n_cs_precedes: lua_char,
    pub int_n_sep_by_space: lua_char,
    pub int_p_sign_posn: lua_char,
    pub int_n_sign_posn: lua_char,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: lua_int,
    pub _IO_read_ptr: *mut lua_char,
    pub _IO_read_end: *mut lua_char,
    pub _IO_read_base: *mut lua_char,
    pub _IO_write_base: *mut lua_char,
    pub _IO_write_ptr: *mut lua_char,
    pub _IO_write_end: *mut lua_char,
    pub _IO_buf_base: *mut lua_char,
    pub _IO_buf_end: *mut lua_char,
    pub _IO_save_base: *mut lua_char,
    pub _IO_backup_base: *mut lua_char,
    pub _IO_save_end: *mut lua_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: lua_int,
    pub _flags2: lua_int,
    pub _old_offset: __off_t,
    pub _cur_column: lua_ushort,
    pub _vtable_offset: lua_schar,
    pub _shortbuf: [lua_char; 1],
    pub _lock: *mut lua_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut lua_void,
    pub __pad5: size_t,
    pub _mode: lua_int,
    pub _unused2: [lua_char; 20],
}
