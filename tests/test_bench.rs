use std::fmt::{Display, Formatter};
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use benchmark_rs::Benchmarks;
use benchmark_rs::BenchmarkComparison;
use benchmark_rs::StopWatch;

#[derive(Clone)]
struct BenchConfig {
    duration: Duration,
}

impl BenchConfig {
    fn new(t: u64) -> BenchConfig {
        let duration = Duration::from_micros(t);
        BenchConfig { duration }
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

impl Display for BenchConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "slowdown: {}", self.duration.as_micros())
    }
}

#[test]
#[should_panic]
fn test_zero_runs() {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("run zero times", zero_runs, BenchConfig::new(0), vec![0], 0, 0).expect("zero runs");
}

fn zero_runs(_stop_watch: &mut StopWatch, _config: BenchConfig, _work: i64) -> Result<(), anyhow::Error> {
    Ok(())
}

#[test]
#[should_panic]
fn test_benchmark_exists() {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("exists", exists, BenchConfig::new(0), vec![0], 1, 1).expect("benchmark exists");
    benchmarks.add("exists", exists, BenchConfig::new(0), vec![0], 1, 1).expect("benchmark exists");
}

fn exists(_stop_watch: &mut StopWatch, _config: BenchConfig, _work: i64) -> Result<(), anyhow::Error> {
    Ok(())
}

#[test]
fn test_analyze_new() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=2).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort n", bench_sort, BenchConfig::new(0), work, 2, 1)?;
    benchmarks.run()?;

    let new_result = benchmarks.analyze(None, 0.0)?;
    assert!(!new_result.new_series().is_empty());
    assert!(new_result.results().is_empty());
    Ok(())
}

#[test]
fn test_analyze_identical() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=3).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort n", bench_sort, BenchConfig::new(0), work.clone(), 2, 1)?;
    benchmarks.add("sort n and rest", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks.run()?;

    let equal_result = benchmarks.analyze(
        Some(benchmarks.summary_as_json()),
        0.0,
    )?;
    assert!(equal_result.new_series().is_empty());
    for (name, benchmark_comparison) in equal_result.results() {
        match benchmark_comparison {
            BenchmarkComparison::Less { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Equal { point, previous, current, change } => {
                assert!(true, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Greater { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
        }
    }
    Ok(())
}

#[test]
fn test_analyze_different_runs_same_configuration() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=3).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort n", bench_sort, BenchConfig::new(0), work.clone(), 30, 1)?;
    benchmarks.add("sort n and rest", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add("sort n", bench_sort, BenchConfig::new(0), work.clone(), 30, 1)?;
    benchmarks2.add("sort n and rest", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    for (name, benchmark_comparison) in different_runs_result.results() {
        match benchmark_comparison {
            BenchmarkComparison::Less { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Equal { point, previous, current, change } => {
                assert!(true, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Greater { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
        }
    }
    Ok(())
}

#[test]
fn test_analyze_different_runs_degradation() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=3).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort n", bench_sort, BenchConfig::new(0), work.clone(), 30, 1)?;
    benchmarks.add("sort n and rest", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add("sort n", bench_sort, BenchConfig::new(10), work.clone(), 30, 1)?;
    benchmarks2.add("sort n and rest", bench_sort, BenchConfig::new(100), work.clone(), 2, 1)?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    for (name, benchmark_comparison) in different_runs_result.results() {
        match benchmark_comparison {
            BenchmarkComparison::Less { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Equal { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Greater { point, previous, current, change } => {
                assert!(true, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
        }
    }
    Ok(())
}

#[test]
fn test_analyze_different_runs_improvement() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=3).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort n", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks.add("sort n and rest", bench_sort, BenchConfig::new(100), work.clone(), 2, 1)?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add("sort n", bench_sort, BenchConfig::new(0), work.clone(), 2, 1)?;
    benchmarks2.add("sort n and rest", bench_sort, BenchConfig::new(10), work.clone(), 2, 1)?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    for (name, benchmark_comparison) in different_runs_result.results() {
        match benchmark_comparison {
            BenchmarkComparison::Less { point, previous, current, change } => {
                assert!(true, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Equal { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
            BenchmarkComparison::Greater { point, previous, current, change } => {
                assert!(false, "name: {name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
            }
        }
    }
    Ok(())
}

#[test]
fn test_csv() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (0..=2).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add("sort-n", bench_sort, BenchConfig::new(0), work, 2, 1)?;
    benchmarks.run()?;

    let _headers = benchmarks.csv_headers();
    // println!("headers: {headers}");

    let _series_csv = benchmarks.summary_as_csv();
    for (_key, _value) in _series_csv {
        // println!("series: {key}");
        // println!("csv: {value:?}");
    }

    for (_key, _value) in benchmarks.configs() {
        // println!("series: {key}");
        // println!("config: {value:?}");
    }

    Ok(())
}

fn bench_sort(stop_watch: &mut StopWatch, config: BenchConfig, work: usize) -> Result<(), anyhow::Error> {
    stop_watch.pause();
    let mut rng = rand::thread_rng();
    let mut v: Vec<u64> = (0..work).map(|_| rng.gen()).collect();
    stop_watch.resume();
    v.sort_by(|x, y| {
        sleep(config.duration());
        x.cmp(y)
    });
    stop_watch.pause();
    Ok(())
}

mod example {
    use std::thread;
    use std::time::Duration;
    use benchmark_rs::{Benchmarks, StopWatch};

    fn example(_stop_watch: &mut StopWatch, _config: &str, work: u64) -> Result<(), anyhow::Error> {
        thread::sleep(Duration::from_millis(work));
        Ok(())
    }

    #[test]
    fn benchmark_example() -> Result<(), anyhow::Error> {
        let mut benchmarks = Benchmarks::new("Example");
        benchmarks.add("A Simple Benchmark", example, "No Configuration", (1..=10).collect(), 2, 1)?;
        benchmarks.run()?;

        let summary = benchmarks.summary_as_json();
        println!("Summary: {summary}");
        Ok(())
    }
}

mod another_example {
    use std::collections::{BTreeMap};
    use std::fmt::{Display, Formatter};
    use std::thread;
    use std::time::Duration;
    use benchmark_rs::{Benchmarks, StopWatch};

    #[derive(Clone)]
    struct Config {
        pub workloads: BTreeMap<u64, Duration>,
    }

    impl Display for Config {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            for k in self.workloads.keys() {
                write!(f, "{k} ")?;
            }
            Ok(())
        }
    }

    fn example(stop_watch: &mut StopWatch, config: Config, work: u64) -> Result<(), anyhow::Error> {
        stop_watch.pause();
        // perform potentially lengthy preparation that will not reflect in the measurement
        let sleep_time = config.workloads.get(&work).unwrap().clone();
        stop_watch.resume();
        // perform measured computation
        thread::sleep(sleep_time);
        stop_watch.pause();
        // perform potentially lengthy cleanup
        Ok(())
    }

    #[test]
    fn another_benchmark_example() -> Result<(), anyhow::Error> {
        let mut benchmarks = Benchmarks::new("Example");
        let workloads: BTreeMap<u64, Duration> = (0..=10).map(|i| (i, Duration::from_millis(i))).collect();
        benchmarks.add("Another Benchmark", example, Config { workloads }, (1..=10).collect(), 2, 1)?;
        benchmarks.run()?;

        let summary = benchmarks.summary_as_json();
        println!("Summary: {summary}");
        let csv_headers = benchmarks.csv_headers();
        let csv_data = benchmarks.summary_as_csv();
        for (k, v) in csv_data {
            println!("Benchmark name: {k}");
            println!("{csv_headers}");
            for line in v {
                println!("{line}")
            }
        }

        // ignore new series
        let analysis_result = benchmarks.analyze(None, 5.0)?;
        println!("Analysis result: {}", analysis_result.to_string());
        // compare results with threshold of 5 percent
        let analysis_result = benchmarks.analyze(Some(summary), 5.0)?;
        println!("Analysis result: {}", analysis_result.to_string());
        Ok(())
    }
}
