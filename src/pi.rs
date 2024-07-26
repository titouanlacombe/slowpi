use crate::constants::WORK;
use rand::Rng;
use rand_pcg::Pcg64;

pub fn pi_ratio_work(rng: &mut Pcg64) -> u16 {
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
