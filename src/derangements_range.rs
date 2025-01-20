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
