// Calculate PI by generating random points in a square and counting how many fall in a circle
use rand::Rng;
use signal_hook::consts::signal;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicI64},
        Arc,
    },
    time::Instant,
};
use threadpool::ThreadPool;

// Global thread work amount
const WORK: i64 = 100_000;

fn thread(in_circle: Arc<AtomicI64>) {
    // Use local in_circle to avoid locking slowdown
    let mut local_in_circle: i64 = 0;
    let mut rng = rand::thread_rng();
    for _ in 0..WORK {
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        if x * x + y * y < 1.0 {
            local_in_circle += 1;
        }
    }

    // Update global in_circle
    in_circle.fetch_add(local_in_circle, std::sync::atomic::Ordering::Relaxed);
}

fn main() {
    let thread_count = num_cpus::get();

    println!(
        "--- SlowPI ---\nThreads: {}\nWork per thread: {:e}\n--------------",
        thread_count, WORK
    );

    let start = Instant::now();

    let thread_pool = ThreadPool::new(thread_count);

    // Thread safe i64
    let in_circle = Arc::new(AtomicI64::new(0));

    // Register SIGINT & SIGTERM handlers to exit gracefully
    let exit_flag = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(signal::SIGINT, exit_flag.clone()).unwrap();
    signal_hook::flag::register(signal::SIGTERM, exit_flag.clone()).unwrap();

    let mut total_work: i64 = 0;
    while !exit_flag.load(std::sync::atomic::Ordering::Relaxed) {
        for _ in 0..thread_count {
            let copy = in_circle.clone();
            thread_pool.execute(move || thread(copy));
        }

        thread_pool.join();

        total_work += WORK * thread_count as i64;

        let pi =
            4.0 * in_circle.load(std::sync::atomic::Ordering::Relaxed) as f64 / total_work as f64;

        println!(
            "\nPI ~= {}\nTotal work: {:e}\nElapsed: {:.2?} s",
            pi,
            total_work,
            start.elapsed()
        );
    }
}
