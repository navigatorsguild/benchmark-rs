use std::fmt::{Display, Formatter};
use std::ops::AddAssign;
use std::time::{Duration, Instant};

use chrono::{DateTime, NaiveDateTime, Utc};

/// Measure elapsed time
pub struct StopWatch {
    accumulated: Duration,
    checkpoint: Instant,
    is_stopped: bool,
}

impl StopWatch {
    /// Create a new [StopWatch]
    pub fn new() -> StopWatch {
        StopWatch {
            accumulated: Duration::from_secs(0),
            checkpoint: Instant::now(),
            is_stopped: true,
        }
    }

    /// Start duration measurement
    pub fn start(&mut self) {
        if self.is_stopped {
            self.checkpoint = Instant::now();
            self.is_stopped = false;
        }
    }

    /// Resume duration measurement
    pub fn resume(&mut self) {
        self.start()
    }

    /// Reset duration measurement
    pub fn reset(&mut self) {
        self.accumulated = Duration::from_secs(0);
        self.checkpoint = Instant::now();
        self.is_stopped = true
    }

    /// Stop duration measurement
    pub fn stop(&mut self) {
        if !self.is_stopped {
            self.accumulated.add_assign(Duration::from_nanos(
                self.checkpoint.elapsed().as_nanos() as u64
            ));
            self.is_stopped = true;
        }
    }

    /// Pause duration measurement
    pub fn pause(&mut self) {
        self.stop()
    }

    /// Get accumulated duration
    pub fn accumulated(&self) -> Duration {
        self.accumulated
    }
}

impl Display for StopWatch {
    /// Format accumulated duration as time
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut accumulated = self.accumulated;
        if !self.is_stopped {
            accumulated.add_assign(Duration::from_nanos(
                self.checkpoint.elapsed().as_nanos() as u64
            ));
        }
        let datetime = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(
                accumulated.as_secs() as i64,
                accumulated.subsec_nanos(),
            )
            .unwrap(),
            Utc,
        );
        let formatted_time = datetime.format("%H:%M:%S.%3f").to_string();
        f.write_fmt(format_args!("{}", formatted_time))
    }
}

impl Default for StopWatch {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_pause() {
        let mut stop_watch = StopWatch::new();
        stop_watch.resume();
        thread::sleep(Duration::from_millis(3));
        stop_watch.pause();
        thread::sleep(Duration::from_millis(3));
        stop_watch.resume();
        assert!(stop_watch.accumulated() >= Duration::from_millis(3));
        assert!(stop_watch.accumulated() <= Duration::from_millis(6));
    }

    #[test]
    fn test_stopwatch() {
        let mut stopwatch = StopWatch::new();
        assert_eq!("00:00:00.000", stopwatch.to_string());
        stopwatch.start();
        thread::sleep(Duration::from_millis(3));
        assert!("00:00:00.003".to_string() <= stopwatch.to_string());
        thread::sleep(Duration::from_millis(1000));
        assert!("00:00:01.003".to_string() <= stopwatch.to_string());
    }
}
