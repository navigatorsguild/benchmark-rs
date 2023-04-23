use std::collections::BTreeMap;
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

fn main() -> Result<(), anyhow::Error> {
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
