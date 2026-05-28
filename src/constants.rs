//! Physical constants used by ballistics calculations.
//!
//! Constants are exposed as `#[inline]` functions because
//! `uom::si::f64::Quantity::new` is not `const`. There is no runtime
//! overhead — the compiler folds each call to a single base-unit literal.

use crate::units::*;

/// Earth's standard gravitational acceleration (32.174 ft/s², ICAO).
#[inline]
pub fn standard_gravity() -> Gravity {
    Gravity::new::<foot_per_second_squared>(32.174)
}

/// Speed of sound at sea level (1116.28 ft/s, ICAO standard atmosphere).
#[inline]
pub fn speed_of_sound_sea_level() -> SpeedOfSound {
    SpeedOfSound::new::<foot_per_second>(1116.28)
}

/// Air density at sea level (0.0765 lb/ft³, ICAO).
#[inline]
pub fn air_density_sea_level() -> AirDensity {
    AirDensity::new::<pound_per_cubic_foot>(0.0765)
}

/// ICAO standard atmospheric pressure (29.92 inHg).
#[inline]
pub fn standard_pressure() -> Pressure {
    Pressure::new::<inch_of_mercury>(29.92)
}

/// ICAO standard temperature (59 °F).
#[inline]
pub fn standard_temperature() -> Temperature {
    Temperature::new::<degree_fahrenheit>(59.0)
}

// ---------------------------------------------------------------------------
// Deprecated 0.1.x shims. Remove in 0.3.
// ---------------------------------------------------------------------------

#[deprecated(since = "0.2.0", note = "use `standard_gravity()`")]
#[allow(non_snake_case)]
pub fn STANDARD_GRAVITY() -> Gravity { standard_gravity() }

#[deprecated(since = "0.2.0", note = "use `speed_of_sound_sea_level()`")]
#[allow(non_snake_case)]
pub fn SPEED_OF_SOUND_SEA_LEVEL() -> SpeedOfSound { speed_of_sound_sea_level() }

#[deprecated(since = "0.2.0", note = "use `air_density_sea_level()`")]
#[allow(non_snake_case)]
pub fn AIR_DENSITY_SEA_LEVEL() -> AirDensity { air_density_sea_level() }

#[deprecated(since = "0.2.0", note = "use `standard_pressure()`")]
#[allow(non_snake_case)]
pub fn STANDARD_PRESSURE() -> Pressure { standard_pressure() }

#[deprecated(since = "0.2.0", note = "use `standard_temperature()`")]
#[allow(non_snake_case)]
pub fn STANDARD_TEMPERATURE() -> Temperature { standard_temperature() }
