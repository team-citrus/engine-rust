/*
*	name: src/core/pair.rs
*	origin: Citrus Engine
*	purpose: Provide a Pair<A, B>
*	author: https://github.com/ComradeYellowCitrusFruit
*	license: LGPL-3.0-only
*/

pub struct Pair<A, B>
{
    pub a: A,
    pub b: B,
}

impl<A, B> Pair<A, B>
{
    #[inline(always)]
    pub fn create_pair<A, B>(ca: A, cb: B) -> Pair<A, B>
    {
        Pair
        {
            a: ca,
            b: cb,
        }
    }

    #[inline(always)]
    pub fn as_tuple(&self) -> (A, B)
    {
        (a, b)
    }

    #[inline(always)]
    pub fn get_a(&'a self) -> &'a A
    {
        &a
    }

    #[inline(always)]
    pub fn get_mut_a(&'a mut self) -> &'a mut A
    {
        &a
    }

    #[inline(always)]
    pub fn get_b(&'a self) -> &'a B
    {
        &b
    }

    #[inline(always)]
    pub fn get_mut_b(&'a mut self) -> &'a mut B
    {
        &b
    }
}