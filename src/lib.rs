use std::time::{Duration, Instant};

pub type Iterations = u128;

/// Measures the execution time of a function once and returns the duration.
#[inline]
pub fn bench_once<F, T>(f: F) -> Duration
where
    F: FnOnce() -> T,
{
    let instant = Instant::now();
    f();
    instant.elapsed()
}

/// Measure a function's execution time.
///
/// 'iterations' defines how many measurements there will be.
/// Returns the lowest value, thereby minimizing OS influence on results.
#[inline]
pub fn bench_times<F, T>(iterations: Iterations, mut f: F) -> Option<Duration>
where
    F: FnMut() -> T,
{
    (1..iterations).map(|_| bench_once(&mut f)).min()
}

/// Calculates the number of iterations of `bench_once` that can be executed within a specified time limit.
#[inline]
pub fn calc_iterations(single_run: Duration, total: Duration) -> Iterations {
    total.as_nanos() / single_run.as_nanos()
}
