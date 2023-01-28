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
extern "C" fn rust_log(string: *const c_char) -> ();

// Functions exactly like println!
macro_rules! log
{
    ($($arg:tt)*) => {{
        rust_log(
            CString::new(
                fmt::format(format_args_nl!($($arg)*)).as_str()
            ).unwrap().as_c_str().as_ptr() as *const c_char
        );
    }}
}