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
        // I seriously hope this works
        type vec2;
        type vec3;
        type vec4;
        type quat;
        type quaternion = quat;
    }
}

// These should work in a union with the C++ ones

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

impl Vec2
{
    fn new() -> Vec2
    {
        Vec2 { x: 0, y: 0 }
    }

    fn mkVec2(xval: f32, yval: f32) -> Vec2
    {
        Vec2 { x: xval, y: yval }
    }
}

impl Vec3
{
    fn new() -> Vec3
    {
        Vec3 { x: 0, y: 0, z: 0 }
    }

    fn mkVec3(xval: f32, yval: f32, zval: f32) -> Vec3
    {
        Vec3 { x: xval, y: yval, z: zval }
    }
}

impl Vec4
{
    fn new() -> Vec4
    {
        Vec4 { x: 0, y: 0, z: 0, w: 0 }
    }

    fn mkVec4(xval: f32, yval: f32, zval: f32, wval: f32) -> Vec4
    {
        Vec4 { x: xval, y: yval, z: zval, w: wval }
    }
}

impl Quat
{
    fn new() -> Quat
    {
        Quat { x: 0, y: 0, z: 0, w: 0 }
    }

    fn mkQuat(xval: f32, yval: f32, zval: f32, wval: f32) -> Quat
    {
        Quat { x: xval, y: yval, z: zval, w: wval }
    }
}

mod internals
{
    // Necessary to make things work
    #[derive(Copy, Clone)]
    union Vec2
    {
        rvec: crate::vec::Vec2,
        cxxvec: crate::vec::ffi::Vec2,
    }

    #[derive(Copy, Clone)]
    union Vec3
    {
        rvec: crate::vec::Vec3,
        cxxvec: crate::vec::ffi::Vec3,
    }

    #[derive(Copy, Clone)]
    union Vec4
    {
        rvec: crate::vec::Vec4,
        cxxvec: crate::vec::ffi::Vec4,
    }
}

// TODO: impl's