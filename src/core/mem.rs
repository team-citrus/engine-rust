using std::{mem::*, iter::*, ops::*, option::*};

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
	ptr: *mut T;
	size: usize;
	count: u64;
	curr: u64;
}

impl<T> MemBox<T>
{
	pub fn new<t>(c: u64) -> MemBox<t>
	{
		let s: usize = size_of<t>();
		unsafe
		{
			MemBox<t> { ptr = ffi::memalloc(count * size, 0) as *mut t, size = s, count = c, curr = 0 }
		}
	}
	
	pub fn resize(&mut self, c: u64)
	{
		self.count = c;
		unsafe
		{
			self.ptr = ffi::memrealloc(self.ptr as *mut c_void, c * self.size, 0) as *mut T;
		}
	}

	pub fn getCount(&self) -> u64
	{
		self.count
	} 
}

impl<T> Drop for MemBox<T>
{
	pub fn drop(&mut self)
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
	
	pub fn index(&mut self, index: usize) -> &self::Output
	{
		unsafe
		{
			let ret: &T = *self.ptr.offset(i);
		}

		return ret;
	}
}

impl<T> Iterator for MemBox<T>
{
	type Item = T;
	
	pub fn next(&mut self) -> Option<self::Item>
	{
		if self.cursor == self.count
		{
			return Option<self::Item> = None;
		}
		
		self.curr += 1;
		Option<self::Item> = Some(self[cursor - 1])
	}
}

// Vec<T> but for the Citrus Engine's box allocator
pub struct Vector<T>
{
	// Suprise! It's additional functionality for MemBox<T>
	content: MemBox<T>;
}

impl<T> Iterator for Vector<T>
{
	type Item = T;
	
	pub fn next(&mut self) -> Option<self::Item>
	{
		self.content.next()
	}
}

impl<T> Index<usize> for Vector<T>
{
	type Output = T;
	
	pub fn index(&mut self, index: usize) -> &self::Output
	{
		self.content[index]
	}
}

impl<T> for Vector<T>
{
	pub fn new<t>()
	{
		Vector<t> { content = MemBox::new<t>() }
	}
	
	pub fn push(&mut self, item: T) -> &T
	{
		self.content.resize(self.content.getCount() + 1);
		self.content[self.content.getCount() - 1)] = item	
	}

	// TODO: Add more functions
}
