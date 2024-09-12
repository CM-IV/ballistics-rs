//! # Ballistics Crate
//!
//! This crate provides constants and equations for solving ballistics problems.
//! It includes definitions for various physical constants and properties
//! related to ballistics, such as gravitational constant, speed of sound,
//! gyroscopic stability, kinetic energy, and ballistic coefficient.

mod constants;
mod equations;

pub use constants::*;
pub use equations::*;