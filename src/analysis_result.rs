use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use crate::benchmark_comparison::BenchmarkComparison;

/// Result of the comparison of two benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    name: String,
    new_series: HashSet<String>,
    results: HashMap<String, BenchmarkComparison>,
}

impl AnalysisResult {
    pub(crate) fn new(name: String) -> AnalysisResult {
        AnalysisResult {
            name,
            new_series: Default::default(),
            results: Default::default(),
        }
    }

    pub(crate) fn add_new(&mut self, name: String) {
        self.new_series.insert(name);
    }

    pub(crate) fn add(&mut self, name: String, ordering: BenchmarkComparison) {
        self.results.insert(name, ordering);
    }

    /// Name of the [crate::benchmarks::Benchmarks] suite that was analyzed
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Names of series that are new in the current run
    pub fn new_series(&self) -> &HashSet<String> {
        &self.new_series
    }

    /// [BenchmarkComparison] values for each series
    pub fn results(&self) -> &HashMap<String, BenchmarkComparison> {
        &self.results
    }
}

impl ToString for AnalysisResult {
    /// Produce a pretty printed JSON string of this result
    fn to_string(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
