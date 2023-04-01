//! This crate provides a fluid simulation engine using the vortex particle method. The fluid 
//! is modelled using a set of particle representing the fluid vorticity. The particles are 
//! convected in a lagrangien manner (ie the particles moves).
//!
mod algebra; pub use algebra::{Point3, Vector3};
mod algorithms; 
pub use algorithms::{VortonToVelocity, 
                     VortonToVelocitySimple, VortonToVelocitySimpleBuilder,
                     VortonToVelocityTree, VortonToVelocityTreeBuilder,
                     };
mod sim; pub use sim::{UniformGrid, Vorton, SuperVorton};
mod configuration; pub use configuration::{InitialConditions, Configuration};
mod profiler; pub use profiler::Profiler;
mod output; pub use output::{Grid, GridBuilder, VortonCollection};
mod simulation; pub use simulation::{Simulation, VortonToVelocityAlgorithm};
mod geometry; pub use geometry::{Geometry, GeometryTrait};

