<p align="center">
  <img src="https://ik.imagekit.io/xbkhabiqcy9/img/rustacean_HVroA1Aw0.png?updatedAt=1725704970530" />
</p>
<h1 align="center">Ballistics</h1>

`ballistics_rs` is a Rust crate designed for external ballistics calculations. It provides utilities and equations to help developers create ballistics solver programs. This crate includes functions to calculate the speed of sound, kinetic energy, gyroscopic stability, and ballistic coefficient of projectiles.


## Getting Started

To use `ballistics_rs` in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
ballistics_rs = "0.1.3"
```

## Usage

### Speed of Sound

Calculate the speed of sound in air given the temperature:

```rust
use ballistics_rs::{SpeedOfSound, Temperature};

let speed = SpeedOfSound::calculate()
    .temperature(Temperature(68.0))
    .solve();

println!("Speed of sound: {} ft/s", speed.0);
```

### Kinetic Energy

Calculate the kinetic energy of a bullet:

```rust
use ballistics_rs::{KineticEnergy, BulletMass, Velocity};

let energy = KineticEnergy::calculate()
    .bullet_weight(BulletMass(150.0))
    .velocity(Velocity(3000.0))
    .solve();

println!("Kinetic energy: {} ft-lbs", energy.0);
```

### Aperture Sight Calibration

Determine the movement of your point of aim for each click of an aperture:

```rust
use ballistics_rs::{ApertureSightCalibration, SightCalibration};

let calibration = ApertureSightCalibration::calculate()
    .sight_movement_twenty_clicks(SightCalibration(0.1))
    .sight_radius(SightCalibration(28.0))
    .solve();

println!("MOA per click: {}", calibration.0);
```

### Form Factor

Calculate the form factor of a bullet:

```rust
use ballistics_rs::{FormFactor, DragCoefficient};

let form_factor = FormFactor::calculate()
    .drag_coefficient(DragCoefficient(0.223))
    .standard_bullet_drag_coefficient(DragCoefficient(0.2))
    .solve();

println!("Form factor: {}", form_factor.0);
```

### Velocity Projection

Project the velocity of a second bullet based on the weight and velocity of a first bullet:

```rust
use ballistics_rs::{VelocityProjection, BulletMass, Velocity};

let projected_velocity = VelocityProjection::calculate()
    .bullet_weight_1(BulletMass(150.0))
    .bullet_weight_2(BulletMass(180.0))
    .bullet_velocity_1(Velocity(3000.0))
    .solve();

println!("Projected velocity of second bullet: {} ft/s", projected_velocity.0);
```

### Lag Time

Calculate the lag time of a bullet:

```rust
use ballistics_rs::{LagTime, TimeOfFlight, Distance, Velocity};

let lag_time = LagTime::calculate()
    .actual_time_of_flight(TimeOfFlight(1.2))
    .distance(Distance(1000.0))
    .muzzle_velocity(Velocity(3000.0))
    .solve();

println!("Lag time: {} seconds", lag_time.0);
```

### Wind Deflection

Calculate the wind deflection of a bullet:

```rust
use ballistics_rs::{WindDeflection, LagTime, WindSpeed};

let wind_deflection = WindDeflection::calculate()
    .lag_time(LagTime(0.1))
    .crosswind_speed(WindSpeed(10.0))
    .solve();

println!("Wind deflection: {} inches", wind_deflection.0);
```

### Aerodynamic Jump

Calculate the aerodynamic jump of a bullet:

```rust
use ballistics_rs::{AerodynamicJump, GyroscopicStability, BulletLength};

let jump = AerodynamicJump::calculate()
    .gyro_stability(GyroscopicStability(1.5))
    .bullet_length(BulletLength(4.0))
    .solve();

println!("Aerodynamic jump: {} MOA", jump.0);
```

### Gyroscopic Stability

Calculate the gyroscopic stability factor of a bullet:

```rust
use ballistics_rs::{GyroscopicStability, BulletMass, RiflingTwist, BulletDiameter, BulletLength, Velocity, Temperature, Pressure};

let stability = GyroscopicStability::calculate()
    .bullet_mass(BulletMass(150.0))
    .rifling_twist(RiflingTwist(10.0))
    .bullet_diameter(BulletDiameter(0.308))
    .bullet_length(BulletLength(4.0))
    .solve();

let velocity_corrected = GyroscopicStability::velocity_correction()
    .muzzle_velocity(Velocity(3000.0))
    .gyro_stability(stability)
    .solve();

let atmospheric_corrected = GyroscopicStability::atmospheric_correction()
    .air_temp(Temperature(68.0))
    .air_pressure(Pressure(29.92))
    .gyro_stability(velocity_corrected)
    .solve();

println!("Gyroscopic stability factor: {}", atmospheric_corrected.0);
```

### Spin Drift

Calculate the spin drift of a bullet in the direction of rifling twist:

```rust
use ballistics_rs::{GyroscopicStability, TimeOfFlight, SpinDrift};

let spin_drift = SpinDrift::calculate()
    .gyro_stability(GyroscopicStability(1.5))
    .actual_time_of_flight(TimeOfFlight(1.2))
    .solve();

println!("Spin drift: {}", spin_drift.0);
```

### Ballistic Coefficient

Calculate the ballistic coefficient of a bullet:

```rust
use ballistics_rs::{BallisticCoefficient, BulletMass, BulletDiameter, FormFactor};

let bc = BallisticCoefficient::calculate()
    .bullet_mass(BulletMass(150.0))
    .bullet_diameter(BulletDiameter(0.308))
    .form_factor(FormFactor(1.0))
    .solve();

println!("Ballistic coefficient: {}", bc.0);
```

### Constants

The crate also provides several constants for use in calculations:

```rust
use ballistics_rs::constants::{STANDARD_GRAVITY, SPEED_OF_SOUND_SEA_LEVEL, AIR_DENSITY_SEA_LEVEL, STANDARD_PRESSURE, STANDARD_TEMPERATURE};

println!("Speed of Sound at Sea Level: {} ft/s", SPEED_OF_SOUND_SEA_LEVEL.0);
println!("Air Density at Sea Level: {} lb/ft³", AIR_DENSITY_SEA_LEVEL.0);
println!("Standard Gravity: {} ft/s²", STANDARD_GRAVITY.0);
println!("Standard Pressure: {} inHg", STANDARD_PRESSURE.0);
println!("Standard Temperature: {} F", STANDARD_TEMPERATURE.0);
```
