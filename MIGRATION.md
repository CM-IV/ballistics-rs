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

* `WindDeflection`'s `17.6` (`mph·s → in`) — replaced by native uom
  `Velocity × Time = Length` arithmetic.

The compiler now checks every dimension at every call site.

## Types that share an underlying uom quantity

Several domain types alias the same underlying `uom` quantity type. This is
intentional — the builder method names disambiguate them at call sites.

| Underlying uom type | Domain aliases |
|---|---|
| `uom::si::f64::Velocity` | `Velocity`, `SpeedOfSound`, `WindSpeed`, `VelocityProjection` |
| `uom::si::f64::Ratio` | `DragCoefficient`, `FormFactor`, `GyroscopicStability`, `BallisticCoefficient`, `BulletLength` |
| `uom::si::f64::Angle` | `AerodynamicJump`, `ApertureSightCalibration` |
| `uom::si::f64::Length` | `Distance`, `BulletDiameter`, `SpinDrift`, `WindDeflection`, `SightCalibration`, `RiflingTwist` |
| `uom::si::f64::Time` | `TimeOfFlight`, `LagTime` |

For types that share an underlying uom quantity, use the calculation's marker
struct directly (e.g. `SpeedOfSoundCalc::calculate()` instead of
`SpeedOfSound::calculate()`). Only `KineticEnergy` and `LagTime` have
extension traits (`KineticEnergy::calculate()`, `LagTime::calculate()`):
`KineticEnergy` uniquely aliases `uom::Energy`, and nothing else defines
`calculate()` on the `uom::Time` base that `LagTime` aliases.

## Deprecated constants

The old `STANDARD_GRAVITY`, `SPEED_OF_SOUND_SEA_LEVEL`, `AIR_DENSITY_SEA_LEVEL`,
`STANDARD_PRESSURE`, and `STANDARD_TEMPERATURE` constants are still available
as functions with a deprecation warning. They will be removed in 0.3.

## 0.2.1: twist-unit fix

`GyroscopicStabilityCalc::calculate` now expects `rifling_twist` in **inches
per turn** (the `RiflingTwist` alias moved from `Ratio` to `Length`) and
normalizes it to calibers per turn internally, as Miller's rule requires.
0.1.x/0.2.0 used raw inches-per-turn as calibers, overstating stability by
`1/D²` (≈10.5× for .308). Update call sites from
`RiflingTwist::new::<ratio>(10.0)` to `RiflingTwist::new::<inch>(10.0)`. Restores
the 0.1.x no-arg `LagTime::calculate()` entry point (0.2.0 briefly shipped a
variant that accepted positional arguments and discarded them).
