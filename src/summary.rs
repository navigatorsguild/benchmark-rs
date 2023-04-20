use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::series_summary::SeriesSummary;

/// Benchmark summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    name: String,
    created_at: String,
    series: HashMap<String, SeriesSummary>,
}

impl Summary {
    pub(crate) fn new(name: String) -> Summary {
        Summary {
            name,
            created_at: chrono::Utc::now().naive_utc().to_string(),
            series: HashMap::new(),
        }
    }

    /// Get benchmark suite name
    pub fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn add(&mut self, name: String, series_summary: SeriesSummary) {
        self.series.insert(name, series_summary);
    }

    /// Get creation time
    pub fn created_at(&self) -> &String {
        &self.created_at
    }

    /// Get reference to [SeriesSummary] by name
    pub fn series(&self) -> &HashMap<String, SeriesSummary> {
        &self.series
    }
}
