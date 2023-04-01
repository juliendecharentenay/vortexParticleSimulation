use std::time::SystemTime;
use std::fs::{self, File};
use std::path::Path;

use crate::{config};
use vortex_particle_simulation::{Simulation, Profiler, VortonToVelocityAlgorithm, Geometry};

pub fn run(config: config::Config) -> Result<(), Box<dyn std::error::Error>> {
  use std::convert::TryFrom;

    /* Run in headless mode */
    /* Initialise the simulation */
    let mut sim = match &config.initial {
      config::Initial::Init(f)    => Some(Simulation::try_from(&serde_json::from_reader(File::open(f)?)?)?),
      config::Initial::Restart(f) => Some(serde_json::from_reader(File::open(f)?)?),
      config::Initial::Nothing    => None,
    };
    
    match &mut sim {
      Some(sim) => {
          /* Hard-coded */
          sim.push_geometry(Geometry::cube())?;

          match &config.vorton_to_velocity_algorithm {
            config::VortonToVelocityAlgorithm::Simple => {
              sim.use_vorton_to_velocity(VortonToVelocityAlgorithm::Simple);
            },
            config::VortonToVelocityAlgorithm::Tree => {
              sim.use_vorton_to_velocity(VortonToVelocityAlgorithm::Tree(6));
            },
          };

          /* Run the simulation */
          match &config.action {
              config::Action::Run     => run_simulation(&config, sim)?,
              config::Action::Nothing => (),
          };

          /* Save the simulation */
          match &config.save {
              config::Save::Save(f) => Ok(serde_json::to_writer_pretty(File::create(f)?, &sim)?),
              config::Save::Nothing => Ok(()),
          }
      },
      None => Ok(()),
    }
}

fn run_simulation(config: &config::Config, simulation: &mut Simulation) -> Result<(), Box<dyn std::error::Error>> {
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

fn output(config: &config::Config, simulation: &Simulation) -> Result<(), Box<dyn std::error::Error>> {
    match &config.output {
        config::Output::CSV(dir) => {
          let file = open_file(dir, format!("vortex_particles_{}.csv", simulation.iteration()))?;
          vortex_particle_simulation::VortonCollection::from(simulation)
          .to_writer_csv(file, "vorticity", |v| Ok(v.vorticity().norm()))?;
          Ok(())
        },

        config::Output::Velocity(dir) => {
          let vorton_to_velocity = simulation.get_vorton_to_velocity()?;
          let file = open_file(dir, format!("velocity_{}.csv", simulation.iteration()))?;
          vortex_particle_simulation::GridBuilder::default()
          .build()?
          .to_writer_csv(file, |p| {Ok(vorton_to_velocity.velocity_at(p)?.x)})?;
          Ok(())
        },
        config::Output::Nothing  => Ok(()),
    }
}

fn open_file(dir: &String, fname: String) -> Result<std::fs::File, Box<dyn std::error::Error>> {
  let path = Path::new(".").join(dir);
  if ! path.exists() { fs::create_dir_all(path.clone())?;}
  Ok(File::create(path.join(fname))?)
}

