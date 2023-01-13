using std::mem;

#[cxx::bridge]
mod engine
{
	#[namespace = "engine"]
	extern "C++"
	{
		// Supposed to be placed on top of the original engine source code
		include!("include/core/mem.hpp");

		unsafe fn memalloc<T>(size: usize, flags: u16) -> *mut T;
		unsafe fn memrealloc<T>(ptr: *mut T, size: usize, flags: u16) -> *mut T;
		unsafe fn memfree<T>(ptr: *mut T);
	}

	extern "Rust"
	{
		pub struct memBox<T>
		{
			ptr: *mut T;
			size: usize;
			count: u64;
			curr: u64;
		}

		impl<T> memBox<T>
		{
			pub fn alloc<'a, T>(c: u64) -> &'a memBox<T>
			{
				let s: usize = size_of<T>();
				unsafe
				{
					memBox<T> { ptr = memalloc<T>(count * size, 0), size = s, count = c, curr = 0 }
				}
			}
			
			pub fn realloc(&mut self, c: u64)
			{
				self.count = c;
				unsafe
				{
					self.ptr = memrealloc<T>(self.ptr, c * self.size, 0);
				}
			}

			pub fn getCount(&self) -> u64
			{
				self.count
			} 
		}

		impl<T> Drop for memBox<T>
		{
			pub fn drop(&mut self)
			{
				unsafe
				{
					memfree<T>(self.ptr);
				}
			}
		}

		impl Index<usize> for memBox<T>
		{
			pub fn index<'a>(&'a mut self, i: usize) -> &'a T
			{
				unsafe
				{
					let ret: &'a T = *self.ptr.offset(i % self.count);
				}

				return ret;
			}
		}

		// TODO: impl<T> Iterator for memBox<T>
	}
}