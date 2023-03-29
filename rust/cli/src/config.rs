use clap::{Arg, Command};

pub struct Config {
    pub action: Action,
    pub output: Output,
    pub initial:Initial,
    pub save: Save,
    pub n_iterations: usize,
    pub time_step: f64,
    pub vorton_to_velocity_algorithm: VortonToVelocityAlgorithm,
}

#[derive(Debug)]
pub enum VortonToVelocityAlgorithm {
  Simple,
  Tree,
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

pub enum Action {
    Run,
    Nothing,
}

pub enum Output {
    CSV(String),
    Velocity(String),
    Nothing,
}

impl Config {
    pub fn new() -> Result<Config, Box<dyn std::error::Error>> {
        let matches = Command::new("VortexParticleSimulation")
            .version("0.0")
            .author("Julien de Charentenay <julien@charentenay.me>")
            .about("Vortex particle fluid simulation")
            .arg(Arg::new("init")
                 .long("init")
                 .help("Initialise the simulation from a json case file")
                 .value_name("PATH/FILE")
                 .action(clap::ArgAction::Set))
            .arg(Arg::new("restart")
                 .long("restart")
                 .help("Restart an existing simulation from a simulation file")
                 .value_name("PATH/FILE")
                 .action(clap::ArgAction::Set))
            .arg(Arg::new("save")
                 .long("save")
                 .help("Save simulation results to a simulation file")
                 .value_name("PATH/FILE")
                 .action(clap::ArgAction::Set))
            .arg(Arg::new("run")
                 .short('r')
                 .long("run")
                 .help("Run simulation")
                 .action(clap::ArgAction::Count))
            .arg(Arg::new("csv")
                 .long("csv")
                 .help("Output vortex particle positions and vorticity to CSV file format")
                 .value_name("DIRECTORY")
                 .action(clap::ArgAction::Set))
            .arg(Arg::new("ovel")
                 .long("out_velocity")
                 .help("Output velocity on a regular grid using the XYZ format")
                 .value_name("DIRECTORY")
                 .action(clap::ArgAction::Set))
            .arg(Arg::new("iteration")
                 .long("iteration")
                 .help("Nominate the number of iteration to run")
                 .action(clap::ArgAction::Set)
                 .value_name("INTEGER")
                 .default_value("100"))
            .arg(Arg::new("time_step")
                 .long("timestep")
                 .help("Nominate the time step in seconds")
                 .action(clap::ArgAction::Set)
                 .value_name("DURATION")
                 .default_value("0.03"))
            .arg(Arg::new("alg_simple")
                 .long("alg_simple")
                 .help("Use the simple algorithm for calculating velocity from vortons")
                 .action(clap::ArgAction::Count))
            .arg(Arg::new("alg_tree")
                 .long("alg_tree")
                 .help("Use the tree algorithm for calculating velocity from vortons")
                 .action(clap::ArgAction::Count))
            .get_matches();
        
        let mut action = Action::Nothing;
        if matches.get_count("run") > 0 { action = Action::Run; }

        let mut output = Output::Nothing;
        if let Some(d) = matches.get_one::<String>("csv")  { output = Output::CSV(d.clone()); }
        if let Some(d) = matches.get_one::<String>("ovel") { output = Output::Velocity(d.clone()); }

        let mut initial = Initial::Nothing;
        if let Some(d) = matches.get_one::<String>("init") { initial = Initial::Init(d.clone()); }
        if let Some(d) = matches.get_one::<String>("restart") { initial = Initial::Restart(d.clone()); }

        let mut save = Save::Nothing;
        if let Some(d) = matches.get_one::<String>("save") { save = Save::Save(d.clone()); }

        let mut n_iterations = 100;
        if let Some(v) = matches.get_one::<String>("iteration") { n_iterations = v.parse::<usize>().unwrap(); }

        let mut time_step = 0.03;
        if let Some(v) = matches.get_one::<String>("time_step") { time_step = v.parse::<f64>().unwrap(); }

        let mut vorton_to_velocity_algorithm = VortonToVelocityAlgorithm::Simple;
        if matches.get_count("alg_simple") > 0 { vorton_to_velocity_algorithm = VortonToVelocityAlgorithm::Simple; }
        if matches.get_count("alg_tree") > 0 { vorton_to_velocity_algorithm = VortonToVelocityAlgorithm::Tree; }

        Ok(Config { action, output, initial, save, n_iterations, time_step, vorton_to_velocity_algorithm, })
    }
}

