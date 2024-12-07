use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_2024_3(c: &mut Criterion) {
    let pz = aoc2024::day::day3::Puzzle::new();

    let mut group = c.benchmark_group("2024/3");
    group.bench_function("parse", |b| {
        b.iter(|| {
            aoc2024::day::day3::Puzzle::new();
        })
    });

    group.bench_function("part1", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part1())
        })
    });

    group.bench_function("part2-single-threaded", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2());
        })
    });
}

fn bench_2024_4(c: &mut Criterion) {
    let pz = aoc2024::day::day4::Puzzle::new();

    let mut group = c.benchmark_group("2024/4");
    group.bench_function("parse", |b| {
        b.iter(|| {
            aoc2024::day::day4::Puzzle::new();
        })
    });

    group.bench_function("part1", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part1())
        })
    });

    group.bench_function("part2-multi-threaded", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2());
        })
    });
}

fn bench_2024_5(c: &mut Criterion) {
    let pz = aoc2024::day::day5::Puzzle::new();

    let mut group = c.benchmark_group("2024/5");
    group.bench_function("parse", |b| {
        b.iter(|| {
            aoc2024::day::day5::Puzzle::new();
        })
    });

    group.bench_function("part1", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part1())
        })
    });

    group.bench_function("part2-single-threaded", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2());
        })
    });

    group.bench_function("part2-multi-threaded", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2_parallel());
        })
    });
}

fn bench_2024_6(c: &mut Criterion) {
    let pz = aoc2024::day::day6::Puzzle::new();

    let mut group = c.benchmark_group("2024/6");
    group.bench_function("parse", |b| {
        b.iter(|| {
            aoc2024::day::day6::Puzzle::new();
        })
    });

    group.bench_function("part1", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part1())
        })
    });

    group.bench_function("part2", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2_parallel());
        })
    });
}

pub fn bench_2024_7(c: &mut Criterion) {
    let pz = aoc2024::day::day7::Puzzle::new();

    let mut group = c.benchmark_group("2024/7");
    group.bench_function("parse", |b| {
        b.iter(|| aoc2024::day::day7::Puzzle::new());
    });

    group.bench_function("part1", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part1())
        })
    });

    group.bench_function("part2", |b| {
        b.iter(|| {
            let pz = pz.clone();
            black_box(pz.part2_parallel())
        })
    });
}

// fn bench_2024_xxx(c: &mut Criterion) {
//     let pz = aoc2024::day::day7::Puzzle::new();

//     let mut group = c.benchmark_group("2024/{day}");
//     group.bench_function("part1", |b| {
//         b.iter(|| {
//             todo!()
//         })
//     });

//     group.bench_function("part2", |b| {
//         b.iter(|| {
//             todo!()
//         })
//     });
// }

criterion_group!(
    benches,
    bench_2024_3,
    bench_2024_4,
    bench_2024_5,
    bench_2024_6,
    bench_2024_7,
);

criterion_main!(benches);
