/*
*	name: src/core/jobs.rs
*	origin: Citrus Engine
*	purpose: Provide access to the Job system from Rust
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/
use std::mem;

#[cxx::bridge]
pub mod ffi
{
	#[namespace = "engine"]
	extern "C++"
	{
		struct Job;
		impl Job
		{
			pub(in super) fn schedule() -> i32;
			pub(in super) fn complete() -> i32;
			pub(in super) fn prioritize() -> ();
		}
	}
}

pub type JobPtr = fn () -> ();

#[repr(C)]
pub struct Job
{
	ptr: JobPtr,
	prioritized: bool,
}

impl Job
{
	fn new(func: JobPtr) -> Job
	{
		Job
		{
			ptr: func,
			prioritized: false,
		}
	}
	fn schedule(&self) -> bool
	{
		let ret: bool;
		unsafe
		{
			ret = transmute_copy<Job, ffi::Job>(self).schedule() != -1;
		}
		ret
	}
	fn complete() -> bool
	{
		let ret: bool;
		unsafe
		{
			ret = transmute_copy<Job, ffi::Job>(self).complete() != -1;
		}
		ret
	}
	fn prioritize() -> ()
	{
		unsafe
		{
			transmute_copy<Job, ffi::Job>(self).prioritize();
		}
	}
}