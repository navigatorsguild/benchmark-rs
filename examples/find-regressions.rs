use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::thread;
use std::time::Duration;

use benchmark_rs::benchmarks::Benchmarks;
use benchmark_rs::stopwatch::StopWatch;
use rand::Rng;

#[derive(Clone)]
struct Config {
    // simulate available resources - CPU cores, memory buffers, etc.
    pub resources: u32,
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

fn example(_stop_watch: &mut StopWatch, config: Config, work: u64) -> Result<(), anyhow::Error> {
    let sleep_time = config.workloads.get(&work).unwrap().clone();
    thread::sleep(sleep_time / config.resources);
    Ok(())
}

fn modified_example(
    _stop_watch: &mut StopWatch,
    config: Config,
    work: u64,
) -> Result<(), anyhow::Error> {
    // we introduce a random deviation in modified example to simulate
    // regression introduced by a code change
    let deviation = Duration::from_millis(rand::thread_rng().gen_range(0..10));
    let sleep_time = config.workloads.get(&work).unwrap().clone();
    thread::sleep(sleep_time / config.resources + deviation);
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let mut previous_benchmarks = Benchmarks::new("benchmarks");
    let workloads: BTreeMap<u64, Duration> = (1..=10)
        .map(|i| (i, Duration::from_millis(i * 25)))
        .collect();
    previous_benchmarks.add(
        "benchmark-1",
        example,
        Config {
            resources: 1,
            workloads: workloads.clone(),
        },
        (1..=10).collect(),
        5,
        3,
    )?;
    previous_benchmarks.run()?;
    let previous_summary = previous_benchmarks.summary_as_json();

    let mut current_benchmarks = Benchmarks::new("benchmarks");
    current_benchmarks.add(
        "benchmark-1",
        modified_example,
        Config {
            resources: 1,
            workloads: workloads.clone(),
        },
        (1..=10).collect(),
        5,
        3,
    )?;
    current_benchmarks.add(
        "benchmark-2",
        modified_example,
        Config {
            resources: 2,
            workloads: workloads.clone(),
        },
        (1..=10).collect(),
        5,
        3,
    )?;
    current_benchmarks.run()?;

    // compare results of this run with the results of previous runs with threshold of 5 percent
    let analysis_result = current_benchmarks.analyze(Some(previous_summary), 5.0)?;
    println!("Analysis result:");
    println!("{}", analysis_result.to_string());
    assert!(!analysis_result.divergent_series().is_empty());
    Ok(())
}
