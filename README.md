<p align="center">
  <img src="https://ik.imagekit.io/xbkhabiqcy9/img/rustacean_HVroA1Aw0.png?updatedAt=1725704970530" />
</p>
<h1 align="center">Ballistics</h1>

`ballistics_rs` is a Rust crate designed for external ballistics calculations. It provides utilities and equations to help developers create ballistics solver programs. This crate includes functions to calculate the speed of sound, kinetic energy, gyroscopic stability, and ballistic coefficient of projectiles.

## Getting Started

To use `ballistics_rs` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ballistics_rs = "0.2.0"
```

## Usage

All calculations use the `bon` builder pattern: call `::calculate()` on the calculation type, chain parameters by name, then call `.solve()` to get the result.

### Quick Example

```rust
use ballistics_rs::prelude::*;

let energy = KineticEnergy::calculate()
    .bullet_weight(BulletWeight::new::<grain>(150.0))
    .velocity(Velocity::new::<foot_per_second>(3000.0))
    .solve();

println!("Kinetic energy: {:.0} ft·lb", energy.get::<foot_pound>());
```

### Speed of Sound

```rust
use ballistics_rs::prelude::*;

let speed = SpeedOfSoundCalc::calculate()
    .temperature(Temperature::new::<degree_fahrenheit>(68.0))
    .solve();

println!("Speed of sound: {} ft/s", speed.get::<foot_per_second>());
```

### Kinetic Energy

```rust
use ballistics_rs::prelude::*;

let energy = KineticEnergy::calculate()
    .bullet_weight(BulletWeight::new::<grain>(150.0))
    .velocity(Velocity::new::<foot_per_second>(3000.0))
    .solve();

println!("Kinetic energy: {:.0} ft·lb", energy.get::<foot_pound>());
```

### Aperture Sight Calibration

```rust
use ballistics_rs::prelude::*;

let calibration = ApertureSightCalibrationCalc::calculate()
    .sight_movement_twenty_clicks(SightCalibration::new::<inch>(0.1))
    .sight_radius(SightCalibration::new::<inch>(28.0))
    .solve();

println!("MOA per click: {:.2}", calibration.get::<moa>());
```

### Form Factor

```rust
use ballistics_rs::prelude::*;

let form_factor = FormFactorCalc::calculate()
    .drag_coefficient(DragCoefficient::new::<ratio>(0.223))
    .standard_bullet_drag_coefficient(DragCoefficient::new::<ratio>(0.2))
    .solve();

println!("Form factor: {:.4}", form_factor.get::<ratio>());
```

### Velocity Projection

```rust
use ballistics_rs::prelude::*;

let projected_velocity = VelocityProjectionCalc::calculate()
    .bullet_weight_1(BulletWeight::new::<grain>(150.0))
    .bullet_weight_2(BulletWeight::new::<grain>(180.0))
    .bullet_velocity_1(Velocity::new::<foot_per_second>(3000.0))
    .solve();

println!("Projected velocity: {} ft/s", projected_velocity.get::<foot_per_second>());
```

### Lag Time

```rust
use ballistics_rs::prelude::*;

let lag_time = LagTimeCalc::calculate()
    .actual_time_of_flight(TimeOfFlight::new::<second>(1.2))
    .distance(Distance::new::<foot>(1000.0))
    .muzzle_velocity(Velocity::new::<foot_per_second>(3000.0))
    .solve();

println!("Lag time: {:.4} seconds", lag_time.get::<second>());
```

### Wind Deflection

```rust
use ballistics_rs::prelude::*;

let wind_deflection = WindDeflectionCalc::calculate()
    .lag_time(LagTime::new::<second>(0.1))
    .crosswind_speed(WindSpeed::new::<mile_per_hour>(10.0))
    .solve();

println!("Wind deflection: {:.2} inches", wind_deflection.get::<inch>());
```

### Aerodynamic Jump

```rust
use ballistics_rs::prelude::*;

let jump = AerodynamicJumpCalc::calculate()
    .gyro_stability(GyroscopicStability::new::<ratio>(1.5))
    .bullet_length(BulletLength::new::<ratio>(4.0))
    .solve();

println!("Aerodynamic jump: {:.4} MOA", jump.get::<moa>());
```

### Gyroscopic Stability

```rust
use ballistics_rs::prelude::*;

let stability = GyroscopicStabilityCalc::calculate()
    .bullet_weight(BulletWeight::new::<grain>(150.0))
    .rifling_twist(RiflingTwist::new::<ratio>(10.0))
    .bullet_diameter(BulletDiameter::new::<inch>(0.308))
    .bullet_length(BulletLength::new::<ratio>(4.0))
    .solve();

let velocity_corrected = GyroscopicStabilityCalc::velocity_correction()
    .muzzle_velocity(Velocity::new::<foot_per_second>(3000.0))
    .gyro_stability(stability)
    .solve();

let atmospheric_corrected = GyroscopicStabilityCalc::atmospheric_correction()
    .air_temp(Temperature::new::<degree_fahrenheit>(68.0))
    .air_pressure(Pressure::new::<inch_of_mercury>(29.92))
    .gyro_stability(velocity_corrected)
    .solve();

println!("Gyroscopic stability: {:.4}", atmospheric_corrected.get::<ratio>());
```

### Spin Drift

```rust
use ballistics_rs::prelude::*;

let spin_drift = SpinDriftCalc::calculate()
    .gyro_stability(GyroscopicStability::new::<ratio>(1.5))
    .actual_time_of_flight(TimeOfFlight::new::<second>(1.2))
    .solve();

println!("Spin drift: {:.2} inches", spin_drift.get::<inch>());
```

### Ballistic Coefficient

```rust
use ballistics_rs::prelude::*;

let bc = BallisticCoefficientCalc::calculate()
    .bullet_weight(BulletWeight::new::<grain>(150.0))
    .bullet_diameter(BulletDiameter::new::<inch>(0.308))
    .form_factor(FormFactor::new::<ratio>(1.0))
    .solve();

println!("Ballistic coefficient: {:.4}", bc.get::<ratio>());
```

### Constants

```rust
use ballistics_rs::prelude::*;

println!("Speed of sound at sea level: {:.2} ft/s", speed_of_sound_sea_level().get::<foot_per_second>());
println!("Air density at sea level: {:.4} lb/ft³", air_density_sea_level().get::<pound_per_cubic_foot>());
println!("Standard gravity: {:.3} ft/s²", standard_gravity().get::<foot_per_second_squared>());
println!("Standard pressure: {:.2} inHg", standard_pressure().get::<inch_of_mercury>());
println!("Standard temperature: {:.1} °F", standard_temperature().get::<degree_fahrenheit>());
```

## Unit Safety

`ballistics_rs` 0.2 uses [`uom`](https://docs.rs/uom) for compile-time dimensional analysis. The compiler will reject code that mixes incompatible units — for example, passing a `Velocity` where a `Time` is expected will not compile.

```rust
use ballistics_rs::prelude::*;

// This will NOT compile — Velocity cannot be used where LagTime is expected:
// let _ = WindDeflectionCalc::calculate()
//     .lag_time(Velocity::new::<foot_per_second>(10.0))
//     .crosswind_speed(WindSpeed::new::<mile_per_hour>(5.0))
//     .solve();
```

See [MIGRATION.md](MIGRATION.md) for details on migrating from 0.1.x.
