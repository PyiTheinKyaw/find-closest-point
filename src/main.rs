use fnp::{KDTree, Point3D, IKDTree, Point};
use criterion::{criterion_group, criterion_main, Criterion};
use std::time::{Duration, Instant};

fn bench_find_closest(c: &mut Criterion) {
    let mut group = c.benchmark_group("find_closest");

    //Generating point.
    println!("[+] Generating 10M Points...........");
    let mut points = Point3D::generate_points(10_000_000, 1.0, 2.0);

    println!("[+] 10M Point generated");

    println!("[+] Creating and Inserting into KDTree Data structure............");
    let c_start = Instant::now();
    let root = KDTree::create_kd_tree(&mut points, 0, 3).unwrap();
    println!("[+] KDTree Created");
    let c_duration = c_start.elapsed();
    println!("[+] Done. Execution time of create_kd_tree: {:?} ns", c_duration.as_nanos());

    let query_point = Point3D::random_point(1.0,1.6);
    println!("[+] Query point: {:?}", query_point);

    group.bench_function("k = 3, limit = 10", |b| {
        b.iter(|| {
            println!("[+] Start finding the nearest points............");
            let start = Instant::now();
            let result = root.find_closest(&query_point, 3, 10);
            let duration = start.elapsed();
            println!("[+] Done. Execution time of find_closest: {:?} ns", duration.as_nanos());
            println!("[+] Done. Execution time: {:?}", duration);

            let len = result.as_ref().unwrap().len();
            println!("[+] Got {:?} long result.", len);

            let point = result.unwrap();
            for i in &point {
                println!("Closest Point: [{:?}] ", i);
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench_find_closest);
criterion_main!(benches);