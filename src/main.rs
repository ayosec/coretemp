extern crate noisy_float;

use noisy_float::prelude::*;

mod hwmon;

fn main() {
    let max_temp = hwmon::load_core_temp()
        .expect("Read hwmon data")
        .into_iter()
        .map(|c| r64(c.input))
        .max()
        .expect("No temp in hwmon");
    println!("{}", max_temp);
}
