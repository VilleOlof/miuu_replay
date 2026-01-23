use criterion::{Criterion, criterion_group, criterion_main};
use miuu_replay::Replay;
use std::hint::black_box;

fn bench_full_replay(file_path: &str, c: &mut Criterion) {
    let file = std::fs::read(&file_path).unwrap();

    let mut group = c.benchmark_group(format!("throughput/{file_path}"));
    group.throughput(criterion::Throughput::Bytes(file.len() as u64));

    group.bench_function("basic_parse", |b| {
        b.iter(|| {
            black_box(Replay::parse(&file).unwrap());
        })
    });

    group.bench_function("full_parse", |b| {
        b.iter(|| {
            black_box(
                Replay::parse(&file)
                    .unwrap()
                    .decode_replay_buffer()
                    .unwrap(),
            );
        })
    });
}

fn bench(c: &mut Criterion) {
    bench_full_replay("benches/medieval_machinations.replay", c);
    // bench_full_replay("benches/ribbon.replay", c);
    // bench_full_replay("benches/amethyst.replay", c);
}

criterion_group!(throughput, bench);
criterion_main!(throughput);
