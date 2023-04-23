use std::thread;
use std::time::Duration;
use benchmark_rs::{Benchmarks, StopWatch};

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
