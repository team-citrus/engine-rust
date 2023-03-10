/*
*	name: src/core/hash.rs
*	origin: Citrus Engine
*	purpose: Provide a HashMap<K, T> and hash()
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use crate::{vector::*, pair::*};
use std::{mem::*, iter::*, ops::*, option::*, slice::*};
use impls::impls;

#[cxx::bridge]
pub mod ffi
{
    #[namespace = "engine"]
	extern "C++"
	{
        pub(super) unsafe fn hash(data: *const c_void, bytes: usize) -> usize;
    }
}

#[inline(always)]
pub fn hash<T>(data: &[T]) -> usize
{
    unsafe
    {
        ffi::hash(data.as_ptr(), size_of<T>() * data.len())
    }
}

pub struct HashMap<K, T>
{
    entries: MemBox<Pair<usize, T>>,
    count: usize,
}

impl<K, T> HashMap<K, T>
{
    pub fn from_vectors<K, T>(keys: vector::Vector<K>, items: vector::Vector<T>) -> Option<HashMap<K, T>>
    {
        HashMap::from_slices(keys.as_slice(), items.as_slice())
    }

    pub fn from_slices<K, T>(keys: &[K], items: &[T]) -> Option<HashMap<K, T>>
    {
        let len = min(aslice.len(), bslice.len());
        let mut hmap = HashMap { entries:  MemBox::new<K, T>(len + 8), count: len };

        'fill_hashmap: loop
        {
            for i in hmap.entries.iter_mut()
            {
                i.a = 0;
            }

            for i in ..min
            {
                let k = key[i];
                let h = hash(slice::from_ref(k));
                
                if hmap.entries[h % hmap.entries.len()].a == h
                {
                    return None;
                }
                else if hmap.entries[h % hmap.entries.len()].a != 0
                {
                    hmap.entries.resize(hmap.entries.len() + 8);
                    continue 'fill_hashmap;
                }
                else
                {
                    hmap.entries[h % hmap.entries.len()] = pair::create_pair(h, items[i]);
                }
            }
        }

        Some(hmap)
    }

    pub fn from_map<K, T>(map: Map<K, T>) -> Option<HashMap<K, T>>
    {
        let len = map.len();
        let mut hmap = HashMap { entries:  MemBox::new<K, T>(len + 8), count: len };

        'fill_hashmap: loop
        {
            for i in hmap.entries.iter_mut()
            {
                i.a = 0;
            }

            for i in ..min
            {
                let k = map[i].a;
                let h = hash(slice::from_ref(k));
                
                if hmap.entries[h % hmap.entries.len()].a == h
                {
                    return None;
                }
                else if hmap.entries[h % hmap.entries.len()].a != 0
                {
                    hmap.entries.resize(hmap.entries.len() + 8);
                    continue 'fill_hashmap;
                }
                else
                {
                    hmap.entries[h % hmap.entries.len()] = pair::create_pair(h, map[i].b);
                }
            }
        }

        Some(hmap)
    }

    pub fn add(&mut self, p: Pair<K, T>) -> Option<&T>
    {
        let h = hash(slice::from_ref(p.a));

        'try_add: loop
        {
            if self.entries[h % self.entries.len()].a == h
            {
                return None
            }
            else if self.entries[h % self.entries.len()].a != 0
            {
                self.entries.resize(self.entries.len() + 8);
                continue 'try_add;
            }
            else
            {
                self.entries[h % self.entries.len()] = create_pair(h, p.b);
                self.count += 1;
                return Some(self.entries[h % self.entries.len()]);
            }
        }       
    }

    pub fn remove(i: K) -> ()
    {
        self.entries[hash(slice::from_ref(i)) % self.entries.len()].a = 0;
        if impls!(T: Drop)
        {
            drop(self.entries[hash(slice::from_ref(i)) % self.entries.len()].b)
        }
    }
}

impl<K, T> Index<K> for Map<K, T>
{
    type Output = T;

    #[inline(always)]
    fn index(&self, i: K) -> &self::Output
    {
        self.entries[hash(slice::from_ref(i)) % self.entries.len()].a
    }
}