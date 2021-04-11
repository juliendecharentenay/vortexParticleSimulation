use std::error::Error;
use std::ptr;
use std::fs::File;
use std::io::{BufReader, BufWriter};

use serde::{Serialize, Deserialize};
use serde_json;
use nalgebra::{Vector3};

mod profiling;
mod sim;
mod configuration;

pub use sim::{Vorton};
pub use configuration::{Configuration};
use profiling::Profiling;

#[derive(Serialize, Deserialize)]
pub struct Simulation {
    configuration: Configuration,
    #[serde(skip)]
    profiling: Profiling,
    time: f64,
    iteration: usize,
    vortons: Vec<sim::Vorton>,
}

impl Simulation {
    /*
     * Accessor functions
     */
    pub fn iteration(&self) -> usize        { self.iteration }
    pub fn time(&self) -> f64               { self.time }
    pub fn vortons(&self) -> &Vec<Vorton>   { &self.vortons }

    /**
     * Create a new simulation from a json file
     */
    pub fn make_from_configuration(filename: &String) -> Result<Simulation, Box<dyn Error>> {
        let configuration = Configuration::make_from_json_file(filename)?;
        let vortons = sim::functions::make_vortons(&configuration)?;
        println!("Domain min (X/Y/Z): {}, {}, {}", configuration.domain.min.x, configuration.domain.min.y, configuration.domain.min.z);
        println!("Domain max (X/Y/Z): {}, {}, {}", configuration.domain.max.x, configuration.domain.max.y, configuration.domain.max.z);
        println!("Number of vortons: {}", vortons.len());
        Ok(Simulation{
            configuration, profiling: Profiling::new()?,
            time: 0.0, iteration: 0, vortons })
    }

    /**
     * Create a new simulation from a saved simulation
     */
    pub fn make_from_sim(filename: &String) -> Result<Simulation, Box<dyn Error>> {
        let reader = BufReader::new(File::open(filename)?);
        Ok(serde_json::from_reader(reader)?)
    }

    /**
     * Save a simulation to a file
     */
    pub fn to_file(&self, filename: &String) -> Result<(), Box<dyn Error>> {
        let writer = BufWriter::new(File::create(filename)?);
        Ok(serde_json::to_writer_pretty(writer, self)?)
    }

    /**
     * Perform an interation with the provided time-step
     */
    pub fn step(&mut self, time_step: f64) -> Result<(), Box<dyn Error>> 
    {
        self.iteration += 1; self.time += time_step;
        self.step_vorticity(time_step)?;
        self.advect_vortons(time_step)?;
        Ok(())
    }

    pub fn print_profiling(&self) {
        self.profiling.print();
    }

    /*
     * Step helper functions
     */
    fn advect_vortons(&mut self, time_step: f64) -> Result<(), Box<dyn Error>> {
        self.profiling.start("Simulation:advect vorton");
        self.vortons = self.vortons.iter()
            .filter(|vorton| vorton.vorticity().norm() > 1e-5)
            .map(|vorton|  {
                let v = sim::functions::velocity_at(vorton.position(), 
                                                    self.vortons.iter().filter(|&vo| ! ptr::eq(vo, vorton)));
                vorton.advect(&v, time_step)
            })
            .collect::<Vec<Vorton>>();
        self.profiling.finish("Simulation:advect vorton")?;
        Ok(())
    }

    fn step_vorticity(&mut self, time_step: f64) -> Result<(), Box<dyn Error>> {
        self.profiling.start("Simulation:step vorticity");
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
        self.profiling.finish("Simulation:step vorticity")?;
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
