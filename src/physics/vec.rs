/*
*	name: src/physics/vec.rs
*	origin: Citrus Engine
*	purpose: Provide the main engine functions
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

#[cxx::bridge]
#[namespace = "engine"]
pub mod ffi
{
    #[namespace = "physics"]
    extern "C++"
    {
        type vec2;
        type vec3;
        type vec4;
        type quat;
        type quaternion = quat;
    }
}

// TODO: Actual stuff