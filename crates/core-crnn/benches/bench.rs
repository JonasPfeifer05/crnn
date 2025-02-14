use core_crnn::activation_function::ActivationFunction;
use core_crnn::thinking_layer::ThinkingLayer;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut small_model = ThinkingLayer::new(4, 32, 8, ActivationFunction::Other(|x| x)).unwrap();
    let mut medium_model = ThinkingLayer::new(4, 256, 8, ActivationFunction::Other(|x| x)).unwrap();
    let mut large_model = ThinkingLayer::new(4, 2048, 8, ActivationFunction::Other(|x| x)).unwrap();

    c.bench_function("small", |b| {
        b.iter(|| small_model.tick(Some(vec![0., 0., 0., 0.])))
    });

    c.bench_function("medium", |b| {
        b.iter(|| medium_model.tick(Some(vec![0., 0., 0., 0.])))
    });

    c.bench_function("large", |b| {
        b.iter(|| large_model.tick(Some(vec![0., 0., 0., 0.])))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
