use chrono::{DateTime, NaiveDateTime, Utc};
use num_traits::cast::ToPrimitive;
use serde::{Deserialize, Serialize};

/// Result of single workload point run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    name: String,
    ramp_up: usize,
    repeat: usize,
    min_nanos: u64,
    min_sec: f64,
    min_str: String,
    max_nanos: u64,
    max_sec: f64,
    max_str: String,
    median_nanos: u64,
    median_sec: f64,
    median_str: String,
    std_dev: Option<f64>,
    std_dev_sec: Option<f64>,
    std_dev_str: String,
}

impl RunSummary {
    pub(crate) fn new(
        name: String,
        ramp_up: usize,
        repeat: usize,
        min: u64,
        max: u64,
        median: u64,
        std_dev: Option<f64>,
    ) -> RunSummary {
        RunSummary {
            name,
            ramp_up,
            repeat,
            min_nanos: min,
            min_sec: min as f64 / 1e9,
            min_str: Self::format_elapsed_nanos(min),
            max_nanos: max,
            max_sec: max as f64 / 1e9,
            max_str: Self::format_elapsed_nanos(max),
            median_nanos: median,
            median_sec: median as f64 / 1e9,
            median_str: Self::format_elapsed_nanos(median),
            std_dev,
            std_dev_sec: std_dev.map(|x| x / 1e9),
            std_dev_str: Self::format_std_dev_nanos(std_dev),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn min_nanos(&self) -> u64 {
        self.min_nanos
    }

    pub fn max_nanos(&self) -> u64 {
        self.max_nanos
    }

    pub fn median_nanos(&self) -> u64 {
        self.median_nanos
    }

    pub fn std_dev(&self) -> Option<f64> {
        self.std_dev
    }

    fn format_elapsed_nanos(t: u64) -> String {
        let (secs, nsecs) = ((t / 1_000_000_000) as i64, (t % 1_000_000_000) as u32);
        let datetime =
            DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(secs, nsecs).unwrap(), Utc);
        datetime.format("%H:%M:%S.%3f").to_string()
    }

    fn format_std_dev_nanos(std_dev_opt: Option<f64>) -> String {
        match std_dev_opt {
            None => "null".to_string(),
            Some(std_dev) => match std_dev.to_i64() {
                None => "null".to_string(),
                Some(t) => {
                    let (secs, nsecs) = ((t / 1_000_000_000), (t % 1_000_000_000) as u32);
                    let datetime = DateTime::<Utc>::from_utc(
                        NaiveDateTime::from_timestamp_opt(secs, nsecs).unwrap(),
                        Utc,
                    );
                    datetime.format("%H:%M:%S.%3f").to_string()
                }
            },
        }
    }

    pub(crate) fn csv_headers() -> String {
        format!(
            "{},{},{},{},{},{}",
            "ramp_up", "repeat", "min_sec", "max_sec", "median_sec", "std_dev_sec",
        )
    }

    pub(crate) fn as_csv(&self) -> String {
        format!(
            "{},{},{},{},{},{}",
            self.ramp_up,
            self.repeat,
            self.min_sec,
            self.max_sec,
            self.median_sec,
            self.std_dev_sec.unwrap_or(0.0),
        )
    }
}
