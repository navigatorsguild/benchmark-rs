use std::collections::{HashMap, HashSet};
use std::fmt::Display;

use anyhow::{anyhow, Error};

use crate::analysis_result::AnalysisResult;
use crate::benchmark::Benchmark;
use crate::benchmark_comparison::BenchmarkComparison;
use crate::run_summary::RunSummary;
use crate::series_summary::SeriesSummary;
use crate::stopwatch::StopWatch;
use crate::summary::Summary;

/// Run and analyze a benchmarks suite
///
/// * `C` - configuration
/// * `W` - workload unit
/// * `E` - error type
///
pub struct Benchmarks<C, W, E> where
    C: Clone + Display,
    W: Clone + Display,
    Error: From<E>
{
    name: String,
    names: HashSet<String>,
    benchmarks: Vec<Benchmark<C, W, E>>,
    summaries: HashMap<String, SeriesSummary>,
}


impl<C, W, E> Benchmarks<C, W, E> where
    C: Clone + Display,
    W: Clone + Display,
    Error: From<E>
{
    /// Create a new [Benchmarks]
    ///
    /// * `name` - the name of the benchmark session
    pub fn new(name: &str) -> Benchmarks<C, W, E> {
        Benchmarks {
            name: name.to_string(),
            names: Default::default(),
            benchmarks: vec![],
            summaries: Default::default(),
        }
    }

    /// Run all benchmarks
    pub fn run(&mut self) -> Result<(), Error> {
        for benchmark in &self.benchmarks {
            let summary = benchmark.run()?;
            self.summaries.insert(benchmark.name().clone(), summary);
        }
        Ok(())
    }

    /// Create and add a Benchmark
    ///
    /// * `name` - the name of the benchmark series. The result will be accessible by the name as
    /// a key from the summary.
    /// * `f` - the function that runs the benchmark.
    /// * `config` - the configuration value for this benchmark series
    /// * `work` - workload points vector for this benchmark series. Elements of this vector are
    /// passed to `f` in each iteration
    /// * `repeat` - number of times the benchmark will be repeated
    /// * `ramp_up` - number of times the benchmark will be performed before the measurement is
    /// taken
    ///
    pub fn add(&mut self, name: &str, f: fn(stop_watch: &mut StopWatch, config: C, workload_point: W) -> Result<(), E>, config: C, work: Vec<W>, repeat: usize, ramp_up: usize) -> Result<(), Error> {
        let exists = !self.names.insert(name.to_string());
        if exists {
            Err(anyhow!("Benchmark with identical name exists: {}", name.to_string()))
        } else if repeat == 0 {
            Err(anyhow!("Cannot benchmark 0 runs"))
        } else {
            self.benchmarks.push(Benchmark::new(name.to_string(), f, config, work, repeat, ramp_up));
            Ok(())
        }
    }

    /// Produce [Summary] for all series
    pub fn summary(&self) -> Summary {
        let mut summary = Summary::new(self.name().clone());
        for (name, series_summary) in &self.summaries {
            summary.add(name.clone(), series_summary.clone());
        }
        summary
    }

    /// Produce [Summary] for all series as JSON string.
    pub fn summary_as_json(&self) -> String {
        let summary = self.summary();
        serde_json::to_string_pretty(&summary).unwrap()
    }

    /// Produce headers for [Self::summary_as_csv] method
    pub fn csv_headers(&self) -> String {
        SeriesSummary::csv_headers()
    }

    /// Produce summary for each series as vector of CSV lines. The key for series data is the
    /// series name used in [Self::add] method, values are placed as the headers returned by [Self::csv_headers]
    pub fn summary_as_csv(&self) -> HashMap<String, Vec<String>> {
        let mut result = HashMap::new();
        for (name, summary) in &self.summaries {
            result.insert(name.clone(), summary.as_csv());
        }
        result
    }

    /// Produce description of configurations for each series. The key for series config is the
    /// series name used in [Self::add] method.
    pub fn configs(&self) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for (name, summary) in &self.summaries {
            result.insert(name.clone(), summary.config());
        }
        result
    }

    ///  The benchmarks suite name
    pub fn name(&self) -> &String {
        &self.name
    }

    fn compare_median(point: &String, current: u64, previous: u64, threshold: f64) -> BenchmarkComparison {
        let change = (current as f64 / (previous as f64 / 100.0)) - 100.0;
        let point = point.clone();
        if current == previous {
            BenchmarkComparison::Equal {
                point,
                previous,
                current,
                change,
            }
        } else if change.abs() <= threshold.abs() {
            BenchmarkComparison::Equal {
                point,
                previous,
                current,
                change,
            }
        } else {
            if change < 0.0 {
                BenchmarkComparison::Less {
                    point,
                    previous,
                    current,
                    change,
                }
            } else {
                BenchmarkComparison::Greater {
                    point,
                    previous,
                    current,
                    change,
                }
            }
        }
    }

    fn compare_series(current_series: &Vec<(String, RunSummary)>, previous_series: &Vec<(String, RunSummary)>, threshold: f64) -> Result<BenchmarkComparison, Error> {
        let current_points: Vec<String> = current_series.into_iter()
            .map(|(point, _run_summary)| point.clone())
            .collect();
        let previous_points: Vec<String> = previous_series.into_iter()
            .map(|(point, _run_summary)| point.clone())
            .collect();

        if current_points.is_empty() || previous_points.is_empty() {
            Err(anyhow!("Can compare only non empty series"))
        } else if current_points != previous_points {
            Err(anyhow!("Can compare series with identical points only"))
        } else {
            let mut benchmark_comparison = BenchmarkComparison::Equal {
                point: "".to_string(),
                previous: 0,
                current: 0,
                change: 0.0,
            };
            for (i, (point, current_run_summary)) in current_series.into_iter().enumerate() {
                benchmark_comparison = Self::compare_median(
                    point,
                    current_run_summary.median_nanos(),
                    previous_series[i].1.median_nanos(),
                    threshold,
                );
            }
            Ok(
                benchmark_comparison
            )
        }
    }

    /// Compare the current result against a previous result.
    ///
    /// * `prev_result_string_opt` - a JSON string of the [Summary] of previous run
    /// * `threshold` - threshold used to determine equality.
    pub fn analyze(&self, prev_result_string_opt: Option<String>, threshold: f64) -> Result<AnalysisResult, Error> {
        let current_summary = self.summary();
        let prev_summary = match prev_result_string_opt {
            None => {
                Summary::new(self.name().clone())
            }
            Some(prev_result_string) => {
                serde_json::from_str::<Summary>(prev_result_string.as_str())?
            }
        };
        if current_summary.name() != prev_summary.name() {
            Err(anyhow!("Comparing differently named benchmarks.rs: {} <=> {}", current_summary.name(), prev_summary.name()))
        } else {
            let mut analysis_result = AnalysisResult::new(current_summary.name().clone());
            for (name, current_series_summary) in current_summary.series() {
                match prev_summary.series().get(name) {
                    None => {
                        analysis_result.add_new(name.clone());
                    }
                    Some(prev_series_summary) => {
                        let ordering = Self::compare_series(
                            current_series_summary.runs(),
                            prev_series_summary.runs(),
                            threshold,
                        )?;
                        analysis_result.add(name.clone(), ordering);
                    }
                }
            }
            Ok(analysis_result)
        }
    }
}
