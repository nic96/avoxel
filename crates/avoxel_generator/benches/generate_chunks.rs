#[macro_use]
extern crate criterion;
use avoxel_chunk::chunk::Chunk;
use avoxel_generator::generator::generate_chunk;
use avoxel_math::Pos;
use criterion::Criterion;

fn generate_chunks(chunk_size: i32) -> Vec<Chunk> {
    let mut chunks = vec![];
    let chunk_count = 2621440 / chunk_size.pow(3);
    for i in 0..chunk_count {
        chunks.push(generate_chunk(&Pos::new(i, 0, 0)));
    }
    chunks
}

fn bench_generate_chunk64(c: &mut Criterion) {
    c.bench_function("generate_chunk64", |b| b.iter(|| generate_chunks(64)));
}

fn bench_iterate(c: &mut Criterion) {
    c.bench_function("iterate_while_262144", |b| {
        b.iter(|| iterate_while_262144())
    });
    c.bench_function("iterate_for_262144", |b| b.iter(|| iterate_for_262144()));
}

fn iterate_while_262144() {
    let mut voxels = vec![0; 262144];
    let mut voxels_iter = voxels.iter_mut().enumerate();
    while let Some((i, v)) = voxels_iter.next() {
        *v = i;
    }
}

fn iterate_for_262144() {
    let mut voxels = vec![0; 262144];
    for (i, v) in voxels.iter_mut().enumerate() {
        *v = i;
    }
}

criterion_group!(benches, bench_generate_chunk64, bench_iterate,);
criterion_main!(benches);
