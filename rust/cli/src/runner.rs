use std::error::Error;
use std::time::SystemTime;
use std::fs::{self, File};
use std::path::Path;
use std::io::Write;

use crate::{config}; // , profiler::Profiler};
use vortex_particle_simulation::{Simulation, Profiler};

pub fn run(config: config::Config) -> Result<(), Box<dyn Error>> {
    /* Run in headless mode */
    /* Initialise the simulation */
    let mut sim = match &config.initial {
      config::Initial::Init(f)    => Some(Simulation::make_from_configuration(&fs::read(f)?)?),
      config::Initial::Restart(f) => Some(Simulation::make_from_sim(&fs::read(f)?)?),
      config::Initial::Nothing    => None,
    };

    match &mut sim {
      Some(sim) => {
          /* Treat the simulation */
          match &config.action {
              config::Action::Run     => run_simulation(&config, sim)?,
              config::Action::Nothing => (),
          };
          match &config.save {
              config::Save::Save(f) => {fs::write(f, &sim.to_content()?)?; Ok(())},
              config::Save::Nothing => Ok(()),
          }
      },
      None => Ok(()),
    }
}

fn run_simulation(config: &config::Config, simulation: &mut Simulation) -> Result<(), Box<dyn Error>> {
    output(config, simulation)?;
    let system_time = SystemTime::now();
    let time_step = config.time_step;
    let start_iteration = simulation.iteration();
    let mut profiler = Profiler::new(|| {system_time.elapsed().unwrap().as_millis() as f64})?;
    while simulation.iteration() < start_iteration + config.n_iterations {
        simulation.step(time_step, &mut profiler)?;
        output(config, simulation)?;
        println!("Iteration {}: {:.2}s [{}]", simulation.iteration(), simulation.time(), 
                 profiler.as_magnitude()
                 .iter().fold("".to_string(), |r, v| format!("{}{}{}: {}ms", r, if r.is_empty() {""} else {"; "}, v.0, v.1))
                 ); 
    }
    println!("Analysis runtime: {:}ms", system_time.elapsed().unwrap().as_millis());
    Ok(())
}

fn output(config: &config::Config, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
    match &config.output {
        config::Output::CSV(dir) => output_vortex_particles_to_csv(dir, simulation),
        config::Output::Nothing  => Ok(()),
    }
}

fn output_vortex_particles_to_csv(dir: &String, simulation: &Simulation) -> Result<(), Box<dyn Error>> {
    let path = Path::new(".").join(dir);
    if ! path.exists() { fs::create_dir_all(path.clone())?;}
    let mut file = File::create(path.join(format!("vortex_particles_{}.csv", simulation.iteration())))?;
    file.write_all(b"x coord, y coord, z coord, vorticity\n")?;
    for vorton in simulation.vortons().iter() { 
        file.write_all(
            format!("{}, {}, {}, {}\n", 
                    vorton.position().x, vorton.position().y, 
                    vorton.position().z, vorton.vorticity().norm()
                    ).as_bytes()
            )?;
    }
    Ok(())
}

