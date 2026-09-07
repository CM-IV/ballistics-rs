use ballistics_rs::prelude::*;

fn main() {
    // speed of sound
    let t = 59.0_f64;
    let c = 15.0_f64;
    let actual_imperial = SpeedOfSoundCalc::calculate()
        .temperature(Temperature::new::<degree_fahrenheit>(t))
        .solve()
        .get::<foot_per_second>();
    let actual_si = SpeedOfSoundCalc::calculate()
        .temperature(Temperature::new::<degree_celsius>(c))
        .solve()
        .get::<meter_per_second>();
    println!("SPEED OF SOUND 'MURICA: {}", actual_imperial);
    println!("SPEED OF SOUND SI: {}", actual_si);
}
