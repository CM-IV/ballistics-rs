# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-09-07

### Fixed

- **Breaking:** `GyroscopicStabilityCalc::calculate` now treats `rifling_twist`
  as inches per turn (`RiflingTwist` alias moved from `Ratio` to `Length`) and
  normalizes it to calibers per turn internally, per Miller's rule. 0.1.x used
  raw inches-per-turn as calibers, overstating stability by `1/D²` (≈10.5× for
  .308). README examples updated.
- **Breaking:** `LagTimeExt::calculate` is now a no-arg forwarder like
  `KineticEnergyExt`. The 0.2.0 signature accepted three quantities and
  silently discarded them; callers must set `actual_time_of_flight`,
  `distance`, and `muzzle_velocity` on the returned builder (same chain as
  0.1.x).
- Update dependency versions
- Update Rust edition to 2024

## [0.2.0] - 2026-05-28

### Changed

- **Breaking:** Replaced hand-rolled `synonym`-based newtypes with [`uom`](https://docs.rs/uom) for compile-time dimensional safety.
- **Breaking:** Quantity construction changed from `TypeName(value)` to `TypeName::new::<unit>(value)`.
- **Breaking:** Scalar extraction changed from `.0` field access to `.get::<unit>()`.
- **Breaking:** Constants changed from `const` values to `#[inline]` functions (e.g. `standard_gravity()` instead of `STANDARD_GRAVITY`).
- **Breaking:** For types sharing the same underlying uom quantity (e.g. `Velocity`, `SpeedOfSound`), use the calculation's marker struct directly (e.g. `SpeedOfSoundCalc::calculate()`) instead of the type alias.
- `WindDeflection` now uses native uom arithmetic (`Velocity × Time = Length`) instead of the magic constant `17.6`.
- `LagTime` now uses native uom arithmetic (`Length / Velocity = Time`) instead of manual extraction.

### Added

- New `units` module providing all quantity aliases and imperial unit markers.
- New `prelude` module for one-stop imports (`use ballistics_rs::prelude::*;`).
- Deprecated shims for old constant names (`STANDARD_GRAVITY()`, etc.) — will be removed in 0.3.

### Removed

- `synonym` dependency.
- `Display`, `FromStr`, `Default`, `Zero`, and `Zero` trait derivations from `synonym`.

### See Also

- [MIGRATION.md](MIGRATION.md) for detailed migration instructions.
