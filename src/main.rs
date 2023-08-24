use nalgebra as na;
use serde_json;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

use na::Matrix4;

fn serialize_matrix(matrix: &Matrix4<f64>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&matrix)
}

fn write_to_file(data: &str, path: &Path) -> io::Result<()> {
    let mut file: File = File::create(path)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

fn main() {
    let matrix = Matrix4::<f64>::identity();
    let path: &Path = Path::new("matrix.json");

    match serialize_matrix(&matrix) {
        Ok(serialized_data) => {
            if let Err(e) = write_to_file(&serialized_data, path) {
                eprintln!("Failed to write data to file: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to serialize matrix: {}", e);
        }
    }
}

