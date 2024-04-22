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

use std::time::Instant;
use fnp::model::kd_tree::KDTree;
use fnp::model::point3d::Point3D;

use fnp::functions::dataset::Dataset;


fn main() {

    let mut points = Point3D::generate_data_list(100000, 1.0, 3.0);

    // let points = vec![
    //     Point3D::new(100.0, 1.0, 7.0),
    //     Point3D::new(100.0, 1.0, 7.0),
    //     Point3D::new(100.0, 1.0, 7.0),
    // ];

    let start = Instant::now();
    let k = 3;
    let min_points_per_subset = 20;
    let kd_tree_result = KDTree::create_kd_tree(points, k, min_points_per_subset);

    let duration = start.elapsed();
    println!("[+] Done. Execution time: {:?} ms", duration.as_millis());

    match kd_tree_result {
        Ok(kd_tree) => {
            // KD-tree creation succeeded, continue with further operations...
            let root = kd_tree.root;
            // println!("Root: {:?}", root);

        }
        Err(error) => {
            // KD-tree creation failed, handle the error...
        }
    }
}
