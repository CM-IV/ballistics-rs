# Ballistics Crate — Complete Summary

## Overview

`ballistics_rs` (v0.1.5) is a Rust library for **external ballistics calculations**. It provides a type-safe, builder-pattern API for computing projectile flight properties including speed of sound, kinetic energy, gyroscopic stability, ballistic coefficient, wind deflection, spin drift, and more. The crate is authored by **CM-IV**, licensed under **MIT**, and published on crates.io and docs.rs.

| Attribute       | Value                                      |
|-----------------|--------------------------------------------|
| Crate name      | `ballistics_rs`                            |
| Version         | 0.1.5                                      |
| Edition         | Rust 2021                                  |
| License         | MIT                                        |
| Repository      | [github.com/CM-IV/ballistics-rs](https://github.com/CM-IV/ballistics-rs) |
| Docs            | [docs.rs/ballistics_rs](https://docs.rs/ballistics_rs) |
| Dependencies    | `synonym` 0.1.5, `bon` 2.3.0               |
| Release profile | Optimized for size (`opt-level = "z"`, LTO fat, panic = "abort") |

---

## Architecture

The crate is organized into two modules:

```
ballistics_rs/
├── src/
│   ├── lib.rs          — Re-exports everything from constants and equations
│   ├── constants.rs    — Domain types (newtype wrappers) + physical constants
│   └── equations.rs    — Calculation implementations via builder patterns
├── Cargo.toml
├── README.md
└── LICENSE
```

- **`lib.rs`** is a thin re-export module that makes all types and functions available from the crate root.
- **`constants.rs`** defines all domain types as `pub f64` newtype wrappers (via the `synonym` crate, which derives `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Clone`, `Copy`, `Debug`, `Display`, `FromStr`, `Default`, and `Zero`) plus five physical constants.
- **`equations.rs`** implements all calculations using the `bon` crate's builder pattern (`#[bon]` macro), with each calculation exposed as a `calculate()` method and a `solve()` finisher function.

---

## Domain Types (Newtype Wrappers)

Every physical quantity is represented as a dedicated newtype struct wrapping `f64`. This provides **compile-time type safety**, preventing accidental mixing of incompatible units. All types derive `Synonym` (from the `synonym` crate), giving them `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Clone`, `Copy`, `Debug`, `Display`, `FromStr`, `Default`, and `Zero`.

| Type | Unit | Description |
|------|------|-------------|
| `Gravity` | ft/s² | Acceleration due to gravity |
| `SpeedOfSound` | ft/s | Speed of sound in air |
| `TimeOfFlight` | s | Time of flight (actual or theoretical) |
| `Distance` | ft | Distance traveled |
| `WindSpeed` | mph | Wind speed |
| `SpinDrift` | in | Lateral spin drift |
| `DragCoefficient` | dimensionless | Bullet drag coefficient |
| `RiflingTwist` | calibers/turn | Barrel rifling twist rate |
| `BulletLength` | calibers | Bullet length in calibers |
| `BulletDiameter` | in | Bullet diameter in inches |
| `SightCalibration` | in | Sight movement or sight radius |
| `AirDensity` | lb/ft³ | Air density |
| `LagTime` | s | Lag time (excess flight time vs. vacuum) |
| `WindDeflection` | in | Lateral wind deflection |
| `VelocityProjection` | ft/s | Projected velocity of a similar bullet |
| `ApertureSightCalibration` | MOA/click | MOA per sight click |
| `FormFactor` | dimensionless | Drag ratio vs. standard bullet |
| `AerodynamicJump` | MOA | Vertical jump at muzzle from crosswind |
| `BulletWeight` | grains | Bullet mass in grains |
| `Temperature` | °F | Temperature in Fahrenheit |
| `Pressure` | inHg | Air pressure in inches of mercury |
| `Velocity` | ft/s | Bullet velocity |
| `GyroscopicStability` | dimensionless | Miller stability factor |
| `KineticEnergy` | ft·lb | Kinetic energy |
| `BallisticCoefficient` | dimensionless | G1/G7 ballistic coefficient |

---

## Physical Constants

Five pre-defined constants are exported from `ballistics_rs::constants`:

| Constant | Value | Unit | Description |
|----------|-------|------|-------------|
| `STANDARD_GRAVITY` | 32.174 | ft/s² | Earth's standard gravitational acceleration |
| `SPEED_OF_SOUND_SEA_LEVEL` | 1116.28 | ft/s | Speed of sound at sea level (ICAO standard) |
| `AIR_DENSITY_SEA_LEVEL` | 0.0765 | lb/ft³ | Air density at sea level |
| `STANDARD_PRESSURE` | 29.92 | inHg | ICAO standard atmospheric pressure |
| `STANDARD_TEMPERATURE` | 59.0 | °F | ICAO standard temperature |

---

## Equations / Calculations

All calculations use the `bon` builder pattern: call `::calculate()` to start, chain required parameters, then call `.solve()` to get the typed result.

### 1. Speed of Sound

```rust
SpeedOfSound::calculate().temperature(Temperature(68.0)).solve()
```

**Formula:** `49.0223 × √(T + 459.67)` where T is temperature in °F.

Returns the speed of sound in ft/s at the given air temperature.

---

### 2. Kinetic Energy

```rust
KineticEnergy::calculate()
    .bullet_weight(BulletWeight(150.0))
    .velocity(Velocity(3000.0))
    .solve()
```

**Formula:** `(W × V²) / 450800` where W is bullet weight in grains and V is velocity in ft/s.

Returns kinetic energy in ft·lb. The divisor 450800 converts grains·(ft/s)² to ft·lb.

---

### 3. Aperture Sight Calibration

```rust
ApertureSightCalibration::calculate()
    .sight_movement_twenty_clicks(SightCalibration(0.1))
    .sight_radius(SightCalibration(28.0))
    .solve()
```

**Formula:** `171.89 × (movement / radius)`

Returns MOA per click. The constant 171.89 converts the ratio of inches to minutes of arc (1 MOA ≈ 1/60 degree; 171.89 ≈ 180×60/π).

---

### 4. Form Factor

```rust
FormFactor::calculate()
    .drag_coefficient(DragCoefficient(0.223))
    .standard_bullet_drag_coefficient(DragCoefficient(0.2))
    .solve()
```

**Formula:** `CD_bullet / CD_standard`

Returns a unitless form factor comparing the bullet's drag to a standard reference projectile (G1, G7, etc.) at the same speed.

---

### 5. Velocity Projection

```rust
VelocityProjection::calculate()
    .bullet_weight_1(BulletWeight(150.0))
    .bullet_weight_2(BulletWeight(180.0))
    .bullet_velocity_1(Velocity(3000.0))
    .solve()
```

**Formula:** `V₁ × √(W₁ / W₂)`

Projects the velocity of a second bullet assuming similar ballistic characteristics (same shape, same drag). Heavier bullets will have proportionally lower velocities for equal energy.

---

### 6. Lag Time

```rust
LagTime::calculate()
    .actual_time_of_flight(TimeOfFlight(1.2))
    .distance(Distance(1000.0))
    .muzzle_velocity(Velocity(3000.0))
    .solve()
```

**Formula:** `t_actual − (D / V₀)`

Lag time is the difference between the bullet's actual time of flight and the theoretical time of flight through a vacuum. It quantifies the drag-induced delay.

---

### 7. Wind Deflection

```rust
WindDeflection::calculate()
    .lag_time(LagTime(0.1))
    .crosswind_speed(WindSpeed(10.0))
    .solve()
```

**Formula:** `17.6 × W × lag_time` where W is crosswind speed in mph.

Estimates lateral wind deflection in inches. The constant 17.6 incorporates unit conversions (mph → in/s) and the assumption that the bullet spends lag_time fully exposed to the wind.

---

### 8. Aerodynamic Jump

```rust
AerodynamicJump::calculate()
    .gyro_stability(GyroscopicStability(1.5))
    .bullet_length(BulletLength(4.0))
    .solve()
```

**Formula:** `0.01 × S − 0.0024 × L + 0.032`

Estimates the vertical jump (in MOA) caused by a 1 mph crosswind at the muzzle. Higher gyroscopic stability increases jump; longer bullets reduce it. Empirical formula.

---

### 9. Gyroscopic Stability (Miller's Formula)

This is the most complex calculation, offered in three stages:

#### 9a. Base Stability

```rust
GyroscopicStability::calculate()
    .bullet_weight(BulletWeight(150.0))
    .rifling_twist(RiflingTwist(10.0))
    .bullet_diameter(BulletDiameter(0.308))
    .bullet_length(BulletLength(4.0))
    .solve()
```

**Formula:** `30 × W / (T² × D³ × L × (1 + L²))`

Miller's stability factor at a reference velocity of 2800 ft/s. A value > 1.0 indicates a stable flight; values below ~1.3 are considered marginal.

#### 9b. Velocity Correction

```rust
GyroscopicStability::velocity_correction()
    .muzzle_velocity(Velocity(3000.0))
    .gyro_stability(stability)
    .solve()
```

**Formula:** `S × (V / 2800)^(1/3)`

Adjusts stability for velocities other than the 2800 ft/s reference. Higher velocity → higher stability.

#### 9c. Atmospheric Correction

```rust
GyroscopicStability::atmospheric_correction()
    .air_temp(Temperature(68.0))
    .air_pressure(Pressure(29.92))
    .gyro_stability(velocity_corrected)
    .solve()
```

**Formula:** `S × ((T + 460) / 519) × (29.92 / P)`

Adjusts for air density effects. Higher temperature → lower density → higher stability. Higher pressure → higher density → lower stability.

---

### 10. Spin Drift

```rust
SpinDrift::calculate()
    .gyro_stability(GyroscopicStability(1.5))
    .actual_time_of_flight(TimeOfFlight(1.2))
    .solve()
```

**Formula:** `1.25 × (S + 1.2) × t^1.83`

Lateral drift in inches caused by the gyroscopic precession of a spinning bullet. Higher stability and longer flight times produce more drift.

---

### 11. Ballistic Coefficient

```rust
BallisticCoefficient::calculate()
    .bullet_weight(BulletWeight(150.0))
    .bullet_diameter(BulletDiameter(0.308))
    .form_factor(FormFactor(1.0))
    .solve()
```

**Formula:** `(W / 7000) / (D² × ff)` where W is in grains, D in inches, ff is form factor.

The 7000 converts grains to pounds. BC measures a bullet's ability to overcome air resistance — higher is better. This uses the G1 reference standard.

---

## Design Patterns & Idioms

### Type-Safe Newtypes
Every physical quantity is a distinct type, so the Rust compiler prevents errors like passing a `Temperature` where a `Velocity` is expected.

### Builder Pattern (via `bon`)
All calculations use the `bon` crate's declarative builder macro (`#[bon]`). This provides:
- Fluent, readable API: `.parameter(value).parameter(value).solve()`
- Compile-time validation of required parameters
- Self-documenting code

### Newtype + Synonym Derive
The `synonym` crate derives common traits on all newtypes, enabling:
- Direct `.0` field access for the inner `f64` value
- `Display` for easy printing
- `FromStr` for parsing from strings
- `PartialEq` / `Eq` for comparison
- `PartialOrd` / `Ord` for ordering
- `Clone` / `Copy` for cheap copying
- `Debug` for debugging
- `Default` and `Zero` for numeric operations

---

## Build Configuration

The release profile is optimized for **minimal binary size**:

| Setting | Value | Effect |
|---------|-------|--------|
| `opt-level` | `"z"` | Optimize for size |
| `lto` | `"fat"` | Full link-time optimization |
| `debug` | `0` | No debug info |
| `overflow-checks` | `false` | Disable integer overflow checks |
| `panic` | `"abort"` | Abort on panic (no unwind overhead) |
| `codegen-units` | `1` | Single codegen unit for better optimization |

---

## Typical Use Cases

1. **Ballistics solver applications** — Compose multiple calculations to build a full trajectory solver.
2. **Ammunition reloading tools** — Calculate kinetic energy, ballistic coefficient, and gyroscopic stability for loaded rounds.
3. **Shooting sports aids** — Estimate wind deflection, spin drift, and sight calibration for long-range shooting.
4. **Educational / reference tools** — Understand how bullet properties affect flight behavior.

---

## Example: Full Workflow

```rust
use ballistics_rs::*;

// Given a 150gr bullet, .308" diameter, 4 calibers long, fired from a 1:10 twist barrel
let stability = GyroscopicStability::calculate()
    .bullet_weight(BulletWeight(150.0))
    .rifling_twist(RiflingTwist(10.0))
    .bullet_diameter(BulletDiameter(0.308))
    .bullet_length(BulletLength(4.0))
    .solve();

// Correct for muzzle velocity and atmosphere
let corrected = GyroscopicStability::atmospheric_correction()
    .air_temp(Temperature(68.0))
    .air_pressure(Pressure(29.92))
    .gyro_stability(
        GyroscopicStability::velocity_correction()
            .muzzle_velocity(Velocity(3000.0))
            .gyro_stability(stability)
            .solve()
    )
    .solve();

// Calculate related properties
let bc = BallisticCoefficient::calculate()
    .bullet_weight(BulletWeight(150.0))
    .bullet_diameter(BulletDiameter(0.308))
    .form_factor(FormFactor(0.5))
    .solve();

let energy = KineticEnergy::calculate()
    .bullet_weight(BulletWeight(150.0))
    .velocity(Velocity(3000.0))
    .solve();

let speed_of_sound = SpeedOfSound::calculate()
    .temperature(Temperature(68.0))
    .solve();

println!("Stability: {}", corrected.0);
println!("BC: {}", bc.0);
println!("Energy: {} ft·lb", energy.0);
println!("Speed of sound: {} ft/s", speed_of_sound.0);
```

---

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `synonym` | 0.1.5 | Derives common traits on newtype wrappers |
| `bon` | 2.3.0 | Builder pattern macro for clean, fluent APIs |

---

## License

MIT License — Copyright 2024 CM-IV. See `LICENSE` for full text.
