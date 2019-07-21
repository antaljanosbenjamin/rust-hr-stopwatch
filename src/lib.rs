//! Simple stopwatch implementation that can be used for high resolution time measurement.
//! 
//! # Examples
//! ```
//! let mut stopwatch = Stopwatch::new();
//! stopwatch.start()
//! ...
//! stopwatch.
//! ```
use std::time::Duration;
use time;

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
    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(time::precise_time_ns());
        }
    }

    /// Stops the measurement. The elapsed duration can be obtained using 'elapsed()'.
    pub fn stop(&mut self) {
        if self.start_time.is_some() {
            self.elapsed_time =
                self.elapsed_time + (time::precise_time_ns() - self.start_time.take().unwrap());
        }
    }
    
    pub fn reset(&mut self) {
        self.start_time = None;
        self.elapsed_time = 0;
    }

    pub fn reset_and_start(&mut self) {
        self.reset();
        self.start();
    }

    pub fn elapsed(&self) -> Duration {
        match self.start_time {
            Some(t) => Duration::from_nanos(self.elapsed_time + (time::precise_time_ns() - t)),
            None => Duration::from_nanos(self.elapsed_time),
        }
    }
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

}
