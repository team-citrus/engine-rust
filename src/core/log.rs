/*
*	name: src/core/log.rs
*	origin: Citrus Engine
*	purpose: Provide Rust logging
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

// Defined in https://github.com/team-citrus/engine/include/core/log.hpp

use std::{ffi::{CString, CStr}, os::raw::c_char, fmt::*};

#[no_mangle]
extern "C" fn rust_log(module: *const c_char, string: *const c_char) -> ();

// Functions exactly like println!, except it takes an extra option prefix arg that specifies the module to log as.
macro_rules! log
{
    ($module::expr, $($arg:tt)*) => {{
        unsafe
        {
            rust_log(
                format!($module).push('\0').as_ptr() as *const c_char,
                format!($($arg)*).push('\0').as_ptr() as *const c_char
            );
        }
    }}

    (($($arg:tt)*)) => {{
        unsafe
        {
            rust_log(
                format!(file!()).push('\0').as_ptr() as &const c_char,
                format!($($arg)*).push('\0').as_ptr() as *const c_char
            );
        }
    }}
}