use types::*;

// stdlib constants
pub const EXIT_FAILURE: i32 = 1; /* Failing exit status.  */
pub const EXIT_SUCCESS: i32 = 0; /* Successful exit status.  */

/*
** $Id: luaconf.h,v 1.259.1.1 2017/04/19 17:29:57 roberto Exp $
** Configuration file for Lua
** See Copyright Notice in lua.h
*/

/*
** ===================================================================
** Search for "@@" to find all configurable definitions.
** ===================================================================
*/

/*
** {====================================================================
** System Configuration: macros to adapt (if needed) Lua to some
** particular platform, for instance compiling it with 32-bit numbers or
** restricting it to C89.
** =====================================================================
*/

/*
@@ LUA_INT_TYPE defines the type for Lua integers.
@@ LUA_FLOAT_TYPE defines the type for Lua floats.
** Lua should work fine with any mix of these options (if supported
** by your C compiler). The usual configurations are 64-bit integers
** and 'double' (the default), 32-bit integers and 'float' (for
** restricted platforms), and 'long'/'double' (for C compilers not
** compliant with C99, which may not have support for 'long long').
*/

/* predefined options for LUA_INT_TYPE */
pub const LUA_INT_INT: u32 = 1;
pub const LUA_INT_LONG: u32 = 2;
pub const LUA_INT_LONGLONG: u32 = 3;

/* predefined options for LUA_FLOAT_TYPE */
pub const LUA_FLOAT_FLOAT: u32 = 1;
pub const LUA_FLOAT_DOUBLE: u32 = 2;
pub const LUA_FLOAT_LONGDOUBLE: u32 = 3;

/*
** default configuration for 64-bit Lua ('long long' and 'double')
*/
pub const LUA_INT_TYPE: u32 = LUA_INT_LONGLONG;

pub const LUA_FLOAT_TYPE: u32 = LUA_FLOAT_DOUBLE;

/* }================================================================== */

/*
** {==================================================================
** Configuration for Paths.
** ===================================================================
*/

/*
** LUA_PATH_SEP is the character that separates templates in a path.
** LUA_PATH_MARK is the string that marks the substitution points in a
** template.
** LUA_EXEC_DIR in a Windows path is replaced by the executable's
** directory.
*/
pub_const_c_str!(LUA_PATH_SEP, b";\x00");
pub_const_c_str!(LUA_PATH_MARK, b"?\x00");
pub_const_c_str!(LUA_EXEC_DIR, b"!\x00");

/*
@@ LUA_PATH_DEFAULT is the default path that Lua uses to look for
** Lua libraries.
@@ LUA_CPATH_DEFAULT is the default path that Lua uses to look for
** C libraries.
** CHANGE them if your machine has a non-conventional directory
** hierarchy or if you want to install your libraries in
** non-conventional directories.
*/
pub_const_c_str!(LUA_PATH_DEFAULT, b"\x00");
pub_const_c_str!(LUA_CPATH_DEFAULT, b"\x00");

/*
@@ LUA_DIRSEP is the directory separator (for submodules).
** CHANGE it if your machine does not use "/" as the directory separator
** and is not Windows. (On Windows Lua automatically uses "\".)
*/
#[cfg(target_os = "windows")]
pub_const_c_str!(LUA_DIRSEP, b"\\\x00");

#[cfg(not(target_os = "windows"))]
pub_const_c_str!(LUA_DIRSEP, b"/\x00");

/* }================================================================== */

/*
** {==================================================================
** Configuration for Numbers.
** Change these definitions if no predefined LUA_FLOAT_* / LUA_INT_*
** satisfy your needs.
** ===================================================================
*/

/*
@@ LUA_NUMBER is the floating-point type used by Lua.
@@ LUAI_UACNUMBER is the result of a 'default argument promotion'
@@ over a floating number.
@@ l_mathlim(x) corrects limit name 'x' to the proper float type
** by prefixing it with one of FLT/DBL/LDBL.
@@ LUA_NUMBER_FRMLEN is the length modifier for writing floats.
@@ LUA_NUMBER_FMT is the format for writing floats.
@@ lua_number2str converts a float to a string.
@@ l_mathop allows the addition of an 'l' or 'f' to all math operations.
@@ l_floor takes the floor of a float.
@@ lua_str2number converts a decimal numeric string to a number.
*/

/* The following definitions are good for most cases here */

pub type LUA_NUMBER = lua_double;

#[macro_export]
macro_rules! l_mathlim {
    ($n:expr) => {
        concat_ident!(DBL_, $n)
    };
}

pub type LUAI_UACNUMBER = f64;

pub_const_c_str!(LUA_NUMBER_FRMLEN, b"\x00");
pub_const_c_str!(LUA_NUMBER_FMT, b"%.14g\x00");

#[macro_export]
macro_rules! l_mathop {
    ($op:expr) => {
        $op
    };
}

#[macro_export]
macro_rules! lua_str2number {
    ($s:expr, $p:expr) => {
        strtod($s, $p)
    };
}

/*
@@ LUA_INTEGER is the integer type used by Lua.
**
@@ LUA_UNSIGNED is the unsigned version of LUA_INTEGER.
**
@@ LUAI_UACINT is the result of a 'default argument promotion'
@@ over a lUA_INTEGER.
@@ LUA_INTEGER_FRMLEN is the length modifier for reading/writing integers.
@@ LUA_INTEGER_FMT is the format for writing integers.
@@ LUA_MAXINTEGER is the maximum value for a LUA_INTEGER.
@@ LUA_MININTEGER is the minimum value for a LUA_INTEGER.
@@ lua_integer2str converts an integer to a string.
*/

/* The following definitions are good for most cases here */

#[macro_export]
macro_rules! l_floor {
    ($x:expr) => {
        l_mathop!(floor)($x)
    };
}

#[macro_export]
macro_rules! lua_number2str {
    ($s:expr, $sz:expr, $n:expr) => {
        l_sprintf!($s, $sz, LUA_INTEGER_FMT, n as LUAI_UACINT)
    };
}

/*
@@ lua_numbertointeger converts a float number to an integer, or
** returns 0 if float is not within the range of a lua_Integer.
** (The range comparisons are tricky because of rounding. The tests
** here assume a two-complement representation, where MININTEGER always
** has an exact representation as a float; MAXINTEGER may not have one,
** and therefore its conversion to float may have an ill-defined value.)
*/
// #define lua_numbertointeger(n,p) \
//   ((n) >= (LUA_NUMBER)(LUA_MININTEGER) && \
//    (n) < -(LUA_NUMBER)(LUA_MININTEGER) && \
//       (*(p) = (LUA_INTEGER)(n), 1))

/*
** use LUAI_UACINT here to avoid problems with promotions (which
** can turn a comparison between unsigneds into a signed comparison)
*/
// pub type LUA_UNSIGNED = f64;
// #define LUA_UNSIGNED		unsigned LUAI_UACINT

/*
@@ LUAI_MAXSTACK limits the size of the Lua stack.
** CHANGE it if you need a different limit. This limit is arbitrary;
** its only purpose is to stop Lua from consuming unlimited stack
** space (and to reserve some numbers for pseudo-indices).
*/
// #if LUAI_BITSINT >= 32
// #define LUAI_MAXSTACK		1000000
// #else
// #define LUAI_MAXSTACK		15000
// #endif
pub const LUAI_MAXSTACK: i32 = 1000000;
