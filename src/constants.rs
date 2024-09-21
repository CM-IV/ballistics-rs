use synonym::Synonym;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum UnitSystem {
    Imperial,
    SI,
}

impl Default for UnitSystem {
    fn default() -> Self {
        UnitSystem::Imperial
    }
}

pub trait UnitConversion {
    fn convert(&self, to: UnitSystem) -> Self;
}

/// Gravitational constant (ft/s²)
///
/// This struct represents the gravitational constant, which is the acceleration
/// due to gravity on Earth's surface.
#[derive(Synonym)]
pub struct Gravity(pub f64);

impl UnitConversion for Gravity {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => Gravity(self.0 * 0.3048), // Convert ft/s² to m/s²
            UnitSystem::Imperial => *self,
        }
    }
}

/// Speed of sound given temperature (ft/s)
///
/// This struct represents the speed of sound in air, which varies with temperature.
#[derive(Synonym)]
pub struct SpeedOfSound(pub f64);

impl UnitConversion for SpeedOfSound {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => SpeedOfSound(self.0 * 0.3048), // Convert ft/s to m/s
            UnitSystem::Imperial => *self,
        }
    }
}

/// Time of Flight (s)
///
/// This struct represents the time of flight (either actual or theoretical) in seconds of the projectile.
#[derive(Synonym)]
pub struct TimeOfFlight(pub f64);

/// Distance (ft)
///
/// This struct represents distance traveled in feet.
#[derive(Synonym)]
pub struct Distance(pub f64);

impl UnitConversion for Distance {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => Distance(self.0 * 0.3048), // Convert ft to m
            UnitSystem::Imperial => *self,
        }
    }
}

/// Wind Speed (mph)
///
/// This struct represents the wind speed in miles per hour.
#[derive(Synonym)]
pub struct WindSpeed(pub f64);

impl UnitConversion for WindSpeed {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => WindSpeed(self.0 * 0.44704), // Convert mph to m/s
            UnitSystem::Imperial => *self,
        }
    }
}

/// Spin Drift (in)
///
/// This struct represents the spin drift in inches in the direction of rifling twist.
#[derive(Synonym)]
pub struct SpinDrift(pub f64);

impl UnitConversion for SpinDrift {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => SpinDrift(self.0 * 0.0254), // Convert in to m
            UnitSystem::Imperial => *self,
        }
    }
}

/// Drag Coefficient
///
/// This struct represents the drag coefficient of a bullet at some speed.
#[derive(Synonym)]
pub struct DragCoefficient(pub f64);

/// Rifling Twist (calibers per turn)
///
/// This struct represents the rifling twist of the barrel in calibers per turn.
#[derive(Synonym)]
pub struct RiflingTwist(pub f64);

/// Bullet Length (calibers)
///
/// This struct represents the bullet's length in calibers.
#[derive(Synonym)]
pub struct BulletLength(pub f64);

/// Bullet Diameter (in)
///
/// This struct represents the diameter (caliber) of the bullet in inches.
#[derive(Synonym)]
pub struct BulletDiameter(pub f64);

impl UnitConversion for BulletDiameter {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => BulletDiameter(self.0 * 0.0254), // Convert in to m
            UnitSystem::Imperial => *self,
        }
    }
}

/// Sight Calibration (in)
///
/// This struct represents either the sight movement for 20 clicks or the sight radius in inches.
#[derive(Synonym)]
pub struct SightCalibration(pub f64);

impl UnitConversion for SightCalibration {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => SightCalibration(self.0 * 0.0254), // Convert in to m
            UnitSystem::Imperial => *self,
        }
    }
}

/// Air density at sea level (lb/ft³)
///
/// This struct represents the the air density in pounds per cubic feet.
#[derive(Synonym)]
pub struct AirDensity(pub f64);

impl UnitConversion for AirDensity {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => AirDensity(self.0 * 16.0185), // Convert lb/ft³ to kg/m³
            UnitSystem::Imperial => *self,
        }
    }
}

/// Lag time of a bullet in seconds (s)
///
/// This struct represents the bullet's lag time, used to determine wind deflection sensitivity.
#[derive(Synonym)]
pub struct LagTime(pub f64);

/// Wind deflection of a bullet in inches (in)
///
/// This struct represents the bullet's wind deflection.
#[derive(Synonym)]
pub struct WindDeflection(pub f64);

/// Get the velocity (ft/s) of a second bullet using the weight and velocity of another bullet.
///
/// This struct represents the second bullet's velocity projection.
#[derive(Synonym)]
pub struct VelocityProjection(pub f64);

impl UnitConversion for VelocityProjection {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => VelocityProjection(self.0 * 0.3048), // Convert ft/s to m/s
            UnitSystem::Imperial => *self,
        }
    }
}

/// Aperture sight calibration value
///
/// This struct represents the calibration value for an aperture sight.
#[derive(Synonym)]
pub struct ApertureSightCalibration(pub f64);

/// Form factor of a projectile
///
/// This struct represents the form factor of a projectile, which is a measure
/// of how streamlined the projectile is. It affects the projectile's aerodynamic properties.
#[derive(Synonym)]
pub struct FormFactor(pub f64);

/// Aerodynamic jump of a projectile
///
/// This struct represents the aerodynamic jump, which is the vertical deflection
/// of a projectile's path as it leaves the muzzle, caused by aerodynamic forces.
#[derive(Synonym)]
pub struct AerodynamicJump(pub f64);

impl UnitConversion for AerodynamicJump {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => AerodynamicJump(self.0 * 0.0254), // Convert in to m
            UnitSystem::Imperial => *self,
        }
    }
}

/// Bullet mass (grains)
///
/// This struct represents the mass of the bullet in grains.
#[derive(Synonym)]
pub struct BulletMass(pub f64);

impl UnitConversion for BulletMass {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => BulletMass(self.0 * 0.0647989), // Convert grains to grams
            UnitSystem::Imperial => *self,
        }
    }
}

/// Temperature (F)
///
/// This struct represents the temperature in Fahrenheit.
#[derive(Synonym)]
pub struct Temperature(pub f64);

impl UnitConversion for Temperature {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => Temperature((self.0 - 32.0) * 5.0 / 9.0), // Convert °F to °C
            UnitSystem::Imperial => *self,
        }
    }
}

/// Pressure (inHg)
///
/// This struct represents air pressure in inches of Mercury
#[derive(Synonym)]
pub struct Pressure(pub f64);

impl UnitConversion for Pressure {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => Pressure(self.0 * 33.8639), // Convert inHg to hPa
            UnitSystem::Imperial => *self,
        }
    }
}

/// Velocity (ft/s)
///
/// This struct represents the bullet velocity in feet per second.
#[derive(Synonym)]
pub struct Velocity(pub f64);

impl UnitConversion for Velocity {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => Velocity(self.0 * 0.3048), // Convert ft/s to m/s
            UnitSystem::Imperial => *self,
        }
    }
}

/// Miller's Stability Formula (dimensionless)
///
/// This struct represents the gyroscopic stability factor of a projectile,
/// calculated using Miller's stability formula.
#[derive(Synonym)]
pub struct GyroscopicStability(pub f64);

/// Kinetic Energy (ft-lb)
///
/// This struct represents the kinetic energy of a projectile, which is the
/// energy it possesses due to its motion.
#[derive(Synonym)]
pub struct KineticEnergy(pub f64);

impl UnitConversion for KineticEnergy {
    fn convert(&self, to: UnitSystem) -> Self {
        match to {
            UnitSystem::SI => KineticEnergy(self.0 * 1.35582), // Convert ft-lb to J
            UnitSystem::Imperial => *self,
        }
    }
}

/// Ballistic Coefficient (dimensionless)
///
/// This struct represents the ballistic coefficient of a projectile, which
/// is a measure of its ability to overcome air resistance in flight.
#[derive(Synonym)]
pub struct BallisticCoefficient(pub f64);

/// Standard gravitational constant (ft/s²)
///
/// This constant represents the standard gravitational acceleration on Earth's
/// surface, which is approximately 32.174 ft/s².
pub const STANDARD_GRAVITY: Gravity = Gravity(32.174);

/// Speed of sound at sea level (ft/s)
///
/// This constant represents the speed of sound in feet per second at sea level.
pub const SPEED_OF_SOUND_SEA_LEVEL: SpeedOfSound = SpeedOfSound(1116.28);

/// Air density at sea level (lb/ft³)
///
/// This constant represents the air density at sea level.
pub const AIR_DENSITY_SEA_LEVEL: AirDensity = AirDensity(0.0765);

/// ICAO definition of standard pressure (inHg)
///
/// This constant represents standard air pressure.
pub const STANDARD_PRESSURE: Pressure = Pressure(29.92);

/// ICAO definition of standard temperature (F)
///
/// This constant represents standard temperature.
pub const STANDARD_TEMPERATURE: Temperature = Temperature(59.0);