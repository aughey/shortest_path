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

pub fn min_path_sum_overflow(grid: &mut Vec<Vec<i32>>) -> Option<i32> {
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
    Some(grid[grid.len() - 1][grid[0].len() - 1])
}

#[cfg(test)]
mod tests {
    use std::panic::PanicHookInfo;

    const PROBLEM: &[&[i32]] = &[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]];
    const PROBLEM_ANS: i32 = 7;

    const ONE_ROW: &[&[i32]] = &[&[1, 3, 1]];
    const ONE_ROW_ANS: i32 = 5;

    const ONE_COL: &[&[i32]] = &[&[1], &[1], &[4]];
    const ONE_COL_ANS: i32 = 6;

    #[test]
    fn problem_validation() {
        assert_eq!(PROBLEM.len(), 3);
        for row in PROBLEM {
            assert_eq!(row.len(), 3);
        }
    }

    fn test_all(f: impl Fn(&mut Vec<Vec<i32>>) -> i32) {
        let mut copy = PROBLEM
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy), PROBLEM_ANS);
        let mut copy = ONE_COL
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy), ONE_COL_ANS);
        let mut copy = ONE_ROW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(f(&mut copy), ONE_ROW_ANS);
    }

    #[test]
    fn test_min_path_sum_youtube_vec() {
        test_all(super::min_path_sum_youtube_vec);
    }

    #[test]
    fn test_min_path_sum_match() {
        let mut copy = PROBLEM
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_match(&mut copy), PROBLEM_ANS);
        let mut copy = ONE_COL
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_match(&mut copy), ONE_COL_ANS);
        let mut copy = ONE_ROW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_match(&mut copy), ONE_ROW_ANS);
    }

    #[test]
    fn test_min_path_sum_overflow() {
        let mut copy = PROBLEM
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_overflow(&mut copy), Some(PROBLEM_ANS));
        let mut copy = ONE_COL
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_overflow(&mut copy), Some(ONE_COL_ANS));
        let mut copy = ONE_ROW
            .iter()
            .copied()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(super::min_path_sum_overflow(&mut copy), Some(ONE_ROW_ANS));
    }
}
