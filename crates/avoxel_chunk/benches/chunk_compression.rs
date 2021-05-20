#[macro_use]
extern crate criterion;
use avoxel_chunk::chunk::Chunk;
use avoxel_generator::generator::generate_chunk;
use avoxel_math::Pos;
use criterion::{BenchmarkId, Criterion};

fn generate_data() -> Chunk {
    println!("Generating data");
    generate_chunk(&Pos::new(0, -1, 0))
}

fn bench_lz4(c: &mut Criterion) {
    let chunk = generate_data();
    for level in 10..11 {
        println!(
            "Size of lz4 compressed data at level {} is: {:.3} KB",
            level,
            chunk.compress(level, false).compressed_voxels.len() as f64 * 0.001
        );
        c.bench_with_input(BenchmarkId::new("lz4_level", level), &level, |b, &level| {
            b.iter(|| chunk.compress(level, false))
        });
    }
    let compressed_chunk = chunk.compress(10, false);
    c.bench_function("lz4_decompression", |b| {
        b.iter(|| compressed_chunk.decompress(false))
    });
}

fn bench_byteorder_lz4_compression(c: &mut Criterion) {
    let chunk = generate_data();
    for level in 10..11 {
        println!(
            "Size of lz4 byteorder compressed data at level {} is: {:.3} KB",
            level,
            chunk.compress(level, true).compressed_voxels.len() as f64 * 0.001
        );
        c.bench_with_input(
            BenchmarkId::new("byteorder_lz4_level", level),
            &level,
            |b, &level| b.iter(|| chunk.compress(level, true)),
        );
    }

    let compressed_chunk = chunk.compress(10, true);
    c.bench_function("byteorder_lz4_decompression", |b| {
        b.iter(|| compressed_chunk.decompress(true))
    });
}

criterion_group!(benches, bench_lz4, bench_byteorder_lz4_compression);
criterion_main!(benches);
