use crate::{sim, Point3, Profiler, Vector3, Vorton,
  VortonToVelocity, VortonToVelocitySimpleBuilder, VortonToVelocityTreeBuilder, 
};

/// Vortex simulation root object
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Simulation {
    time: f64,
    iteration: usize,
    viscosity: f64,
    free_stream_velocity: Vector3<f64>,
    vortons: Vec<sim::Vorton>,
    #[serde(default="default_vorton_to_velocity")]
    vorton_to_velocity_algorithm: VortonToVelocityAlgorithm,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum VortonToVelocityAlgorithm {
  Simple,
  Tree(usize),
}


fn default_vorton_to_velocity() -> VortonToVelocityAlgorithm {
  VortonToVelocityAlgorithm::Simple
}

impl std::convert::TryFrom<&crate::configuration::Configuration> for Simulation {
  type Error = Box<dyn std::error::Error>;
  /// Make a new simulation from a configuration
  fn try_from(c: &crate::configuration::Configuration) -> Result<Self, Self::Error> {
    Ok(Simulation {
      time: 0.0,
      iteration: 0,
      viscosity: c.viscosity,
      free_stream_velocity: c.get_initial_conditions().free_stream_velocity(),
      vortons: crate::sim::functions::make_vortons(&c)?,
      vorton_to_velocity_algorithm: VortonToVelocityAlgorithm::Simple,
    })
  }
}

impl Simulation {
    /*
     * Accessor functions
     */
    pub fn iteration(&self) -> usize        { self.iteration }
    pub fn time(&self) -> f64               { self.time }
    pub fn vortons(&self) -> &Vec<Vorton>   { &self.vortons }

    pub fn use_vorton_to_velocity(&mut self, vorton_to_velocity_algorithm: VortonToVelocityAlgorithm) {
      self.vorton_to_velocity_algorithm = vorton_to_velocity_algorithm;
    }
    pub fn get_vorton_to_velocity(&self) -> Result<Box<dyn VortonToVelocity + '_>, Box<dyn std::error::Error>> {
      match &self.vorton_to_velocity_algorithm {
        VortonToVelocityAlgorithm::Simple 
        => Ok(Box::new(VortonToVelocitySimpleBuilder::default().vortons(&self.vortons).build()?)),
        VortonToVelocityAlgorithm::Tree(n_grids) 
        => Ok(Box::new(VortonToVelocityTreeBuilder::default().vortons(&self.vortons).n_grids(*n_grids).build()?.initialize()?)),
      }
    }

    /*
     * Create a new simulation from a string slice
    pub fn make_from_configuration(configuration: Configuration) -> Result<Simulation, Box<dyn std::error::Error>> {
        let vortons = sim::functions::make_vortons(&configuration)?;
        let free_stream_velocity = configuration.get_initial_conditions().free_stream_velocity();
        println!("Domain min (X/Y/Z): {}, {}, {}", configuration.domain.min.x, configuration.domain.min.y, configuration.domain.min.z);
        println!("Domain max (X/Y/Z): {}, {}, {}", configuration.domain.max.x, configuration.domain.max.y, configuration.domain.max.z);
        println!("Number of vortons: {}", vortons.len());
        Ok(Simulation{
            configuration,
            time: 0.0, iteration: 0, vortons,
            free_stream_velocity,
         })
    }
     */

    /*
     * Perform an interation with the provided time-step
     */
    pub fn step<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn std::error::Error>> 
        where F: Fn() -> f64
    {
        self.iteration += 1; self.time += time_step;
        self.step_vorticity(time_step, profiler)?; // Simulation::make_timer(profiler, "step_vorticity"))?;
        self.advect_vortons(time_step, profiler)?; // Simulation::make_timer(profiler, "advect_vortons"))?;
        Ok(())
    }

    /*
    pub fn print_profiling(&self) {
        self.profiling.print();
    }
    */

    /// Report the velocity at a given point
    pub fn velocity_at(&self, position: &Point3<f64>) -> Result<Vector3<f64>, Box<dyn std::error::Error>> {
      Ok(
      sim::functions::velocity_at(position, 
                                  self.free_stream_velocity.clone(),
                                  self.vortons.iter())
      )
/*
      Ok(
        self.free_stream_velocity
        + self.vorton_to_velocity.velocity_at(position)?
      )
*/
    }

    /*
     * Step helper functions
     */
    fn advect_vortons<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn std::error::Error>> 
        where F: Fn() -> f64
    {
        profiler.start("advect_vortons".to_string());
        self.vortons = {
          let vorton_to_velocity = self.get_vorton_to_velocity()?;
          self.vortons.iter()
            .filter(|vorton| vorton.vorticity().norm() > 1e-5)
            .map(|vorton|  {
/*
                let v = sim::functions::velocity_at(vorton.position(), 
                                                    self.free_stream_velocity.clone(),
                                                    self.vortons.iter().filter(|&vo| ! std::ptr::eq(vo, vorton)));
*/
                let v = &self.free_stream_velocity + vorton_to_velocity.velocity_at(vorton.position())?;
                Ok(vorton.advect(&v, time_step))
            })
            .collect::<Result<Vec<Vorton>, Box<dyn std::error::Error>>>()?
        };
        profiler.finish("advect_vortons".to_string());
        Ok(())
    }

    fn step_vorticity<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn std::error::Error>> 
        where F: Fn() -> f64
    {
        profiler.start("step_vorticity".to_string());
        self.vortons = self.vortons.iter()
            .map(|vorton| 
                 {
                     let rhs = Vector3::<f64>::new(0.0, 0.0, 0.0)
                         // .add(&self.grid.point_interpolate(&velocity_gradient, vorton.position())
                              // .mult(vorton.vorticity())
                              // ) 
                         + vorton.vorticity().scale(-0.1*self.viscosity); 
                     vorton.step(&rhs, time_step)
                 })
            .collect::<Vec<Vorton>>();
        profiler.finish("step_vorticity".to_string());
        Ok(())
    }

    /*
    fn calculate_vorton_diffusion(&self) -> Result<Vec<Vector>, Box<dyn std::error::Error>> {
        let average_vorticity = self.vortons.iter()
            .fold(Vector::zero(), |sum, vorton| sum.add(&vorton.vorticity()) )
            .multiply(&(1.0 / self.vortons.len() as f64));
        Ok(self.vortons.iter()
            .map(|vorton| 
                 average_vorticity
                 .add(&vorton.vorticity().multiply(&-1.0)) 
                 .multiply(&self.viscosity)
                 )
            .collect::<Vec<Vector>>())
    }
    */

}


/*
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
*/
