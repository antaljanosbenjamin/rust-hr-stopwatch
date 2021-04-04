//! Simple stopwatch implementation that can be used for high resolution time measurement.
//!
//! # Example
//!
//! ```no_run
//! use hrsw::Stopwatch;
//!
//! let mut stopwatch = Stopwatch::new();
//! stopwatch.start();
//! // do something and get the elapsed time
//! let elapsed = stopwatch.elapsed();
//! // do something other and get the total elapsed time
//! stopwatch.stop();
//! let total_elapsed = stopwatch.elapsed();
//! ```
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    start_time: Option<u64>,
    elapsed_time: u64,
}

impl Stopwatch {
    /// Creates a Stopwatch.
    pub fn new() -> Stopwatch {
        Stopwatch {
            start_time: None,
            elapsed_time: 0,
        }
    }

    /// Creates and immediately starts a Stopwatch.
    pub fn new_started() -> Stopwatch {
        let mut stopwatch = Stopwatch {
            start_time: None,
            elapsed_time: 0,
        };
        stopwatch.start();
        stopwatch
    }

    /// Starts the measurement.
    /// If the stopwatch is already running, then the call has no effect.
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(time::precise_time_ns());
        }
    }

    /// Stops the measurement.
    /// The elapsed duration can be obtained using `elapsed()`. If the stopwatch has never been started or has already been stoped, then the call has no effect.
    pub fn stop(&mut self) {
        if self.start_time.is_some() {
            self.elapsed_time =
                self.elapsed_time + (time::precise_time_ns() - self.start_time.take().unwrap());
        }
    }

    /// Restores the original state of the stopwatch.
    /// If the stopwatch is running, then it will be stoped and the elapsed will be cleared, so it can't be obtained.
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed_time = 0;
    }

    /// Restores the original state of the stopwatch and then starts the measurement.
    /// It is the same as calling `reset()` and `start()` in that sequence.
    pub fn reset_and_start(&mut self) {
        self.reset();
        self.start();
    }

    /// Returns the elapsed time. In case of multiple `start()` and `stop()` the elapsed intervals are accumulated. The elapsed time can be cleared by `reset()` or reset_and_start()`.
    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(t) => Duration::from_nanos(self.elapsed_time + (time::precise_time_ns() - t)),
            None => Duration::from_nanos(self.elapsed_time),
        }
    }

    /// Returns whether the stopwatch is running or not.
    pub fn is_running(&self) -> bool {
        self.start_time.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    static DURATION_TO_USE: Duration = Duration::from_micros(500);
    static TOLERANCE: Duration = Duration::from_millis(100);

    fn assert_eq_dur_with_min(measured: Duration, expected: Duration) {
        assert!(expected <= measured);
        assert!(measured < (expected + TOLERANCE));
    }

    fn assert_eq_with_min(stopwatch: &Stopwatch, duration: Duration) {
        let elapsed = stopwatch.elapsed();
        assert_eq_dur_with_min(elapsed, duration);
    }

    #[test]
    fn simple_start_stop() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        thread::sleep(DURATION_TO_USE);
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
    }

    #[test]
    fn multipe_start() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        thread::sleep(DURATION_TO_USE);
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        thread::sleep(DURATION_TO_USE);
        assert_eq_with_min(&stopwatch, 2 * DURATION_TO_USE);
    }

    #[test]
    fn multipe_start_without_stop() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        thread::sleep(DURATION_TO_USE);
        assert_eq_with_min(&stopwatch, 3 * DURATION_TO_USE);
    }
    #[test]
    fn get_elapsed_multiple_times() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        thread::sleep(DURATION_TO_USE);
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
        thread::sleep(DURATION_TO_USE);
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
        assert_eq!(stopwatch.elapsed(), stopwatch.elapsed());
    }

    #[test]
    fn get_elapsed_without_stop() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        let elapsed = stopwatch.elapsed();
        assert_eq_dur_with_min(elapsed, DURATION_TO_USE);
    }

    #[test]
    fn reset_simple() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
        thread::sleep(DURATION_TO_USE);
        stopwatch.reset();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
    }

    #[test]
    fn reset_without_stop() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.reset();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
    }

    #[test]
    fn reset_and_start_simple() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.reset_and_start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
    }

    #[test]
    fn reset_and_start_after_stop() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        stopwatch.reset_and_start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, DURATION_TO_USE);
    }

    #[test]
    fn reset_and_start_multiple_start() {
        let mut stopwatch = Stopwatch::new();
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.reset_and_start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.start();
        thread::sleep(DURATION_TO_USE);
        stopwatch.stop();
        assert_eq_with_min(&stopwatch, 2 * DURATION_TO_USE);
    }

    #[test]
    fn is_running_simple() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.stop();
        assert!(!stopwatch.is_running());
    }

    #[test]
    fn is_running_multiple_start() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.stop();
        assert!(!stopwatch.is_running());
    }

    #[test]
    fn is_running_after_reset() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        stopwatch.reset();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.stop();
        assert!(!stopwatch.is_running());
    }

    #[test]
    fn is_running_complex_scenario() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.reset();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.reset_and_start();
        assert!(stopwatch.is_running());
    }

    #[test]
    fn is_running_after_reset_and_start() {
        let mut stopwatch = Stopwatch::new();
        assert!(!stopwatch.is_running());
        stopwatch.start();
        stopwatch.reset_and_start();
        assert!(stopwatch.is_running());
        stopwatch.start();
        assert!(stopwatch.is_running());
        stopwatch.stop();
        assert!(!stopwatch.is_running());
    }

}
