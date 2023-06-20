#![cfg_attr(feature = "cargo-clippy", allow(inline_always))]

// TODO: parametrize on floating point type (currently num)

#[allow(non_camel_case_types)]
type num = f64;

mod traits;
pub use traits::Distribution;
pub use traits::DistributionStorage;

pub mod distribution;
pub mod physics;
mod solver;
pub use solver::Solver;
pub mod boundary;
pub mod geometry;
pub mod grid;
pub mod io;
