#[macro_use]
extern crate criterion;
use avoxel_blocks::BlockLibrary;
use avoxel_chunk::Chunk;
use avoxel_generator::default_generator;
use avoxel_math::Pos;
use avoxel_mesher::mesher_culling::generate_mesh_culled;
use bevy::prelude::Vec3;
use criterion::Criterion;
use noise::{NoiseFn, Seedable};
use rand_core::{RngCore, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::sync::Arc;

fn generate_data() -> Chunk {
    println!("Generating data");
    default_generator::generate_chunk(&Pos::new(0, -1, 0))
}

fn bench_mesher_culling(c: &mut Criterion) {
    let chunk = generate_data();
    let block_library = Arc::new(BlockLibrary::default());
    c.bench_function("mesher_culling", |b| {
        b.iter(|| generate_mesh_culled(&chunk, block_library.clone()))
    });
}

fn bench_random_rotation(c: &mut Criterion) {
    let perlin = noise::Perlin::new();
    perlin.set_seed(1);

    c.bench_function("random_rotation", |b| {
        b.iter(|| random_rotation(Vec3::new(1., 1., 1.)))
    });

    c.bench_function("random_rotation_perlin", |b| {
        b.iter(|| random_rotation_perlin(Vec3::new(1., 1., 1.), &perlin))
    });

    c.bench_function("random_rotation_custom", |b| {
        b.iter(|| random_rotation_custom(Vec3::new(1., 1., 1.)))
    });
}

// seeded with a pos
fn random_rotation(pos: Vec3) -> f32 {
    let x = XorShiftRng::seed_from_u64((pos.x + (pos.y * 64.) + (pos.z * 4032.)) as u64).next_u32();
    let rotation = x % 4 * 90;
    rotation as f32
}

fn random_rotation_perlin(pos: Vec3, perlin: &noise::Perlin) -> f32 {
    let x = (perlin.get([pos.x as f64, pos.y as f64, pos.z as f64]).abs()) * 100.;
    let rotation = x as u32 % 4 * 90;
    rotation as f32
}

fn random_rotation_custom(pos: Vec3) -> f32 {
    let mut r = (pos.x as i32)
        .overflowing_mul(374761393)
        .0
        .overflowing_add((pos.y as i32).overflowing_mul(668265263).0)
        .0
        .overflowing_add((pos.z as i32).overflowing_mul(987643213).0)
        .0; //all constants are prime
    r = (r ^ (r >> 13)).overflowing_mul(1274126177).0;
    r = r ^ (r >> 16);
    (r % 4 * 90) as f32
}

criterion_group!(benches, bench_mesher_culling, bench_random_rotation,);
criterion_main!(benches);
