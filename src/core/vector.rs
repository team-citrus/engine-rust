/*
*	name: src/core/vector.rs
*	origin: Citrus Engine
*	purpose: Provide the main engine functions
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/


use std::{mem::*, iter::*, ops::*, option::*};
use crate::mem::MemBox;

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
