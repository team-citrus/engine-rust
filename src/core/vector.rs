/*
*	name: src/core/vector.rs
*	origin: Citrus Engine
*	purpose: Provide the main engine functions
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/


use std::{mem::*, iter::*, ops::*, option::*};
use crate::mem::{MemBox, ffi::*};

// Vec<T> but for the Citrus Engine's box allocator
pub struct Vector<T>
{
	// Suprise! It's additional functionality for MemBox<T>
	content: MemBox<T>,
}

impl<T> Index<usize> for Vector<T>
{
	type Output = T;
	
	fn index(&self, index: usize) -> &self::Output
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
	
	pub fn push(&mut self, item: T) -> &mut T
	{
		self.content.resize(self.content.get_count() + 1);
		*self.content[self.content.get_count() - 1] = item;
		self.content[self.content.get_count() - 1]
	}

	pub fn pop(&mut self) -> T
	{
		let ret = *self.content[self.content.get_count() - 1];
		self.content.resize(self.content.get_count() - 1);
		ret
	}

	pub fn rm(&mut self, index: usize) -> ()
	{
		self.content.count -= 1;
		for i in (index..=self.content.count).rev()
		{
			*self.content[i - 1] = *self.content[i];
		}
		self.content.ptr = memrealloc(self.content.ptr as *mut c_void, self.content.count * size_of<T>(), 0) as *mut T;
	}

	pub fn insert(&mut self, index: usize, obj: T) -> &mut T
	{
		self.content.resize(self.content.get_count() + 1);
		for i in index..(self.content.count - 1)
		{
			*self.content[i + 1] = *self.content[i];
		}
		*self.content[index] = obj;
		self.content[index]
	}

	pub fn as_slice(&'a self) -> &'a [T]
	{
		self.content.as_slice()
	}

	pub fn as_mut_slice(&'a mut self) -> &'a mut [T]
	{
		self.content.as_mut_slice()
	}
}
