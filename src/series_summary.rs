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

    fn csv_headers(&self, with_config: bool) -> String {
        if with_config {
            format!("{}\t{}\t\tconfiguration: {}", "point", RunSummary::csv_headers(), self.config)
        }else {
            format!("{}\t{}", "point", RunSummary::csv_headers())

        }
    }

    pub(crate) fn as_csv(&self, with_headers: bool, with_config: bool) -> Vec<String> {
        let mut result = Vec::new();
        if with_headers {
            result.push(self.csv_headers(with_config));
        }
        for (point, summary) in &self.runs {
            result.push(format!("{}\t{}", point, summary.as_csv())) ;
        }
        result
    }

    pub(crate) fn config(&self) -> String {
        self.config.clone()
    }
}