use serde::{Deserialize, Serialize};

/// Describe equality or inequality relationship between two runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BenchmarkComparison {
    Less {
        point: String,
        previous: u64,
        current: u64,
        change: f64,
    },
    Equal {
        point: String,
        previous: u64,
        current: u64,
        change: f64,
    },
    Greater {
        point: String,
        previous: u64,
        current: u64,
        change: f64,
    },
}
