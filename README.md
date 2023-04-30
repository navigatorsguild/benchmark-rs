![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# benchmark-rs

A benchmarking library for Rust libraries

This crate provides measurement facilities for rust libraries. For now only time measurements
are supported. Next step will be to support disk , CPU and memory usage

Each benchmark is repeated `repeat` times for every workload point after it was ramped up for
the same workload point. Any type `W: Clone + Display` can be used to specify workloads. It could
be an integer that specifies the size of the workload, a path to a file or a key that we can use
to fetch the workload from the benchmark configuration. See examples below.

## Issues
Issues are welcome and appreciated. Please submit to https://github.com/navigatorsguild/benchmark-rs/issues

## Examples
A simple benchmark that measures execution time for increasing workloads. In this case the workload is simulated by
by a `u64` value passed to `thread::sleep` function
```rust
use std::thread;
use std::time::Duration;
use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;

fn example(_stop_watch: &mut StopWatch, _config: &str, work: u64) -> Result<(), anyhow::Error> {
    thread::sleep(Duration::from_millis(work));
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("Example");
    benchmarks.add("A Simple Benchmark", example, "No Configuration", (1..=10).collect(), 2, 1)?;
    benchmarks.run()?;

    let summary = benchmarks.summary_as_json();
    println!("Summary: {summary}");
    Ok(())
}
```

A more complex example that shows how to use Benchmark configuration, how to control the
stopwatch from within the benchmark, and how to show and analyze results.
```rust
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;

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

fn main() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("Example");
    let workloads: BTreeMap<u64, Duration> = (0..=10).map(|i| (i, Duration::from_millis(i))).collect();
    benchmarks.add("Another Benchmark", example, Config { workloads }, (1..=10).collect(), 2, 1)?;
    benchmarks.run()?;

    let summary = benchmarks.summary_as_json();
    println!("Summary: {summary}");
    let csv_data = benchmarks.summary_as_csv(true, true);
    for (k, v) in csv_data {
        println!("Benchmark name: {k}");
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
```


License: MIT OR Apache-2.0
