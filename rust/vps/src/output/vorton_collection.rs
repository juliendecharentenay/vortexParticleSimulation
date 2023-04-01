use crate::{Vorton, Simulation};

#[derive(Debug)]
pub struct VortonCollection<'a> {
  vortons: &'a Vec<Vorton>,
}

impl<'a> From<&'a Simulation> for VortonCollection<'a> {
  fn from(s: &'a Simulation) -> VortonCollection {
    VortonCollection { vortons: s.vortons() }
  }
}

impl<'a> VortonCollection<'a> {
  pub fn to_writer_csv<W, F>(&self, mut writer: W, label: &str, f: F) -> Result<(), Box<dyn std::error::Error>>
  where W: std::io::Write,
        F: Fn(&Vorton) -> Result<f64, Box<dyn std::error::Error>>
  {
    writer.write_all(format!("x coord, y coord, z coord, {label}\n").as_bytes())?;
    for v in self.vortons.iter() {
      writer.write_all(format!("{}, {}, {}, {}\n", 
                                v.position().x, v.position().y, v.position().z, f(v)?).as_bytes())?;
    }
    Ok(())
  }
}
