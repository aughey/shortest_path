use criterion::{criterion_group, criterion_main, Criterion};
use shortest_path::{min_path_sum_match, min_path_sum_overflow, min_path_sum_youtube_vec};

const PROBLEM: &[&[i32]] = &[&[1, 3, 1], &[1, 5, 1], &[4, 2, 1]];
const PROBLEM_ANS: i32 = 7;

fn criterion_benchmark(c: &mut Criterion) {
    let copy = PROBLEM
        .iter()
        .copied()
        .map(|row| row.to_vec())
        .collect::<Vec<Vec<i32>>>();

    c.bench_function("youtube_solution", |b| {
        b.iter(|| {
            let ans = min_path_sum_youtube_vec(&mut copy.clone());
            assert_eq!(ans, PROBLEM_ANS);
        })
    });

    c.bench_function("min_path_sum_match", |b| {
        b.iter(|| {
            let ans = min_path_sum_match(&mut copy.clone());
            assert_eq!(ans, PROBLEM_ANS);
        })
    });

    // generate a 100x100 grid
    let copy = (0..100)
        .map(|_| (0..100).map(|x| x as i32).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let real_ans = min_path_sum_match(&mut copy.clone());
    c.bench_function("youtube_solution_100_100", |b| {
        b.iter(|| {
            let _ans = min_path_sum_youtube_vec(&mut copy.clone());
            assert_eq!(_ans, real_ans);
        })
    });

    c.bench_function("min_path_sum_match_100_100", |b| {
        b.iter(|| {
            let _ans = min_path_sum_match(&mut copy.clone());
            assert_eq!(_ans, real_ans);
        })
    });

    c.bench_function("min_path_sum_overflow_handle", |b| {
        b.iter(|| {
            let ans = min_path_sum_overflow(&mut copy.clone());
            assert_eq!(ans, Some(real_ans));
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
