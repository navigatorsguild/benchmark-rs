use benchmark_rs::benchmark_comparison::BenchmarkComparison;
use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use rand::Rng;
use std::fmt::{Display, Formatter};
use std::thread::sleep;
use std::time::Duration;

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
    benchmarks
        .add(
            "run zero times",
            zero_runs,
            BenchConfig::new(0),
            vec![0],
            0,
            0,
        )
        .expect("zero runs");
}

fn zero_runs(
    _stop_watch: &mut StopWatch,
    _config: BenchConfig,
    _work: i64,
) -> Result<(), anyhow::Error> {
    Ok(())
}

#[test]
#[should_panic]
fn test_benchmark_exists() {
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks
        .add("exists", exists, BenchConfig::new(0), vec![0], 1, 1)
        .expect("benchmark exists");
    benchmarks
        .add("exists", exists, BenchConfig::new(0), vec![0], 1, 1)
        .expect("benchmark exists");
}

fn exists(
    _stop_watch: &mut StopWatch,
    _config: BenchConfig,
    _work: i64,
) -> Result<(), anyhow::Error> {
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
    benchmarks.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.run()?;

    let equal_result = benchmarks.analyze(Some(benchmarks.summary_as_json()), 0.0)?;
    assert!(equal_result.new_series().is_empty());
    assert!(!equal_result.equal_series().is_empty());

    Ok(())
}

#[test]
fn test_analyze_different_runs_same_configuration() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (1..=4).map(|i| i * 100 as usize).collect();
    // ramp-up?
    let mut benchmarks0 = Benchmarks::new("Test");
    benchmarks0.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        30,
        1,
    )?;
    benchmarks0.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks0.run()?;

    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        30,
        1,
    )?;
    benchmarks.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        30,
        1,
    )?;
    benchmarks2.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    assert!(!different_runs_result.equal_series().is_empty());
    assert!(different_runs_result.divergent_series().is_empty());

    Ok(())
}

#[test]
fn test_analyze_different_runs_degradation() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (1..=4).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        30,
        1,
    )?;
    benchmarks.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add(
        "sort n",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        30,
        1,
    )?;
    benchmarks2.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(100),
        work.clone(),
        2,
        1,
    )?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    for (series_name, series) in different_runs_result.divergent_series() {
        for (_point, benchmark_comparison) in series {
            match benchmark_comparison {
                BenchmarkComparison::Less {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(false, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
                BenchmarkComparison::Equal {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(false, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
                BenchmarkComparison::Greater {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(true, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
            }
        }
    }
    Ok(())
}

#[test]
fn test_analyze_different_runs_improvement() -> Result<(), anyhow::Error> {
    let work: Vec<usize> = (1..=4).map(|i| i * 100 as usize).collect();
    let mut benchmarks = Benchmarks::new("Test");
    benchmarks.add(
        "sort n",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(100),
        work.clone(),
        2,
        1,
    )?;
    benchmarks.run()?;

    let first_run_result = benchmarks.summary_as_json();

    let mut benchmarks2 = Benchmarks::new("Test");
    benchmarks2.add(
        "sort n",
        bench_sort,
        BenchConfig::new(0),
        work.clone(),
        2,
        1,
    )?;
    benchmarks2.add(
        "sort n and rest",
        bench_sort,
        BenchConfig::new(10),
        work.clone(),
        2,
        1,
    )?;
    benchmarks2.run()?;

    let different_runs_result = benchmarks2.analyze(Some(first_run_result), 10.0)?;
    for (series_name, series) in different_runs_result.divergent_series() {
        for (_point, benchmark_comparison) in series {
            match benchmark_comparison {
                BenchmarkComparison::Less {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(true, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
                BenchmarkComparison::Equal {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(false, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
                BenchmarkComparison::Greater {
                    point,
                    previous,
                    current,
                    change,
                } => {
                    assert!(false, "series_name: {series_name}, point: {point}, previous: {previous}, current: {current}, change: {change}");
                }
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

    let _series_csv = benchmarks.summary_as_csv(true, true);
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

fn bench_sort(
    stop_watch: &mut StopWatch,
    config: BenchConfig,
    work: usize,
) -> Result<(), anyhow::Error> {
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
