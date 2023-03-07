/*
*	name: src/core/component.rs
*	origin: Citrus Engine
*	purpose: Proide Rust component traits
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use crate::{mem::*, vector::Vector};
use std::ptr::*;
use impls::impls;

#[allow(dead_code)]
#[feature(ptr_as_ref)]

// TODO: OPTIMIZE THE FUCK OUT OF THIS SHIT
// TODO: Strip out the dark magic as best as possible

type ComponentFuncPtr = fn (usize) -> (); // Dark magic, references are pointers and a usize is the same size as a pointer

// Dark magic, all components are #[repr(C)]
// This exists purely to enable execution directly from C++
#[repr(C)]
pub struct Base
{
    awake_fn: ComponentFuncPtr,
    start_fn: ComponentFuncPtr,
    update_fn: ComponentFuncPtr,
    fixed_update_fn: ComponentFuncPtr,
    on_trigger_enter_fn: ComponentFuncPtr,
    on_trigger_stay_fn: ComponentFuncPtr,
    on_trigger_exit_fn: ComponentFuncPtr,
}

pub trait Component
{

}

pub trait ComponentAwake
{
    pub fn awake(&self) -> ();
}

pub trait ComponentStart
{
    pub fn start(&self) -> ();
}

pub trait ComponentUpdate
{
    pub fn update(&self) -> ();
}

pub trait ComponentFixedUpdate
{
    pub fn fixed_update(&self) -> ();
}

pub trait ComponentOnTriggers
{
    pub fn on_trigger_enter(&self) -> ();
    pub fn on_trigger_stay(&self) -> ();
    pub fn on_trigger_exit(&self) -> ();
}

macro_rules! __load_base
{
    ($x:ident: $type:ty) => {
        let base: &mut Base

        unsafe {
           base = &$x.cast<Base>().cast_mut().as_ref();
        }

        *base.awake_fn = if impls!(T: ComponentAwake) {
            x.awake
        } else {
            0 as ComponentFuncPtr
        }

        *base.start_fn = if impls!(T: ComponentStart) {
            x.start
        } else {
            0 as ComponentFuncPtr
        }

        *base.update_fn = if impls!(T: ComponentUpdate) {
            x.update
        } else {
            0 as ComponentFuncPtr
        }

        *base.fixed_update_fn = if impls!(T: ComponentFixedUpdate) {
            x.fixed_update
        } else {
            0 as ComponentFuncPtr
        }

        if impls!(T: ComponentOnTriggers) {
            *base.on_trigger_enter_fn = x.on_trigger_enter;
            *base.on_trigger_stay_fn = x.on_trigger_stay;
            *base.on_trigger_exit_fn = x.on_trigger_exit;
        } else {
            *base.on_trigger_enter_fn = 0 as ComponentFuncPtr;
            *base.on_trigger_stay_fn = 0 as ComponentFuncPtr;
            *base.on_trigger_exit_fn = 0 as ComponentFuncPtr;
        }
    }
}