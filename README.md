![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# benchmark-rs

A benchmarking library for Rust package authors.

Benchmark-rs crate provides tools for Rust package authors to evaluate performance of their implementation for
varying workloads with varying configurations, find performance regressions between versions and also some crude
facilities to measure execution times of code blocks.

For example, if you implement a concurrent algorithm with an expectation of reduced execution time when more
CPU cores are added, benchmark-rs will help validate that the concurrency has the desired effect and that there
is no anomaly for varying data sets, that is, the algorithm performs consistently within boundaries defined for it.

However, this doesn't have to be a fancy algorithm that we want to test, some banal looking code can exhibit
radically different behavior for different data sets, depending on database queries, buffer sizes, number of
open files, etc. We may want to compare the behavior of our code with different querying strategies or different
buffer sizes for a range of data sets to find the best fit for our use-case.


Another common use-case is to verify that there is no regression in performance introduced by the new code. Benchmark-rs
supports comparison to previous results with user defined equality threshold.

## Design

Benchmark-rs is designed to evaluate performance of subjects processing a set of workloads with modifiable configurations.
For example if we write a protocol parser we may define our workload points as 10MB, 20MB, ..., 100MB input sizes. Workloads
are user defined in benchmark-rs. Any type `W: Clone + Display` can be used to specify workloads. It could
be an integer that specifies the size of the workload, a path to a file or a key that we can use
to fetch the workload from the benchmark configuration. The `Display` is required
for the workload point to produce the key in result series, so it is recommended to keep it short and descriptive.
See examples below.

Each benchmark is repeated `repeat` times for every workload point after it was ramped up for that point. Upon
completion the summary is available as JSON or as CSV.

## Issues
Issues are welcome and appreciated. Please submit to https://github.com/navigatorsguild/benchmark-rs/issues

## Examples
A real world example can be found at
[command-executor](https://github.com/navigatorsguild/command-executor) project
[blocking_queue.rs](https://github.com/navigatorsguild/command-executor/blob/main/benches/blocking_queue.rs)
benchmark and at [Benchmarks](https://github.com/navigatorsguild/command-executor/wiki/Benchmarks)
wiki page which was built from the generated data.

![link](https://user-images.githubusercontent.com/122003456/235414598-727d804a-b8ad-4520-871b-5fd8be33bf44.png)

### Simple Benchmark
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

### Benchmark Workloads

A more complex example that shows how to use Benchmark configuration and how to control the
stopwatch from within the benchmark to avoid measuring the housekeeping tasks.

<details>
 <summary>Benchmark Workloads Example</summary>

```rust
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
```
</details>

<details>
 <summary>Benchmark Workloads Summary</summary>

```json
{
  "name": "benchmark-workloads",
  "created_at": "2023-08-13 04:09:13.923036",
  "series": {
    "benchmark-workload-2": {
      "name": "benchmark-workload-2",
      "config": "0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10",
      "runs": [
        [
          "1",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 639791,
            "min_sec": 0.000639791,
            "min_str": "00:00:00.000",
            "max_nanos": 644250,
            "max_sec": 0.00064425,
            "max_str": "00:00:00.000",
            "median_nanos": 642020,
            "median_sec": 0.00064202,
            "median_str": "00:00:00.000",
            "std_dev": 3152.9891373108153,
            "std_dev_sec": 3.1529891373108154e-6,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "2",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 1264124,
            "min_sec": 0.001264124,
            "min_str": "00:00:00.001",
            "max_nanos": 1264250,
            "max_sec": 0.00126425,
            "max_str": "00:00:00.001",
            "median_nanos": 1264187,
            "median_sec": 0.001264187,
            "median_str": "00:00:00.001",
            "std_dev": 89.09545442950498,
            "std_dev_sec": 8.909545442950498e-8,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "3",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 1891833,
            "min_sec": 0.001891833,
            "min_str": "00:00:00.001",
            "max_nanos": 1892875,
            "max_sec": 0.001892875,
            "max_str": "00:00:00.001",
            "median_nanos": 1892354,
            "median_sec": 0.001892354,
            "median_str": "00:00:00.001",
            "std_dev": 736.8052659963826,
            "std_dev_sec": 7.368052659963826e-7,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "4",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 2566417,
            "min_sec": 0.002566417,
            "min_str": "00:00:00.002",
            "max_nanos": 2576416,
            "max_sec": 0.002576416,
            "max_str": "00:00:00.002",
            "median_nanos": 2571416,
            "median_sec": 0.002571416,
            "median_str": "00:00:00.002",
            "std_dev": 7070.360705084288,
            "std_dev_sec": 7.0703607050842884e-6,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "5",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 2928666,
            "min_sec": 0.002928666,
            "min_str": "00:00:00.002",
            "max_nanos": 3174917,
            "max_sec": 0.003174917,
            "max_str": "00:00:00.003",
            "median_nanos": 3051791,
            "median_sec": 0.003051791,
            "median_str": "00:00:00.003",
            "std_dev": 174125.75197396852,
            "std_dev_sec": 0.00017412575197396852,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "6",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 3377791,
            "min_sec": 0.003377791,
            "min_str": "00:00:00.003",
            "max_nanos": 3784625,
            "max_sec": 0.003784625,
            "max_str": "00:00:00.003",
            "median_nanos": 3581208,
            "median_sec": 0.003581208,
            "median_str": "00:00:00.003",
            "std_dev": 287675.0802172479,
            "std_dev_sec": 0.00028767508021724787,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "7",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 4404499,
            "min_sec": 0.004404499,
            "min_str": "00:00:00.004",
            "max_nanos": 4915541,
            "max_sec": 0.004915541,
            "max_str": "00:00:00.004",
            "median_nanos": 4660020,
            "median_sec": 0.00466002,
            "median_str": "00:00:00.004",
            "std_dev": 361361.26367113565,
            "std_dev_sec": 0.00036136126367113564,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "8",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 4882584,
            "min_sec": 0.004882584,
            "min_str": "00:00:00.004",
            "max_nanos": 5029209,
            "max_sec": 0.005029209,
            "max_str": "00:00:00.005",
            "median_nanos": 4955896,
            "median_sec": 0.004955896,
            "median_str": "00:00:00.004",
            "std_dev": 103679.53179147754,
            "std_dev_sec": 0.00010367953179147753,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "9",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 5638959,
            "min_sec": 0.005638959,
            "min_str": "00:00:00.005",
            "max_nanos": 5643416,
            "max_sec": 0.005643416,
            "max_str": "00:00:00.005",
            "median_nanos": 5641187,
            "median_sec": 0.005641187,
            "median_str": "00:00:00.005",
            "std_dev": 3151.5749237484424,
            "std_dev_sec": 3.1515749237484423e-6,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "10",
          {
            "name": "benchmark-workload-2",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 5086250,
            "min_sec": 0.00508625,
            "min_str": "00:00:00.005",
            "max_nanos": 5587292,
            "max_sec": 0.005587292,
            "max_str": "00:00:00.005",
            "median_nanos": 5336771,
            "median_sec": 0.005336771,
            "median_str": "00:00:00.005",
            "std_dev": 354290.19585927017,
            "std_dev_sec": 0.00035429019585927015,
            "std_dev_str": "00:00:00.000"
          }
        ]
      ]
    },
    "benchmark-workload-1": {
      "name": "benchmark-workload-1",
      "config": "0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10",
      "runs": [
        [
          "1",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 1259375,
            "min_sec": 0.001259375,
            "min_str": "00:00:00.001",
            "max_nanos": 1276125,
            "max_sec": 0.001276125,
            "max_str": "00:00:00.001",
            "median_nanos": 1267750,
            "median_sec": 0.00126775,
            "median_str": "00:00:00.001",
            "std_dev": 11844.038584874672,
            "std_dev_sec": 0.000011844038584874672,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "2",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 2513584,
            "min_sec": 0.002513584,
            "min_str": "00:00:00.002",
            "max_nanos": 2520875,
            "max_sec": 0.002520875,
            "max_str": "00:00:00.002",
            "median_nanos": 2517229,
            "median_sec": 0.002517229,
            "median_str": "00:00:00.002",
            "std_dev": 5155.515541631118,
            "std_dev_sec": 5.155515541631118e-6,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "3",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 3755750,
            "min_sec": 0.00375575,
            "min_str": "00:00:00.003",
            "max_nanos": 3784000,
            "max_sec": 0.003784,
            "max_str": "00:00:00.003",
            "median_nanos": 3769875,
            "median_sec": 0.003769875,
            "median_str": "00:00:00.003",
            "std_dev": 19975.766568519968,
            "std_dev_sec": 0.00001997576656851997,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "4",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 5003833,
            "min_sec": 0.005003833,
            "min_str": "00:00:00.005",
            "max_nanos": 5030084,
            "max_sec": 0.005030084,
            "max_str": "00:00:00.005",
            "median_nanos": 5016958,
            "median_sec": 0.005016958,
            "median_str": "00:00:00.005",
            "std_dev": 18562.26011292806,
            "std_dev_sec": 0.00001856226011292806,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "5",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 5518500,
            "min_sec": 0.0055185,
            "min_str": "00:00:00.005",
            "max_nanos": 6283084,
            "max_sec": 0.006283084,
            "max_str": "00:00:00.006",
            "median_nanos": 5900792,
            "median_sec": 0.005900792,
            "median_str": "00:00:00.005",
            "std_dev": 540642.5311867353,
            "std_dev_sec": 0.0005406425311867353,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "6",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 6942292,
            "min_sec": 0.006942292,
            "min_str": "00:00:00.006",
            "max_nanos": 7541458,
            "max_sec": 0.007541458,
            "max_str": "00:00:00.007",
            "median_nanos": 7241875,
            "median_sec": 0.007241875,
            "median_str": "00:00:00.007",
            "std_dev": 423674.3416564189,
            "std_dev_sec": 0.00042367434165641895,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "7",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 8784417,
            "min_sec": 0.008784417,
            "min_str": "00:00:00.008",
            "max_nanos": 8812875,
            "max_sec": 0.008812875,
            "max_str": "00:00:00.008",
            "median_nanos": 8798646,
            "median_sec": 0.008798646,
            "median_str": "00:00:00.008",
            "std_dev": 20122.84477900677,
            "std_dev_sec": 0.00002012284477900677,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "8",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 9426875,
            "min_sec": 0.009426875,
            "min_str": "00:00:00.009",
            "max_nanos": 11596125,
            "max_sec": 0.011596125,
            "max_str": "00:00:00.011",
            "median_nanos": 10511500,
            "median_sec": 0.0105115,
            "median_str": "00:00:00.010",
            "std_dev": 1533891.3850889183,
            "std_dev_sec": 0.0015338913850889183,
            "std_dev_str": "00:00:00.001"
          }
        ],
        [
          "9",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 9498333,
            "min_sec": 0.009498333,
            "min_str": "00:00:00.009",
            "max_nanos": 9953333,
            "max_sec": 0.009953333,
            "max_str": "00:00:00.009",
            "median_nanos": 9725833,
            "median_sec": 0.009725833,
            "median_str": "00:00:00.009",
            "std_dev": 321733.5854398791,
            "std_dev_sec": 0.0003217335854398791,
            "std_dev_str": "00:00:00.000"
          }
        ],
        [
          "10",
          {
            "name": "benchmark-workload-1",
            "ramp_up": 1,
            "repeat": 2,
            "min_nanos": 11674000,
            "min_sec": 0.011674,
            "min_str": "00:00:00.011",
            "max_nanos": 12627167,
            "max_sec": 0.012627167,
            "max_str": "00:00:00.012",
            "median_nanos": 12150583,
            "median_sec": 0.012150583,
            "median_str": "00:00:00.012",
            "std_dev": 673990.849303238,
            "std_dev_sec": 0.0006739908493032379,
            "std_dev_str": "00:00:00.000"
          }
        ]
      ]
    }
  }
}
```
</details>

<details>
 <summary>Benchmark Workloads Series</summary>

Benchmark name: benchmark-workload-2
```csv
point,ramp_up,repeat,min_sec,max_sec,median_sec,std_dev_sec
1,1,2,0.00063575,0.000637376,0.000636563,0.0000011497556262093261
2,1,2,0.001265333,0.001269584,0.001267458,0.0000030059109268240135
3,1,2,0.001890958,0.001892333,0.001891645,0.0000009722718241315028
4,1,2,0.002512042,0.002604791,0.002558416,0.0000655834468482711
5,1,2,0.003129084,0.00314525,0.003137167,0.000011431088224661727
6,1,2,0.003092583,0.003767833,0.003430208,0.0004774738539962162
7,1,2,0.004055333,0.004084166,0.004069749,0.000020388009821951726
8,1,2,0.004171542,0.005756541,0.004964041,0.0011207635410738967
9,1,2,0.004549334,0.005149792,0.004849563,0.0004245879236177119
10,1,2,0.006351,0.007404083,0.006877541,0.000744642130452273
```

Benchmark name: benchmark-workload-1
```csv
point,ramp_up,repeat,min_sec,max_sec,median_sec,std_dev_sec
1,1,2,0.001265417,0.001266709,0.001266063,0.0000009135819612930194
2,1,2,0.002518501,0.002534999,0.00252675,0.000011665847676015662
3,1,2,0.003762958,0.004072459,0.003917708,0.00021885025588401765
4,1,2,0.004471833,0.004559375,0.004515604,0.00006190154183863275
5,1,2,0.005676917,0.005765751,0.005721334,0.00006281512379992577
6,1,2,0.006610875,0.007539833,0.007075354,0.0006568725012374928
7,1,2,0.007831792,0.008021959,0.007926875,0.0001344683752579022
8,1,2,0.009668583,0.009832959,0.009750771,0.00011623138426431993
9,1,2,0.009677333,0.011178501,0.010427917,0.0010614860725002473
10,1,2,0.011440417,0.012526,0.011983208,0.0007676231008408358
```

</details>


<details>
 <summary>Find Regressions Example</summary>

```rust
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
```

</details>


<details>
 <summary>Find Regressions Output</summary>

Analysis result. Current and previous values are in nanosecond units, the change is in percents.
```json
  {
    "name": "benchmarks",
    "new_series": [
      "benchmark-2"
    ],
    "equal_series": {},
    "divergent_series": {
      "benchmark-1": {
        "8": {
          "Equal": {
            "point": "8",
            "previous": 200686042,
            "current": 206411375,
            "change": 2.852880520709064
          }
        },
        "7": {
          "Equal": {
            "point": "7",
            "previous": 176700166,
            "current": 183395333,
            "change": 3.788998704166474
          }
        },
        "10": {
          "Equal": {
            "point": "10",
            "previous": 251701083,
            "current": 256856000,
            "change": 2.0480313149864315
          }
        },
        "2": {
          "Equal": {
            "point": "2",
            "previous": 52684667,
            "current": 55052334,
            "change": 4.494034288951653
          }
        },
        "5": {
          "Equal": {
            "point": "5",
            "previous": 127510583,
            "current": 130817709,
            "change": 2.5936090340046434
          }
        },
        "6": {
          "Equal": {
            "point": "6",
            "previous": 152803084,
            "current": 158141750,
            "change": 3.4938208446106955
          }
        },
        "9": {
          "Equal": {
            "point": "9",
            "previous": 225706250,
            "current": 229522083,
            "change": 1.6906191122310474
          }
        },
        "1": {
          "Greater": {
            "point": "1",
            "previous": 26413209,
            "current": 35264166,
            "change": 33.50958605597674
          }
        },
        "4": {
          "Equal": {
            "point": "4",
            "previous": 104201208,
            "current": 109166958,
            "change": 4.7655397622645665
          }
        },
        "3": {
          "Equal": {
            "point": "3",
            "previous": 79801875,
            "current": 84060834,
            "change": 5.336915955922095
          }
        }
      }
    }
  }
```
</details>

## Similar Projects
* [criterion](https://crates.io/crates/criterion)
* [iai](https://crates.io/crates/iai)



License: MIT OR Apache-2.0
