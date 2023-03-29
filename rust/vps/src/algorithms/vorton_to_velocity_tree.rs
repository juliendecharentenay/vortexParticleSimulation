use crate::{VortonToVelocity, Point3, SuperVorton, Vorton, Vector3};

mod grid; pub use grid::Grid;

/// Algorithm to calculate veloctiy from a field of vorton
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct VortonToVelocityTree<'a> {
  vortons: &'a Vec<Vorton>,
  n_grids: usize,
  #[builder(setter(skip))]
  grids: Vec<Grid>,
  #[builder(setter(skip))]
  infos: Vec<std::collections::HashMap<(usize, usize, usize), Info<'a>>>,
}

#[derive(Debug, Clone)]
enum Info<'a> {
  SuperVorton ( SuperVorton ),
  Vortons ( Vec<&'a Vorton> ),
}

impl From<&Vec<&Vorton>> for Info<'static> {
  fn from(vortons: &Vec<&Vorton>) -> Info<'static> {
    Info::SuperVorton ( vortons.iter().fold(SuperVorton::default(), |a, v| a + *v) )
  }
}

/*
impl<'a> Info<'a> {
  pub fn new_super_vorton() -> Info<'a> {
    Info::SuperVorton ( SuperVorton::default() )
  }

  pub fn with_info(&'a mut self, v: Info) -> Re() {
    println!("push_info {:?} + {:?}", &self, v);
    if let (Info::SuperVorton {vorton, max_vorticity, n_vortons},
            Info::SuperVorton {vorton: other_vorton, max_vorticity: other_max_vorticity, n_vortons: other_n_vortons}) = (self, v) {
      *vorton = &*vorton + &other_vorton;
      *max_vorticity = max_vorticity.max(other_max_vorticity);
      *n_vortons += other_n_vortons;
    }
    println!("= {:?}", &self);
  }

  pub fn push(&'a mut self, v: &'a Vorton) -> () {
    match self {
      Info::SuperVorton ( super_vorton ) => {
        *super_vorton = &*super_vorton + v;
      },
      Info::Vortons(arr) => { arr.push(v); },
    };
  }
}
*/

impl<'a> VortonToVelocity for VortonToVelocityTree<'a> {
  fn velocity_at(&self, position: &Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>> {
    if self.grids.len() != self.infos.len() { return Err("Not the same length...".into()); }
    let v = self.traverse(position, 0, (0, 0, 0))?;
    Ok(v)
  }
}

impl<'a> VortonToVelocityTree<'a> {
  pub fn initialize(mut self) -> Result<VortonToVelocityTree<'a>, Box<dyn std::error::Error>> {
    self.make_grids()?; self.make_infos()?; Ok(self)
  }

  fn traverse(&self, 
              position: &Point3<f64>, 
              level: usize, (i,j,k): (usize, usize, usize)
              ) -> Result<Vector3<f64>, Box<dyn std::error::Error>> {
    let grid = &self.grids[level];
    let info = &self.infos[level];
    // println!("Traverse level {level} cell {i},{j},{k}");
    match info.get(&(i,j,k)) {
      Some(Info::SuperVorton ( super_vorton )) if super_vorton.n_vortons() == 1 => {
        Ok(super_vorton.vorton().velocity_contribution(position))
      },

      Some(Info::SuperVorton ( super_vorton )) => {
        let calculate = true;
        let calculate = calculate
             && (position - grid.cell_center(i, j, k)).norm() > 0.7*1.7*grid.cell_length() // #1 position is within the cell
             && (! super_vorton.vorton().is_inside(position));                             // #2 position is not within the vorton direct influence

        let v = position - super_vorton.vorton().position();
        let vc = Vorton::new(super_vorton.vorton().position().clone(),                      // #3 Maximum possible contribution to velocity
                            v.orthogonal().normalize().scale(super_vorton.max_vorticity()), //
                            super_vorton.vorton().volume())                                 //
                 .velocity_contribution(position).norm();                                   //
        let calculate = calculate && (vc < 0.05);                                           // less than 0.05 m/s

        if calculate {
          Ok(super_vorton.vorton().velocity_contribution(position))

        } else {
          Ok(
            self.traverse(position, level+1, (2*i,   2*j,   2*k))?
          + self.traverse(position, level+1, (2*i+1, 2*j+0, 2*k+0))?
          + self.traverse(position, level+1, (2*i+0, 2*j+1, 2*k+0))?
          + self.traverse(position, level+1, (2*i+1, 2*j+1, 2*k+0))?
          + self.traverse(position, level+1, (2*i+0, 2*j+0, 2*k+1))?
          + self.traverse(position, level+1, (2*i+1, 2*j+0, 2*k+1))?
          + self.traverse(position, level+1, (2*i+0, 2*j+1, 2*k+1))?
          + self.traverse(position, level+1, (2*i+1, 2*j+1, 2*k+1))?
          )
        }
      },

      Some(Info::Vortons(vortons)) => {
        Ok(vortons.iter().map(|v| v.velocity_contribution(position)).fold(Vector3::default(), |r, v| r+v))
      },

      None => Ok(Vector3::default()),
    }
  }

  fn make_infos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    if self.infos.len() == 0 {
      self.make_grids()?;

      let mut infos = Vec::new();
      let grid = &self.grids[self.n_grids - 1];
      let info = self.vortons.iter()
          .map(|v| (grid.cell_ijk(v.position()), v) )
          .filter(|(ijk, _v)| ijk.is_some())
          .map(|(ijk, v)| (ijk.unwrap(), v))
          .fold(std::collections::HashMap::<(usize, usize, usize), Vec<&Vorton>>::new(),
              |mut map, (ijk, v)| {
                map.entry(ijk).or_insert(Vec::new()).push(v);
                map
              })
          .into_iter()
          .map(|(key, val)| (key, Info::Vortons(val)))
          .collect::<std::collections::HashMap::<(usize, usize, usize), Info>>();
      infos.push(info);

      while infos.len() < self.n_grids {
        let info = infos.first().ok_or("Unable to retrieve info")?
          .iter()
          .map(|(key, info)| {
            let (i, j, k) = key;
            ((i.div_euclid(2), j.div_euclid(2), k.div_euclid(2)),
             if let Info::Vortons(vortons) = info {
               vortons.into()
             } else {
               info.clone()
             })
          })
          .fold(std::collections::HashMap::<(usize, usize, usize), Info>::new(),
                |mut map, (ijk, info)| {
                  if let Some(Info::SuperVorton ( super_vorton )) = map.remove(&ijk) {
                    if let Info::SuperVorton ( other_super_vorton ) = info {
                      map.insert(ijk, Info::SuperVorton ( super_vorton + &other_super_vorton ));
                    } else {
                      eprintln!("Something wrong...");
                    }
                  } else {
                    map.insert(ijk, info);
                  }
                  // println!("after: {:?}", map.get(&ijk));
                  map
                });
        infos.insert(0, info);
      }
      self.infos = infos;
    }
    Ok(())
  }

  fn make_grids(&mut self) -> Result<(), Box<dyn std::error::Error>> {
    use std::convert::TryInto;
    if self.grids.len() == 0 {
      self.grids.push(self.vortons.try_into()?);
      while self.grids.len() < self.n_grids { self.grids.push(self.grids.last().ok_or("Grid unavailable")?.into()); }
    }
    Ok(())
  }
}

