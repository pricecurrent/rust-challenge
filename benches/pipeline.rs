use criterion::{criterion_group, criterion_main, Criterion};
use rust_challenge::factories::defaults::generator;
use rust_challenge::services::pipeline_orig;
use rust_challenge::services::stats::pipeline;

fn bench_pipelines(c: &mut Criterion) {
    // Generate a fixed set of transfers for fair comparison
    let transfers = generator().build().generate(1_000_000);

    c.bench_function("pipeline_new", |b| {
        b.iter(|| {
            pipeline::calculate_user_stats(&transfers);
        })
    });

    c.bench_function("pipeline_orig", |b| {
        b.iter(|| {
            pipeline_orig::calculate_user_stats(&transfers);
        })
    });
}

criterion_group!(benches, bench_pipelines);
criterion_main!(benches);
