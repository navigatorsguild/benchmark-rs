use std::fmt::Display;
use anyhow::Error;
use statrs::statistics::{Distribution, Max, Median, Min};
use crate::run_summary::RunSummary;
use crate::series_summary::SeriesSummary;
use crate::stopwatch::StopWatch;

pub(crate) struct Benchmark<C, W, E> where
    C: Clone + Display,
    W: Clone + Display,
    Error: From<E>
{
    name: String,
    config: C,
    work: Vec<W>,
    f: fn(&mut StopWatch, C, W) -> Result<(), E>,
    repeat: usize,
    ramp_up: usize,
}

impl<C, W, E> Benchmark<C, W, E> where
    C: Clone + Display,
    W: Clone + Display,
    Error: From<E>
{
    pub(crate) fn new(
        name: String,
        f: fn(&mut StopWatch, C, W) -> Result<(), E>,
        config: C,
        work: Vec<W>,
        repeat: usize,
        ramp_up: usize,
    ) -> Benchmark<C, W, E> {
        Benchmark {
            name,
            config,
            work,
            f,
            repeat,
            ramp_up,
        }
    }

    pub(crate) fn name(&self) -> &String {
        &self.name
    }

    pub(crate) fn run(&self) -> Result<SeriesSummary, Error> {
        let mut series_summary = SeriesSummary::new(self.name.clone(), self.config.to_string());
        let mut error: Option<Error> = None;
        for w in &self.work {
            for _i in 0..self.ramp_up {
                let mut stop_watch = StopWatch::new();
                (self.f)(&mut stop_watch, self.config.clone(), w.clone())?
            }
            let mut durations = Vec::new();
            for _i in 0..self.repeat {
                let mut stop_watch = StopWatch::new();
                stop_watch.start();
                match (self.f)(&mut stop_watch, self.config.clone(), w.clone()) {
                    Ok(_) => {
                        stop_watch.stop();
                        durations.push(stop_watch.accumulated())
                    }
                    Err(e) => {
                        error = Some(e.into());
                        break;
                    }
                }
            }
            match error {
                None => {
                    let data: statrs::statistics::Data<Vec<f64>> = statrs::statistics::Data::new(
                        durations.into_iter()
                            .map(|d| d.as_nanos() as f64)
                            .collect()
                    );

                    let run_summary = RunSummary::new(
                        self.name.clone(),
                        self.ramp_up,
                        self.repeat,
                        data.min() as u64,
                        data.max() as u64,
                        data.median() as u64,
                        data.std_dev(),
                    );
                    series_summary.add(w.to_string(), run_summary);
                }
                Some(_) => {
                    break;
                }
            }
        }

        match error {
            None => {
                Ok(
                    series_summary
                )
            }
            Some(e) => {
                Err(e)
            }
        }
    }
}
