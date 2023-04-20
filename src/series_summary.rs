use serde::{Deserialize, Serialize};
use crate::run_summary::RunSummary;

/// Summary of series of runs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesSummary {
    name: String,
    config: String,
    runs: Vec<(String, RunSummary)>
}

impl SeriesSummary {
    pub(crate) fn new(name: String, config: String) -> SeriesSummary {
        SeriesSummary {
            name,
            config,
            runs: vec![],
        }
    }

    pub(crate) fn add(&mut self, point: String, run_summary: RunSummary) {
        self.runs.push((point, run_summary))
    }

    pub(crate) fn runs(&self) -> &Vec<(String, RunSummary)> {
        &self.runs
    }

    pub(crate) fn csv_headers() -> String {
        format!("{}\t{}", "point", RunSummary::csv_headers())
    }

    pub(crate) fn as_csv(&self) -> Vec<String> {
        let mut result = Vec::new();
        for (point, summary) in &self.runs {
            result.push(format!("{}\t{}", point, summary.as_csv())) ;
        }
        result
    }

    pub(crate) fn config(&self) -> String {
        self.config.clone()
    }
}