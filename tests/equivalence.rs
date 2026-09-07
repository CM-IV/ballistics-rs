//! Numerical-equivalence tests against hand-computed 0.1.x results.
//!
//! Every assertion uses the exact scalar formulas that v0.1.5 used,
//! computed independently in this test file, and compares them against
//! the v0.2.0 typed implementation. They must agree to 1e-9 relative
//! error — confirming the migration is a pure refactor at the
//! arithmetic level.
//!
//! Exception: the gyroscopic-stability base test deviates from the 0.1.x
//! formula on purpose. 0.1.x passed raw twist-in-inches where Miller's rule
//! expects calibers-per-turn (a ~1/D² error); the fix normalizes twist
//! internally, so that test's expected value is recomputed with the
//! corrected formula, and a pinned reference-value test guards the result.

use ballistics_rs::prelude::*;

fn approx_eq(a: f64, b: f64, rel: f64) {
    let denom = a.abs().max(b.abs()).max(1.0);
    assert!(
        (a - b).abs() / denom < rel,
        "values diverge: a={a}, b={b}, rel_err={}",
        (a - b).abs() / denom
    );
}

#[test]
fn speed_of_sound_matches_legacy() {
    let t = 68.0_f64;
    let expected = 49.0223 * (t + 459.67).sqrt();
    let actual = SpeedOfSoundCalc::calculate()
        .temperature(Temperature::new::<degree_fahrenheit>(t))
        .solve()
        .get::<foot_per_second>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn kinetic_energy_matches_legacy() {
    let (w, v) = (150.0_f64, 3000.0_f64);
    let expected = (w * v * v) / 450_800.0;
    let actual = KineticEnergy::calculate()
        .bullet_weight(BulletWeight::new::<grain>(w))
        .velocity(Velocity::new::<foot_per_second>(v))
        .solve()
        .get::<foot_pound>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn aperture_sight_calibration_matches_legacy() {
    let (mvmt, radius) = (0.1_f64, 28.0_f64);
    let expected = 171.89 * (mvmt / radius);
    let actual = ApertureSightCalibrationCalc::calculate()
        .sight_movement_twenty_clicks(SightCalibration::new::<inch>(mvmt))
        .sight_radius(SightCalibration::new::<inch>(radius))
        .solve()
        .get::<moa>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn form_factor_matches_legacy() {
    let (cd, cd_std) = (0.223_f64, 0.2_f64);
    let expected = cd / cd_std;
    let actual = FormFactorCalc::calculate()
        .drag_coefficient(DragCoefficient::new::<ratio>(cd))
        .standard_bullet_drag_coefficient(DragCoefficient::new::<ratio>(cd_std))
        .solve()
        .get::<ratio>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn velocity_projection_matches_legacy() {
    let (w1, w2, v1) = (150.0_f64, 180.0_f64, 3000.0_f64);
    let expected = v1 * (w1 / w2).sqrt();
    let actual = VelocityProjectionCalc::calculate()
        .bullet_weight_1(BulletWeight::new::<grain>(w1))
        .bullet_weight_2(BulletWeight::new::<grain>(w2))
        .bullet_velocity_1(Velocity::new::<foot_per_second>(v1))
        .solve()
        .get::<foot_per_second>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn lag_time_matches_legacy() {
    let (t, d, v) = (1.2_f64, 1000.0_f64, 3000.0_f64);
    let expected = t - d / v;
    let actual = LagTimeCalc::calculate()
        .actual_time_of_flight(TimeOfFlight::new::<second>(t))
        .distance(Distance::new::<foot>(d))
        .muzzle_velocity(Velocity::new::<foot_per_second>(v))
        .solve()
        .get::<second>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn wind_deflection_matches_legacy() {
    let (lag, w) = (0.1_f64, 10.0_f64);
    let expected = 17.6 * w * lag;
    let actual = WindDeflectionCalc::calculate()
        .lag_time(LagTime::new::<second>(lag))
        .crosswind_speed(WindSpeed::new::<mile_per_hour>(w))
        .solve()
        .get::<inch>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn aerodynamic_jump_matches_legacy() {
    let (s, l) = (1.5_f64, 4.0_f64);
    let expected = 0.01 * s - 0.0024 * l + 0.032;
    let actual = AerodynamicJumpCalc::calculate()
        .gyro_stability(GyroscopicStability::new::<ratio>(s))
        .bullet_length(BulletLength::new::<ratio>(l))
        .solve()
        .get::<moa>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn gyroscopic_stability_base_matches_miller_rule() {
    // Intentionally NOT legacy-equal: 0.1.x used raw inches-per-turn as
    // calibers-per-turn. Twist is now inches per turn and is normalized
    // to calibers per turn internally (Miller's rule).
    let (w, t_in, d, l) = (150.0_f64, 10.0_f64, 0.308_f64, 4.0_f64);
    let t_cal = t_in / d;
    let expected = (30.0 * w) / (t_cal * t_cal * d * d * d * l * (1.0 + l * l));
    let actual = GyroscopicStabilityCalc::calculate()
        .bullet_weight(BulletWeight::new::<grain>(w))
        .rifling_twist(RiflingTwist::new::<inch>(t_in))
        .bullet_diameter(BulletDiameter::new::<inch>(d))
        .bullet_length(BulletLength::new::<ratio>(l))
        .solve()
        .get::<ratio>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn gyroscopic_stability_reference_value() {
    // Independently pinned Miller result for the README example
    // (150 gr, 1:10" twist, .308" diameter, 4 calibers long): Sg ≈ 2.1486.
    let actual = GyroscopicStabilityCalc::calculate()
        .bullet_weight(BulletWeight::new::<grain>(150.0))
        .rifling_twist(RiflingTwist::new::<inch>(10.0))
        .bullet_diameter(BulletDiameter::new::<inch>(0.308))
        .bullet_length(BulletLength::new::<ratio>(4.0))
        .solve()
        .get::<ratio>();
    approx_eq(actual, 2.1486, 1e-3);
}

#[test]
fn gyroscopic_stability_velocity_correction_matches_legacy() {
    let (v, s) = (3000.0_f64, 1.5_f64);
    let expected = s * (v / 2800.0).cbrt();
    let actual = GyroscopicStabilityCalc::velocity_correction()
        .muzzle_velocity(Velocity::new::<foot_per_second>(v))
        .gyro_stability(GyroscopicStability::new::<ratio>(s))
        .solve()
        .get::<ratio>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn gyroscopic_stability_atmospheric_correction_matches_legacy() {
    let (t, p, s) = (68.0_f64, 29.92_f64, 1.5_f64);
    let expected = s * ((t + 460.0) / 519.0) * (29.92 / p);
    let actual = GyroscopicStabilityCalc::atmospheric_correction()
        .air_temp(Temperature::new::<degree_fahrenheit>(t))
        .air_pressure(Pressure::new::<inch_of_mercury>(p))
        .gyro_stability(GyroscopicStability::new::<ratio>(s))
        .solve()
        .get::<ratio>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn spin_drift_matches_legacy() {
    let (s, t) = (1.5_f64, 1.2_f64);
    let expected = 1.25 * (s + 1.2) * t.powf(1.83);
    let actual = SpinDriftCalc::calculate()
        .gyro_stability(GyroscopicStability::new::<ratio>(s))
        .actual_time_of_flight(TimeOfFlight::new::<second>(t))
        .solve()
        .get::<inch>();
    approx_eq(actual, expected, 1e-9);
}

#[test]
fn ballistic_coefficient_matches_legacy() {
    let (w, d, ff) = (150.0_f64, 0.308_f64, 1.0_f64);
    let expected = (w / 7000.0) / (d * d * ff);
    let actual = BallisticCoefficientCalc::calculate()
        .bullet_weight(BulletWeight::new::<grain>(w))
        .bullet_diameter(BulletDiameter::new::<inch>(d))
        .form_factor(FormFactor::new::<ratio>(ff))
        .solve()
        .get::<ratio>();
    approx_eq(actual, expected, 1e-9);
}
