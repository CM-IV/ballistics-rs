# PLAN.md — Migrating `ballistics_rs` to `uom`

> **Mission for the implementing agent:** Replace the hand-rolled `synonym`-based newtype quantities in `ballistics_rs` with [`uom`](https://docs.rs/uom) (`/iliekturtles/uom`) for dimensional safety, while keeping the public API ergonomic via a curated façade in `units.rs` plus a `prelude` module. Do the work in the **exact order** below. After every numbered step, run the listed acceptance commands and do not proceed if any fail.
>
> **Crate to modify:** `ballistics_rs` v0.1.5 → v0.2.0
> **Repo layout (current):** `src/lib.rs`, `src/constants.rs`, `src/equations.rs`, `Cargo.toml`
> **Final repo layout:** `src/lib.rs`, `src/units.rs` (new), `src/constants.rs`, `src/equations.rs`, `Cargo.toml`, `MIGRATION.md` (new), updated `README.md`

---

## 0. Background The Agent Must Internalize Before Starting

### 0.1 How `uom` works (the only API surface used in this migration)

`uom` exposes quantity types in `uom::si::f64::*` (e.g. `Length`, `Mass`, `Velocity`, `Time`, `Pressure`, `Energy`, `Ratio`, `Angle`, `ThermodynamicTemperature`, `MassDensity`, `Acceleration`). Each quantity has many **unit constructors** in a submodule named after the quantity (e.g. `uom::si::length::{meter, foot, inch}`, `uom::si::mass::{kilogram, pound, grain}`).

**The only three operations needed:**

```rust
// 1. Construct (value is normalized to SI base unit at construction time):
let v = uom::si::f64::Velocity::new::<uom::si::velocity::foot_per_second>(3000.0);

// 2. Extract a scalar in any compatible unit:
let fps: f64 = v.get::<uom::si::velocity::foot_per_second>();

// 3. Arithmetic (dimensionally checked at compile time):
let length = uom::si::f64::Length::new::<uom::si::length::foot>(1000.0);
let time   = length / v;                          // → Time, automatically
let accel  = v / time;                            // → Acceleration
let energy = mass * v * v * 0.5;                  // → Energy
```

**Critical facts:**
- `Quantity::new::<Unit>(value)` is **not `const`**. Constants must be `pub fn` helpers, not `const`.
- `Ratio` (dimensionless) base unit is `ratio` (factor 1.0). `Ratio::new::<ratio>(0.5).get::<ratio>() == 0.5`.
- To take `sqrt`, `powi`, etc. of a dimensionless quantity, extract the scalar with `.get::<ratio>()`, do the math, then re-wrap. `uom` does **not** provide `Quantity::sqrt` for general dimensions.
- Temperature in `uom` distinguishes **`ThermodynamicTemperature`** (absolute, supports °C/°F/K with offset) from `TemperatureInterval` (deltas). All our formulas treat temperature as absolute, so use `ThermodynamicTemperature`. Unit `degree_fahrenheit` exists.
- `Angle`'s `minute` unit is the arc-minute — i.e. **MOA** in ballistics.
- `MassDensity` unit `pound_per_cubic_foot` exists in `uom::si::mass_density`.
- `Energy` unit `foot_pound` exists in `uom::si::energy`.
- `Pressure` unit `inch_of_mercury` exists in `uom::si::pressure`.
- `Acceleration` unit `foot_per_second_squared` exists in `uom::si::acceleration`.
- `Velocity` units `foot_per_second` and `mile_per_hour` exist in `uom::si::velocity`.
- `Time` unit `second` exists in `uom::si::time`.
- `Length` units `foot` and `inch` exist in `uom::si::length`.
- `Mass` unit `grain` exists in `uom::si::mass`.

If `cargo build` reports any of these are missing in the chosen `uom` version, fall back to the workaround documented inline in step 4.4.

### 0.2 The old API (must keep working at the call-site level, with minimal mechanical changes)

Today users write:
```rust
use ballistics_rs::*;
let v = Velocity(3000.0);
let result = KineticEnergy::calculate()
    .bullet_weight(BulletWeight(150.0))
    .velocity(Velocity(3000.0))
    .solve();
let ftlb: f64 = result.0;
```

After migration:
```rust
use ballistics_rs::prelude::*;
let v = Velocity::new::<foot_per_second>(3000.0);
let result = KineticEnergy::calculate()
    .bullet_weight(BulletWeight::new::<grain>(150.0))
    .velocity(Velocity::new::<foot_per_second>(3000.0))
    .solve();
let ftlb: f64 = result.get::<foot_pound>();
```

Only two things change at every call site: **constructor** (`T(x)` → `T::new::<unit>(x)`) and **extraction** (`r.0` → `r.get::<unit>()`). Builder method names and chain shape do **not** change.

### 0.3 Type-alias decisions (FINAL — do not deviate)

| Old newtype | New alias | Underlying `uom` type | Canonical input unit |
|---|---|---|---|
| `Gravity` | `Gravity` | `Acceleration` | `foot_per_second_squared` |
| `SpeedOfSound` | `SpeedOfSound` | `Velocity` | `foot_per_second` |
| `Velocity` | `Velocity` | `Velocity` | `foot_per_second` |
| `WindSpeed` | `WindSpeed` | `Velocity` | `mile_per_hour` |
| `VelocityProjection` | `VelocityProjection` | `Velocity` | `foot_per_second` |
| `TimeOfFlight` | `TimeOfFlight` | `Time` | `second` |
| `LagTime` | `LagTime` | `Time` | `second` |
| `Distance` | `Distance` | `Length` | `foot` |
| `BulletDiameter` | `BulletDiameter` | `Length` | `inch` |
| `SpinDrift` | `SpinDrift` | `Length` | `inch` |
| `WindDeflection` | `WindDeflection` | `Length` | `inch` |
| `SightCalibration` | `SightCalibration` | `Length` | `inch` |
| `AirDensity` | `AirDensity` | `MassDensity` | `pound_per_cubic_foot` |
| `BulletWeight` | `BulletWeight` | `Mass` | `grain` |
| `Temperature` | `Temperature` | `ThermodynamicTemperature` | `degree_fahrenheit` |
| `Pressure` | `Pressure` | `Pressure` | `inch_of_mercury` |
| `KineticEnergy` | `KineticEnergy` | `Energy` | `foot_pound` |
| `AerodynamicJump` | `AerodynamicJump` | `Angle` | `moa` (alias of `minute`) |
| `ApertureSightCalibration` | `ApertureSightCalibration` | `Angle` | `moa` |
| `DragCoefficient` | `DragCoefficient` | `Ratio` | `ratio` |
| `FormFactor` | `FormFactor` | `Ratio` | `ratio` |
| `GyroscopicStability` | `GyroscopicStability` | `Ratio` | `ratio` |
| `BallisticCoefficient` | `BallisticCoefficient` | `Ratio` | `ratio` |
| `BulletLength` | `BulletLength` | `Ratio` | `ratio` (it's "calibers", dimensionless) |
| `RiflingTwist` | `RiflingTwist` | `Ratio` | `ratio` (it's "calibers per turn", dimensionless) |

**Why `Ratio` for `BulletLength` and `RiflingTwist`:** They are length-in-calibers and length-per-turn-in-calibers; the caliber conversion is a per-bullet quantity, not a universal constant. Storing them as `Ratio` is honest and prevents accidental mixing with real `Length`.

**Why many aliases collapse to the same underlying type:** `Velocity` ≡ `SpeedOfSound` ≡ `WindSpeed` ≡ `VelocityProjection` is fine — the **builder method name** (`.crosswind_speed(...)`, `.muzzle_velocity(...)`) carries the semantic distinction; the alias gives doc clarity without runtime cost.

---

## 1. Step 1 — Branch & Baseline

**Goal:** Reproducible starting point.

1. Create branch: `git switch -c migrate-to-uom`.
2. Run baseline and **save the output** to `BASELINE.txt`:
   ```bash
   cargo test --all 2>&1 | tee BASELINE.txt
   cargo build --release 2>&1 | tee -a BASELINE.txt
   ls -la target/release/libballistics_rs.* >> BASELINE.txt
   ```
3. Commit the baseline file so we can compare later: `git add BASELINE.txt && git commit -m "chore: record pre-migration baseline"`.

**Acceptance gate:** `BASELINE.txt` exists and shows all tests passing.

---

## 2. Step 2 — Update `Cargo.toml`

**Goal:** Swap dependencies, bump version.

Edit `Cargo.toml`:
- Bump `version = "0.1.5"` → `version = "0.2.0"`.
- In `[dependencies]`:
  - **Remove** the line `synonym = "0.1.5"`.
  - **Keep** `bon = "2.3.0"` exactly as-is.
  - **Add** `uom = { version = "0.36", default-features = false, features = ["f64", "si", "std", "autoconvert"] }`.
- Add a top-level field (if not present): `rust-version = "1.65"`.
- Leave the entire `[profile.release]` block untouched.

**Acceptance gate:**
```bash
cargo metadata --format-version=1 > /dev/null   # must succeed (parses Cargo.toml)
cargo tree | grep -q '^uom '                    # uom must appear
! cargo tree | grep -q '^synonym '              # synonym must NOT appear
```
Commit: `git add Cargo.toml && git commit -m "deps: replace synonym with uom, bump to 0.2.0"`.

> At this point `cargo build` **will fail** because `synonym` is gone but the old code still references it. That is expected. Proceed to step 3.

---

## 3. Step 3 — Create `src/units.rs` (the façade)

**Goal:** Single module that re-exports `uom` types under our domain names plus all imperial unit markers.

Create new file `src/units.rs` with **exactly** the following contents:

```rust
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
```

**Acceptance gate (run BEFORE proceeding):**
```bash
cargo check 2>&1 | tee /tmp/check.log
```
This **will still fail** because `lib.rs` doesn't include `units` yet and `equations.rs`/`constants.rs` still reference `synonym`. But the error count should be limited to those two files. If you see errors *inside* `units.rs` itself (e.g. "cannot find `pound_per_cubic_foot` in `uom::si::mass_density`"), the installed `uom` version doesn't ship that unit. Then **and only then**, apply the fallback in step 4.4 immediately before continuing.

Commit: `git add src/units.rs && git commit -m "feat: add units façade module"`.

---

## 4. Step 4 — Rewrite `src/lib.rs`

**Goal:** Wire up `units`, expose a `prelude`, and keep the crate-root glob behavior so most downstream code keeps working.

Replace the **entire** contents of `src/lib.rs` with:

```rust
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

pub mod units;
pub mod constants;
pub mod equations;

/// One-stop import. Re-exports every public item from [`units`],
/// [`constants`], and [`equations`].
pub mod prelude {
    pub use crate::units::*;
    pub use crate::constants::*;
    pub use crate::equations::*;
}

// Crate-root glob re-export preserves `use ballistics_rs::*;` ergonomics
// for code migrated from 0.1.x.
pub use crate::prelude::*;
```

### 4.4 Fallback: missing unit in installed `uom` version

If step 3's `cargo check` reported a missing unit (most likely `pound_per_cubic_foot`), append the following block to the **bottom** of `src/units.rs` and replace the corresponding `pub use uom::si::…::missing_unit;` line in step 3 with `pub use self::custom_units::missing_unit;`:

```rust
#[allow(non_camel_case_types, dead_code)]
mod custom_units {
    // Conversion: 1 lb/ft³ = 16.018463 kg/m³.
    uom::unit! {
        system: uom::si;
        quantity: uom::si::mass_density;

        @pound_per_cubic_foot: 1.6018463E1; "lb/ft³",
            "pound per cubic foot", "pounds per cubic foot";
    }
}
```

(Apply the same pattern for any other missing unit, with the appropriate conversion factor.)

**Acceptance gate:**
```bash
cargo check 2>&1 | grep -E '^error' | grep -vE '(constants|equations)\.rs' | wc -l   # must print 0
```
i.e. all remaining compile errors must be confined to `constants.rs` and `equations.rs`.

Commit: `git add src/lib.rs && git commit -m "feat: prelude + crate-root re-exports for new façade"`.

---

## 5. Step 5 — Rewrite `src/constants.rs`

**Goal:** Replace newtypes-with-numeric-consts with `pub fn` returning `uom` quantities. Provide deprecated `SCREAMING_CASE` shims so any downstream code that referenced the constants by name still compiles (with a warning).

Replace the **entire** contents of `src/constants.rs` with:

```rust
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
```

**Acceptance gate:**
```bash
cargo check --lib 2>&1 | grep -E '^error' | grep 'constants\.rs' | wc -l   # must print 0
```
Remaining errors should now be only in `equations.rs`. Commit: `git add src/constants.rs && git commit -m "feat: constants as typed uom helpers"`.

---

## 6. Step 6 — Rewrite `src/equations.rs`

**Goal:** Convert every builder to use the new aliases. Inside each `solve()`, extract scalars with `.get::<unit>()`, keep the *exact original scalar formula*, and re-wrap the result. (Native-`uom` arithmetic comes in step 9, after tests are green.)

**Replace the entire contents of `src/equations.rs` with the following.** This is a **complete** implementation — copy verbatim except where the original file used a different parameter name (in which case, preserve the original name).

```rust
//! Ballistics calculations.
//!
//! Each calculation uses the `bon` builder pattern: call the associated
//! `calculate()` (or stage-specific) function, chain the required parameters
//! by name, and call `.solve()` to finish.
//!
//! Inputs and outputs are strongly-typed `uom` quantities — see the
//! [`crate::units`] module for the alias table.

use bon::bon;

use crate::units::*;

// ---------------------------------------------------------------------------
// 1. Speed of Sound
// ---------------------------------------------------------------------------

#[bon]
impl SpeedOfSoundCalc {
    /// `49.0223 × √(T_°F + 459.67)`  →  ft/s
    #[builder(finish_fn = solve)]
    pub fn calculate(temperature: Temperature) -> SpeedOfSound {
        let t_f = temperature.get::<degree_fahrenheit>();
        let fps = 49.0223 * (t_f + 459.67).sqrt();
        SpeedOfSound::new::<foot_per_second>(fps)
    }
}
/// Zero-sized marker type used to host the speed-of-sound builder.
pub struct SpeedOfSoundCalc;

// ---------------------------------------------------------------------------
// 2. Kinetic Energy
// ---------------------------------------------------------------------------

pub struct KineticEnergyCalc;

#[bon]
impl KineticEnergyCalc {
    /// `(W_grains × V_fps²) / 450800`  →  ft·lb
    #[builder(finish_fn = solve)]
    pub fn calculate(bullet_weight: BulletWeight, velocity: Velocity) -> KineticEnergy {
        let w = bullet_weight.get::<grain>();
        let v = velocity.get::<foot_per_second>();
        let ftlb = (w * v * v) / 450_800.0;
        KineticEnergy::new::<foot_pound>(ftlb)
    }
}

// ---------------------------------------------------------------------------
// 3. Aperture Sight Calibration
// ---------------------------------------------------------------------------

pub struct ApertureSightCalibrationCalc;

#[bon]
impl ApertureSightCalibrationCalc {
    /// `171.89 × (movement_in / radius_in)`  →  MOA per click
    #[builder(finish_fn = solve)]
    pub fn calculate(
        sight_movement_twenty_clicks: SightCalibration,
        sight_radius: SightCalibration,
    ) -> ApertureSightCalibration {
        let mvmt = sight_movement_twenty_clicks.get::<inch>();
        let radius = sight_radius.get::<inch>();
        let moa_per_click = 171.89 * (mvmt / radius);
        ApertureSightCalibration::new::<moa>(moa_per_click)
    }
}

// ---------------------------------------------------------------------------
// 4. Form Factor
// ---------------------------------------------------------------------------

pub struct FormFactorCalc;

#[bon]
impl FormFactorCalc {
    /// `CD_bullet / CD_standard`  →  dimensionless
    #[builder(finish_fn = solve)]
    pub fn calculate(
        drag_coefficient: DragCoefficient,
        standard_bullet_drag_coefficient: DragCoefficient,
    ) -> FormFactor {
        let cd = drag_coefficient.get::<ratio>();
        let cd_std = standard_bullet_drag_coefficient.get::<ratio>();
        FormFactor::new::<ratio>(cd / cd_std)
    }
}

// ---------------------------------------------------------------------------
// 5. Velocity Projection
// ---------------------------------------------------------------------------

pub struct VelocityProjectionCalc;

#[bon]
impl VelocityProjectionCalc {
    /// `V₁ × √(W₁ / W₂)`  →  ft/s
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight_1: BulletWeight,
        bullet_weight_2: BulletWeight,
        bullet_velocity_1: Velocity,
    ) -> VelocityProjection {
        let w1 = bullet_weight_1.get::<grain>();
        let w2 = bullet_weight_2.get::<grain>();
        let v1 = bullet_velocity_1.get::<foot_per_second>();
        let v2 = v1 * (w1 / w2).sqrt();
        VelocityProjection::new::<foot_per_second>(v2)
    }
}

// ---------------------------------------------------------------------------
// 6. Lag Time
// ---------------------------------------------------------------------------

pub struct LagTimeCalc;

#[bon]
impl LagTimeCalc {
    /// `t_actual − D / V₀`  →  seconds
    #[builder(finish_fn = solve)]
    pub fn calculate(
        actual_time_of_flight: TimeOfFlight,
        distance: Distance,
        muzzle_velocity: Velocity,
    ) -> LagTime {
        let t = actual_time_of_flight.get::<second>();
        let d = distance.get::<foot>();
        let v = muzzle_velocity.get::<foot_per_second>();
        LagTime::new::<second>(t - d / v)
    }
}

// ---------------------------------------------------------------------------
// 7. Wind Deflection
// ---------------------------------------------------------------------------

pub struct WindDeflectionCalc;

#[bon]
impl WindDeflectionCalc {
    /// `17.6 × W_mph × lag_s`  →  inches
    #[builder(finish_fn = solve)]
    pub fn calculate(
        lag_time: LagTime,
        crosswind_speed: WindSpeed,
    ) -> WindDeflection {
        let lag = lag_time.get::<second>();
        let w = crosswind_speed.get::<mile_per_hour>();
        WindDeflection::new::<inch>(17.6 * w * lag)
    }
}

// ---------------------------------------------------------------------------
// 8. Aerodynamic Jump
// ---------------------------------------------------------------------------

pub struct AerodynamicJumpCalc;

#[bon]
impl AerodynamicJumpCalc {
    /// `0.01·S − 0.0024·L + 0.032`  →  MOA (per 1 mph crosswind)
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability,
        bullet_length: BulletLength,
    ) -> AerodynamicJump {
        let s = gyro_stability.get::<ratio>();
        let l = bullet_length.get::<ratio>();
        AerodynamicJump::new::<moa>(0.01 * s - 0.0024 * l + 0.032)
    }
}

// ---------------------------------------------------------------------------
// 9. Gyroscopic Stability (three stages share a host type)
// ---------------------------------------------------------------------------

pub struct GyroscopicStabilityCalc;

#[bon]
impl GyroscopicStabilityCalc {
    /// 9a. Miller base stability: `30·W / (T²·D³·L·(1+L²))`
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight,
        rifling_twist: RiflingTwist,
        bullet_diameter: BulletDiameter,
        bullet_length: BulletLength,
    ) -> GyroscopicStability {
        let w = bullet_weight.get::<grain>();
        let t = rifling_twist.get::<ratio>();
        let d = bullet_diameter.get::<inch>();
        let l = bullet_length.get::<ratio>();
        let s = (30.0 * w) / (t * t * d * d * d * l * (1.0 + l * l));
        GyroscopicStability::new::<ratio>(s)
    }

    /// 9b. Velocity correction: `S · (V/2800)^(1/3)`
    #[builder(finish_fn = solve)]
    pub fn velocity_correction(
        muzzle_velocity: Velocity,
        gyro_stability: GyroscopicStability,
    ) -> GyroscopicStability {
        let v = muzzle_velocity.get::<foot_per_second>();
        let s = gyro_stability.get::<ratio>();
        GyroscopicStability::new::<ratio>(s * (v / 2800.0).cbrt())
    }

    /// 9c. Atmospheric correction: `S · ((T+460)/519) · (29.92/P)`
    #[builder(finish_fn = solve)]
    pub fn atmospheric_correction(
        air_temp: Temperature,
        air_pressure: Pressure,
        gyro_stability: GyroscopicStability,
    ) -> GyroscopicStability {
        let t = air_temp.get::<degree_fahrenheit>();
        let p = air_pressure.get::<inch_of_mercury>();
        let s = gyro_stability.get::<ratio>();
        GyroscopicStability::new::<ratio>(s * ((t + 460.0) / 519.0) * (29.92 / p))
    }
}

// ---------------------------------------------------------------------------
// 10. Spin Drift
// ---------------------------------------------------------------------------

pub struct SpinDriftCalc;

#[bon]
impl SpinDriftCalc {
    /// `1.25 · (S + 1.2) · t^1.83`  →  inches
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability,
        actual_time_of_flight: TimeOfFlight,
    ) -> SpinDrift {
        let s = gyro_stability.get::<ratio>();
        let t = actual_time_of_flight.get::<second>();
        SpinDrift::new::<inch>(1.25 * (s + 1.2) * t.powf(1.83))
    }
}

// ---------------------------------------------------------------------------
// 11. Ballistic Coefficient
// ---------------------------------------------------------------------------

pub struct BallisticCoefficientCalc;

#[bon]
impl BallisticCoefficientCalc {
    /// `(W_grains / 7000) / (D_in² · ff)`
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight,
        bullet_diameter: BulletDiameter,
        form_factor: FormFactor,
    ) -> BallisticCoefficient {
        let w = bullet_weight.get::<grain>();
        let d = bullet_diameter.get::<inch>();
        let ff = form_factor.get::<ratio>();
        BallisticCoefficient::new::<ratio>((w / 7000.0) / (d * d * ff))
    }
}

// ---------------------------------------------------------------------------
// Compatibility re-exports so users can still write
// `KineticEnergy::calculate()` instead of `KineticEnergyCalc::calculate()`.
// These are inherent-impl shims: associated functions on the alias's "calc"
// type, exposed through type aliases that mirror the old direct-on-type API.
// ---------------------------------------------------------------------------

/// Old-style entry point for [`KineticEnergyCalc`].
pub trait KineticEnergyExt {
    fn calculate() -> KineticEnergyCalcCalculateBuilder;
}
impl KineticEnergyExt for KineticEnergy {
    fn calculate() -> KineticEnergyCalcCalculateBuilder { KineticEnergyCalc::calculate() }
}
// ... (repeat the same trait+impl pattern for each calculation; see step 6.1)
```

### 6.1 Compatibility shim pattern (FOLLOW THIS RULE FOR ALL 11 CALCULATIONS)

The previous `bon`-based API let users write `KineticEnergy::calculate()` because the builder was attached directly to the newtype. Now that `KineticEnergy` is a `uom` type alias, we cannot add inherent methods to it. We solve this with **extension traits** named `<Name>Ext`, one per calculation, exporting a `calculate()` (and where applicable `velocity_correction()` / `atmospheric_correction()`) associated function.

The trait names — `KineticEnergyExt`, `SpeedOfSoundExt`, `ApertureSightCalibrationExt`, `FormFactorExt`, `VelocityProjectionExt`, `LagTimeExt`, `WindDeflectionExt`, `AerodynamicJumpExt`, `GyroscopicStabilityExt`, `SpinDriftExt`, `BallisticCoefficientExt` — must all be re-exported from `crate::prelude` (they already are via `pub use crate::equations::*;`).

After defining the traits, **users continue writing `KineticEnergy::calculate()` exactly as before**, provided they `use ballistics_rs::prelude::*;` (which brings the `Ext` traits into scope).

⚠️ The exact name of the builder type produced by `#[bon] #[builder]` follows `bon`'s naming convention: `<TypeName><MethodCamelCase>Builder`. If `bon` 2.3 produces a different name, run `cargo check` and copy the name from the error message into the trait signature.

### 6.2 If `#[bon]` rejects a non-`self` `impl` block

If `bon` 2.3 requires builders to live on a type with `self`-receiving methods rather than associated functions on a marker struct, fall back to the **free-function builder** form documented in `bon`'s docs:

```rust
#[bon::builder(finish_fn = solve)]
fn kinetic_energy(
    bullet_weight: BulletWeight,
    velocity: Velocity,
) -> KineticEnergy { /* body */ }
```

Then the `Ext` trait simply calls `kinetic_energy()` as its `calculate()` body. If you take this path, name the free functions `snake_case_of_the_calculation` and keep them `pub(crate)` so they don't appear in the public API.

**Acceptance gate:**
```bash
cargo build 2>&1 | tee /tmp/build.log
echo "---"
grep -c '^error' /tmp/build.log    # must print 0
cargo build --release              # must succeed
```

Commit: `git add src/equations.rs && git commit -m "feat: rewrite equations on top of uom"`.

---

## 7. Step 7 — Migrate Existing Tests

**Goal:** Get the original test suite green again. Do **not** change test logic — only constructors and field-access patterns.

For every test file under `tests/` and every `#[test]` in `src/`:

1. Replace `TypeName(literal)` → `TypeName::new::<canonical_unit>(literal)` using the table in §0.3.
2. Replace `result.0` → `result.get::<canonical_unit>()` using the same table.
3. Replace `use ballistics_rs::*;` → `use ballistics_rs::prelude::*;` (the old form still works via crate-root glob, but the prelude form is the documented one).

**Mechanical search-and-replace recipes** (run from repo root, review each diff):

```bash
# Show every occurrence of `Name(<number>)` to update:
rg -n '\b(Velocity|BulletWeight|BulletDiameter|BulletLength|RiflingTwist|Distance|TimeOfFlight|LagTime|WindSpeed|SpinDrift|SightCalibration|AirDensity|Temperature|Pressure|DragCoefficient|FormFactor|GyroscopicStability|BallisticCoefficient|KineticEnergy|SpeedOfSound|AerodynamicJump|ApertureSightCalibration|VelocityProjection|WindDeflection|Gravity)\(\s*[-0-9.eE_]+\s*\)'

# Show every `.0` access on a probable quantity binding:
rg -n '\.0\b'
```

For each match, manually edit using the §0.3 unit table. (Do **not** sed-replace blindly — `.0` may appear on tuple indices unrelated to quantities.)

**Acceptance gate:**
```bash
cargo test --all
```
Every previously-passing test must pass. If a test asserts on a floating-point value, the assertion must still hold to within at least 6 significant figures — `uom`'s unit conversion factors match the constants the old code used.

If any test now reports a value off by a few ULPs, that's expected from one extra multiplication on input and one on output; widen the comparison with `assert!((a - b).abs() < 1e-9)` rather than changing the expected value.

Commit: `git add tests/ src/ && git commit -m "test: port test suite to uom API"`.

---

## 8. Step 8 — Add Migration Safety Tests

**Goal:** Lock in dimensional safety and numerical equivalence with the 0.1.x results.

### 8.1 Equivalence test

Create `tests/equivalence.rs`:

```rust
//! Numerical-equivalence tests against hand-computed 0.1.x results.
//!
//! Every assertion uses the exact scalar formulas that v0.1.5 used,
//! computed independently in this test file, and compares them against
//! the v0.2.0 typed implementation. They must agree to 1e-9 relative
//! error — confirming the migration is a pure refactor at the
//! arithmetic level.

use ballistics_rs::prelude::*;

fn approx_eq(a: f64, b: f64, rel: f64) {
    let denom = a.abs().max(b.abs()).max(1.0);
    assert!(
        (a - b).abs() / denom < rel,
        "values diverge: a={a}, b={b}, rel_err={}",
        (a - b).abs() / denom
    );
}

#[test]
fn speed_of_sound_matches_legacy() {
    let t = 68.0_f64;
    let expected = 49.0223 * (t + 459.67).sqrt();
    let actual = SpeedOfSound::calculate()
        .temperature(Temperature::new::<degree_fahrenheit>(t))
        .solve()
        .get::<foot_per_second>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn kinetic_energy_matches_legacy() {
    let (w, v) = (150.0_f64, 3000.0_f64);
    let expected = (w * v * v) / 450_800.0;
    let actual = KineticEnergy::calculate()
        .bullet_weight(BulletWeight::new::<grain>(w))
        .velocity(Velocity::new::<foot_per_second>(v))
        .solve()
        .get::<foot_pound>();
    approx_eq(actual, expected, 1e-9);
}

// REPEAT the same pattern for: aperture_sight_calibration, form_factor,
// velocity_projection, lag_time, wind_deflection, aerodynamic_jump,
// gyroscopic_stability (all 3 stages), spin_drift, ballistic_coefficient.
// Use the inputs from the SUMMARY.md examples as the canonical test points.
```

### 8.2 Dimensional safety doc-tests

Append to `src/lib.rs`:

```rust
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
```

**Acceptance gate:**
```bash
cargo test --all                       # both unit and integration tests
cargo test --doc                       # doc-tests, including compile_fail ones
```

Commit: `git add tests/equivalence.rs src/lib.rs && git commit -m "test: equivalence + compile-fail dimensional checks"`.

---

## 9. Step 9 — Optional: Native-`uom` Arithmetic Pass

**Goal:** Replace the three places where native `uom` arithmetic obsoletes a magic constant. **Do each in its own commit.** This step is optional but strongly recommended — it's the visible payoff of the migration.

Only refactor these three; leave the rest as-is.

### 9.1 `WindDeflection`: drop the `17.6`

Replace the body of `WindDeflectionCalc::calculate` with:

```rust
// Direct uom arithmetic: WindSpeed × Time = Length. No magic constant.
crosswind_speed * lag_time
```

Run `tests/equivalence.rs` — `wind_deflection_matches_legacy` must still pass to 1e-9.

### 9.2 `LagTime`: drop the manual `d/v`

Replace the body of `LagTimeCalc::calculate` with:

```rust
actual_time_of_flight - distance / muzzle_velocity
```

Run equivalence tests.

### 9.3 `KineticEnergy`: drop the `450_800`

Replace the body of `KineticEnergyCalc::calculate` with:

```rust
// E = ½ m v². uom infers Mass × Velocity² = Energy.
0.5 * bullet_weight * velocity * velocity
```

Run equivalence tests. **This is the headline demonstration that the migration delivered real safety value.** Call it out in `MIGRATION.md`.

**Acceptance gate after each substep:** `cargo test --all` green; equivalence within 1e-9.

---

## 10. Step 10 — Documentation

### 10.1 `MIGRATION.md` (new file at repo root)

Create with exactly this skeleton, filling in each row:

````markdown
# Migrating from `ballistics_rs` 0.1.x to 0.2.x

0.2 internally uses [`uom`](https://docs.rs/uom) for compile-time dimensional
safety. The public API still uses domain names like `BulletWeight` and
`Velocity`, but those are now aliases for `uom` quantity types, not `f64`
newtypes.

## What changed

| Concern | 0.1.x | 0.2.x |
|---|---|---|
| Construct a quantity | `Velocity(3000.0)` | `Velocity::new::<foot_per_second>(3000.0)` |
| Extract a scalar | `result.0` | `result.get::<foot_per_second>()` |
| Parse from string | `"3000.0".parse::<Velocity>()` | `Velocity::new::<foot_per_second>("3000.0".parse()?)` |
| Constants | `STANDARD_GRAVITY` | `standard_gravity()` |
| Imports | `use ballistics_rs::*;` | `use ballistics_rs::prelude::*;` (recommended) |

## Removed traits

`synonym`-derived `Display`, `FromStr`, `Default`, `Zero`, and the public
`.0` field are gone. The replacements all live on `uom::si::f64::Quantity`:

* `Default::default()` — still works (zero in base units).
* `Display` — still works; the formatted string includes the SI base-unit
  symbol. To format in your unit of choice, use `q.get::<unit>()` first.
* `FromStr` — replace with `Quantity::new::<unit>(s.parse()?)`.

## Why

Three magic constants are gone from the codebase:

* `KineticEnergy`'s `450_800` (`grain·(ft/s)² → ft·lb`)
* `WindDeflection`'s `17.6` (`mph·s → in`)
* `LagTime`'s implicit `ft / (ft/s) → s`

The compiler now checks every dimension at every call site.
````

### 10.2 `README.md`

Replace the example block(s) with the prelude-based form shown in §0.2. Add a short "Unit safety" section that links to `MIGRATION.md` and to `docs.rs/uom`.

### 10.3 Crate-level rustdoc

Already added in step 4. Verify it renders:

```bash
cargo doc --no-deps --open
```

**Acceptance gate:**
```bash
cargo doc --no-deps 2>&1 | grep -E 'warning|error' | wc -l     # must print 0
```

Commit: `git add MIGRATION.md README.md && git commit -m "docs: migration guide + README refresh"`.

---

## 11. Step 11 — Pre-Release Verification

Run the full gauntlet:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all
cargo test --doc
cargo build --release
cargo doc --no-deps
cargo package --allow-dirty   # dry-run packaging
```

All must pass with **zero warnings**.

**Binary-size check** (informational, do not block on this):
```bash
ls -l target/release/libballistics_rs.* | tee SIZE_AFTER.txt
diff BASELINE.txt SIZE_AFTER.txt || true
```
Document any size delta > ±10% in the PR description.

---

## 12. Step 12 — Release

1. Update `CHANGELOG.md` (create if absent) with the 0.2.0 entry summarizing §10.1.
2. `git tag v0.2.0-rc.1`.
3. `cargo publish --dry-run`.
4. After human review of the diff: `cargo publish` for the rc.
5. After a soak period (humans decide): bump to `0.2.0` final, tag `v0.2.0`, publish.

---

## Appendix A — Quick Reference For The Agent

**The five things you do at every call site:**

| You see | You write |
|---|---|
| `BulletWeight(150.0)` | `BulletWeight::new::<grain>(150.0)` |
| `Velocity(3000.0)` | `Velocity::new::<foot_per_second>(3000.0)` |
| `WindSpeed(10.0)` | `WindSpeed::new::<mile_per_hour>(10.0)` |
| `Temperature(68.0)` | `Temperature::new::<degree_fahrenheit>(68.0)` |
| `Pressure(29.92)` | `Pressure::new::<inch_of_mercury>(29.92)` |
| `Distance(1000.0)` | `Distance::new::<foot>(1000.0)` |
| `BulletDiameter(0.308)` | `BulletDiameter::new::<inch>(0.308)` |
| `BulletLength(4.0)` | `BulletLength::new::<ratio>(4.0)` |
| `RiflingTwist(10.0)` | `RiflingTwist::new::<ratio>(10.0)` |
| `TimeOfFlight(1.2)` | `TimeOfFlight::new::<second>(1.2)` |
| `LagTime(0.1)` | `LagTime::new::<second>(0.1)` |
| `DragCoefficient(0.223)` | `DragCoefficient::new::<ratio>(0.223)` |
| `FormFactor(1.0)` | `FormFactor::new::<ratio>(1.0)` |
| `GyroscopicStability(1.5)` | `GyroscopicStability::new::<ratio>(1.5)` |
| `result.0` (energy) | `result.get::<foot_pound>()` |
| `result.0` (velocity-ish) | `result.get::<foot_per_second>()` |
| `result.0` (length-ish in inches) | `result.get::<inch>()` |
| `result.0` (MOA) | `result.get::<moa>()` |
| `result.0` (ratio) | `result.get::<ratio>()` |

**The five things you NEVER do:**

1. Never wrap a `uom` quantity in another newtype.
2. Never store a quantity as `f64` and re-wrap repeatedly — pass `Quantity` end-to-end.
3. Never write `q.value` to extract — always `q.get::<unit>()` (the public, unit-checked API).
4. Never `use uom::…` outside `src/units.rs`. All `uom` re-exports flow through that module.
5. Never declare a `const` of a `uom` quantity — use `#[inline] pub fn`.
