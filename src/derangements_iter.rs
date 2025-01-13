use crate::fast_permutations;
use crate::fast_permutations::FastPermutations;
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

#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct Derangements<I: Iterator> {
    permutations: FastPermutations<I>,
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

pub fn derangements_iter<I>(iter: I, k: usize) -> Derangements<I>
where
    I: Iterator,
    I::Item: Clone + Ord,
    usize: From<I::Item>,
{
    Derangements {
        permutations: fast_permutations(iter, k),
    }
}

impl<I> Iterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize> + Ord + Copy + Debug,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.permutations.next() {
            None => return None,
            Some(x) => {
                if !x
                    .iter()
                    .enumerate()
                    .any(|x| x.0 == (*x.1).try_into().unwrap_or(usize::MAX))
                {
                    return Some(x);
                }
            }
        };
        self.next()
    }
}

impl<I> FusedIterator for Derangements<I>
where
    I: Iterator,
    I::Item: Clone + TryInto<usize> + Ord + Copy + Debug,
{
}

#[derive(Debug, Clone)]
pub struct DistinctDerangements<I: Iterator> {
    buffer: Vec<I::Item>,
    start: bool,
    index: usize,
}

pub fn distinct_derangements<I>(iter: I) -> DistinctDerangements<I>
where
    I: Iterator,
    I::Item: Ord + Clone,
    usize: From<I::Item>,
{
    let mut buffer = Vec::from_iter(iter);
    buffer.sort_unstable_by(|a, b| b.cmp(a));
    let length = buffer.len();
    DistinctDerangements {
        buffer,
        start: true,
        index: length.saturating_sub(2),
    }
}

impl<I> Iterator for DistinctDerangements<I>
where
    I: Iterator,
    I::Item: Ord + Copy + Clone,
    usize: From<I::Item>,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        // Start iteration with buffer itself
        if self.start {
            self.start = false;
            if !self
                .buffer
                .iter()
                .enumerate()
                .any(|x| x.0 == <I::Item as Into<usize>>::into(*x.1))
            {
                return Some(self.buffer.clone());
            }
        }

        // Exhausted iteration
        let has_two_next = self.index + 2 < self.buffer.len();
        if !has_two_next
            && (self.buffer.len() <= self.index + 1
                || self.buffer[0] <= self.buffer[self.index + 1])
        {
            return None;
        }

        // Determine shift index
        let shift_index = if has_two_next && self.buffer[self.index + 2] <= self.buffer[self.index]
        {
            self.index + 2
        } else {
            self.index + 1
        };

        // Prefix shift
        let shift_elem = self.buffer[shift_index];
        let mut swap_index = shift_index;
        while swap_index > 0 {
            self.buffer[swap_index] = self.buffer[swap_index - 1];
            swap_index -= 1;
        }
        self.buffer[0] = shift_elem;

        // Update index
        if self.buffer[0] < self.buffer[1] {
            self.index = 0;
        } else {
            self.index += 1;
        }
        if !self
            .buffer
            .iter()
            .enumerate()
            .any(|x| x.0 == <I::Item as Into<usize>>::into(*x.1))
        {
            Some(self.buffer.clone())
        } else {
            self.next()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{derangements, derangements_range, derangements_range_fast, distinct_permutations};
    use itertools::{assert_equal, Itertools};
    use std::time::Instant;

    #[test]
    fn derangements_manual() {
        assert_equal(derangements_iter(0..1, 1), Vec::<Vec<usize>>::new());
        assert_equal(derangements_iter(0..2, 2), vec![[1usize, 0]]);
        assert_equal(
            derangements_iter(0..3, 3).sorted(),
            vec![[1usize, 2, 0], [2, 0, 1]],
        );
        assert_equal(
            derangements_iter(0..3, 2).sorted(),
            vec![[1usize, 0], [1, 2], [2, 0]],
        );
        // assert_equal(
        //     derangements_iter([0, 0, 1].into_iter(), 3),
        //     vec![[1, 0, 0], [1, 0, 0]],
        // );  // TODO: fix for fast_permutations
        for k in 5..11 {
            // println!("{:?}", (0..k).permutations(k).collect_vec().len());
            // println!("{:?}", multiset_permutations(0..k).collect_vec().len());
            // println!("{:?}", distinct_derangements(0..k).collect_vec().len());
            // println!("{:?}", derangements_iter(0..k, k).collect_vec().len());

            let before_true = Instant::now();
            _ = (0..k).permutations(k).collect_vec().len();
            let before = Instant::now();
            _ = fast_permutations(0..k, k).collect_vec().len();
            let between = Instant::now();
            _ = distinct_permutations(0..k).collect_vec().len();
            let between2 = Instant::now();
            _ = distinct_derangements(0..k).collect_vec().len();
            let after = Instant::now();
            _ = derangements_iter(0..k, k).collect_vec().len();
            let after2 = Instant::now();

            println!(
                "{:?}, default perm {:?}, new perm {:?}, distinct perm {:?}, distinct derang {:?}, derang iter: {:?}; multiset ratio: {:?}, default ratio: {:?}",
                k,
                before - before_true,
                between - before,
                between2 - between,
                after - between2,
                after2 - after,
                (after - between2).as_secs_f64() / (between2 - between).as_secs_f64(),
                (after2 - after).as_secs_f64() / (between - before).as_secs_f64(),
            )
        }
    }

    #[test]
    fn test_time() {
        use std::time::Instant;
        for k in 0..10 {
            let before = Instant::now();
            _ = derangements_range(k).len();
            let between = Instant::now();
            _ = derangements_range_fast(k).len();
            let between2 = Instant::now();
            _ = derangements(0..k, k).len();
            let after = Instant::now();
            _ = derangements_iter(0..k, k).collect_vec().len();
            let after2 = Instant::now();
            _ = distinct_derangements(0..k).collect_vec().len();
            let after3 = Instant::now();
            println!(
                "{:?}, range old {:?}, range new {:?}, non-range {:?}, iter: {:?}, distinct_iter: {:?}",
                k,
                between - before,
                between2 - between,
                after - between2,
                after2 - after,
                after3 - after2
            )
        }
    }
}
