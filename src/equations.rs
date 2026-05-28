//! Ballistics calculations.
//!
//! Each calculation uses the `bon` builder pattern: call the associated
//! `calculate()` (or stage-specific) function, chain the required parameters
//! by name, and call `.solve()` to finish.
//!
//! Inputs and outputs are strongly-typed `uom` quantities — see the
//! [`crate::units`] module for the alias table.

use bon::bon;

use crate::units::*;

// ---------------------------------------------------------------------------
// 1. Speed of Sound
// ---------------------------------------------------------------------------

#[bon]
impl SpeedOfSoundCalc {
    /// `49.0223 × √(T_°F + 459.67)`  →  ft/s
    #[builder(finish_fn = solve)]
    pub fn calculate(temperature: Temperature) -> SpeedOfSound {
        let t_f = temperature.get::<degree_fahrenheit>();
        let fps = 49.0223 * (t_f + 459.67).sqrt();
        SpeedOfSound::new::<foot_per_second>(fps)
    }
}
/// Zero-sized marker type used to host the speed-of-sound builder.
pub struct SpeedOfSoundCalc;

// ---------------------------------------------------------------------------
// 2. Kinetic Energy
// ---------------------------------------------------------------------------

pub struct KineticEnergyCalc;

#[bon]
impl KineticEnergyCalc {
    /// `(W_grains × V_fps²) / 450800`  →  ft·lb
    #[builder(finish_fn = solve)]
    pub fn calculate(bullet_weight: BulletWeight, velocity: Velocity) -> KineticEnergy {
        let w = bullet_weight.get::<grain>();
        let v = velocity.get::<foot_per_second>();
        let ftlb = (w * v * v) / 450_800.0;
        KineticEnergy::new::<foot_pound>(ftlb)
    }
}

// ---------------------------------------------------------------------------
// 3. Aperture Sight Calibration
// ---------------------------------------------------------------------------

pub struct ApertureSightCalibrationCalc;

#[bon]
impl ApertureSightCalibrationCalc {
    /// `171.89 × (movement_in / radius_in)`  →  MOA per click
    #[builder(finish_fn = solve)]
    pub fn calculate(
        sight_movement_twenty_clicks: SightCalibration,
        sight_radius: SightCalibration,
    ) -> ApertureSightCalibration {
        let mvmt = sight_movement_twenty_clicks.get::<inch>();
        let radius = sight_radius.get::<inch>();
        let moa_per_click = 171.89 * (mvmt / radius);
        ApertureSightCalibration::new::<moa>(moa_per_click)
    }
}

// ---------------------------------------------------------------------------
// 4. Form Factor
// ---------------------------------------------------------------------------

pub struct FormFactorCalc;

#[bon]
impl FormFactorCalc {
    /// `CD_bullet / CD_standard`  →  dimensionless
    #[builder(finish_fn = solve)]
    pub fn calculate(
        drag_coefficient: DragCoefficient,
        standard_bullet_drag_coefficient: DragCoefficient,
    ) -> FormFactor {
        let cd = drag_coefficient.get::<ratio>();
        let cd_std = standard_bullet_drag_coefficient.get::<ratio>();
        FormFactor::new::<ratio>(cd / cd_std)
    }
}

// ---------------------------------------------------------------------------
// 5. Velocity Projection
// ---------------------------------------------------------------------------

pub struct VelocityProjectionCalc;

#[bon]
impl VelocityProjectionCalc {
    /// `V₁ × √(W₁ / W₂)`  →  ft/s
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight_1: BulletWeight,
        bullet_weight_2: BulletWeight,
        bullet_velocity_1: Velocity,
    ) -> VelocityProjection {
        let w1 = bullet_weight_1.get::<grain>();
        let w2 = bullet_weight_2.get::<grain>();
        let v1 = bullet_velocity_1.get::<foot_per_second>();
        let v2 = v1 * (w1 / w2).sqrt();
        VelocityProjection::new::<foot_per_second>(v2)
    }
}

// ---------------------------------------------------------------------------
// 6. Lag Time
// ---------------------------------------------------------------------------

pub struct LagTimeCalc;

#[bon]
impl LagTimeCalc {
    /// `t_actual − D / V₀`  →  seconds (native uom: Time − Length / Velocity = Time)
    #[builder(finish_fn = solve)]
    pub fn calculate(
        actual_time_of_flight: TimeOfFlight,
        distance: Distance,
        muzzle_velocity: Velocity,
    ) -> LagTime {
        // Direct uom arithmetic: Length / Velocity = Time. No manual extraction.
        let vacuum_time: TimeOfFlight = distance / muzzle_velocity;
        LagTime::new::<second>(actual_time_of_flight.get::<second>() - vacuum_time.get::<second>())
    }
}

// ---------------------------------------------------------------------------
// 7. Wind Deflection
// ---------------------------------------------------------------------------

pub struct WindDeflectionCalc;

#[bon]
impl WindDeflectionCalc {
    /// `crosswind_speed × lag_time`  →  inches (native uom: Velocity × Time = Length)
    #[builder(finish_fn = solve)]
    pub fn calculate(lag_time: LagTime, crosswind_speed: WindSpeed) -> WindDeflection {
        // Direct uom arithmetic: WindSpeed × Time = Length. No magic constant.
        let length: Distance = crosswind_speed * lag_time;
        WindDeflection::new::<inch>(length.get::<inch>())
    }
}

// ---------------------------------------------------------------------------
// 8. Aerodynamic Jump
// ---------------------------------------------------------------------------

pub struct AerodynamicJumpCalc;

#[bon]
impl AerodynamicJumpCalc {
    /// `0.01·S − 0.0024·L + 0.032`  →  MOA (per 1 mph crosswind)
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability,
        bullet_length: BulletLength,
    ) -> AerodynamicJump {
        let s = gyro_stability.get::<ratio>();
        let l = bullet_length.get::<ratio>();
        AerodynamicJump::new::<moa>(0.01 * s - 0.0024 * l + 0.032)
    }
}

// ---------------------------------------------------------------------------
// 9. Gyroscopic Stability (three stages share a host type)
// ---------------------------------------------------------------------------

pub struct GyroscopicStabilityCalc;

#[bon]
impl GyroscopicStabilityCalc {
    /// 9a. Miller base stability: `30·W / (T²·D³·L·(1+L²))`
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight,
        rifling_twist: RiflingTwist,
        bullet_diameter: BulletDiameter,
        bullet_length: BulletLength,
    ) -> GyroscopicStability {
        let w = bullet_weight.get::<grain>();
        let t = rifling_twist.get::<ratio>();
        let d = bullet_diameter.get::<inch>();
        let l = bullet_length.get::<ratio>();
        let s = (30.0 * w) / (t * t * d * d * d * l * (1.0 + l * l));
        GyroscopicStability::new::<ratio>(s)
    }

    /// 9b. Velocity correction: `S · (V/2800)^(1/3)`
    #[builder(finish_fn = solve)]
    pub fn velocity_correction(
        muzzle_velocity: Velocity,
        gyro_stability: GyroscopicStability,
    ) -> GyroscopicStability {
        let v = muzzle_velocity.get::<foot_per_second>();
        let s = gyro_stability.get::<ratio>();
        GyroscopicStability::new::<ratio>(s * (v / 2800.0).cbrt())
    }

    /// 9c. Atmospheric correction: `S · ((T+460)/519) · (29.92/P)`
    #[builder(finish_fn = solve)]
    pub fn atmospheric_correction(
        air_temp: Temperature,
        air_pressure: Pressure,
        gyro_stability: GyroscopicStability,
    ) -> GyroscopicStability {
        let t = air_temp.get::<degree_fahrenheit>();
        let p = air_pressure.get::<inch_of_mercury>();
        let s = gyro_stability.get::<ratio>();
        GyroscopicStability::new::<ratio>(s * ((t + 460.0) / 519.0) * (29.92 / p))
    }
}

// ---------------------------------------------------------------------------
// 10. Spin Drift
// ---------------------------------------------------------------------------

pub struct SpinDriftCalc;

#[bon]
impl SpinDriftCalc {
    /// `1.25 · (S + 1.2) · t^1.83`  →  inches
    #[builder(finish_fn = solve)]
    pub fn calculate(
        gyro_stability: GyroscopicStability,
        actual_time_of_flight: TimeOfFlight,
    ) -> SpinDrift {
        let s = gyro_stability.get::<ratio>();
        let t = actual_time_of_flight.get::<second>();
        SpinDrift::new::<inch>(1.25 * (s + 1.2) * t.powf(1.83))
    }
}

// ---------------------------------------------------------------------------
// 11. Ballistic Coefficient
// ---------------------------------------------------------------------------

pub struct BallisticCoefficientCalc;

#[bon]
impl BallisticCoefficientCalc {
    /// `(W_grains / 7000) / (D_in² · ff)`
    #[builder(finish_fn = solve)]
    pub fn calculate(
        bullet_weight: BulletWeight,
        bullet_diameter: BulletDiameter,
        form_factor: FormFactor,
    ) -> BallisticCoefficient {
        let w = bullet_weight.get::<grain>();
        let d = bullet_diameter.get::<inch>();
        let ff = form_factor.get::<ratio>();
        BallisticCoefficient::new::<ratio>((w / 7000.0) / (d * d * ff))
    }
}

// ---------------------------------------------------------------------------
// Compatibility extension traits.
//
// Only types with a UNIQUE underlying uom quantity can have an extension
// trait (otherwise multiple traits would define `calculate()` for the same
// type, causing ambiguity).  Types that share an underlying uom alias
// (Velocity, Ratio, Angle, Length) must use their marker struct directly:
//   SpeedOfSoundCalc::calculate()   (not SpeedOfSound::calculate())
//   GyroscopicStabilityCalc::calculate()
//   etc.
//
// KineticEnergy and LagTime are the only types with a unique uom base,
// so they get extension traits.
// ---------------------------------------------------------------------------

/// Old-style entry point for [`KineticEnergyCalc`].
/// Only works because `KineticEnergy` uniquely aliases `uom::Energy`.
pub trait KineticEnergyExt {
    fn calculate() -> KineticEnergyCalcCalculateBuilder;
}
impl KineticEnergyExt for KineticEnergy {
    fn calculate() -> KineticEnergyCalcCalculateBuilder {
        KineticEnergyCalc::calculate()
    }
}

/// Old-style entry point for [`LagTimeCalc`].
/// Only works because `LagTime` uniquely aliases `uom::Time` (TimeOfFlight
/// has no `calculate` method, so no conflict).
pub trait LagTimeExt {
    fn calculate(
        actual_time_of_flight: TimeOfFlight,
        distance: Distance,
        muzzle_velocity: Velocity,
    ) -> LagTimeCalcCalculateBuilder;
}
impl LagTimeExt for LagTime {
    #[allow(unused_variables)]
    fn calculate(
        actual_time_of_flight: TimeOfFlight,
        distance: Distance,
        muzzle_velocity: Velocity,
    ) -> LagTimeCalcCalculateBuilder {
        LagTimeCalc::calculate()
    }
}
