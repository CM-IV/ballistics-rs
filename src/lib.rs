//! # ballistics_rs
//!
//! Type-safe external-ballistics calculations backed by [`uom`](https://docs.rs/uom)
//! for compile-time dimensional analysis.
//!
//! ## Quick start
//!
//! ```rust
//! use ballistics_rs::prelude::*;
//!
//! let energy = KineticEnergy::calculate()
//!     .bullet_weight(BulletWeight::new::<grain>(150.0))
//!     .velocity(Velocity::new::<foot_per_second>(3000.0))
//!     .solve();
//!
//! println!("{:.0} ft·lb", energy.get::<foot_pound>());
//! ```
//!
//! ## Where things live
//!
//! * [`units`] — every quantity alias (`Velocity`, `BulletWeight`, …) and
//!   every imperial unit marker (`foot_per_second`, `grain`, `moa`, …).
//! * [`constants`] — physical constants (`standard_gravity()`, …).
//! * [`equations`] — every calculation, exposed via `bon` builder chains.
//! * [`prelude`] — `use ballistics_rs::prelude::*;` pulls everything above.

#![forbid(unsafe_code)]
#![allow(unexpected_cfgs)]

pub mod constants;
pub mod equations;
pub mod units;

/// One-stop import. Re-exports every public item from [`units`],
/// [`constants`], and [`equations`].
pub mod prelude {
    pub use crate::constants::*;
    pub use crate::equations::*;
    pub use crate::units::*;
}

// Crate-root glob re-export preserves `use ballistics_rs::*;` ergonomics
// for code migrated from 0.1.x.
pub use crate::prelude::*;

/// Demonstrates that the type system rejects mixing incompatible units:
///
/// ```compile_fail
/// use ballistics_rs::prelude::*;
/// // Passing a Velocity where a LagTime is expected must NOT compile:
/// let _ = WindDeflection::calculate()
///     .lag_time(Velocity::new::<foot_per_second>(10.0))
///     .crosswind_speed(WindSpeed::new::<mile_per_hour>(5.0))
///     .solve();
/// ```
#[allow(dead_code)]
fn _compile_fail_marker_lag_time_must_be_time() {}
