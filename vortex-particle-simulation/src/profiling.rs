use std::error::Error;
use std::time::SystemTime;
use std::collections::HashMap;

pub struct Profiling {
    current: HashMap<&'static str, SystemTime>,
    cumulative: HashMap<&'static str, u128>
}

impl Default for Profiling {
    fn default() -> Self {
        Profiling::new().unwrap()
    }
}

impl Profiling {
    pub fn new() -> Result<Profiling, Box<dyn Error>> {
        Ok(Profiling {
            current: HashMap::new(),
            cumulative: HashMap::new(),
        })
    }

    pub fn start(&mut self, key: &'static str) {
        self.current.insert(key, SystemTime::now());
    }

    pub fn finish(&mut self, key: &'static str) -> Result<(), Box<dyn Error>> {
        let mut d = match self.cumulative.remove(key) {
                Some(d) => d,
                None => 0u128,
            };
        d += match self.current.remove(key) {
                Some(t) => t.elapsed()?.as_millis(),
                None => {
                    eprintln!("Profiling: key {} starting point was not defined", key);
                    0u128
                }
            };
        self.cumulative.insert(key, d);
        Ok(())
    }

    pub fn get(&self, key: &'static str) -> Option<&u128> {
        self.cumulative.get(key)
    }

    pub fn print(&self) {
        println!("Profiling information:");
        for (key, val) in self.cumulative.iter() {
            println!("  {}: {}ms", key, val);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{thread, time};
    use super::*;

    #[test]
    fn test_profiling() -> Result<(), Box<dyn Error>> {
        let mut profiling = Profiling::new()?;
        profiling.start("test profiling");
        thread::sleep(time::Duration::from_millis(500));
        profiling.finish("test profiling")?;
        assert!(*profiling.get("test profiling").unwrap() > 500);
        profiling.print();
        Ok(())
    }
}
