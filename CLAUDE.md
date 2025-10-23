# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

benchmark-rs is a benchmarking library for Rust package authors that evaluates performance across varying workloads and configurations. It supports regression detection by comparing benchmark results against previous runs with user-defined thresholds.

## Common Commands

### Build and Test
```bash
cargo build          # Build the library
cargo test           # Run all tests
cargo test <name>    # Run a specific test
cargo clippy         # Run linter
```

### Run Examples
```bash
cargo run --example benchmark-example    # Simple benchmark example
cargo run --example bench-workloads      # Workload configuration example
cargo run --example find-regressions     # Regression detection example
```

### Documentation
```bash
cargo doc --open     # Generate and open documentation
```

## Architecture

### Core Components

**Benchmarks** (`src/benchmarks.rs`)
- Main entry point for running benchmark suites
- Manages multiple `Benchmark` instances
- Generic over three types: `C` (configuration), `W` (workload), `E` (error)
- Generates summaries in JSON and CSV formats
- Provides `analyze()` method for comparing results against previous runs

**Benchmark** (`src/benchmark.rs`)
- Executes a single benchmark series with a specific configuration
- Runs the benchmark function for each workload point
- Performs ramp-up iterations before measurements
- Collects timing data for statistical analysis

**StopWatch** (`src/stopwatch.rs`)
- Timing utility passed to benchmark functions
- Supports pause/resume for excluding setup/teardown from measurements
- Essential for accurate timing of only the relevant code sections

**Summary Types**
- `Summary` (`src/summary.rs`): Container for all benchmark series results
- `SeriesSummary` (`src/series_summary.rs`): Results for a single benchmark series
- `RunSummary` (`src/run_summary.rs`): Statistics for a single workload point (min, max, median, std_dev)

**Analysis**
- `AnalysisResult` (`src/analysis_result.rs`): Categorizes series as new, equal, or divergent
- `BenchmarkComparison` (`src/benchmark_comparison.rs`): Comparison enum (Equal/Greater/Less) for workload points

### Key Design Patterns

**Generic Workloads**: Any type implementing `Clone + Display` can be a workload. The `Display` trait generates keys in result series, so keep implementations concise.

**Configuration Flexibility**: Configurations are also generic (`C: Clone + Display`), allowing complex setup data to be passed to benchmark functions.

**Statistical Measurements**: Each workload point is run `ramp_up` times (discarded), then `repeat` times (measured), producing min/max/median/std_dev statistics.

**Comparison Logic**: The `analyze()` method compares median execution times between current and previous runs, categorizing each workload point based on a percentage threshold.

## Development Notes

- Benchmark functions receive `&mut StopWatch`, configuration, and workload as parameters
- Use `stop_watch.pause()` and `stop_watch.resume()` to exclude setup/teardown from measurements
- When adding benchmarks via `Benchmarks::add()`, names must be unique
- The `repeat` parameter must be > 0 (enforced by validation)
- Results can be saved to disk using `save_to_csv()` or `save_to_json()`
