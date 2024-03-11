use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use dirwalk_bench::{async_walkdir, fts, jwalk, walkdir};

fn bench_implementations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Walkdirs");
    let bench_dirs = vec!["./", "~/"];

    for bench_dir in bench_dirs {
        println!("\n\n\nRunning at bench dir {bench_dir:?}  (Current dir: {})\n", std::env::current_dir().unwrap().display());

        group.bench_with_input(BenchmarkId::new("walkdir ", bench_dir), bench_dir, 
            |b, bench_dir: &str| b.iter(|| walkdir(bench_dir.to_string())));
        group.bench_with_input(BenchmarkId::new("async_walkdir ", bench_dir), bench_dir, 
            |b, bench_dir: &str| b.iter(|| async_walkdir(bench_dir.to_string())));
        group.bench_with_input(BenchmarkId::new("fts ", bench_dir), bench_dir, 
            |b, bench_dir: &str| b.iter(|| fts(bench_dir.to_string())));
        group.bench_with_input(BenchmarkId::new("jwalk ", bench_dir), bench_dir, 
            |b, bench_dir: &str| b.iter(|| jwalk(bench_dir.to_string())));
    }

    group.finish();
}

criterion_group!(benches, bench_implementations);
criterion_main!(benches);
