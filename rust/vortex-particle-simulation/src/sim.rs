use std::error::Error;
use crate::algebra::{Vector3, Point3};

mod uniformgrid;
pub mod vorton;

use uniformgrid::UniformGrid;
pub use vorton::Vorton;

use crate::configuration::{InitialConditions, Configuration};

pub trait Positionable {
    fn position(&self) -> &Point3<f64>;
}

pub trait Aggregatable<RHS=Self> {
    fn aggregate(&self, rhs: &RHS) -> RHS;
}

pub mod functions {
    use super::*;

    pub fn make_vortons(configuration: &Configuration) -> Result<Vec<Vorton>, Box<dyn Error>> {
        let mut n_cells = configuration.n_vortons;
        let vortons = make_vortons_from_ncells(n_cells, configuration.get_initial_conditions())?;
        if vortons.len() > 0 {
            n_cells = n_cells * configuration.n_vortons/vortons.len(); 
            make_vortons_from_ncells(n_cells, configuration.get_initial_conditions())
        } else {
            Ok(vortons)
        }
    }

    fn make_vortons_from_ncells(n_cells: usize, initial_conditions: &dyn InitialConditions) -> Result<Vec<Vorton>, Box<dyn Error>> {
        let (min, max) = initial_conditions.domain();
        let uniform_grid = UniformGrid::from_n_cells_target(min, max, n_cells)?;
        Ok(uniform_grid.cell_index_iter()
            .map(|index| {
                let p = uniform_grid.cell_position(index);
                let v = uniform_grid.cell_volume(index);
                let vorticity = initial_conditions.vorticity(&p);
                Vorton::new(p, vorticity, v)
            })
            .filter(|vorton| vorton.vorticity().norm() > 1e-5)
            .collect::<Vec<Vorton>>())
    }

    /*
     * Calculate a velocity at a given point
     */
    pub fn velocity_at<'a, I>(position: &Point3<f64>, free_stream_velocity: Vector3<f64>, vortons: I) -> Vector3<f64> 
        where I: Iterator<Item = &'a Vorton>
    {
        vortons.map(|vorton| vorton.velocity_contribution(position) ) 
            .fold(free_stream_velocity, |res, vel| res + vel )
    }

        /*
    #[cfg(test)]
    mod make_velocity {
        use super::*;
        
         * Test tree grid against point
        #[test]
        fn test() -> Result<(), Box<dyn Error>> {
            let vortons = vec![
                // Vorton::new(
                    // Position::new(-1.0, -1.0, -1.0),
                    // Vector::new(0.0, 3.0, 0.0),
                    // 1e-5,
                    // 1.0)?,
                Vorton::new(
                    Position::new( 0.0,  0.0,  0.0),
                    Vector::new(0.0, 3.0, 0.0),
                    1e-5,
                    2.0, 0.0)?,
                Vorton::new(
                    Position::new( 0.0,  0.0,  0.0),
                    Vector::new(3.0, 3.0, 0.0),
                    1e-5,
                    1.0, 0.0)?,
            ];
            let grid = UniformGrid::from_n_points(Position::new(5.0, 5.0, 5.0), Position::new(5.1, 5.1, 5.1), 2, 2, 2)?;
            let v_points = make_velocity_at_points(&grid, &vortons)?;
            let v_tree   = make_velocity_using_tree_grid(&grid, &vortons)?;
            println!("point/tree velocity {}, {}, {} / {}, {}, {}", 
                     v_points[0].x(), v_points[0].y(), v_points[0].z(),
                     v_tree[0].x(), v_tree[0].y(), v_tree[0].z());
            assert!((v_points[0].x() - v_tree[0].x()).abs() < 1e-4);
            assert!((v_points[0].y() - v_tree[0].y()).abs() < 1e-4);
            assert!((v_points[0].z() - v_tree[0].z()).abs() < 1e-4);
            Ok(())
        }
    }
         */
}
