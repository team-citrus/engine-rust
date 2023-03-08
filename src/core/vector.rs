/*
*	name: src/core/vector.rs
*	origin: Citrus Engine
*	purpose: Provide a replacement for Vec<T>
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use std::{mem::*, iter::*, ops::*, option::*};
use crate::mem::{MemBox, ffi};

// Vec<T> but for the Citrus Engine's box allocator
pub struct Vector<T>
{
	// Suprise! It's additional functionality for MemBox<T>
	content: MemBox<T>,
}

#[macro_export]
macro_rules! vector
{
	( $( $i:expr ),* ) =>
	{
		let mut vec = Vector::new();
		$(
			vec.push($i);
		)
		vec
	};
}

impl<T> Index<usize> for Vector<T>
{
	type Output = T;
	
	#[inline(always)]
	fn index(&self, index: usize) -> &self::Output
	{
		self.content[index]
	}
}

impl<T> Vector<T>
{
	#[inline(always)]
	pub fn new<T>()
	{
		Vector<T> { content: MemBox::new<T>() }
	}
	
	#[inline(always)]
	pub fn push(&mut self, item: T) -> &mut T
	{
		self.content.resize(self.content.get_count() + 1);
		*self.content[self.content.get_count() - 1] = item;
		self.content[self.content.get_count() - 1]
	}

	#[inline(always)]
	pub fn pop(&mut self) -> T
	{
		let ret = *self.content[self.content.get_count() - 1];
		self.content.resize(self.content.get_count() - 1);
		ret
	}

	#[inline(always)]
	pub fn rm(&mut self, index: usize) -> ()
	{
		self.content.count -= 1;
		for i in (index..=self.content.count).rev()
		{
			*self.content[i - 1] = *self.content[i];
		}
		self.content.ptr = memrealloc(self.content.ptr as *mut c_void, self.content.count * size_of<T>(), 0) as *mut T;
	}

	#[inline(always)]
	pub fn insert(&mut self, index: usize, obj: T) -> &mut T
	{
		self.content.resize(self.content.len() + 1);
		for i in index..(self.content.count - 1)
		{
			*self.content[i + 1] = *self.content[i];
		}
		*self.content[index] = obj;
		self.content[index]
	}

	#[inline(always)]
	pub fn get_last(&self) -> &T
	{
		self.content.ptr[self.content.count - 1]
	}

	#[inline(always)]
	pub fn get_mut_last(&mut self) -> &mut T
	{
		self.content.ptr[self.content.count - 1]
	}

	#[inline(always)]
	pub fn get_first(&self) -> &T
	{
		self.content.ptr[self.content.count - 1]
	}

	#[inline(always)]
	pub fn get_mut_first(&mut self) -> &mut T
	{
		self.content.ptr[self.content.count - 1]
	}

	#[inline(always)]
	pub fn as_slice(&'a self) -> &'a [T]
	{
		self.content.as_slice()
	}

	#[inline(always)]
	pub fn as_mut_slice(&'a mut self) -> &'a mut [T]
	{
		self.content.as_mut_slice()
	}

	#[inline(always)]
	pub fn into_raw_parts(&mut self) -> (*mut T, u64)
	{
		self.content.into_raw_parts()
	}

	#[inline(always)]
	pub fn from_raw_parts(raw: *mut T, c: u64) -> Vector<T>
	{
		Vector { content: MemBox::from_raw_parts(raw, c) }
	}

	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.content.len()
	}
}
