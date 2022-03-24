use std::error::Error;
use clap::{Arg, Command};


pub struct Config {
    pub action: Action,
    pub output: Output,
    pub initial:Initial,
    pub save: Save,
    pub n_iterations: usize,
    pub time_step: f64,
}

pub enum Initial {
    Init(String),
    Restart(String),
    Nothing,
}

pub enum Save {
    Save(String),
    Nothing,
}

#[derive(Debug)]
pub enum Action {
    Run,
    Nothing,
}

pub enum Output {
    CSV(String),
    Nothing,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn Error>> {
        let matches = Command::new("VortexParticleSimulation")
            .version("0.0")
            .author("Julien de Charentenay <julien@charentenay.me>")
            .about("Vortex particle fluid simulation")
            .arg(Arg::new("init")
                 .long("init")
                 .help("Initialise the simulation from a json case file")
                 .value_name("PATH/FILE")
                 .takes_value(true))
            .arg(Arg::new("restart")
                 .long("restart")
                 .help("Restart an existing simulation from a simulation file")
                 .value_name("PATH/FILE")
                 .takes_value(true))
            .arg(Arg::new("save")
                 .long("save")
                 .help("Save simulation results to a simulation file")
                 .value_name("PATH/FILE")
                 .takes_value(true))
            .arg(Arg::new("run")
                 .short('r')
                 .long("run")
                 .help("Run simulation")
                 .takes_value(false))
            .arg(Arg::new("csv")
                 .long("csv")
                 .help("Output vortex particle positions and vorticity to CSV file format")
                 .value_name("DIRECTORY")
                 .takes_value(true))
            .arg(Arg::new("iteration")
                 .long("iteration")
                 .help("Nominate the number of iteration to run")
                 .takes_value(true)
                 .value_name("INTEGER")
                 .default_value("100"))
            .arg(Arg::new("time_step")
                 .long("timestep")
                 .help("Nominate the time step in seconds")
                 .takes_value(true)
                 .value_name("DURATION")
                 .default_value("0.03"))
            .get_matches();
        
        let mut action = Action::Nothing;
        if matches.is_present("run")               { action = Action::Run; }

        let mut output = Output::Nothing;
        if let Some(d) = matches.value_of("csv") { output = Output::CSV(d.to_string()); }

        let mut initial = Initial::Nothing;
        if let Some(d) = matches.value_of("init") { initial = Initial::Init(d.to_string()); }
        if let Some(d) = matches.value_of("restart") { initial = Initial::Restart(d.to_string()); }

        let mut save = Save::Nothing;
        if let Some(d) = matches.value_of("save") { save = Save::Save(d.to_string()); }

        let mut n_iterations = 100;
        if let Some(v) = matches.value_of("iteration") { n_iterations = v.parse::<usize>().unwrap(); }

        let mut time_step = 0.03;
        if let Some(v) = matches.value_of("time_step") { time_step = v.parse::<f64>().unwrap(); }

        Ok(Config { action, output, initial, save, n_iterations, time_step, })
    }
}


