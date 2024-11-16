fn min_path_sum_youtube(grid: &[&[i32]]) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    const PROBLEM: &[&[i32]] = &[
        &[1,3,1],
        &[1,5,1],
        &[4,2,1]
    ];

    #[test]
    fn problem_validation() {
        assert_eq!(PROBLEM.len(), 3);
        for row in PROBLEM {
            assert_eq!(row.len(), 3);
        }
    }

    #[test]
    fn test_min_path_sum_youtube() {
        assert_eq!(super::min_path_sum_youtube(PROBLEM), 7);
    }
}