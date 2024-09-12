use bon::bon;

use crate::{
    constants::{GyroscopicStability, KineticEnergy, SpeedOfSound},
    AerodynamicJump, ApertureSightCalibration, BallisticCoefficient, BulletDiameter, BulletLength,
    BulletMass, Distance, DragCoefficient, FormFactor, LagTime, Pressure, RiflingTwist,
    SightCalibration, Temperature, TimeOfFlight, Velocity, VelocityProjection, WindDeflection,
    WindSpeed,
};

#[bon]
impl SpeedOfSound {
    /// Calculates the speed of sound in air given the temperature.
    ///
    /// # Parameters
    /// - `temperature`: The temperature in degrees Fahrenheit.
    ///
    /// # Returns
    /// A `SpeedOfSound` instance representing the speed of sound at the given temperature.
    #[builder]
    pub fn calculate(temperature: Temperature) -> Self {
        SpeedOfSound(49.0223 * (temperature.0 + 459.67).sqrt())
    }
}

#[bon]
impl KineticEnergy {
    /// Calculates the kinetic energy of a bullet given its weight and velocity.
    ///
    /// # Parameters
    /// - `bullet_weight`: The weight of the bullet in grains.
    /// - `velocity`: The velocity of the bullet in feet per second (ft/s).
    ///
    /// # Returns
    /// A `KineticEnergy` instance representing the kinetic energy of the bullet.
    #[builder]
    pub fn calculate(bullet_weight: BulletMass, velocity: Velocity) -> Self {
        KineticEnergy((bullet_weight.0 * velocity.0.powi(2)) / 450800.0)
    }
}

#[bon]
impl ApertureSightCalibration {
    /// Determines the movement of your point of aim for each click of an aperture
    /// based on the sight radius and the sight movement over 20 clicks (caliper measured).
    ///
    /// # Parameters
    /// - `sight_movement_twenty_clicks`: The sight movement for 20 clicks (inches).
    /// - `sight_radius`: The sight radius (inches).
    ///
    /// # Returns
    /// A `ApertureSightCalibration` instance representing MOA per click.
    #[builder]
    pub fn calculate(
        sight_movement_twenty_clicks: SightCalibration,
        sight_radius: SightCalibration,
    ) -> Self {
        ApertureSightCalibration(171.89 * (sight_movement_twenty_clicks.0 / sight_radius.0))
    }
}

#[bon]
impl FormFactor {
    /// Determines the relation of drag between a bullet and a standard bullet.
    ///
    /// # Parameters
    /// - `drag_coefficient`: The drag coefficient of a bullet at some speed.
    /// - `standard_bullet_drag_coefficient`: The drag coefficient of a standard (G1, G7, etc.) bullet at the same speed.
    ///
    /// # Returns
    /// A `FormFactor` instance representing a unitless form factor.
    #[builder]
    pub fn calculate(
        drag_coefficient: DragCoefficient,
        standard_bullet_drag_coefficient: DragCoefficient,
    ) -> Self {
        FormFactor(drag_coefficient.0 / standard_bullet_drag_coefficient.0)
    }
}

#[bon]
impl VelocityProjection {
    /// Projects the velocity of a second bullet based on the weight and velocity of a first bullet.
    ///
    /// This function uses the square root of the ratio of bullet weights to estimate
    /// the velocity of a second bullet, assuming similar ballistic characteristics.
    ///
    /// # Parameters
    /// - `bullet_weight_1`: The weight of the first bullet in grains.
    /// - `bullet_weight_2`: The weight of the second bullet in grains.
    /// - `bullet_velocity_1`: The velocity of the first bullet in feet per second (ft/s).
    ///
    /// # Returns
    /// A `VelocityProjection` instance representing the projected velocity of the second bullet in ft/s.
    #[builder]
    pub fn calculate(
        bullet_weight_1: BulletMass,
        bullet_weight_2: BulletMass,
        bullet_velocity_1: Velocity,
    ) -> Self {
        VelocityProjection(bullet_velocity_1.0 * (bullet_weight_1.0 / bullet_weight_2.0).sqrt())
    }
}

#[bon]
impl LagTime {
    /// Calculates the Lag Time of a bullet.
    ///
    /// Lag Time is the difference between the actual time of flight and
    /// the theoretical time of flight in a vacuum.
    ///
    /// # Parameters
    /// - `actual_time_of_flight`: The actual time of flight of the bullet in seconds.
    /// - `distance`: The distance the bullet travels in feet.
    /// - `muzzle_velocity`: The initial velocity of the bullet in feet per second (ft/s).
    ///
    /// # Returns
    /// A `LagTime` instance representing the lag time in seconds.
    #[builder]
    pub fn calculate(
        actual_time_of_flight: TimeOfFlight,
        distance: Distance,
        muzzle_velocity: Velocity,
    ) -> Self {
        let vacuum_time_of_flight = distance.0 / muzzle_velocity.0;

        let lag_time = actual_time_of_flight.0 - vacuum_time_of_flight;

        LagTime(lag_time)
    }
}

#[bon]
impl WindDeflection {
    /// Calculates the wind deflection of a bullet.
    ///
    /// This function determines how much a crosswind will deflect a bullet
    /// from its path during flight.
    ///
    /// # Parameters
    /// - `lag_time`: The lag time of the bullet in seconds.
    /// - `crosswind_speed`: The speed of the crosswind in miles per hour (mph).
    ///
    /// # Returns
    /// A `WindDeflection` instance representing the wind deflection in inches.
    #[builder]
    pub fn calculate(lag_time: LagTime, crosswind_speed: WindSpeed) -> Self {
        WindDeflection(17.6 * crosswind_speed.0 * lag_time.0)
    }
}

#[bon]
impl AerodynamicJump {
    /// Calculates the aerodynamic jump (vertical deflection in MOA of a 1 MPH crosswind) based
    /// on gyroscopic stability and bullet length.
    ///
    /// This method uses an empirical formula to estimate the aerodynamic jump of a projectile.
    ///
    /// # Parameters
    ///
    /// - `gyro_stability` - The gyroscopic stability factor of the projectile.
    /// - `bullet_length` - The length of the bullet in calibers.
    ///
    /// # Returns
    /// Returns an `AerodynamicJump` instance containing the calculated value.
    #[builder]
    pub fn calculate(gyro_stability: GyroscopicStability, bullet_length: BulletLength) -> Self {
        AerodynamicJump(0.01 * gyro_stability.0 - 0.0024 * bullet_length.0 + 0.032)
    }
}

#[bon]
impl GyroscopicStability {
    /// Calculates the gyroscopic stability factor of a bullet using Miller's stability formula.
    ///
    /// # Parameters
    /// - `bullet_mass`: The mass of the bullet in grains.
    /// - `rifling_twist`: The rifling twist rate of the barrel in calibers per turn.
    /// - `bullet_diameter`: The diameter (caliber) of the bullet in inches.
    /// - `bullet_length`: The length of the bullet in calibers.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the gyroscopic stability factor of the bullet at 2800 ft/s.
    #[builder]
    pub fn calculate(
        bullet_mass: BulletMass,
        rifling_twist: RiflingTwist,
        bullet_diameter: BulletDiameter,
        bullet_length: BulletLength,
    ) -> Self {
        GyroscopicStability(
            (30.0 * bullet_mass.0)
                / (rifling_twist.0.powi(2)
                    * bullet_diameter.0.powi(3)
                    * bullet_length.0
                    * (1.0 + bullet_length.0.powi(2))),
        )
    }

    /// Applies a velocity correction to the gyroscopic stability factor for bullet velocities other than 2800 ft/s.
    ///
    /// This function adjusts the gyroscopic stability factor based on the actual muzzle velocity of the bullet.
    ///
    /// # Parameters
    /// - `muzzle_velocity`: The muzzle velocity of the bullet in feet per second (ft/s).
    /// - `gyro_stability`: The initial gyroscopic stability factor calculated at 2800 ft/s.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the corrected gyroscopic stability factor of the bullet.
    #[builder]
    pub fn velocity_correction(
        muzzle_velocity: Velocity,
        gyro_stability: GyroscopicStability,
    ) -> Self {
        GyroscopicStability((gyro_stability.0) * (muzzle_velocity.0 / 2800.0).powi(1 / 3))
    }

    /// Applies an atmospheric correction to the gyroscopic stability factor accounting for
    /// air temperature and pressure.
    ///
    ///
    /// # Parameters
    /// - `air_temp`: The air temperature in degrees Fahrenheit.
    /// - `air_pressure`: The air pressure in inches of Mercury.
    /// - `gyro_stability`: The initial gyroscopic stability factor calculated at 2800 ft/s.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the corrected gyroscopic stability factor of the bullet.
    #[builder]
    pub fn atmospheric_correction(
        air_temp: Temperature,
        air_pressure: Pressure,
        gyro_stability: GyroscopicStability,
    ) -> Self {
        GyroscopicStability(
            (gyro_stability.0) * ((air_temp.0 + 460.0) / (59.0 + 460.0) * (29.92 / air_pressure.0)),
        )
    }
}

#[bon]
impl BallisticCoefficient {
    /// Calculates the ballistic coefficient of a bullet.
    ///
    /// The ballistic coefficient (BC) is a measure of a bullet's ability to overcome air resistance in flight.
    /// It is calculated using the bullet's mass, diameter, and form factor.
    ///
    /// # Parameters
    /// - `bullet_mass`: The mass of the bullet in grains.
    /// - `bullet_diameter`: The diameter (caliber) of the bullet in inches.
    /// - `form_factor`: The form factor of the bullet, which is a unitless number that describes the bullet's shape.
    ///
    /// # Returns
    /// A `BallisticCoefficient` instance representing the ballistic coefficient of the bullet.
    #[builder]
    pub fn calculate(
        bullet_mass: BulletMass,
        bullet_diameter: BulletDiameter,
        form_factor: FormFactor,
    ) -> Self {
        BallisticCoefficient((bullet_mass.0 / 7000.0) / (bullet_diameter.0.powi(2) * form_factor.0))
    }
}
