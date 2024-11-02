use bon::bon;

use crate::{
    AerodynamicJump, ApertureSightCalibration, BallisticCoefficient, BulletDiameter, BulletLength,
    BulletWeight, Distance, DragCoefficient, FormFactor, GyroscopicStability, KineticEnergy,
    LagTime, Pressure, RiflingTwist, SightCalibration, SpeedOfSound, SpinDrift, Temperature,
    TimeOfFlight, UnitSystem, UnitSystemType, Velocity, VelocityProjection, WindDeflection,
    WindSpeed,
};

fn unit_system_match<T: UnitSystem, R>(
    imperial_expr: impl FnOnce() -> R,
    si_expr: impl FnOnce() -> R,
) -> R {
    match T::unit_system_type() {
        UnitSystemType::Imperial => imperial_expr(),
        UnitSystemType::SI => si_expr(),
    }
}

#[bon]
impl<T: UnitSystem> SpeedOfSound<T> {
    /// Calculates the speed of sound in air given the temperature.
    ///
    /// # Parameters
    /// - `temperature`: The temperature in degrees Fahrenheit or Celsius.
    ///
    /// # Returns
    /// A `SpeedOfSound` instance representing the speed of sound at the given temperature.
    #[builder(finish_fn = solve)]
    pub fn calculate(temperature: Temperature<T>) -> Self {
        unit_system_match::<T, Self>(
            || {
                let temp_f = temperature.value();
                SpeedOfSound::new(49.0223 * (temp_f + 459.67).sqrt())
            },
            || {
                let temp_c = temperature.value();
                let temp_k = temp_c + 273.15;
                SpeedOfSound::new(331.3 * (temp_k / 273.15).sqrt())
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> KineticEnergy<T> {
    /// Calculates the kinetic energy of a bullet given its weight and velocity.
    ///
    /// # Parameters
    /// - `bullet_weight`: The weight of the bullet.
    /// - `velocity`: The velocity of the bullet.
    ///
    /// # Returns
    /// A `KineticEnergy` instance representing the kinetic energy of the bullet.
    #[builder(finish_fn = solve)]
    pub fn calculate(bullet_weight: BulletWeight<T>, velocity: Velocity<T>) -> Self {
        unit_system_match::<T, Self>(
            || {
                let weight_grains = bullet_weight.value();
                let velocity_ft_s = velocity.value();
                KineticEnergy::new((weight_grains * velocity_ft_s.powi(2)) / 450_800.0)
            },
            || {
                let weight_kg = bullet_weight.value();
                let velocity_m_s = velocity.value();
                KineticEnergy::new(0.5 * weight_kg * velocity_m_s.powi(2))
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> ApertureSightCalibration<T> {
    /// Determines the movement of your point of aim for each click of an aperture
    /// based on the sight radius and the sight movement over 20 clicks (caliper measured).
    ///
    /// # Parameters
    /// - `sight_movement_twenty_clicks`: The sight movement for 20 clicks.
    /// - `sight_radius`: The sight radius.
    ///
    /// # Returns
    /// A `ApertureSightCalibration` instance representing MOA per click.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        sight_movement_twenty_clicks: SightCalibration<T>,
        sight_radius: SightCalibration<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let movement_inches = sight_movement_twenty_clicks.value();
                let radius_inches = sight_radius.value();
                ApertureSightCalibration::new(171.89 * (movement_inches / radius_inches) / 20.0)
            },
            || {
                let movement_mm = sight_movement_twenty_clicks.value();
                let radius_mm = sight_radius.value();
                // Convert mm to inches, then calculate
                let movement_inches = movement_mm / 25.4;
                let radius_inches = radius_mm / 25.4;
                ApertureSightCalibration::new(171.89 * (movement_inches / radius_inches) / 20.0)
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> FormFactor<T> {
    /// Determines the relation of drag between a bullet and a standard bullet.
    ///
    /// # Parameters
    /// - `drag_coefficient`: The drag coefficient of a bullet at some speed.
    /// - `standard_bullet_drag_coefficient`: The drag coefficient of a standard (G1, G7, etc.) bullet at the same speed.
    ///
    /// # Returns
    /// A `FormFactor` instance representing a unitless form factor.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        drag_coefficient: DragCoefficient<T>,
        standard_bullet_drag_coefficient: DragCoefficient<T>,
    ) -> Self {
        FormFactor::new(drag_coefficient.value() / standard_bullet_drag_coefficient.value())
    }
}

#[bon]
impl<T: UnitSystem> VelocityProjection<T> {
    /// Projects the velocity of a second bullet based on the weight and velocity of a first bullet.
    ///
    /// This function uses the square root of the ratio of bullet weights to estimate
    /// the velocity of a second bullet, assuming similar ballistic characteristics.
    ///
    /// # Parameters
    /// - `bullet_weight_1`: The weight of the first bullet.
    /// - `bullet_weight_2`: The weight of the second bullet.
    /// - `bullet_velocity_1`: The velocity of the first bullet.
    ///
    /// # Returns
    /// A `VelocityProjection` instance representing the projected velocity of the second bullet.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight_1: BulletWeight<T>,
        bullet_weight_2: BulletWeight<T>,
        bullet_velocity_1: Velocity<T>,
    ) -> Self {
        let weight_1 = bullet_weight_1.value();
        let weight_2 = bullet_weight_2.value();
        let velocity_1 = bullet_velocity_1.value();
        VelocityProjection::new(velocity_1 * (weight_1 / weight_2).sqrt())
    }
}

#[bon]
impl<T: UnitSystem> LagTime<T> {
    /// Calculates the Lag Time of a bullet.
    ///
    /// Lag Time is the difference between the actual time of flight and
    /// the theoretical time of flight in a vacuum.
    ///
    /// # Parameters
    /// - `actual_time_of_flight`: The actual time of flight of the bullet.
    /// - `distance`: The distance the bullet travels.
    /// - `muzzle_velocity`: The initial velocity of the bullet.
    ///
    /// # Returns
    /// A `LagTime` instance representing the lag time.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        actual_time_of_flight: TimeOfFlight<T>,
        distance: Distance<T>,
        muzzle_velocity: Velocity<T>,
    ) -> Self {
        let vacuum_time_of_flight = distance.value() / muzzle_velocity.value();
        let lag_time = actual_time_of_flight.value() - vacuum_time_of_flight;
        LagTime::new(lag_time)
    }
}

#[bon]
impl<T: UnitSystem> WindDeflection<T> {
    /// Calculates the wind deflection of a bullet.
    ///
    /// This function determines how much a crosswind will deflect a bullet
    /// from its path during flight.
    ///
    /// # Parameters
    /// - `lag_time`: The lag time of the bullet.
    /// - `crosswind_speed`: The speed of the crosswind.
    ///
    /// # Returns
    /// A `WindDeflection` instance representing the wind deflection.
    #[builder(finish_fn = solve)]
    pub fn calculate(lag_time: LagTime<T>, crosswind_speed: WindSpeed<T>) -> Self {
        unit_system_match::<T, Self>(
            || {
                let deflection = 17.6 * crosswind_speed.value() * lag_time.value();
                WindDeflection::new(deflection)
            },
            || {
                let deflection =
                    0.0254 * 17.6 * (crosswind_speed.value() * 2.23694) * lag_time.value();
                WindDeflection::new(deflection)
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> AerodynamicJump<T> {
    /// Calculates the aerodynamic jump (vertical deflection) based on gyroscopic stability and bullet length.
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
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability<T>,
        bullet_length: BulletLength<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let jump = 0.01 * gyro_stability.value() - 0.0024 * bullet_length.value() + 0.032;
                AerodynamicJump::new(jump)
            },
            || {
                let jump = (0.01 * gyro_stability.value() - 0.0024 * bullet_length.value() + 0.032)
                    * 0.0572958;
                AerodynamicJump::new(jump)
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> GyroscopicStability<T> {
    /// Calculates the gyroscopic stability factor of a bullet using Miller's stability formula.
    ///
    /// # Parameters
    /// - `bullet_weight`: The weight of the bullet.
    /// - `rifling_twist`: The rifling twist rate of the barrel.
    /// - `bullet_diameter`: The diameter (caliber) of the bullet.
    /// - `bullet_length`: The length of the bullet in calibers.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the gyroscopic stability factor of the bullet.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight<T>,
        rifling_twist: RiflingTwist<T>,
        bullet_diameter: BulletDiameter<T>,
        bullet_length: BulletLength<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let stability = (30.0 * bullet_weight.value())
                    / (rifling_twist.value().powi(2)
                        * bullet_diameter.value().powi(3)
                        * bullet_length.value()
                        * (1.0 + bullet_length.value().powi(2)));
                GyroscopicStability::new(stability)
            },
            || {
                let weight_grains = bullet_weight.value() * 15432.3584; // Convert kg to grains
                let diameter_inches = bullet_diameter.value() * 39.3701; // Convert meters to inches
                let stability = (30.0 * weight_grains)
                    / (rifling_twist.value().powi(2)
                        * diameter_inches.powi(3)
                        * bullet_length.value()
                        * (1.0 + bullet_length.value().powi(2)));
                GyroscopicStability::new(stability)
            },
        )
    }

    /// Applies a velocity correction to the gyroscopic stability factor.
    ///
    /// # Parameters
    /// - `muzzle_velocity`: The muzzle velocity of the bullet.
    /// - `gyro_stability`: The initial gyroscopic stability factor.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the corrected gyroscopic stability factor.
    #[builder(finish_fn = solve)]
    pub fn velocity_correction(
        muzzle_velocity: Velocity<T>,
        gyro_stability: GyroscopicStability<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let corrected =
                    gyro_stability.value() * (muzzle_velocity.value() / 2800.0).powf(1.0 / 3.0);
                GyroscopicStability::new(corrected)
            },
            || {
                let velocity_fps = muzzle_velocity.value() * 3.28084; // Convert m/s to ft/s
                let corrected = gyro_stability.value() * (velocity_fps / 2800.0).powf(1.0 / 3.0);
                GyroscopicStability::new(corrected)
            },
        )
    }

    /// Applies an atmospheric correction to the gyroscopic stability factor.
    ///
    /// # Parameters
    /// - `air_temp`: The air temperature.
    /// - `air_pressure`: The air pressure.
    /// - `gyro_stability`: The initial gyroscopic stability factor.
    ///
    /// # Returns
    /// A `GyroscopicStability` instance representing the corrected gyroscopic stability factor.
    #[builder(finish_fn = solve)]
    pub fn atmospheric_correction(
        air_temp: Temperature<T>,
        air_pressure: Pressure<T>,
        gyro_stability: GyroscopicStability<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let corrected = gyro_stability.value()
                    * ((air_temp.value() + 460.0) / (59.0 + 460.0)
                        * (29.92 / air_pressure.value()));
                GyroscopicStability::new(corrected)
            },
            || {
                let temp_f = air_temp.value() * 9.0 / 5.0 + 32.0; // Convert °C to °F
                let pressure_inhg = air_pressure.value() / 33.8639; // Convert hPa to inHg
                let corrected = gyro_stability.value()
                    * ((temp_f + 460.0) / (59.0 + 460.0) * (29.92 / pressure_inhg));
                GyroscopicStability::new(corrected)
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> SpinDrift<T> {
    /// Calculates the spin drift of a bullet.
    ///
    /// Spin drift is the lateral deviation of a bullet's trajectory due to the gyroscopic effects
    /// of the bullet's spin. This function calculates the spin drift based on the gyroscopic stability
    /// factor and the actual time of flight.
    ///
    /// # Parameters
    /// - `gyro_stability`: The gyroscopic stability factor of the bullet.
    /// - `actual_time_of_flight`: The actual time of flight of the bullet.
    ///
    /// # Returns
    /// A `SpinDrift` instance representing the calculated spin drift of the bullet.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability<T>,
        actual_time_of_flight: TimeOfFlight<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let drift = 1.25
                    * (gyro_stability.value() + 1.2)
                    * actual_time_of_flight.value().powf(1.83);
                SpinDrift::new(drift)
            },
            || {
                let drift = 1.25
                    * (gyro_stability.value() + 1.2)
                    * actual_time_of_flight.value().powf(1.83);
                SpinDrift::new(drift * 0.0254) // Convert inches to meters
            },
        )
    }
}

#[bon]
impl<T: UnitSystem> BallisticCoefficient<T> {
    /// Calculates the ballistic coefficient of a bullet.
    ///
    /// The ballistic coefficient (BC) is a measure of a bullet's ability to overcome air resistance in flight.
    /// It is calculated using the bullet's weight, diameter, and form factor.
    ///
    /// # Parameters
    /// - `bullet_weight`: The weight of the bullet.
    /// - `bullet_diameter`: The diameter (caliber) of the bullet.
    /// - `form_factor`: The form factor of the bullet, which is a unitless number that describes the bullet's shape.
    ///
    /// # Returns
    /// A `BallisticCoefficient` instance representing the ballistic coefficient of the bullet.
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight<T>,
        bullet_diameter: BulletDiameter<T>,
        form_factor: FormFactor<T>,
    ) -> Self {
        unit_system_match::<T, Self>(
            || {
                let bc = (bullet_weight.value() / 7000.0)
                    / (bullet_diameter.value().powi(2) * form_factor.value());
                BallisticCoefficient::new(bc)
            },
            || {
                let weight_grains = bullet_weight.value() * 15432.3584; // Convert kg to grains
                let diameter_inches = bullet_diameter.value() * 39.3701; // Convert meters to inches
                let bc = (weight_grains / 7000.0) / (diameter_inches.powi(2) * form_factor.value());
                BallisticCoefficient::new(bc)
            },
        )
    }
}
