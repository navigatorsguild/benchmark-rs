use crate::benchmark_comparison::BenchmarkComparison;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

/// Result of the comparison of two benchmarks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    name: String,
    new_series: HashSet<String>,
    equal_series: HashMap<String, HashMap<String, BenchmarkComparison>>,
    divergent_series: HashMap<String, HashMap<String, BenchmarkComparison>>,
}

impl AnalysisResult {
    pub(crate) fn new(name: String) -> AnalysisResult {
        AnalysisResult {
            name,
            new_series: Default::default(),
            equal_series: Default::default(),
            divergent_series: Default::default(),
        }
    }

    pub(crate) fn add_new(&mut self, name: String) {
        self.new_series.insert(name);
    }

    pub(crate) fn add(&mut self, name: String, comparisons: HashMap<String, BenchmarkComparison>) {
        let equal = comparisons
            .iter()
            .fold(
                true,
                |equal, (_, benchmark_comparison)| match benchmark_comparison {
                    BenchmarkComparison::Less { .. } => false,
                    BenchmarkComparison::Equal { .. } => equal,
                    BenchmarkComparison::Greater { .. } => false,
                },
            );
        if equal {
            self.equal_series.insert(name, comparisons);
        } else {
            self.divergent_series.insert(name, comparisons);
        }
    }

    /// Name of the [crate::benchmarks::Benchmarks] suite that was analyzed
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Names of series that are new in the current run
    pub fn new_series(&self) -> &HashSet<String> {
        &self.new_series
    }

    /// Series that are equal within provided threshold
    pub fn equal_series(&self) -> &HashMap<String, HashMap<String, BenchmarkComparison>> {
        &self.equal_series
    }

    /// Series that are divergent within provided threshold
    pub fn divergent_series(&self) -> &HashMap<String, HashMap<String, BenchmarkComparison>> {
        &self.divergent_series
    }

    /// Series that are divergent within provided threshold
    pub fn results(&self) -> &HashMap<String, HashMap<String, BenchmarkComparison>> {
        self.divergent_series()
    }
}

impl Display for AnalysisResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
