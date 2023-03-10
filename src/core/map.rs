/*
*	name: src/core/map.rs
*	origin: Citrus Engine
*	purpose: Provide a Map<A, B>
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

use crate::{pair::*, mem::*, vector::*};
use std::{cmp::*, option::*};

pub struct Map<A, B>
{
    entries: Vector<Pair<A, B>>,
}

impl<A, B> Map<A, B>
{
    #[inline(always)]
    fn from_vectors<A, B>(avec: Vector<A>, bvec: Vector<B>) -> Map<A, B>
    {
        Map::from_slices(avec.as_slice(), bvec.as_slice())
    }

    #[inline(always)]
    fn from_slices<A, B>(aslice: &[A], bslice: &[B])
    {
        let len = min(aslice.len(), bslice.len());
        let vec: Vector<Pair<A, B>> = Vector::new();
        for i in ..len
        {
            vec.push(Pair::create_pair(aslice[i], bslice[i]));
        }

        Map
        {
            entries: vec
        }
    }

    #[inline(always)]
    fn lookup_a(&'a self, b: B) -> Option<&'a A>
    {
        for i in ..self.vec.len()
        {
            if self.vec[i].b == b
            {
                return Some(&self.vec[i].A);
            }
        }
        
        None
    }

    #[inline(always)]
    fn lookup_b(&'a self, a: A) -> Option<&'a B>
    {
        for i in ..self.vec.len()
        {
            if self.vec[i].a == a
            {
                return Some(&self.vec[i].b);
            }
        }

        None
    }

    #[inline(always)]
    fn lookup_mut_a(&'a mut self, b: B) -> Option<&'a mut A>
    {
        for i in ..self.vec.len()
        {
            if self.vec[i].b == b
            {
                return Some(&self.vec[i].A);
            }
        }
        
        None
    }

    #[inline(always)]
    fn lookup_mut_b(&'a mut self, a: A) -> Option<&'a mut B>
    {
        for i in ..self.vec.len()
        {
            if self.vec[i].a == a
            {
                return Some(&self.vec[i].b);
            }
        }

        None
    }

    #[inline(always)]
    fn len(&self)
    {
        entries.len()
    }

    // TODO: More stuff
}

impl<A, B> Index<usize> for Map<A, B>
{
    type Output = Pair<A, B>;

    #[inline(always)]
    fn index(&self, i: usize) -> &self::Output
    {
        *entries[i]
    }
}