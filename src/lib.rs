//! # Ballistics Crate
//!
//! This crate provides constants and equations for solving ballistics problems.
//! It includes definitions for various physical constants and properties
//! related to ballistics, such as gravitational constant, speed of sound,
//! gyroscopic stability, kinetic energy, and ballistic coefficient.
//!
//! ## Version 0.2.0: Support for Imperial and SI Units
//!
//! Starting from version `0.2.0`, the crate supports both Imperial and SI units.
//! Each function and constant can be used with either unit system, and the examples
//! below demonstrate how to use both.
//!
//! ### Examples
//!
//! #### Speed of Sound
//!
//! **Imperial Units:**
//! ```rust
//! use ballistics_rs::{SpeedOfSound, Temperature};
//! use float_cmp::approx_eq;
//!
//! let speed: SpeedOfSound<Imperial> = SpeedOfSound::calculate()
//!     .temperature(Temperature::new(68.0)) // Temperature in Fahrenheit
//!     .solve();
//!
//! assert!(approx_eq!(f64, speed.value(), 1126.1, epsilon = 0.1)); // Speed in feet per second
//! ```
//!
//! **SI Units:**
//! ```rust
//! use ballistics_rs::{SpeedOfSound, Temperature};
//! use float_cmp::approx_eq;
//!
//! let speed: SpeedOfSound<SI> = SpeedOfSound::calculate()
//!     .temperature(Temperature::new(20.0)) // Temperature in Celsius
//!     .solve();
//!
//! assert!(approx_eq!(f64, speed.value(), 343.2, epsilon = 0.1)); // Speed in meters per second
//! ```
//!
//! #### Kinetic Energy
//!
//! **Imperial Units:**
//! ```rust
//! use ballistics_rs::{KineticEnergy, BulletWeight, Velocity};
//! use float_cmp::approx_eq;
//!
//! let energy: KineticEnergy<Imperial> = KineticEnergy::calculate()
//!     .bullet_weight(BulletWeight::new(150.0)) // Bullet weight in grains
//!     .velocity(Velocity::new(3000.0)) // Velocity in feet per second
//!     .solve();
//!
//! assert!(approx_eq!(f64, energy.value(), 2994.6, epsilon = 0.1)); // Energy in foot-pounds
//! ```
//!
//! **SI Units:**
//! ```rust
//! use ballistics_rs::{KineticEnergy, BulletWeight, Velocity};
//! use float_cmp::approx_eq;
//!
//! let energy: KineticEnergy<SI> = KineticEnergy::calculate()
//!     .bullet_weight(BulletWeight::new(0.0097)) // Bullet weight in kilograms
//!     .velocity(Velocity::new(914.4000)) // Velocity in meters per second
//!     .solve();
//!
//! assert!(approx_eq!(f64, energy.value(), 4055.2, epsilon = 0.1)); // Energy in joules
//! ```

mod constants;
mod equations;

pub use constants::*;
pub use equations::*;

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn test_speed_of_sound() {
        let speed: SpeedOfSound<Imperial> = SpeedOfSound::calculate()
            .temperature(Temperature::new(68.0))
            .solve();

        assert!(approx_eq!(f64, speed.value(), 1126.1, epsilon = 0.1));
    }

    #[test]
    fn test_speed_of_sound_si() {
        let speed: SpeedOfSound<SI> = SpeedOfSound::calculate()
            .temperature(Temperature::new(20.0))
            .solve();

        assert!(approx_eq!(f64, speed.value(), 343.2, epsilon = 0.1));
    }

    #[test]
    fn test_kinetic_energy() {
        let energy: KineticEnergy<Imperial> = KineticEnergy::calculate()
            .bullet_weight(BulletWeight::new(150.0))
            .velocity(Velocity::new(3000.0))
            .solve();

        assert!(approx_eq!(f64, energy.value(), 2994.6, epsilon = 0.1));
    }

    #[test]
    fn test_kinetic_energy_si() {
        let energy: KineticEnergy<SI> = KineticEnergy::calculate()
            .bullet_weight(BulletWeight::new(0.0097))
            .velocity(Velocity::new(914.4000))
            .solve();

        assert!(approx_eq!(f64, energy.value(), 4055.2, epsilon = 0.1));
    }

    #[test]
    fn test_aperture_sight_calibration() {
        let moa: ApertureSightCalibration<Imperial> = ApertureSightCalibration::calculate()
            .sight_movement_twenty_clicks(SightCalibration::new(2.0))
            .sight_radius(SightCalibration::new(3.0))
            .solve();

        println!("{}", moa);
    }
}
