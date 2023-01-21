/*
*	name: src/core/mem.rs
*	origin: Citrus Engine
*	purpose: Provide the main engine functions
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use std::{mem::*, iter::*, ops::*, option::*};

#[cxx::bridge]
mod ffi
{
	#[namespace = "engine"]
	extern "C++"
	{
		pub unsafe fn memalloc(size: usize, flags: u16) -> *mut c_void;
		pub unsafe fn memrealloc(ptr: *mut c_void, size: usize, flags: u16) -> *mut c_void;
		pub unsafe fn memfree(ptr: *mut c_void);
	}
}

// MemBox<T> is an implementation of Box<T>, but for the Citrus Engine box allocator
pub struct MemBox<T>
{
	ptr: *mut T,
	size: usize,
	count: u64,
	curr: u64,
}

impl<T> MemBox<T>
{
	pub fn new<T>(c: u64) -> MemBox<T>
	{
		let s: usize = size_of<T>();
		let b: MemBox<T>;
		unsafe
		{
			b = MemBox<T>
			{
				ptr: ffi::memalloc(count * size, 0) as *mut T,
				size: s,
				count: c, 
				curr: 0,
			};
		};
		b
	}
	
	pub fn resize(&mut self, c: u64)
	{
		self.count = c;
		unsafe
		{
			self.ptr = ffi::memrealloc(self.ptr as *mut c_void, c * self.size, 0) as *mut T;
		}
	}

	pub fn get_count(&self) -> u64
	{
		self.count
	} 
}

impl<T> Drop for MemBox<T>
{
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
	
	fn index(&mut self, index: usize) -> &self::Output
	{
		unsafe
		{
			let ret: &T = *self.ptr.offset(i);
		}

		return ret;
	}
}

// TODO: Redo this
impl<T> Iterator for MemBox<T>
{
	type Item = T;
	
	fn next(&mut self) -> Option<self::Item>
	{
		if self.cursor == self.count
		{
			return None;
		}
		
		self.curr += 1;
		Some(self[cursor - 1])
	}
}

// Vec<T> but for the Citrus Engine's box allocator
pub struct Vector<T>
{
	// Suprise! It's additional functionality for MemBox<T>
	content: MemBox<T>,
}

impl<T> Iterator for Vector<T>
{
	type Item = T;
	
	fn next(&mut self) -> Option<self::Item>
	{
		self.content.next()
	}
}

impl<T> Index<usize> for Vector<T>
{
	type Output = T;
	
	fn index(&mut self, index: usize) -> &self::Output
	{
		self.content[index]
	}
}

impl<T> Vector<T>
{
	pub fn new<T>()
	{
		Vector<T> { content = MemBox::new<T>() }
	}
	
	pub fn push(&mut self, item: T) -> &T
	{
		self.content.resize(self.content.get_count() + 1);
		self.content[self.content.get_count() - 1] = item;
		self.content[self.content.get_count() - 1]
	}

	// TODO: Add more functions
}
