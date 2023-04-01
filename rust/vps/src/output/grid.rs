/// Define a generic grid
#[derive(derive_builder::Builder)]
pub struct Grid {
}

impl Grid {
  /// Output the grid information and result to a CSV file.
  pub fn to_writer_csv<W, F>(&self, mut writer: W, f: F) -> Result<(), Box<dyn std::error::Error>> 
  where W: std::io::Write,
        F: Fn(&crate::algebra::Point3<f64>) -> Result<f64, Box<dyn std::error::Error>>
  {
    writer.write_all(b"x coord, y coord, z coord, value\n")?;
    let n = 100; let w = 4.0; let d = w / (n as f64);
    for k in 0..2 {
      let z = 0.5 - 0.5 * d + (k as f64) * d;
      for j in 0..n {
        let y = 0.5 - 0.5 * w + (j as f64) * d;
        for i in 0..n {
          let x = 0.5 - 0.5 * w + (i as f64) * d;
          let v = f(&crate::algebra::Point3::new(x, y, z))?;
          writer.write_all(format!("{x},{y},{z},{v}\n").as_bytes())?;
        }
      }
    }
    Ok(())
  }
}

