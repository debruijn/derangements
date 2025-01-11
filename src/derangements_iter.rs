use itertools::{Itertools, Permutations};
use std::fmt::Debug;
use std::iter::FusedIterator;

macro_rules! clone_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the clone func
    ($($field:ident),*) => {
        #[inline] // TODO is this sensible?
        fn clone(&self) -> Self {
            Self {
                $($field: self.$field.clone(),)*
            }
        }
    }
}
macro_rules! debug_fmt_fields {  // Note: copied from Itertools - need to attribute before release - or just directly write the fmt func
    ($tyname:ident, $($($field:tt/*TODO ideally we would accept ident or tuple element here*/).+),*) => {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            f.debug_struct(stringify!($tyname))
                $(
              .field(stringify!($($field).+), &self.$($field).+)
              )*
              .finish()
        }
    }
}

pub struct Derangements<I: Iterator> {
    permutations: Permutations<I>,
    // restrictions: Vec<I>
}

impl<I> Clone for Derangements<I>
where
    I: Clone + Iterator,
    I::Item: Clone,
{
    clone_fields!(permutations);
}

impl<I> Debug for Derangements<I>
where
    I: Iterator + Debug,
    I::Item: Debug,
{
    debug_fmt_fields!(Derangements, permutations);
}

pub fn derangements_iter<I: Iterator>(iter: I, k: usize) -> Derangements<I>
where
    <I as Iterator>::Item: Clone,
{
    Derangements {
        permutations: Itertools::permutations(iter, k),
    }
}

impl<I> Iterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize>,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => None,
            Some(x) => {
                if !x
                    .iter()
                    .enumerate()
                    .any(|x| x.0 == x.1.clone().try_into().unwrap_or(usize::MAX))
                {
                    Some(x)
                } else {
                    self.next()
                }
            }
        }
    }
}

impl<I> FusedIterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::assert_equal;

    #[test]
    fn derangements_manual() {
        assert_equal(derangements_iter(0..1, 1), Vec::<Vec<usize>>::new());
        assert_equal(derangements_iter(0..2, 2), vec![[1, 0]]);
        assert_equal(derangements_iter(0..3, 3), vec![[1, 2, 0], [2, 0, 1]]);
        assert_equal(derangements_iter(0..3, 2), vec![[1, 0], [1, 2], [2, 0]]);
        assert_equal(
            derangements_iter([0, 0, 1].into_iter(), 3),
            vec![[1, 0, 0], [1, 0, 0]],
        );
    }
}
