

#[cfg(test)]
mod tests {
    use nalgebra as na;
    use na::{Rotation3, Vector3};
    use spin3d::RotationRepresentations;

    #[test]
    fn test_description() {
        let axisangle = Vector3::y() * std::f64::consts::FRAC_PI_2;
        let rotation: Rotation3<f64> = Rotation3::new(axisangle);
    
        let description = RotationRepresentations::new(rotation);
    
        println!("{}", description.to_json().unwrap());
        // Add more test cases as needed
    }
}
