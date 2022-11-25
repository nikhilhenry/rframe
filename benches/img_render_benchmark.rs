use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rframe::Config;
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("img render", |b| {
        b.iter(|| {
            rframe::run(Config {
                file_path: String::from("screen.png"),
                output_path: String::from("output.png"),
            })
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
