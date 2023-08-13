use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

pub type GenericTestError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(thiserror::Error, Debug)]
#[error("{message:}")]
pub struct SpecificTestError {
    pub message: String,
}

impl SpecificTestError {
    pub fn new(message: String) -> SpecificTestError {
        SpecificTestError { message }
    }
}

#[derive(Copy, Clone)]
pub struct BenchConfig {}

impl BenchConfig {
    pub fn new() -> BenchConfig {
        BenchConfig {}
    }
}

impl Display for BenchConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("empty")
    }
}

#[test]
fn test_generic_error() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "test",
        generic_error_bench,
        BenchConfig::new(),
        vec![1],
        1,
        1,
    )?;

    benchmarks.run()?;

    benchmarks.analyze(Some(benchmarks.summary_as_json()), 5.0)?;
    Ok(())
}

fn subject() -> Result<(), GenericTestError> {
    Ok(())
}

fn generic_error_bench(
    _stop_watch: &mut StopWatch,
    _config: BenchConfig,
    _work: i64,
) -> Result<(), anyhow::Error> {
    subject().map_err(|e| SpecificTestError::new(format!("{e}")).into())
}

#[test]
fn test_specific_error() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "test",
        specific_error_bench,
        BenchConfig::new(),
        vec![1],
        1,
        1,
    )?;

    benchmarks.run()?;

    benchmarks.analyze(Some(benchmarks.summary_as_json()), 5.0)?;
    Ok(())
}

fn specific_error_bench(
    _stop_watch: &mut StopWatch,
    _config: BenchConfig,
    _work: i64,
) -> Result<(), SpecificTestError> {
    Ok(())
}

#[test]
fn test_io_error() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "test",
        io_error_bench,
        BenchConfig::new(),
        vec!["1".to_string()],
        1,
        1,
    )?;

    benchmarks.run()?;

    benchmarks.analyze(Some(benchmarks.summary_as_json()), 5.0)?;
    Ok(())
}

fn io_error_bench(
    _stop_watch: &mut StopWatch,
    _config: BenchConfig,
    _work: String,
) -> Result<(), std::io::Error> {
    match std::fs::read(PathBuf::from("./tests/fixtures/1.5K/512")) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
