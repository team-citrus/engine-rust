/*
*	name: src/core/component.rs
*	origin: Citrus Engine
*	purpose: Proide Rust component traits
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use crate::{mem::*, Vector};

#[allow(dead_code)]

// TODO: OPTIMIZE THE FUCK OUT OF THIS SHIT
// TODO: Strip out the dark magic

type ComponentFuncPtr = fn (usize) -> (); // Dark magic, references are pointers and a usize is the same size as a pointer

// Dark magic, all components are #[repr(C)]
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
    // TODO: object and stuff
}

pub trait ComponentAwake
{
    fn awake(&self) -> ();
}

pub trait ComponentStart
{
    fn start(&self) -> ();
}

pub trait ComponentUpdate
{
    fn update(&self) -> ();
}

pub trait ComponentFixedUpdate
{
    fn fixed_update(&self) -> ();
}

pub trait ComponentOnTriggers
{
    fn on_trigger_enter(&self) -> ();
    fn on_trigger_stay(&self) -> ();
    fn on_trigger_exit(&self) -> ();
}

rust_components: Vector<usize>; // Vector of pointers to components, stored as integers for ease of access

#[no_mangle]
extern "C" fn rust_exec_gameplay() -> ()
{
    // TODO: stuff
}