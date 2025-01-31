use itertools::Itertools;

/// Derange all elements of a range of 0 to n (non-inclusive).
///
/// # Arguments
///
/// * `n`: usize integer that determines the range to derange
///
/// returns: `Vec<Vec<usize>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::derangements_range;
/// assert_equal(derangements_range(3), [[2, 0, 1], [1, 2, 0]]);
/// ```
///
/// This version is slower than ``derange_range_fast`` but
/// consumes less memory.
///
pub fn derangements_range(n: usize) -> Vec<Vec<usize>> {
    match n {
        1 => Vec::new(),
        0 => vec![Vec::new()],
        _ => {
            let mut derangements = Vec::new();
            let lag1 = derangements_range(n - 1);
            for lag in lag1.iter() {
                for split in 0..lag.len() {
                    let mut temp = lag
                        .iter()
                        .enumerate()
                        .map(|x| if x.0 == split { n - 1 } else { *x.1 })
                        .collect_vec();
                    temp.push(lag[split]);
                    derangements.push(temp);
                }
            }

            let lag2 = derangements_range(n - 2);
            for lag in lag2.iter() {
                let mut temp = lag.clone();
                let mut temp2 = lag.clone();
                temp.push(n - 1);
                temp.push(n - 2);
                derangements.push(temp);

                for k in (0..n - 2).rev() {
                    let mut temp = Vec::new();
                    for (i, el) in temp2.iter_mut().enumerate() {
                        if i == k {
                            temp.push(n - 1);
                        }
                        if *el == k {
                            *el = k + 1;
                        }
                        temp.push(*el)
                    }
                    temp.push(k);
                    derangements.push(temp);
                }
            }
            derangements
        }
    }
}

/// Derange all elements of a range of 0 to n (non-inclusive) with caching.
///
/// # Arguments
///
/// * `n`: usize integer that determines the range to derange
///
/// returns: `Vec<Vec<usize>>`
///
/// # Examples
///
/// ```
/// use itertools::{assert_equal, Itertools};
/// use derangements::derangements_range_fast;
/// assert_equal(derangements_range_fast(3), [[2, 0, 1], [1, 2, 0]]);
/// ```
pub fn derangements_range_fast(n: usize) -> Vec<Vec<usize>> {
    match n {
        1 => Vec::new(),
        0 => vec![Vec::new()],
        _ => {
            let mut lag2 = derangements_range(0);
            let mut lag1 = derangements_range(1);
            let mut lag0: Vec<Vec<usize>>;
            let mut out: Vec<Vec<usize>> = vec![];

            for j in 2..n + 1 {
                lag0 = Vec::new();
                for lag in lag1.iter() {
                    for split in 0..lag.len() {
                        let mut temp = lag
                            .iter()
                            .enumerate()
                            .map(|x| if x.0 == split { j - 1 } else { *x.1 })
                            .collect_vec();
                        temp.push(lag[split]);
                        lag0.push(temp);
                    }
                }
                for lag in lag2.iter() {
                    let mut temp = lag.clone();
                    let mut temp2 = lag.clone();
                    temp.push(j - 1);
                    temp.push(j - 2);
                    lag0.push(temp);

                    for k in (0..j - 2).rev() {
                        let mut temp = Vec::new();
                        for (i, el) in temp2.iter_mut().enumerate() {
                            if i == k {
                                temp.push(j - 1);
                            }
                            if *el == k {
                                *el = k + 1;
                            }
                            temp.push(*el)
                        }
                        temp.push(k);
                        lag0.push(temp);
                    }
                }
                if j < n {
                    (lag2, lag1) = (lag1, lag0)
                } else {
                    out = lag0
                }
            }
            out
        }
    }
}

#[derive(Debug, Clone)]
pub struct DerangementsRangeIterator {
    lag1_done: bool,
    lag: Option<Box<DerangementsRangeIterator>>,
    init: bool,
    n: usize,
    curr_lag: Vec<usize>,
    count: usize,
}

pub fn derangements_by_range(n: usize) -> DerangementsRangeIterator {
    DerangementsRangeIterator {
        lag1_done: false,
        lag: None,
        init: true,
        n,
        curr_lag: vec![],
        count: n.saturating_sub(1),
    }
}

impl Iterator for DerangementsRangeIterator {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.n {
            0 => {
                return if self.lag1_done {
                    None
                } else {
                    self.lag1_done = true;
                    Some(vec![])
                }
            }
            1 => return None,
            _ => {}
        };

        // First iteration: init derangements for 1 lag lower
        if self.init {
            self.init = false;
            self.lag = Option::from(Box::from(derangements_by_range(self.n - 1)))
        }

        // For each draw of lag derangements: get a new one after n-1 iterations
        // Store new lagged derangements in self.curr_lag
        if self.count == self.n - 1 {
            self.count = 0;
            let next = self.lag.as_mut().unwrap().next();
            match next {
                // If no lagged derangements left: go to n-2 if in 1st part, else stop
                None => {
                    if self.lag1_done {
                        return None;
                    };
                    self.lag1_done = true;
                    self.lag = Option::from(Box::from(derangements_by_range(self.n - 2)));
                    let next2 = self.lag.as_mut().unwrap().next();
                    match next2 {
                        None => return None,
                        Some(x) => self.curr_lag = x,
                    }
                }
                Some(x) => self.curr_lag = x,
            }
        }

        // Actually generate the derangement, depending on whether it's based on a lag of 1 or 2
        // TODO: document approach somewhere
        let mut new = self.curr_lag.clone();
        if !self.lag1_done {
            // Part 1: swap new el with each el in lagged vec
            new.push(self.n - 1);
            new.swap(self.count, self.n - 1);
        } else {
            // Part 2: find options where new el will make not-deranged lagged vec deranged
            if self.count == 0 {
                new.push(self.n - 1);
                new.push(self.n - 2);
            } else {
                let i = self.n - 2 - self.count;
                let (j, el) = new.iter_mut().find_position(|x| **x == i).unwrap();
                *el += 1;
                self.curr_lag[j] += 1;
                new.insert(i, self.n - 1);
                new.push(i);
            }
        }
        self.count += 1;
        Some(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::derangements;
    use itertools::assert_equal;

    #[test]
    fn test_range_manual() {
        assert_equal(derangements_range(2), [[1, 0]]);
        assert_equal(derangements_range(3), [[2, 0, 1], [1, 2, 0]]);
        assert_equal(
            derangements_range(4),
            [
                [3, 0, 1, 2],
                [2, 3, 1, 0],
                [2, 0, 3, 1],
                [3, 2, 0, 1],
                [1, 3, 0, 2],
                [1, 2, 3, 0],
                [1, 0, 3, 2],
                [2, 3, 0, 1],
                [3, 2, 1, 0],
            ],
        );
        assert_eq!(derangements_range(8).len(), 14833);
    }

    #[test]
    fn test_range_fast_manual() {
        assert_equal(derangements_range_fast(2), [[1, 0]]);
        assert_equal(derangements_range_fast(3), [[2, 0, 1], [1, 2, 0]]);
        assert_equal(
            derangements_range_fast(4),
            [
                [3, 0, 1, 2],
                [2, 3, 1, 0],
                [2, 0, 3, 1],
                [3, 2, 0, 1],
                [1, 3, 0, 2],
                [1, 2, 3, 0],
                [1, 0, 3, 2],
                [2, 3, 0, 1],
                [3, 2, 1, 0],
            ],
        );
        assert_eq!(derangements_range_fast(8).len(), 14833);
    }

    #[test]
    fn test_nonrange_range() {
        for k in 0..8 {
            assert_equal(
                derangements_range(k).into_iter().sorted(),
                derangements(0..k, k).sorted(),
            );
        }
    }
}
