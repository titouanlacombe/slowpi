use rand::SeedableRng;
use rand_pcg::Pcg64;
use slowpi::{constants::WORK, single_thread::single_thread};
use std::f64::consts::PI as REAL_PI;
use std::time::Instant;

pub fn main() {
    println!("Calculating PI:");

    let mut total_work = 0;
    let mut total_in_circle = 0;
    let rng = Pcg64::seed_from_u64(42);

    let timer = Instant::now();

    for in_circle in single_thread(rng) {
        total_work += WORK as u64;
        total_in_circle += in_circle as u64;

        let pi = 4.0 * total_in_circle as f64 / total_work as f64;

        print!(
            "PI: {:.12} (speed: {:.2e} w/s) (error: {:.2e})\r",
            pi,
            total_work as f64 / timer.elapsed().as_secs_f64(),
            (pi - REAL_PI).abs()
        );
    }
}
