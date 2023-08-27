use nalgebra as na;

use na::{Rotation3, SMatrix, Unit, UnitQuaternion, Vector3};

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RotationRepresentations {
    rotation_matrix: SMatrix<f64, 3, 3>,
    quaternion: UnitQuaternion<f64>,
    euler_angles: (f64, f64, f64),
    axis: Unit<Vector3<f64>>,
    angle: f64,
}

impl RotationRepresentations {
    pub fn new(rotation: Rotation3<f64>) -> Self {
        let quaternion = UnitQuaternion::from_rotation_matrix(&rotation);
        let euler_angles = rotation.euler_angles();
        let (axis, angle) = rotation
            .axis_angle()
            .unwrap_or((Unit::new_unchecked(Vector3::x()), 0.0));

        RotationRepresentations {
            rotation_matrix: rotation.into_inner(),
            quaternion,
            euler_angles,
            axis,
            angle,
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(&self)
    }
}
