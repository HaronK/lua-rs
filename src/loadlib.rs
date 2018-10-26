use super::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn luaopen_package(mut L: *mut lua_State) -> lua_int {
    createclibstable(L);
    /* create 'package' table */
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(16i32 as lua_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as lua_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg; 8]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, pk_funcs.as_ptr(), 0i32);
    createsearcherstable(L);
    /* set paths */
    setpath(L, s!(b"path\x00"),
            s!(b"LUA_PATH\x00"),
            s!(b"/usr/local/share/lua/5.3/?.lua;/usr/local/share/lua/5.3/?/init.lua;/usr/local/lib/lua/5.3/?.lua;/usr/local/lib/lua/5.3/?/init.lua;./?.lua;./?/init.lua\x00"));
    setpath(
        L,
        s!(b"cpath\x00"),
        s!(b"LUA_CPATH\x00"),
        s!(b"/usr/local/lib/lua/5.3/?.so;/usr/local/lib/lua/5.3/loadall.so;./?.so\x00"),
    );
    /* store config information */
    lua_pushstring(L, s!(b"/\n;\n?\n!\n-\n\x00"));
    lua_setfield(L, -2i32, s!(b"config\x00"));
    /* set field 'loaded' */
    luaL_getsubtable(L, -1000000i32 - 1000i32, s!(b"_LOADED\x00"));
    lua_setfield(L, -2i32, s!(b"loaded\x00"));
    /* set field 'preload' */
    luaL_getsubtable(L, -1000000i32 - 1000i32, s!(b"_PRELOAD\x00"));
    lua_setfield(L, -2i32, s!(b"preload\x00"));
    lua_rawgeti(L, -1000000i32 - 1000i32, 2i32 as lua_Integer);
    /* set 'package' as upvalue for next lib */
    lua_pushvalue(L, -2i32);
    /* open lib into global table */
    luaL_setfuncs(L, ll_funcs.as_ptr(), 1i32);
    /* pop global table */
    lua_settop(L, -1i32 - 1i32);
    /* return 'package' table */
    return 1i32;
}
/* placeholders */
static mut ll_funcs: [luaL_Reg; 2] = [lua_reg!(b"require\x00", ll_require), lua_reg_none!(0)];
unsafe extern "C" fn ll_require(mut L: *mut lua_State) -> lua_int {
    let mut name: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    /* LOADED table will be at index 2 */
    lua_settop(L, 1i32);
    lua_getfield(L, -1000000i32 - 1000i32, s!(b"_LOADED\x00"));
    /* LOADED[name] */
    lua_getfield(L, 2i32, name);
    /* is it there? */
    if 0 != lua_toboolean(L, -1i32) {
        /* package is already loaded */
        return 1i32;
    } else {
        /* else must load package */
        /* remove 'getfield' result */
        lua_settop(L, -1i32 - 1i32);
        findloader(L, name);
        /* pass name as argument to module loader */
        lua_pushstring(L, name);
        /* name is 1st argument (before search data) */
        lua_rotate(L, -2i32, 1i32);
        /* run loader to load module */
        lua_callk(L, 2i32, 1i32, 0i32 as lua_KContext, None);
        /* non-nil return? */
        if !(lua_type(L, -1i32) == 0i32) {
            /* LOADED[name] = returned value */
            lua_setfield(L, 2i32, name);
        }
        if lua_getfield(L, 2i32, name) == 0i32 {
            /* module set no value? */
            /* use true as result */
            lua_pushboolean(L, 1i32);
            /* extra copy to be returned */
            lua_pushvalue(L, -1i32);
            /* LOADED[name] = true */
            lua_setfield(L, 2i32, name);
        }
        return 1i32;
    };
}
unsafe extern "C" fn findloader(mut L: *mut lua_State, mut name: *const lua_char) -> () {
    let mut i: lua_int = 0;
    /* to build error message */
    let mut msg: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    /* push 'package.searchers' to index 3 in the stack */
    if lua_getfield(L, -1000000i32 - 1000i32 - 1i32, s!(b"searchers\x00")) != 5i32 {
        luaL_error!(L, s!(b"\'package.searchers\' must be a table\x00"));
    }
    /*  iterate over available searchers to find a loader */
    i = 1i32;
    loop {
        if lua_rawgeti(L, 3i32, i as lua_Integer) == 0i32 {
            /* no more searchers? */
            /* remove nil */
            lua_settop(L, -1i32 - 1i32);
            /* create error message */
            luaL_pushresult(&mut msg);
            luaL_error!(
                L,
                s!(b"module \'%s\' not found:%s\x00"),
                name,
                lua_tolstring(L, -1i32, 0 as *mut size_t),
            );
        }
        lua_pushstring(L, name);
        /* call it */
        lua_callk(L, 1i32, 2i32, 0i32 as lua_KContext, None);
        /* did it find a loader? */
        if lua_type(L, -2i32) == 6i32 {
            /* module loader found */
            return;
        } else {
            if 0 != lua_isstring(L, -2i32) {
                /* searcher returned error message? */
                /* remove extra return */
                lua_settop(L, -1i32 - 1i32);
                /* concatenate error message */
                luaL_addvalue(&mut msg);
            } else {
                /* remove both returns */
                lua_settop(L, -2i32 - 1i32);
            }
            i += 1
        }
    }
}
/*
** Set a path
*/
unsafe extern "C" fn setpath(
    mut L: *mut lua_State,
    mut fieldname: *const lua_char,
    mut envname: *const lua_char,
    mut dft: *const lua_char,
) -> () {
    let mut nver: *const lua_char = lua_pushfstring!(L, s!(b"%s%s\x00"), envname, s!(b"_5_3\x00"),);
    /* use versioned name */
    let mut path: *const lua_char = getenv(nver);
    /* no environment variable? */
    if path.is_null() {
        /* try unversioned name */
        path = getenv(envname)
    }
    /* no environment variable? */
    if path.is_null() || 0 != noenv(L) {
        /* use default */
        lua_pushstring(L, dft);
    } else {
        /* replace ";;" by ";AUXMARK;" and then AUXMARK by default path */
        path = luaL_gsub(L, path, s!(b";;\x00"), s!(b";\x01;\x00"));
        luaL_gsub(L, path, s!(b"\x01\x00"), dft);
        /* remove result from 1st 'gsub' */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* package[fieldname] = path value */
    lua_setfield(L, -3i32, fieldname);
    /* pop versioned variable name */
    lua_settop(L, -1i32 - 1i32);
}
/* }====================================================== */
/* }{ */
/* } */
/*
** {==================================================================
** Set Paths
** ===================================================================
*/
/*
** LUA_PATH_VAR and LUA_CPATH_VAR are the names of the environment
** variables that Lua check to set its paths.
*/
/* auxiliary mark */
/*
** return registry.LUA_NOENV as a boolean
*/
unsafe extern "C" fn noenv(mut L: *mut lua_State) -> lua_int {
    let mut b: lua_int = 0;
    lua_getfield(L, -1000000i32 - 1000i32, s!(b"LUA_NOENV\x00"));
    b = lua_toboolean(L, -1i32);
    /* remove value */
    lua_settop(L, -1i32 - 1i32);
    return b;
}
unsafe extern "C" fn createsearcherstable(mut L: *mut lua_State) -> () {
    static mut searchers: [lua_CFunction; 5] = [
        Some(searcher_preload),
        Some(searcher_Lua),
        Some(searcher_C),
        Some(searcher_Croot),
        None,
    ];
    let mut i: lua_int = 0;
    /* create 'searchers' table */
    lua_createtable(
        L,
        (::std::mem::size_of::<[lua_CFunction; 5]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_CFunction>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
        0i32,
    );
    /* fill it with predefined searchers */
    i = 0i32;
    while searchers[i as usize].is_some() {
        /* set 'package' as upvalue for all searchers */
        lua_pushvalue(L, -2i32);
        lua_pushcclosure(L, searchers[i as usize], 1i32);
        lua_rawseti(L, -2i32, (i + 1i32) as lua_Integer);
        i += 1
    }
    /* put it in field 'searchers' */
    lua_setfield(L, -2i32, s!(b"searchers\x00"));
}
unsafe extern "C" fn searcher_Croot(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = 0 as *const lua_char;
    let mut name: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut p: *const lua_char = strchr(name, '.' as i32);
    let mut stat: lua_int = 0;
    if p.is_null() {
        /* is root */
        return 0i32;
    } else {
        lua_pushlstring(L, name, p.wrapping_offset_from(name) as lua_long as size_t);
        filename = findfile(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            s!(b"cpath\x00"),
            s!(b"/\x00"),
        );
        if filename.is_null() {
            /* root not found */
            return 1i32;
        } else {
            stat = loadfunc(L, filename, name);
            if stat != 0i32 {
                if stat != 2i32 {
                    /* real error */
                    return checkload(L, 0i32, filename);
                } else {
                    /* open function not found */
                    lua_pushfstring!(
                        L,
                        s!(b"\n\tno module \'%s\' in file \'%s\'\x00"),
                        name,
                        filename,
                    );
                    return 1i32;
                }
            } else {
                /* will be 2nd argument to module */
                lua_pushstring(L, filename);
                return 2i32;
            }
        }
    };
}
unsafe extern "C" fn checkload(
    mut L: *mut lua_State,
    mut stat: lua_int,
    mut filename: *const lua_char,
) -> lua_int {
    if 0 != stat {
        /* module loaded successfully? */
        /* will be 2nd argument to module */
        lua_pushstring(L, filename);
        /* return open function and file name */
        return 2i32;
    } else {
        return luaL_error!(
            L,
            s!(b"error loading module \'%s\' from file \'%s\':\n\t%s\x00"),
            lua_tolstring(L, 1i32, 0 as *mut size_t),
            filename,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
        );
    };
}
/*
** Try to find a load function for module 'modname' at file 'filename'.
** First, change '.' to '_' in 'modname'; then, if 'modname' has
** the form X-Y (that is, it has an "ignore mark"), build a function
** name "luaopen_X" and look for it. (For compatibility, if that
** fails, it also tries "luaopen_Y".) If there is no ignore mark,
** look for a function named "luaopen_modname".
*/
unsafe extern "C" fn loadfunc(
    mut L: *mut lua_State,
    mut filename: *const lua_char,
    mut modname: *const lua_char,
) -> lua_int {
    let mut openfunc: *const lua_char = 0 as *const lua_char;
    let mut mark: *const lua_char = 0 as *const lua_char;
    modname = luaL_gsub(L, modname, s!(b".\x00"), s!(b"_\x00"));
    mark = strchr(modname, *(s!(b"-\x00")) as lua_int);
    if !mark.is_null() {
        let mut stat: lua_int = 0;
        openfunc = lua_pushlstring(
            L,
            modname,
            mark.wrapping_offset_from(modname) as lua_long as size_t,
        );
        openfunc = lua_pushfstring!(L, s!(b"luaopen_%s\x00"), openfunc,);
        stat = lookforfunc(L, filename, openfunc);
        if stat != 2i32 {
            return stat;
        } else {
            modname = mark.offset(1isize)
        }
    }
    openfunc = lua_pushfstring!(L, s!(b"luaopen_%s\x00"), modname,);
    return lookforfunc(L, filename, openfunc);
}
/* error codes for 'lookforfunc' */
/*
** Look for a C function named 'sym' in a dynamically loaded library
** 'path'.
** First, check whether the library is already loaded; if not, try
** to load it.
** Then, if 'sym' is '*', return true (as library has been loaded).
** Otherwise, look for symbol 'sym' in the library and push a
** C function with that symbol.
** Return 0 and 'true' or a function in the stack; in case of
** errors, return an error code and an error message in the stack.
*/
unsafe extern "C" fn lookforfunc(
    mut L: *mut lua_State,
    mut path: *const lua_char,
    mut sym: *const lua_char,
) -> lua_int {
    /* check loaded C libraries */
    let mut reg: *mut lua_void = checkclib(L, path);
    if reg.is_null() {
        /* must load library? */
        /* global symbols if 'sym'=='*' */
        reg = lsys_load(L, path, (*sym as lua_int == '*' as i32) as lua_int);
        if reg.is_null() {
            /* unable to load library */
            return 1i32;
        } else {
            addtoclib(L, path, reg);
        }
    }
    if *sym as lua_int == '*' as i32 {
        /* loading only library (no function)? */
        /* return 'true' */
        lua_pushboolean(L, 1i32);
        /* no errors */
        return 0i32;
    } else {
        let mut f: lua_CFunction = lsys_sym(L, reg, sym);
        if f.is_none() {
            /* unable to find function */
            return 2i32;
        } else {
            /* else create new function */
            lua_pushcclosure(L, f, 0i32);
            /* no errors */
            return 0i32;
        }
    };
}
/* }================================================================== */
/*
** return registry.CLIBS[path]
*/
unsafe extern "C" fn checkclib(mut L: *mut lua_State, mut path: *const lua_char) -> *mut lua_void {
    let mut plib: *mut lua_void = 0 as *mut lua_void;
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const lua_int as *const lua_void,
    );
    lua_getfield(L, -1i32, path);
    /* plib = CLIBS[path] */
    plib = lua_touserdata(L, -1i32);
    /* pop CLIBS table and 'plib' */
    lua_settop(L, -2i32 - 1i32);
    return plib;
}
/*
** $Id: loadlib.c,v 1.130.1.1 2017/04/19 17:20:42 roberto Exp $
** Dynamic library loader for Lua
** See Copyright Notice in lua.h
**
** This module contains an implementation of loadlib for Unix systems
** that have dlfcn, an implementation for Windows, and a stub for other
** systems.
*/
/*
** LUA_IGMARK is a mark to ignore all before it when building the
** luaopen_ function name.
*/
/*
** LUA_CSUBSEP is the character that replaces dots in submodule names
** when searching for a C loader.
** LUA_LSUBSEP is the character that replaces dots in submodule names
** when searching for a Lua loader.
*/
/* prefix for open functions in C libraries */
/* separator for open functions in C libraries */
/*
** unique key for table in the registry that keeps handles
** for all loaded C libraries
*/
static mut CLIBS: lua_int = 0i32;
/*
** Try to find a function named 'sym' in library 'lib'.
** Returns the function; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_sym(
    mut L: *mut lua_State,
    mut lib: *mut lua_void,
    mut sym: *const lua_char,
) -> lua_CFunction {
    let mut f: lua_CFunction =
        ::std::mem::transmute::<*mut lua_void, lua_CFunction>(dlsym(lib, sym));
    if f.is_none() {
        lua_pushstring(L, dlerror());
    }
    return f;
}
/*
** registry.CLIBS[path] = plib        -- for queries
** registry.CLIBS[#CLIBS + 1] = plib  -- also keep a list of all libraries
*/
unsafe extern "C" fn addtoclib(
    mut L: *mut lua_State,
    mut path: *const lua_char,
    mut plib: *mut lua_void,
) -> () {
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const lua_int as *const lua_void,
    );
    lua_pushlightuserdata(L, plib);
    lua_pushvalue(L, -1i32);
    /* CLIBS[path] = plib */
    lua_setfield(L, -3i32, path);
    /* CLIBS[#CLIBS + 1] = plib */
    lua_rawseti(L, -2i32, luaL_len(L, -2i32) + 1i32 as lua_longlong);
    /* pop CLIBS table */
    lua_settop(L, -1i32 - 1i32);
}
/*
** load C library in file 'path'. If 'seeglb', load with all names in
** the library global.
** Returns the library; in case of error, returns NULL plus an
** error string in the stack.
*/
unsafe extern "C" fn lsys_load(
    mut L: *mut lua_State,
    mut path: *const lua_char,
    mut seeglb: lua_int,
) -> *mut lua_void {
    let mut lib: *mut lua_void = dlopen(path, 0x2i32 | if 0 != seeglb { 0x100i32 } else { 0i32 });
    if lib.is_null() {
        lua_pushstring(L, dlerror());
    }
    return lib;
}
unsafe extern "C" fn findfile(
    mut L: *mut lua_State,
    mut name: *const lua_char,
    mut pname: *const lua_char,
    mut dirsep: *const lua_char,
) -> *const lua_char {
    let mut path: *const lua_char = 0 as *const lua_char;
    lua_getfield(L, -1000000i32 - 1000i32 - 1i32, pname);
    path = lua_tolstring(L, -1i32, 0 as *mut size_t);
    if path.is_null() {
        luaL_error!(L, s!(b"\'package.%s\' must be a string\x00"), pname);
    }
    return searchpath(L, name, path, s!(b".\x00"), dirsep);
}
unsafe extern "C" fn searchpath(
    mut L: *mut lua_State,
    mut name: *const lua_char,
    mut path: *const lua_char,
    mut sep: *const lua_char,
    mut dirsep: *const lua_char,
) -> *const lua_char {
    /* to build error message */
    let mut msg: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut msg);
    /* non-empty separator? */
    if *sep as lua_int != '\u{0}' as i32 {
        /* replace it by 'dirsep' */
        name = luaL_gsub(L, name, sep, dirsep)
    }
    loop {
        path = pushnexttemplate(L, path);
        if path.is_null() {
            break;
        }
        let mut filename: *const lua_char = luaL_gsub(
            L,
            lua_tolstring(L, -1i32, 0 as *mut size_t),
            s!(b"?\x00"),
            name,
        );
        /* remove path template */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
        /* does file exist and is readable? */
        if 0 != readable(filename) {
            /* return that file name */
            return filename;
        } else {
            lua_pushfstring!(L, s!(b"\n\tno file \'%s\'\x00"), filename,);
            /* remove file name */
            lua_rotate(L, -2i32, -1i32);
            lua_settop(L, -1i32 - 1i32);
            /* concatenate error msg. entry */
            luaL_addvalue(&mut msg);
        }
    }
    /* create error message */
    luaL_pushresult(&mut msg);
    /* not found */
    return 0 as *const lua_char;
}
/*
** {======================================================
** 'require' function
** =======================================================
*/
unsafe extern "C" fn readable(mut filename: *const lua_char) -> lua_int {
    /* try to open file */
    let mut f: *mut FILE = fopen(filename, s!(b"r\x00"));
    if f.is_null() {
        /* open failed */
        return 0i32;
    } else {
        fclose(f);
        return 1i32;
    };
}
unsafe extern "C" fn pushnexttemplate(
    mut L: *mut lua_State,
    mut path: *const lua_char,
) -> *const lua_char {
    let mut l: *const lua_char = 0 as *const lua_char;
    while *path as lua_int == *(s!(b";\x00")) as lua_int {
        /* skip separators */
        path = path.offset(1isize)
    }
    if *path as lua_int == '\u{0}' as i32 {
        /* no more templates */
        return 0 as *const lua_char;
    } else {
        /* find next separator */
        l = strchr(path, *(s!(b";\x00")) as lua_int);
        if l.is_null() {
            l = path.offset(strlen(path) as isize)
        }
        /* template */
        lua_pushlstring(L, path, l.wrapping_offset_from(path) as lua_long as size_t);
        return l;
    };
}
unsafe extern "C" fn searcher_C(mut L: *mut lua_State) -> lua_int {
    let mut name: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut filename: *const lua_char = findfile(L, name, s!(b"cpath\x00"), s!(b"/\x00"));
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (loadfunc(L, filename, name) == 0i32) as lua_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_Lua(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = 0 as *const lua_char;
    let mut name: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    filename = findfile(L, name, s!(b"path\x00"), s!(b"/\x00"));
    if filename.is_null() {
        /* module not found in this path */
        return 1i32;
    } else {
        return checkload(
            L,
            (luaL_loadfilex(L, filename, 0 as *const lua_char) == 0i32) as lua_int,
            filename,
        );
    };
}
unsafe extern "C" fn searcher_preload(mut L: *mut lua_State) -> lua_int {
    let mut name: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    lua_getfield(L, -1000000i32 - 1000i32, s!(b"_PRELOAD\x00"));
    /* not found? */
    if lua_getfield(L, -1i32, name) == 0i32 {
        lua_pushfstring!(L, s!(b"\n\tno field package.preload[\'%s\']\x00"), name,);
    }
    return 1i32;
}
/* }====================================================== */
/*
** {======================================================
** 'module' function
** =======================================================
*/
/* }====================================================== */
static mut pk_funcs: [luaL_Reg; 8] = [
    lua_reg!(b"loadlib\x00", ll_loadlib),
    lua_reg!(b"searchpath\x00", ll_searchpath),
    lua_reg_none!(b"preload\x00"),
    lua_reg_none!(b"cpath\x00"),
    lua_reg_none!(b"path\x00"),
    lua_reg_none!(b"searchers\x00"),
    lua_reg_none!(b"loaded\x00"),
    lua_reg_none!(0),
];
unsafe extern "C" fn ll_searchpath(mut L: *mut lua_State) -> lua_int {
    let mut f: *const lua_char = searchpath(
        L,
        luaL_checklstring(L, 1i32, 0 as *mut size_t),
        luaL_checklstring(L, 2i32, 0 as *mut size_t),
        luaL_optlstring(L, 3i32, s!(b".\x00"), 0 as *mut size_t),
        luaL_optlstring(L, 4i32, s!(b"/\x00"), 0 as *mut size_t),
    );
    if !f.is_null() {
        return 1i32;
    } else {
        /* error message is on top of the stack */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        /* return nil + error message */
        return 2i32;
    };
}
unsafe extern "C" fn ll_loadlib(mut L: *mut lua_State) -> lua_int {
    let mut path: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut init: *const lua_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
    let mut stat: lua_int = lookforfunc(L, path, init);
    /* no errors? */
    if stat == 0i32 {
        /* return the loaded function */
        return 1i32;
    } else {
        /* error; error message is on stack top */
        lua_pushnil(L);
        lua_rotate(L, -2i32, 1i32);
        lua_pushstring(
            L,
            if stat == 1i32 {
                s!(b"open\x00")
            } else {
                s!(b"init\x00")
            },
        );
        /* return nil, error message, and where */
        return 3i32;
    };
}
/*
** create table CLIBS to keep track of loaded C libraries,
** setting a finalizer to close all libraries when closing state.
*/
unsafe extern "C" fn createclibstable(mut L: *mut lua_State) -> () {
    /* create CLIBS table */
    lua_createtable(L, 0i32, 0i32);
    /* create metatable for CLIBS */
    lua_createtable(L, 0i32, 1i32);
    lua_pushcclosure(L, Some(gctm), 0i32);
    /* set finalizer for CLIBS table */
    lua_setfield(L, -2i32, s!(b"__gc\x00"));
    lua_setmetatable(L, -2i32);
    /* set CLIBS table in registry */
    lua_rawsetp(
        L,
        -1000000i32 - 1000i32,
        &CLIBS as *const lua_int as *const lua_void,
    );
}
/*
** __gc tag method for CLIBS table: calls 'lsys_unloadlib' for all lib
** handles in list CLIBS
*/
unsafe extern "C" fn gctm(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_Integer = luaL_len(L, 1i32);
    while n >= 1i32 as lua_longlong {
        /* for each handle, in reverse order */
        /* get handle CLIBS[n] */
        lua_rawgeti(L, 1i32, n);
        lsys_unloadlib(lua_touserdata(L, -1i32));
        /* pop handle */
        lua_settop(L, -1i32 - 1i32);
        n -= 1
    }
    return 0i32;
}
/*
** system-dependent functions
*/
/*
** unload library 'lib'
*/
unsafe extern "C" fn lsys_unloadlib(mut lib: *mut lua_void) -> () {
    dlclose(lib);
}
