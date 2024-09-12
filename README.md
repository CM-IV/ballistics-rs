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
    .call();

println!("Speed of sound: {} ft/s", speed.0);
```

### Kinetic Energy

Calculate the kinetic energy of a bullet:

```rust
use ballistics_rs::{KineticEnergy, BulletMass};

let energy = KineticEnergy::calculate()
    .bullet_weight(BulletMass(150.0))
    .velocity(3000.0)
    .call();

println!("Kinetic energy: {} ft-lbs", energy.0);
```

### Aperture Sight Calibration

Determine the movement of your point of aim for each click of an aperture:

```rust
use ballistics_rs::ApertureSightCalibration;

let calibration = ApertureSightCalibration::calculate()
    .sight_movement_twenty_clicks(0.1)
    .sight_radius(28.0)
    .call();

println!("MOA per click: {}", calibration.0);
```

### Form Factor

Calculate the form factor of a bullet:

```rust
use ballistics_rs::FormFactor;

let form_factor = FormFactor::calculate()
    .drag_coefficient(0.223)
    .standard_bullet_drag_coefficient(0.2)
    .call();

println!("Form factor: {}", form_factor.0);
```

### Velocity Projection

Project the velocity of a second bullet based on the weight and velocity of a first bullet:

```rust
use ballistics_rs::{VelocityProjection, BulletMass};

let projected_velocity = VelocityProjection::calculate()
    .bullet_weight_1(BulletMass(150.0))
    .bullet_weight_2(BulletMass(180.0))
    .bullet_velocity_1(3000.0)
    .call();

println!("Projected velocity of second bullet: {} ft/s", projected_velocity.0);
```

### Lag Time

Calculate the lag time of a bullet:

```rust
use ballistics_rs::{LagTime, MuzzleVelocity};

let lag_time = LagTime::calculate()
    .actual_time_of_flight(1.2)
    .distance(1000.0)
    .muzzle_velocity(MuzzleVelocity(3000.0))
    .call();

println!("Lag time: {} seconds", lag_time.0);
```

### Wind Deflection

Calculate the wind deflection of a bullet:

```rust
use ballistics_rs::{WindDeflection, LagTime};

let wind_deflection = WindDeflection::calculate()
    .lag_time(LagTime(0.1))
    .crosswind_speed(10.0)
    .call();

println!("Wind deflection: {} inches", wind_deflection.0);
```

### Aerodynamic Jump

Calculate the aerodynamic jump of a bullet:

```rust
use ballistics_rs::{AerodynamicJump, GyroscopicStability};

let jump = AerodynamicJump::calculate()
    .gyro_stability(GyroscopicStability(1.5))
    .bullet_length(4.0)
    .call();

println!("Aerodynamic jump: {} MOA", jump.0);
```

### Gyroscopic Stability

Calculate the gyroscopic stability factor of a bullet:

```rust
use ballistics_rs::{GyroscopicStability, BulletMass, MuzzleVelocity, Temperature, Pressure};

let stability = GyroscopicStability::calculate()
    .bullet_mass(BulletMass(150.0))
    .rifling_twist(10.0)
    .bullet_diameter(0.308)
    .bullet_length(4.0)
    .call();

let velocity_corrected = GyroscopicStability::velocity_correction()
    .muzzle_velocity(MuzzleVelocity(3000.0))
    .gyro_stability(stability)
    .call();

let atmospheric_corrected = GyroscopicStability::atmospheric_correction()
    .air_temp(Temperature(68.0))
    .air_pressure(Pressure(29.92))
    .gyro_stability(velocity_corrected)
    .call();

println!("Gyroscopic stability factor: {}", atmospheric_corrected.0);
```

### Ballistic Coefficient

Calculate the ballistic coefficient of a bullet:

```rust
use ballistics_rs::{BallisticCoefficient, BulletMass, FormFactor};

let bc = BallisticCoefficient::calculate()
    .bullet_mass(BulletMass(150.0))
    .bullet_diameter(0.308)
    .form_factor(FormFactor(1.0))
    .call();

println!("Ballistic coefficient: {}", bc.0);
```

### Constants

The crate also provides several constants for use in calculations:

- `STANDARD_GRAVITY`: Standard gravitational constant (32.174 ft/s²).

```rust
use ballistics_rs::constants::STANDARD_GRAVITY;

println!("Standard Gravity: {} ft/s²", STANDARD_GRAVITY.0);
```

- `SPEED_OF_SOUND_SEA_LEVEL`: Speed of sound at sea level (1116.28 ft/s).

```rust
use ballistics_rs::constants::SPEED_OF_SOUND_SEA_LEVEL;

println!("Speed of Sound: {} ft/s", SPEED_OF_SOUND_SEA_LEVEL.0);
```

- `AIR_DENSITY_SEA_LEVEL`: Air density at sea level (0.0765 lb/ft³).

```rust
use ballistics_rs::constants::AIR_DENSITY_SEA_LEVEL;

println!("Air Density: {} lb/ft³", AIR_DENSITY_SEA_LEVEL.0);
```

- `STANDARD_PRESSURE`: ICAO standard air pressure (29.92 inHg).

```rust
use ballistics_rs::constants::STANDARD_PRESSURE;

println!("Standard Pressure: {} inHg", STANDARD_PRESSURE.0);
```

- `STANDARD_TEMPERATURE`: ICAO standard temperature (59.0 F).

```rust
use ballistics_rs::constants::STANDARD_TEMPERATURE;

println!("Standard Temperature: {} F", STANDARD_TEMPERATURE.0);
```
