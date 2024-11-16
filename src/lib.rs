#[cfg(test)]
mod tests {

    const PROBLEM: &[&[u8]] = &[
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
}