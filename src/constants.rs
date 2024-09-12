use synonym::Synonym;

/// Gravitational constant (ft/s²)
///
/// This struct represents the gravitational constant, which is the acceleration
/// due to gravity on Earth's surface.
#[derive(Synonym)]
pub struct Gravity(pub f64);

/// Speed of sound given temperature (ft/s)
///
/// This struct represents the speed of sound in air, which varies with temperature.
#[derive(Synonym)]
pub struct SpeedOfSound(pub f64);

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

/// Wind Speed (mph)
///
/// This struct represents the wind speed in miles per hour.
#[derive(Synonym)]
pub struct WindSpeed(pub f64);

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

/// Sight Calibration (in)
///
/// This struct represents either the sight movement for 20 clicks or the sight radius in inches.
#[derive(Synonym)]
pub struct SightCalibration(pub f64);

/// Air density at sea level (lb/ft³)
///
/// This struct represents the the air density in pounds per cubic feet.
#[derive(Synonym)]
pub struct AirDensity(pub f64);

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

/// Bullet mass (grains)
///
/// This struct represents the mass of the bullet in grains.
#[derive(Synonym)]
pub struct BulletMass(pub f64);

/// Temperature (F)
///
/// This struct represents the temperature in Fahrenheit.
#[derive(Synonym)]
pub struct Temperature(pub f64);

/// Pressure (inHg)
///
/// This struct represents air pressure in inches of Mercury
#[derive(Synonym)]
pub struct Pressure(pub f64);

/// Velocity (ft/s)
///
/// This struct represents the bullet velocity in feet per second.
#[derive(Synonym)]
pub struct Velocity(pub f64);

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
