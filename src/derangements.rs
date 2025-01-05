use itertools::Itertools;

type T = usize;

pub fn derangements_range(n: T) -> Vec<Vec<T>> {
    match n {
        2 => vec![vec![1, 0]],
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
                    if k == temp2.len() {
                        temp.push(n - 1)
                    }
                    temp.push(k);

                    derangements.push(temp);
                }
            }
            derangements
        }
    }
}

pub fn derangements(iterable: Vec<T>, k: T) -> Vec<Vec<T>> {
    iterable
        .into_iter()
        .permutations(k)
        .filter(|i| !i.iter().enumerate().any(|x| x.0 == *x.1))
        .collect_vec()
}

// To add:
// - dinstict_derangements
// - derangement_self
// - derangement_map
// - random_derangement
// - examples
// - tests
// - docstrings

#[cfg(test)]
mod tests {
    use super::*;
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
    fn test_nonrange_range() {
        for k in 0..8 {
            assert_equal(
                derangements_range(k).into_iter().sorted(),
                derangements((0..k).collect_vec(), k),
            );
        }
    }

    #[test]
    fn test_nonrange_manual() {
        assert_equal(derangements(vec![0, 2], 2), [[2, 0]]);
        assert_equal(
            derangements(vec![0, 1, 3], 3),
            [[1, 0, 3], [1, 3, 0], [3, 0, 1]],
        );
        assert_equal(derangements(vec![0, 1, 3], 2), [[1, 0], [1, 3], [3, 0]]);
        assert_equal(derangements(vec![0, 1, 1], 3), [[1, 0, 1], [1, 0, 1]]);
        assert_equal(derangements(vec![0, 1, 1], 2), [[1, 0], [1, 0]]);
    }
}
