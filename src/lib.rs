/// Straight syntax translation from the youtube video's python solution.
pub fn min_path_sum_youtube(grid: &mut [&mut [i32]]) -> i32 {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if i == 0 && j == 0 {
                continue;
            }
            let (mut left_path, mut up_path) = (std::i32::MAX, std::i32::MAX);
            if i != 0 {
                up_path = grid[i][j] + grid[i - 1][j];
            }
            if j != 0 {
                left_path = grid[i][j] + grid[i][j - 1];
            }
            grid[i][j] = std::cmp::min(left_path, up_path);
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1]
}

/// Straight syntax translation from the youtube video's python solution using mutable vectors.
pub fn min_path_sum_youtube_vec(grid: &mut Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if i == 0 && j == 0 {
                continue;
            }
            let (mut left_path, mut up_path) = (std::i32::MAX, std::i32::MAX);
            if i != 0 {
                up_path = grid[i][j] + grid[i - 1][j];
            }
            if j != 0 {
                left_path = grid[i][j] + grid[i][j - 1];
            }
            grid[i][j] = std::cmp::min(left_path, up_path);
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1]
}

/// Idiomatic Rust solution using pattern matching.
pub fn min_path_sum_match(grid: &mut Vec<Vec<i32>>) -> i32 {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = match (i, j) {
                (0, 0) => grid[i][j],
                (0, _) => grid[i][j] + grid[i][j - 1],
                (_, 0) => grid[i][j] + grid[i - 1][j],
                _ => grid[i][j] + std::cmp::min(grid[i - 1][j], grid[i][j - 1]),
            };
        }
    }
    grid[grid.len() - 1][grid[0].len() - 1]
}

/// Idiomatic Rust solution using pattern matching with overflow handling.
pub fn min_path_sum_overflow(grid: &mut Vec<Vec<i32>>) -> Option<i32> {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j] = match (i, j) {
                (0, 0) => grid[i][j],
                (0, _) => grid[i][j].checked_add(grid[i][j - 1])?,
                (_, 0) => grid[i][j].checked_add(grid[i - 1][j])?,
                _ => grid[i][j].checked_add(std::cmp::min(grid[i - 1][j], grid[i][j - 1]))?,
            };
        }
    }
    Some(grid[grid.len() - 1][grid[0].len() - 1])
}

/// Returns an iterator that will iterate over the grid where each item is the
/// value above, the value to the left, and the current value.
pub fn grid_iterator<'a, OUTER, INNER>(
    grid: OUTER,
) -> impl Iterator<Item = (Option<i32>, Option<i32>, &'a mut i32)>
where
    OUTER: IntoIterator<Item = INNER> + 'a,
    INNER: AsRef<&'a mut i32>,
{
    // First run has no prior row so no above value
    // return an empty iterator
    let empty = std::iter::empty();
    empty
}

#[cfg(test)]
mod tests {
    const PROBLEM: &[&[i32]] = &[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]];
    const PROBLEM_ANS: i32 = 7;

    const ONE_ROW: &[&[i32]] = &[&[1, 3, 1]];
    const ONE_ROW_ANS: i32 = 5;

    const ONE_COL: &[&[i32]] = &[&[1], &[1], &[4]];
    const ONE_COL_ANS: i32 = 6;

    const PROBLEM_WITH_OVERFLOW: &[&[i32]] = &[&[1, std::i32::MAX, 1]];

    trait MaybeI32 {
        fn maybe(self) -> Option<i32>;
    }

    impl MaybeI32 for i32 {
        fn maybe(self) -> Option<i32> {
            Some(self)
        }
    }
    impl MaybeI32 for Option<i32> {
        fn maybe(self) -> Option<i32> {
            self
        }
    }

    #[test]
    fn problem_validation() {
        assert_eq!(PROBLEM.len(), 3);
        for row in PROBLEM {
            assert_eq!(row.len(), 3);
        }
    }

    fn test_all<R>(f: impl Fn(&mut Vec<Vec<i32>>) -> R)
    where
        R: MaybeI32,
    {
        let mut copy = PROBLEM
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy).maybe(), Some(PROBLEM_ANS));
        let mut copy = ONE_COL
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy).maybe(), Some(ONE_COL_ANS));
        let mut copy = ONE_ROW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy).maybe(), Some(ONE_ROW_ANS));
    }

    #[test]
    fn test_min_path_sum_youtube_vec() {
        test_all(super::min_path_sum_youtube_vec);
    }

    #[test]
    fn test_min_path_sum_match() {
        test_all(super::min_path_sum_match);
    }

    #[test]
    fn test_min_path_sum_overflow() {
        test_all(super::min_path_sum_overflow);
    }

    #[test]
    #[should_panic]
    fn test_overflow_panics() {
        let mut copy = PROBLEM_WITH_OVERFLOW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        _ = super::min_path_sum_match(&mut copy);
    }

    #[test]
    fn test_overflow_correctly_handled() {
        let mut copy = PROBLEM_WITH_OVERFLOW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_overflow(&mut copy), None);
    }
}
