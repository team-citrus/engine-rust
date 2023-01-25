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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec2
{
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vec4
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Quat
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

type Quaternion = Quat;

// TODO: impl's