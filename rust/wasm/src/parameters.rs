use std::{
    default::Default,
    error::Error,
};

use serde::{Serialize, Deserialize};

use vortex_particle_simulation::{Configuration};

#[derive(Serialize, Deserialize)]
pub struct Parameters {
    configuration: Configuration,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters { 
            configuration: Configuration::new_vortex_ring()
        }
    }
}

impl Parameters {
    pub fn from_json(data: &str) -> Result<Parameters, Box<dyn Error>> {
        Ok(serde_json::from_str(data)?)
    }

    pub fn to_json(&self) -> Result<String, Box<dyn Error>> {
        Ok(serde_json::to_string(&self)?)
    }

    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}

