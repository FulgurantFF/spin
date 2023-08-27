extern crate criterion;

use na::Rotation3;
use nalgebra as na;

use criterion::{criterion_group, criterion_main, Criterion};

use rand::Rng;

use spin3d::RotationRepresentations;

fn random_rotation<R: Rng>(rng: &mut R) -> Rotation3<f64> {
    let angle_x: f64 = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    let angle_y: f64 = rng.gen_range(0.0..std::f64::consts::PI * 2.0);
    let angle_z: f64 = rng.gen_range(0.0..std::f64::consts::PI * 2.0);

    Rotation3::from_euler_angles(angle_x, angle_y, angle_z)
}

fn rotation_describe(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let rotation = random_rotation(&mut rng);

    c.bench_function("Rotation to Quaternion", |b| {
        b.iter(|| {
            let description = RotationRepresentations::new(rotation);
            let _json_payload = description.to_json().unwrap();
        });
    });
}

criterion_group!(benches, rotation_describe);
criterion_main!(benches);
