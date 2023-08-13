use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;

#[derive(Clone)]
struct Config {
    // simulate available resources - CPU cores, memory buffers, etc.
    pub resources: u32,
    pub workloads: BTreeMap<u64, Duration>,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let keys: Vec<String> = self.workloads.keys().map(|k| k.to_string()).collect();
        write!(f, "{}", keys.join(", "))
    }
}

fn example(stop_watch: &mut StopWatch, config: Config, work: u64) -> Result<(), anyhow::Error> {
    stop_watch.pause();
    // perform potentially lengthy preparation that will not reflect in the measurement
    // ...
    // fetch the workload definition from configuration associated with the 'work' point
    let sleep_time = config.workloads.get(&work).unwrap().clone();
    // resume the stopwatch to measure the actual work
    stop_watch.resume();
    // perform measured computation
    thread::sleep(sleep_time / config.resources);
    stop_watch.pause();
    // perform potentially lengthy cleanup
    // ...
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let mut benchmarks = Benchmarks::new("benchmark-workloads");
    let workloads: BTreeMap<u64, Duration> =
        (0..=10).map(|i| (i, Duration::from_millis(i))).collect();
    benchmarks.add(
        "benchmark-workload-1",
        example,
        Config {
            resources: 1,
            workloads: workloads.clone(),
        },
        (1..=10).collect(),
        2,
        1,
    )?;

    benchmarks.add(
        "benchmark-workload-2",
        example,
        Config {
            resources: 2,
            workloads: workloads.clone(),
        },
        (1..=10).collect(),
        2,
        1,
    )?;

    benchmarks.run()?;

    let summary = benchmarks.summary_as_json();
    println!("Benchmark summary in JSON format.");
    println!("Summary:");
    println!("{summary}");
    println!();

    println!("Benchmark series in CSV format.");
    let csv_data = benchmarks.summary_as_csv(true, false);
    for (k, v) in csv_data {
        println!("Benchmark name: {k}");
        for line in v {
            println!("{line}")
        }
        println!();
    }

    Ok(())
}
