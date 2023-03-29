use crate::{Point3, Vorton, Vector3};

#[derive(Debug)]
pub struct Grid {
  start: Point3<f64>,
  delta: f64,
  n_cell: usize,
}

impl Grid {
  /// Returns the `x` position associated with index `i`
  // pub fn x(&self, i: usize) -> f64 { self.start.x + (i as f64/ self.n_cell as f64) * self.delta }

  /// Returns the `y` position associated with index `i`
  // pub fn y(&self, i: usize) -> f64 { self.start.y + (i as f64/ self.n_cell as f64) * self.delta }

  /// Returns the `z` position associated with index `i`
  // pub fn z(&self, i: usize) -> f64 { self.start.z + (i as f64/ self.n_cell as f64) * self.delta }

  /// Return the cell index (in a linear array) associated with the cell index `i, j, k`
  // pub fn cell_index(&self, i: usize, j: usize, k: usize) -> usize { i + self.n_cell * (j + self.n_cell * k) }

  /// Return the cell `i, j, k` associated with the provided `Point3`. Returns None if the 
  /// `Point3` is outside of the grid.
  pub fn cell_ijk(&self, p: &Point3<f64>) -> Option<(usize, usize, usize)> {
    let i = (p.x - self.start.x) / self.delta; if i < 0.0 || i > 1.0 { return None };
    let j = (p.y - self.start.y) / self.delta; if j < 0.0 || j > 1.0 { return None };
    let k = (p.z - self.start.z) / self.delta; if k < 0.0 || k > 1.0 { return None };
    Some((((i * (self.n_cell as f64)).floor() as usize).min(self.n_cell),
          ((j * (self.n_cell as f64)).floor() as usize).min(self.n_cell),
          ((k * (self.n_cell as f64)).floor() as usize).min(self.n_cell)))
  }

  /// Returns the position of the cell center
  pub fn cell_center(&self, i: usize, j: usize, k: usize) -> Point3<f64> {
    &self.start 
    + &(Vector3::x().scale((i as f64 + 0.5)/(self.n_cell as f64)*self.delta))
    +  (Vector3::y().scale((j as f64 + 0.5)/(self.n_cell as f64)*self.delta))
    +  (Vector3::z().scale((k as f64 + 0.5)/(self.n_cell as f64)*self.delta))
  }

  /// Returns the cell length, ie the characteristic length of a side of a cell.
  /// Cells are cubic so the length is the same in all direction
  pub fn cell_length(&self) -> f64 {
    self.delta / (self.n_cell as f64)
  }
}

impl std::convert::TryFrom<&Vec<Vorton>> for Grid {
  type Error = Box<dyn std::error::Error>;
  /// Generate a grid to fit an array of vortons
  fn try_from(vortons: &Vec<Vorton>) -> Result<Self, Self::Error> {
    use std::convert::TryInto;
    vortons.iter()
    .fold(None, |acc: Option<(Point3<f64>, Point3<f64>)>, v| 
                   acc.and_then(|(min, max)| Some((min.min(v.position()), max.max(v.position()))))
                   .or(Some((v.position().clone(), v.position().clone()))))
    .unwrap_or((Point3::new(0.0, 0.0, 0.0), Point3::new(1.0, 1.0, 1.0)))
    .try_into()
  }
}

impl std::convert::TryFrom<(Point3<f64>, Point3<f64>)> for Grid {
  type Error = Box<dyn std::error::Error>;
  /// Generate a grid from a bounding box. The bouding box is increased by 1% to ensure
  /// that the bounding box remains contained within the grid
  fn try_from((min, max): (Point3<f64>, Point3<f64>)) -> Result<Self, Self::Error> {
    let v = &max - &min; let mid = &min + &v.scale(0.5); let delta = 1.01*v.x.max(v.y).max(v.z);
    let v = Vector3::new(delta, delta, delta);
    Ok(Grid {
      start: mid - v.scale(0.5),
      delta,
      n_cell: 1
    })
  }
}

impl From<&Grid> for Grid {
  /// Generate a subgrid (ie a grid with a number of cell multiplied by 2).
  fn from(g: &Grid) -> Grid {
    Grid {
      start: g.start.clone(),
      delta: g.delta,
      n_cell: g.n_cell * 2,
    }
  }
}

