/*
*	name: src/core/vector.rs
*	origin: Citrus Engine
*	purpose: Provide a replacement for Vec<T>
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use std::{mem::*, iter::*, ops::*, option::*, slice::*};
use crate::mem::{MemBox, ffi};
use impls::impls;

// Vec<T> but for the Citrus Engine's box allocator
pub struct Vector<T>
{
	// Suprise! It's additional functionality for MemBox<T>
	pub(in crate) content: MemBox<T>,
	pub(in crate) count: usize,
}

#[macro_export]
macro_rules! memvec
{
	() => { Vector::new() }
	($elem:expr; $n:expr) =>
	{
		Vector::from_elem($elem, $n)
	}
	( $($x:expr),+ $(,)? ) =>
	{
		Vector::from_box(MemBox::from_slice(&[$($x),+]))
	}
}

impl<T> Index<usize> for Vector<T>
{
	type Output = T;
	
	#[inline(always)]
	fn index(&self, index: usize) -> &self::Output
	{
		// The C++ version looks cleaner imo
		index = if index >= self.count
		{
			self.count - 1
		}
		else
		{
			index
		};

		self.content[index]
	}
}

impl<T> Vector<T>
{
	#[inline(always)]
	pub fn new<T>() -> Vector<T>
	{
		Vector { content: MemBox::new(8), count: 0 }
	}

	#[inline(always)]
	pub fn from_elem<T>(elem: T, n: usize) -> Vector<T>
	{
		let vec = Vector { content: MemBox::new<T>(n + 8), count: n };
		for i in ..n
		{
			*vec[i] = elem;
		}
		vec
	}
	
	#[inline(always)]
	pub fn push(&mut self, item: T) -> &mut T
	{
		self.count += 1;
		if self.content.len() >= self.count - 2
		{
			self.content.resize(self.content.len() + 8);
		}

		*self.content[self.count - 1] = item;
		self.content[self.count - 1]
	}

	#[inline(always)]
	pub fn pop(&mut self) -> T
	{
		let ret = *self.content[self.count - 1];

		self.count -= 1;
		if self.content.len() - 8 >= self.count
		{
			self.content.resize(self.content.len() - 8);
		}

		ret
	}

	#[inline(always)]
	pub fn rm(&mut self, index: usize) -> ()
	{
		self.content.count -= 1;

		for i in ((index + 1)..self.count).rev()
		{
			*self.content[i - 1] = *self.content[i];
		}

		if self.content.len() - 8 >= self.count
		{
			self.content.resize(self.content.len() - 8);
		}
	}

	#[inline(always)]
	pub fn insert(&mut self, index: usize, obj: T) -> &mut T
	{
		self.count += 1;
		if self.content.len() >= self.count - 2
		{
			self.content.resize(self.content.len() + 8);
		}

		for i in index..(self.count - 1)
		{
			*self.content[i + 1] = *self.content[i];
		}

		*self.content[index] = obj;
		self.content[index]
	}

	#[inline(always)]
	pub fn get_last(&self) -> &T
	{
		self.content.ptr[self.count - 1]
	}

	#[inline(always)]
	pub fn get_mut_last(&mut self) -> &mut T
	{
		self.content.ptr[self.count - 1]
	}

	#[inline(always)]
	pub fn get_first(&self) -> &T
	{
		self.content.ptr[self.count - 1]
	}

	#[inline(always)]
	pub fn get_mut_first(&mut self) -> &mut T
	{
		self.content.ptr[self.count - 1]
	}

	#[inline(always)]
	pub fn as_slice(&'a self) -> &'a [T]
	{
		unsafe
		{
			slice::from_raw_parts(self.content.ptr, self.count)
		}
	}

	#[inline(always)]
	pub fn as_mut_slice(&'a mut self) -> &'a mut [T]
	{
		unsafe
		{
			slice::from_raw_parts_mut(self.content.ptr, self.count)
		}
	}

	#[inline(always)]
	pub fn into_raw_parts(&mut self) -> (*mut T, usize, usize)
	{
		(self.content.ptr, self.count, self.content)
	}

	#[inline(always)]
	pub fn from_raw_parts(raw: *mut T, c: usize) -> Vector<T>
	{
		Vector { content: MemBox::from_raw_parts(raw, c), count: c + 8 }
	}

	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.count
	}

	#[inline(always)]
	pub fn shrink_to_capacity(&mut self) -> ()
	{
		self.content.resize(self.count)
	}

	#[inline(always)]
	pub fn iter(&self) -> slice::Iter<T>
	{
		self.as_slice().iter()
	}

	#[inline(always)]
	pub fn iter_mut(&mut self) -> slice::IterMut<T>
	{
		self.as_mut_slice().iter_mut()
	}

	#[inline(always)]
	pub fn from_box<T>(membox: MemBox<T>) -> Vector<T>
	{
		Vector
		{
			content: membox.clone(),
			count: membox.count,
		}
	}
}

impl<T> Clone for Vector<T>
{
	#[inline(always)]
	fn clone(&self) -> Vector<T>
	{
		Vector
		{
			content: self.content.clone(),
			count: self.count,
		}
	}
}

impl<T> Drop for Vector<T>
{
	fn drop(&mut self)
	{
		if impls!(T: Drop)
		{
			for i in self.into_iter()
			{
				i.drop
			}
		}
	}
}

impl<T> IntoIterator for Vector<T>
{
	type Item = T;
	type IntoIter = slice::Iter<Item>;

	fn into_iter(&self) -> IntoIter
	{
		self.as_slice().into_iter()
	}
}