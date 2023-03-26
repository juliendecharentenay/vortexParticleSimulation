use std::error::Error;
use std::collections::HashMap;

pub struct Profiler<F> 
where F: Fn() -> f64
{
    f: F,
    start: HashMap<String, f64>,
    time: HashMap<String, f64>,
}

impl<F> Profiler<F> 
where F: Fn() -> f64
{
    pub fn new(f: F) -> Result<Profiler<F>, Box<dyn Error>> {
        Ok(Profiler {
            f,
            start: HashMap::new(),
            time: HashMap::new()
        })
    }

    pub fn time(&self, name: &String) -> f64 {
        match self.time.get(name) {
            Some(t) => *t,
            None => 0f64,
        }
    }

    fn as_vector(&self) -> Vec<(String, f64)> {
        self.time
            .keys()
            .map(|k| {(k.to_string(), self.time.get(k).unwrap().clone())})
            .collect::<Vec::<(String, f64)>>()
    }

    pub fn as_alphabetic(&self) -> Vec<(String, f64)> {
        let mut v = self.as_vector();
        v.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        v
    }

    pub fn as_magnitude(&self) -> Vec<(String, f64)> {
        let mut v = self.as_vector();
        v.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        v
    }

    pub fn start(&mut self, name: String) -> ()
    {
        self.start.insert(name, (self.f)());
    }

    pub fn finish(&mut self, name: String) -> ()
    {
        let d = match self.start.remove(&name) {
            Some(t) => (self.f)() - t,
            None => {
                eprintln!("Profiling key {} undefined", name);
                0f64
            }
        };
        self.time.insert(name, d);
    }
}

