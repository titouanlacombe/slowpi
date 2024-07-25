use std::time::Instant;

use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;

use std::f64::consts::PI as REAL_PI;

const WORK: u16 = 50_000;

fn pi_ratio_work(rng: &mut Pcg64Mcg) -> u16 {
    let mut in_circle = 0;

    for _ in 0..WORK {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);

        if x * x + y * y < 1.0 {
            in_circle += 1;
        }
    }

    in_circle
}

pub fn single_thread() {
    println!("Calculating PI:");

    let mut total_work = 0;
    let mut total_in_circle = 0;
    let mut rng = Pcg64Mcg::seed_from_u64(42);

    let timer = Instant::now();

    loop {
        let in_circle = pi_ratio_work(&mut rng);
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
