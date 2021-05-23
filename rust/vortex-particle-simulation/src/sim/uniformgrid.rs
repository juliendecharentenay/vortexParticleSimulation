use std::error::Error;

use nalgebra::{Point3, Vector3};

pub struct CellIndexIterator<'a> {
    uniform_grid: &'a UniformGrid,
    index: usize,
}

impl CellIndexIterator<'_> {
    fn new<'a>(uniform_grid: &'a UniformGrid) -> CellIndexIterator<'a> {
        CellIndexIterator { uniform_grid, index: 0 }
    }
}

impl Iterator for CellIndexIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.uniform_grid.n_cells() {
            let r = self.index;
            self.index += 1;
            Some(r)
        } else {
            None
        }
    }
}

pub struct UniformGrid {
    min: Point3<f64>,
    // max: Point3<f64>,
    n_points: (usize, usize, usize),
    delta: (f64, f64, f64),
}

impl UniformGrid {
    fn new(min: Point3<f64>, max: Point3<f64>, n_points: (usize, usize, usize)) -> Result<UniformGrid, Box<dyn Error>> {
        let v = max - min;
        let delta = (
                v.x / (n_points.0 - 1) as f64,
                v.y / (n_points.1 - 1) as f64,
                v.z / (n_points.2 - 1) as f64
                );
        Ok(UniformGrid { min, n_points, delta })
    }
    
    /*
    pub fn from_n_points(min: Point3<f64>, max: Point3<f64>, n_x: usize, n_y: usize, n_z: usize) -> Result<UniformGrid, Box<dyn Error>> {
        UniformGrid::new(min, max, (n_x, n_y, n_z))
    }
    */

    pub fn from_delta(min: Point3<f64>, max: Point3<f64>, delta: f64) -> Result<UniformGrid, Box<dyn Error>> {
        let v = max - min;
        let n_points = ((v.x / delta) as usize + 1,
                        (v.y / delta) as usize + 1,
                        (v.z / delta) as usize + 1);
        UniformGrid::new(min, max, n_points)
    }

    pub fn from_n_cells_target(min: Point3<f64>, max: Point3<f64>, n_cells_target: usize) -> Result<UniformGrid, Box<dyn Error>> {
        let v = max - min;
        let cell_volume = (v.x * v.y * v.z) / n_cells_target as f64;
        let d = cell_volume.cbrt();
        UniformGrid::from_delta(min, max, d)
    }

    // pub fn min(&self)     -> &Point3<f64> { &self.min }
    // pub fn max(&self)     -> &Point3<f64> { &self.max }

    pub fn n_cells(&self) -> usize { (self.n_points.0 - 1) * (self.n_points.1 - 1) * (self.n_points.2 - 1) }
    // pub fn n_points(&self)-> usize { self.n_points.0 * self.n_points.1 * self.n_points.2 }
    // pub fn n_x(&self)     -> usize { self.n_points.0 }
    // pub fn n_y(&self)     -> usize { self.n_points.1 }
    // pub fn n_z(&self)     -> usize { self.n_points.2 }
    // pub fn delta_avg(&self) -> f64 { (self.delta.0 + self.delta.1 + self.delta.2)/3.0 }
    // pub fn delta_x(&self)   -> f64 { self.delta.0 }
    // pub fn delta_y(&self)   -> f64 { self.delta.1 }
    // pub fn delta_z(&self)   -> f64 { self.delta.2 }
    pub fn cell_volume(&self, _index: usize) -> f64 { self.delta.0 * self.delta.1 * self.delta.2 }

    pub fn cell_index_iter(&self) -> CellIndexIterator {
        CellIndexIterator::new(self)
    }

    pub fn cell_position(&self, index: usize) -> Point3<f64> {
        let (i, j, k) = UniformGrid::index_to_ijk(index, self.n_points.0 - 1, self.n_points.1 - 1, self.n_points.2 - 1);
        self.min 
            + Vector3::x().scale(((i as f64)+0.5)*self.delta.0)
            + Vector3::y().scale(((j as f64)+0.5)*self.delta.1)
            + Vector3::z().scale(((k as f64)+0.5)*self.delta.2)
    }

    fn index_to_ijk(index: usize, n_x: usize, n_y: usize, _n_z: usize) -> (usize, usize, usize) {
        let k = ((index as f64) / ((n_x*n_y) as f64)).floor() as usize;
        let j = (((index - k * n_x*n_y) as f64)/(n_x as f64)).floor() as usize;
        let i = index - k * n_x*n_y - j*n_x;
        (i, j, k)
    }
}

/*
#[cfg(test)]
mod test_uniform_grid {
    use super::*;

    #[test]
    fn make_uniform_grid() -> Result<(), Box<dyn Error>> {
        let g = UniformGrid::from_n_cells_target(Position::new(-1.0, -1.0, -1.0), 
                                 Position::new( 1.0,  0.0,  2.0),
                                 7)?;
        assert_eq!(g.delta_x(), 1.0);
        assert_eq!(g.delta_y(), 1.0);
        assert_eq!(g.delta_z(), 1.0);
        assert_eq!(g.n_x(), 3);
        assert_eq!(g.n_y(), 2);
        assert_eq!(g.n_z(), 4);
        assert_eq!(g.n_cells(), 6);
        Ok(())
    }

    #[test]
    fn points_functions() -> Result<(), Box<dyn Error>> {
        let g = UniformGrid::from_n_cells_target(Position::new(0.0, 0.0, 0.0),
            Position::new(1.0, 1.0, 1.0),
            1000)?;
        assert_eq!(0.1, g.delta_x());
        assert_eq!(11, g.n_x());
        assert_eq!(11*11*11, g.n_points());
        assert_eq!(g.point_ijk_to_ind(&3,&2,&1), (3 + 11 * (2 + 11 * 1)));
        assert_eq!(g.point_ind_to_ijk(&(3+11*(2+11*1))), (3, 2, 1));
        assert!((g.point_x(&5) - 0.5).abs() < 1e-5);
        assert!((g.point_y(&3) - 0.3).abs() < 1e-5);
        assert!((g.point_z(&6) - 0.6).abs() < 1e-5);
        assert_eq!(g.point_i(&0.499), 4);
        assert_eq!(g.point_j(&0.599), 5);
        assert_eq!(g.point_k(&0.601), 6);
        Ok(())
    }

    #[test]
    fn cells_functions() -> Result<(), Box<dyn Error>> {
        let g = UniformGrid::from_n_cells_target(Position::new(0.0, 0.0, 0.0),
            Position::new(1.0, 1.0, 1.0),
            1000)?;
        assert_eq!(0.1, g.delta_x());
        assert_eq!(11, g.n_x());
        assert_eq!(10*10*10, g.n_cells());
        assert_eq!(g.cell_ijk_to_ind(&3,&2,&1), (3 + 10 * (2 + 10 * 1)));
        assert_eq!(g.cell_ind_to_ijk(&(3+10*(2+10*1))), (3, 2, 1));
        assert!((g.cell_x(&5) - 0.55).abs() < 1e-5);
        assert!((g.cell_y(&3) - 0.35).abs() < 1e-5);
        assert!((g.cell_z(&8) - 0.85).abs() < 1e-5);
        assert_eq!(g.cell_i(&0.549), 4);
        assert_eq!(g.cell_j(&0.600), 5);
        assert_eq!(g.cell_k(&0.651), 6);
        Ok(())
    }
}
*/
