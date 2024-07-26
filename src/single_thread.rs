use crate::pi::pi_ratio_work;
use rand_pcg::Pcg64;
use std::iter::from_fn;

pub fn single_thread(mut rng: Pcg64) -> impl Iterator<Item = u16> {
    from_fn(move || Some(pi_ratio_work(&mut rng)))
}
