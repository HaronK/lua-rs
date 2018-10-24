// copy of libc::c_void
#[repr(u8)]
pub enum lua_void {
    #[doc(hidden)]
    __variant1,
    #[doc(hidden)]
    __variant2,
}

pub type lua_char = i8;
pub type lua_schar = i8;
pub type lua_uchar = u8;
pub type lua_short = i16;
pub type lua_ushort = u16;
pub type lua_int = i32;
pub type lua_uint = u32;
pub type lua_float = f32;
pub type lua_double = f64;
pub type lua_long = i64;
pub type lua_ulong = u64;
pub type lua_longlong = i64;
pub type lua_ulonglong = u64;

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
pub type unnamed = lua_uint;
pub const _ISalnum: unnamed = 8;
pub const _ISpunct: unnamed = 4;
pub const _IScntrl: unnamed = 2;
pub const _ISblank: unnamed = 1;
pub const _ISgraph: unnamed = 32768;
pub const _ISprint: unnamed = 16384;
pub const _ISspace: unnamed = 8192;
pub const _ISxdigit: unnamed = 4096;
pub const _ISdigit: unnamed = 2048;
pub const _ISalpha: unnamed = 1024;
pub const _ISlower: unnamed = 512;
pub const _ISupper: unnamed = 256;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
}

pub type __clock_t = lua_long;
pub type __time_t = lua_long;
pub type clock_t = __clock_t;
pub type time_t = __time_t;

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
