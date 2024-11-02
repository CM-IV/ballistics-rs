use std::marker::PhantomData;

use derive_more::derive::{AsRef, Display, From};

/// Represents the type of unit system being used.
pub enum UnitSystemType {
    /// Imperial unit system (e.g., feet, pounds, degrees Fahrenheit)
    Imperial,
    /// SI unit system (e.g., meters, kilograms, degrees Celsius)
    SI,
}

/// Trait for defining a unit system.
pub trait UnitSystem {
    /// Returns the type of unit system.
    fn unit_system_type() -> UnitSystemType;
}

/// Represents the SI (International System of Units) unit system.
pub struct SI;

/// Represents the Imperial unit system.
pub struct Imperial;

impl UnitSystem for Imperial {
    fn unit_system_type() -> UnitSystemType {
        UnitSystemType::Imperial
    }
}

impl UnitSystem for SI {
    fn unit_system_type() -> UnitSystemType {
        UnitSystemType::SI
    }
}

/// Gravitational constant.
///
/// For Imperial: ft/s²
///
/// For SI: m/s²
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Gravity: {_0}")]
pub struct Gravity<T: UnitSystem>(f64, PhantomData<T>);

/// Speed of sound given temperature.
///
/// For Imperial: ft/s
///
/// For SI: m/s
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Speed Of Sound: {_0}")]
pub struct SpeedOfSound<T: UnitSystem>(f64, PhantomData<T>);

/// Time of Flight.
///
/// Measured in seconds for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Time Of Flight: {_0}")]
pub struct TimeOfFlight<T: UnitSystem>(f64, PhantomData<T>); // Time is the same in both systems

/// Distance.
///
/// For Imperial: yards or feet
///
/// For SI: meters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Distance: {_0}")]
pub struct Distance<T: UnitSystem>(f64, PhantomData<T>);

/// Wind Speed.
///
/// For Imperial: mph
///
/// For SI: m/s
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Wind Speed: {_0}")]
pub struct WindSpeed<T: UnitSystem>(f64, PhantomData<T>);

/// Spin Drift.
///
/// For Imperial: inches
///
/// For SI: centimeters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Spin Drift: {_0}")]
pub struct SpinDrift<T: UnitSystem>(f64, PhantomData<T>);

/// Drag Coefficient.
///
/// Dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Drag Coefficient: {_0}")]
pub struct DragCoefficient<T: UnitSystem>(f64, PhantomData<T>);

/// Rifling Twist.
///
/// Measured in calibers per turn, dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Rifling Twist: {_0}")]
pub struct RiflingTwist<T: UnitSystem>(f64, PhantomData<T>);

/// Bullet Length.
///
/// Measured in calibers, dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Bullet Length: {_0}")]
pub struct BulletLength<T: UnitSystem>(f64, PhantomData<T>); // In calibers, dimensionless

/// Bullet Diameter.
///
/// For Imperial: inches
///
/// For SI: millimeters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Bullet Diameter: {_0}")]
pub struct BulletDiameter<T: UnitSystem>(f64, PhantomData<T>);

/// Sight Calibration.
///
/// For Imperial: MOA or inches
///
/// For SI: mils or centimeters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Sight Calibration: {_0}")]
pub struct SightCalibration<T: UnitSystem>(f64, PhantomData<T>);

/// Air density.
///
/// For Imperial: lb/ft³
///
/// For SI: kg/m³
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Air Density: {_0}")]
pub struct AirDensity<T: UnitSystem>(f64, PhantomData<T>);

/// Lag time of a bullet in seconds.
///
/// Measured in seconds for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Lag Time: {_0}")]
pub struct LagTime<T: UnitSystem>(f64, PhantomData<T>); // Time is the same in both systems

/// Wind deflection of a bullet.
///
/// For Imperial: inches
///
/// For SI: centimeters
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Wind Deflection: {_0}")]
pub struct WindDeflection<T: UnitSystem>(f64, PhantomData<T>);

/// Velocity projection.
///
/// For Imperial: ft/s
///
/// For SI: m/s
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Velocity Projection: {_0}")]
pub struct VelocityProjection<T: UnitSystem>(f64, PhantomData<T>);

/// Aperture sight calibration value.
///
/// Typically dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Aperture Sight Calibration: {_0}")]
pub struct ApertureSightCalibration<T: UnitSystem>(f64, PhantomData<T>); // Typically dimensionless

/// Form factor of a projectile.
///
/// Dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Form Factor: {_0}")]
pub struct FormFactor<T: UnitSystem>(f64, PhantomData<T>);

/// Aerodynamic jump of a projectile.
///
/// For Imperial: MOA
///
/// For SI: mils
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Aerodynamic Jump: {_0}")]
pub struct AerodynamicJump<T: UnitSystem>(f64, PhantomData<T>);

/// Bullet weight.
///
/// For Imperial: grains
///
/// For SI: grams
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Bullet Weight: {_0}")]
pub struct BulletWeight<T: UnitSystem>(f64, PhantomData<T>);

/// Temperature.
///
/// For Imperial: degrees Fahrenheit
///
/// For SI: degrees Celsius
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Temperature: {_0}")]
pub struct Temperature<T: UnitSystem>(f64, PhantomData<T>);

/// Pressure.
///
/// For Imperial: inHg
///
/// For SI: hPa
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Pressure: {_0}")]
pub struct Pressure<T: UnitSystem>(f64, PhantomData<T>);

/// Velocity.
///
/// For Imperial: ft/s
///
/// For SI: m/s
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Velocity: {_0}")]
pub struct Velocity<T: UnitSystem>(f64, PhantomData<T>);

/// Gyroscopic Stability.
///
/// Dimensionless for both Imperial and SI systems.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Gyroscopic Stability: {_0}")]
pub struct GyroscopicStability<T: UnitSystem>(f64, PhantomData<T>);

/// Kinetic Energy.
///
/// For Imperial: ft⋅lb
///
/// For SI: Joules
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Kinetic Energy: {_0}")]
pub struct KineticEnergy<T: UnitSystem>(f64, PhantomData<T>);

/// Ballistic Coefficient.
///
/// Dimensionless for both Imperial and SI systems.
///
/// Typically, lb/in² for Imperial and kg/m² for SI, but often expressed as a unitless value.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default, Display, AsRef, From)]
#[display("Ballistic Coefficient: {_0}")]
pub struct BallisticCoefficient<T: UnitSystem>(f64, PhantomData<T>);

impl<T: UnitSystem> Gravity<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> SpeedOfSound<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> TimeOfFlight<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> Distance<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> WindSpeed<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> SpinDrift<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> DragCoefficient<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> RiflingTwist<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> BulletLength<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> BulletDiameter<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> SightCalibration<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> AirDensity<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> LagTime<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> WindDeflection<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> VelocityProjection<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
}

impl<T: UnitSystem> ApertureSightCalibration<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> FormFactor<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> AerodynamicJump<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> BulletWeight<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> Temperature<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> Pressure<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> Velocity<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> GyroscopicStability<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> KineticEnergy<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl<T: UnitSystem> BallisticCoefficient<T> {
    pub const fn new(value: f64) -> Self {
        Self(value, PhantomData)
    }
    pub fn value(&self) -> f64 {
        self.0
    }
}

/// Constants
pub const STANDARD_GRAVITY: Gravity<Imperial> = Gravity::new(32.174);
pub const SPEED_OF_SOUND_SEA_LEVEL: SpeedOfSound<Imperial> = SpeedOfSound::new(1116.28);
pub const AIR_DENSITY_SEA_LEVEL: AirDensity<Imperial> = AirDensity::new(0.0765);
pub const STANDARD_PRESSURE: Pressure<Imperial> = Pressure::new(29.92);
pub const STANDARD_TEMPERATURE: Temperature<Imperial> = Temperature::new(59.0);
