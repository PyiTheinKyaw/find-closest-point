// // use fnp::{KDTree, Point3D, IKDTree, Point};
// use criterion::{criterion_group, criterion_main, Criterion};
// use std::time::{Duration, Instant};
//
// fn bench_find_closest(c: &mut Criterion) {
//     let mut group = c.benchmark_group("find_closest");
//
//     //Generating point.
//     println!("[+] Generating 10M Points...........");
//     // let mut points = Point3D::generate_points(10_000_000);
//     println!("[+] 10M Point generated");
//
//     println!("[+] Creating and Inserting into KDTree Data structure............");
//     // let root = KDTree::create_kd_tree(&mut [Point3D::new(10.0,2.0,3.0)], 0, 3).unwrap();
//     println!("[+] KDTree Created");
//
//     // let query_point = Point3D::random_point();
//     // println!("[+] Query point: {:?}", query_point);
//
//     group.bench_function("k = 3, limit = 10", |b| {
//         b.iter(|| {
//             // println!("[+] Start finding the nearest points............");
//             // let start = Instant::now();
//             // let result = root.find_closest(&query_point, 3, 20);
//             // let duration = start.elapsed();
//             // println!("[+] Done. Execution time: {:?} ns", duration.as_nanos());
//             // println!("[+] Done. Execution time: {:?}", duration);
//
//             // let len = result.as_ref().unwrap().len();
//             // println!("[+] Got {:?} long result.", len);
//
//             // let point = result.unwrap();
//             // for i in &point {
//             //     println!("Closest Point: [{:?}] ", i);
//             // }
//         })
//     });
//
//     group.finish();
// }
//
// criterion_group!(benches, bench_find_closest);
// criterion_main!(benches);

use fnp::model::point3d::Point3D;
use fnp::model::sah::SAH;

fn main() {
    let points = vec![
        Point3D::new(1.0, 2.0, 3.0),
        Point3D::new(7.0, 8.0, 9.0),
        Point3D::new(4.0, 52.0, 6.0),
    ];

    // Call the function to select the optimal splitting plane
    let optimal_splitting_plane = SAH::select_optimal_splitting_plane(points.clone(), 3);
    println!("optimal_splitting_plane cost: {:?}", optimal_splitting_plane);
}