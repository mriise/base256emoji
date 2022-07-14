use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use base256utf8_rs::{Emoji, EmojiE, Base};
use rand::Rng;

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    // 256 bits, typical hash lengths
    let mut inputs = Vec::<[u8; 32]>::with_capacity(10);
    for _ in 0..10 {
        inputs.push(rng.gen());
    }
    
    let encoded: Vec<String> = inputs.iter().map(|a| Emoji::default().encode(&a.to_vec())).collect();

    let mut group = c.benchmark_group("map");
    for (i, str) in encoded.iter().enumerate() {
        // group.throughput(Bytes(32));
        group.bench_with_input(BenchmarkId::from_parameter(i), str, |b, str| {
            b.iter(|| {
                Emoji::default().decode(black_box(str)).unwrap();
            });
        });
    }
    group.finish();

    let mut group = c.benchmark_group("linear");
    for (i, str) in encoded.iter().enumerate() {
        // group.throughput(Bytes(32));
        group.bench_with_input(BenchmarkId::from_parameter(i), str, |b, str| {
            b.iter(|| {
                EmojiE::default().decode(black_box(str)).unwrap();
            });
        });
    }
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);