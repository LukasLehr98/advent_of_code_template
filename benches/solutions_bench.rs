use advent_2024::Result;
use criterion::{criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn benchmark_day01(c: &mut Criterion) {
    let input = read_to_string("input/day01.txt").unwrap();
    
    c.bench_function("day01_part1", |b| {
        b.iter(|| {
        })
    });
}

criterion_group!(benches, benchmark_day01);
criterion_main!(benches); 