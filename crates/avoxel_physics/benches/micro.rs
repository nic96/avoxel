#[macro_use]
extern crate criterion;
use bevy::prelude::Vec3;
use criterion::{black_box, BenchmarkId, Criterion};

fn sqrt_clamp_velocity(velocity: Vec3, max_velocity: f32) -> Vec3 {
    let mut velocity = velocity;
    if velocity.length() > max_velocity {
        velocity = velocity.normalize() * max_velocity;
    }
    velocity
}

fn squared_clamp_velocity(velocity: Vec3, max_velocity: f32) -> Vec3 {
    let mut velocity = velocity;
    if velocity.length_squared() > max_velocity * max_velocity {
        velocity = velocity.normalize() * max_velocity;
    }
    velocity
}

fn bench_clamp_velocity(c: &mut Criterion) {
    let max_velocity: f32 = 50_f32;

    c.bench_with_input(
        BenchmarkId::new("sqrt_clamp_velocity", max_velocity),
        &max_velocity,
        |b, &s| b.iter(|| sqrt_clamp_velocity(black_box(Vec3::new(23., 43., 98.)), s)),
    );

    c.bench_with_input(
        BenchmarkId::new("squared_clamp_velocity", max_velocity),
        &max_velocity,
        |b, &s| b.iter(|| squared_clamp_velocity(black_box(Vec3::new(23., 43., 98.)), s)),
    );
}

criterion_group!(benches, bench_clamp_velocity);
criterion_main!(benches);
