//! Domain-specific aliases over `uom` SI quantities, plus the imperial unit
//! markers used throughout the `ballistics_rs` public API.
//!
//! Users almost never need to import directly from `uom`. Either pull this
//! module's contents wholesale (`use ballistics_rs::units::*;`) or, more
//! commonly, use the crate-level prelude (`use ballistics_rs::prelude::*;`).
//!
//! Internally, every quantity is stored in its SI base unit (meters,
//! kilograms, seconds, kelvin, pascal, …). Conversions happen at the API
//! boundary via `Quantity::new::<unit>(value)` (input) and
//! `quantity.get::<unit>()` (output), both of which monomorphize to a single
//! multiplication.

// ---------------------------------------------------------------------------
// Quantity type aliases (one per physical concept in the domain).
// ---------------------------------------------------------------------------
// NOTE: Several aliases intentionally point at the same underlying `uom`
// type (e.g. `Velocity`, `SpeedOfSound`, `WindSpeed` are all `uom::si::f64::Velocity`).
// They exist for readability of signatures and documentation; the *builder
// method names* in `equations.rs` disambiguate them at call sites.

pub use uom::si::f64::Acceleration            as Gravity;
pub use uom::si::f64::Velocity;
pub use uom::si::f64::Time                    as TimeOfFlight;
pub use uom::si::f64::Length                  as Distance;
pub use uom::si::f64::MassDensity             as AirDensity;
pub use uom::si::f64::Mass                    as BulletWeight;
pub use uom::si::f64::ThermodynamicTemperature as Temperature;
pub use uom::si::f64::Pressure;
pub use uom::si::f64::Ratio;
pub use uom::si::f64::Angle;
pub use uom::si::f64::Energy                  as KineticEnergy;

// Secondary aliases over types already imported above. These are plain
// `pub type` (not `pub use`) so that rustdoc renders them as distinct items.
pub type SpeedOfSound             = Velocity;
pub type WindSpeed                = Velocity;
pub type VelocityProjection       = Velocity;
pub type LagTime                  = uom::si::f64::Time;
pub type BulletDiameter           = uom::si::f64::Length;
pub type SpinDrift                = uom::si::f64::Length;
pub type WindDeflection           = uom::si::f64::Length;
pub type SightCalibration         = uom::si::f64::Length;
pub type BulletLength             = Ratio;
pub type RiflingTwist             = Ratio;
pub type DragCoefficient          = Ratio;
pub type FormFactor               = Ratio;
pub type GyroscopicStability      = Ratio;
pub type BallisticCoefficient     = Ratio;
pub type AerodynamicJump          = Angle;
pub type ApertureSightCalibration = Angle;

// ---------------------------------------------------------------------------
// Unit markers re-exported in one place, so that callers never need to
// know which `uom::si::*` submodule a given unit lives in.
// ---------------------------------------------------------------------------

pub use uom::si::acceleration::foot_per_second_squared;
pub use uom::si::velocity::{foot_per_second, mile_per_hour};
pub use uom::si::time::second;
pub use uom::si::length::{foot, inch};
pub use uom::si::mass_density::pound_per_cubic_foot;
pub use uom::si::mass::grain;
pub use uom::si::thermodynamic_temperature::degree_fahrenheit;
pub use uom::si::pressure::inch_of_mercury;
pub use uom::si::energy::foot_pound;
pub use uom::si::ratio::ratio;

/// Minute of arc — the ballistics community's "MOA". Re-export of
/// `uom::si::angle::minute` under a domain-friendlier name.
pub use uom::si::angle::minute as moa;
