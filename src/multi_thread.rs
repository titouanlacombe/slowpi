use crate::single_thread::single_thread;
use rand_pcg::Pcg64;
use rayon::prelude::*;

// pub fn multi_thread(rng: Pcg64) -> impl ParallelIterator<Item = u16> {
//     // Create tasks iterator
//     let it = single_thread(rng);

//     // Compute 1000 tasks in parallel
//     for _ in 0..1000 {
//         it.clone();
//     }
// }
