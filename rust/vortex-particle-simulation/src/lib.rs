use std::error::Error;
use std::ptr;

use serde::{Serialize, Deserialize};
use serde_json;
use nalgebra::{Vector3};

mod sim;
mod configuration;
mod profiler;

pub use sim::{Vorton};
pub use configuration::{InitialConditions, Configuration};
pub use profiler::Profiler;

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    configuration: Configuration,
    time: f64,
    iteration: usize,
    vortons: Vec<sim::Vorton>,
    free_stream_velocity: Vector3<f64>,
}

impl Simulation {
    /*
     * Accessor functions
     */
    pub fn iteration(&self) -> usize        { self.iteration }
    pub fn time(&self) -> f64               { self.time }
    pub fn vortons(&self) -> &Vec<Vorton>   { &self.vortons }

    /**
     * Create a new simulation from a string slice
     */
    pub fn make_from_configuration(content: &[u8]) -> Result<Simulation, Box<dyn Error>> {
        let configuration = Configuration::make_from(content)?;
        let vortons = sim::functions::make_vortons(&configuration)?;
        let free_stream_velocity = configuration.get_initial_conditions().free_stream_velocity();
        println!("Domain min (X/Y/Z): {}, {}, {}", configuration.domain.min.x, configuration.domain.min.y, configuration.domain.min.z);
        println!("Domain max (X/Y/Z): {}, {}, {}", configuration.domain.max.x, configuration.domain.max.y, configuration.domain.max.z);
        println!("Number of vortons: {}", vortons.len());
        Ok(Simulation{
            configuration, 
            time: 0.0, iteration: 0, vortons,
            free_stream_velocity})
    }

    /**
     * Create a new simulation from a saved simulation
     */
    pub fn make_from_sim(content: &[u8]) -> Result<Simulation, Box<dyn Error>> {
        Ok(serde_json::from_slice(content)?)
    }

    /**
     * Save a simulation to a file
     */
    pub fn to_content(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        Ok(serde_json::to_vec_pretty(self)?)
    }

    /**
     * Perform an interation with the provided time-step
     */
    pub fn step<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn Error>> 
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

    /*
     * Step helper functions
     */
    fn advect_vortons<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn Error>> 
        where F: Fn() -> f64
    {
        profiler.start("advect_vortons".to_string());
        self.vortons = self.vortons.iter()
            .filter(|vorton| vorton.vorticity().norm() > 1e-5)
            .map(|vorton|  {
                let v = sim::functions::velocity_at(vorton.position(), 
                                                    self.free_stream_velocity.clone(),
                                                    self.vortons.iter().filter(|&vo| ! ptr::eq(vo, vorton)));
                vorton.advect(&v, time_step)
            })
            .collect::<Vec<Vorton>>();
        profiler.finish("advect_vortons".to_string());
        Ok(())
    }

    fn step_vorticity<F>(&mut self, time_step: f64, profiler: &mut Profiler<F>) -> Result<(), Box<dyn Error>> 
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
                         + vorton.vorticity().scale(-0.1*self.configuration.viscosity); 
                     vorton.step(&rhs, time_step)
                 })
            .collect::<Vec<Vorton>>();
        profiler.finish("step_vorticity".to_string());
        Ok(())
    }

    /*
    fn calculate_vorton_diffusion(&self) -> Result<Vec<Vector>, Box<dyn Error>> {
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
