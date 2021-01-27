use criterion::{black_box, criterion_group, criterion_main, Criterion};
use imanager::vptree::VPTree;

const VPTREE_DATA_PATH: &'static str = "examples/data/bench/vptree_data.bin";

fn tree_creation_benchmark(c: &mut Criterion) {
    let vptree_data = std::fs::read(VPTREE_DATA_PATH).unwrap();
    let (points, needles): (Vec<(f32, f32)>, Vec<usize>) =
        black_box(bincode::deserialize(&vptree_data).unwrap());
    c.bench_function("Tree creation", |b| {
        b.iter(|| {
            let tree = VPTree::new(&points, |a, b| {
                ((a.0 - b.0 as f32).powi(2) + (a.1 - b.1 as f32).powi(2)).sqrt()
            });
            tree.find_nearest_neighbor(&points[needles[0]]);
        })
    });
}

fn nearest_neighbor_search_benchmark(c: &mut Criterion) {
    let vptree_data = std::fs::read(VPTREE_DATA_PATH).unwrap();
    let (points, needles): (Vec<(f32, f32)>, Vec<usize>) =
        black_box(bincode::deserialize(&vptree_data).unwrap());
    let tree = VPTree::new(&points, |a, b| {
        ((a.0 - b.0 as f32).powi(2) + (a.1 - b.1 as f32).powi(2)).sqrt()
    });
    c.bench_function("Nearest neighbor search", |b| {
        b.iter(|| {
            for needle in needles.iter() {
                tree.find_nearest_neighbor(&points[*needle]);
            }
        })
    });
}

fn hundred_nearest_neighbor_search_benchmark(c: &mut Criterion) {
    let vptree_data = std::fs::read(VPTREE_DATA_PATH).unwrap();
    let (points, needles): (Vec<(f32, f32)>, Vec<usize>) =
        black_box(bincode::deserialize(&vptree_data).unwrap());
    let tree = VPTree::new(&points, |a, b| {
        ((a.0 - b.0 as f32).powi(2) + (a.1 - b.1 as f32).powi(2)).sqrt()
    });
    c.bench_function("100 nearest neighbors search", |b| {
        b.iter(|| {
            for needle in needles.iter() {
                tree.find_k_nearest_neighbors(&points[*needle], 100);
            }
        })
    });
}

fn neighbors_within_radius_search_benchmark(c: &mut Criterion) {
    let vptree_data = std::fs::read(VPTREE_DATA_PATH).unwrap();
    let (points, needles): (Vec<(f32, f32)>, Vec<usize>) =
        black_box(bincode::deserialize(&vptree_data).unwrap());
    let tree = VPTree::new(&points, |a, b| {
        ((a.0 - b.0 as f32).powi(2) + (a.1 - b.1 as f32).powi(2)).sqrt()
    });
    c.bench_function("Neighbors within radius search", |b| {
        b.iter(|| {
            for needle in needles.iter() {
                tree.find_neighbors_within_radius(&points[*needle], 20000.0);
            }
        })
    });
}

criterion_group!(
    benches,
    tree_creation_benchmark,
    nearest_neighbor_search_benchmark,
    hundred_nearest_neighbor_search_benchmark,
    neighbors_within_radius_search_benchmark
);
criterion_main!(benches);
