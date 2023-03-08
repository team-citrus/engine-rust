/*
*	name: src/core/mem.rs
*	origin: Citrus Engine
*	purpose: Provide the main engine functions
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use std::{mem::*, iter::*, ops::*, option::*, slice::*};

#[cxx::bridge]
pub mod ffi
{
	#[namespace = "engine"]
	extern "C++"
	{
		pub unsafe fn memalloc(size: usize, flags: u16) -> *mut c_void;
		pub unsafe fn memrealloc(ptr: *mut c_void, size: usize, flags: u16) -> *mut c_void;
		pub unsafe fn memfree(ptr: *mut c_void);
	}
}

// MemBox<T> is an implementation of Box<T>, but for the Citrus Engine block allocator
pub struct MemBox<T>
{
	pub(in crate::vec) ptr: *mut T,
	pub(in crate::vec) count: u64,
}

impl<T> MemBox<T>
{
	#[inline(always)]
	pub fn new<T>(c: u64) -> MemBox<T>
	{
		unsafe
		{
			MemBox<T>
			{
				ptr: ffi::memalloc(c * size_of<T>(), 0),
				count: c,
			}
		}
	}
	
	#[inline(always)]
	pub fn resize(&mut self, c: u64) -> ()
	{
		self.count = c;
		unsafe
		{
			self.ptr = ffi::memrealloc(self.ptr as *mut c_void, c * size_of<T>(), 0) as *mut T;
		}
	}

	#[inline(always)]
	pub fn len(&self) -> u64
	{
		self.count
	} 

	#[inline(always)]
	pub fn as_slice(&'a self) -> &'a [T]
	{
		unsafe { from_raw_parts(self.ptr, self.count) }
	}

	#[inline(always)]
	pub fn as_mut_slice(&'a mut self) -> &'a [T]
	{
		unsafe { from_raw_parts_mut(self.ptr, self.count) }
	}

	#[inline(always)]
	pub fn into_raw_parts(&mut self) -> (*mut T, u64)
	{
		(self.ptr, self.count)
	}

	#[inline(always)]
	pub fn from_raw_parts(raw: *mut T, c: u64) -> MemBox<T>
	{
		unsafe
		{
			MemBox<T>
			{
				ptr: raw,
				count: c,
			}
		}
	}
}

impl<T> Drop for MemBox<T>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe
		{
			memfree<T>(self.ptr);
		}
	}
}

impl<T> Index<usize> for MemBox<T>
{
	type Output = T;
	
	#[inline(always)]
	fn index(&mut self, index: usize) -> &mut self::Output
	{
		unsafe
		{
			let ret: &mut T = *(self.ptr.offset(i) as *mut T);
		}

		return ret;
	}
}

impl<T> Clone for MemBox<T>
{
	#[inline(always)]
	fn clone(&self) -> MemBox<T>
	{
		let membox: MemBox<T> = MemBox::new(self.count);
		for i in ..self.count
		{
			membox[i] = self[i];
		}
		membox
	}
}