
/// Measures the execution time of a given function.
pub mod performance {
    /// Measures the execution time of a given function.
    pub fn exec_time<F>(f: F) -> u128
    where
        F: FnOnce() -> (),
    {
        let start = std::time::Instant::now();
        f();
        start.elapsed().as_micros()
    }
}

