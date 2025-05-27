use crate::event::Event;
use crate::round_trip_statistics::RoundTripStatistics;
// use numeric_statistics::f64::{min::*, max::*, average::*, variance::*, standard_deviation::*};

#[derive(Debug, Clone)]
pub struct Events {
    pub vec: Vec<Event>,
}

impl Events {
    pub fn new() -> Self {
        Events {
            vec: Vec::new(),
        }
    }

    // TODO memoize
    pub fn attempt_count(&self) -> usize {
        self.vec.len()
    }

    // TODO memoize
    pub fn success_count(&self) -> usize {
        self.vec.iter().filter(|x| x.success).count()
    }

    // TODO memoize
    pub fn failure_count(&self) -> usize {
        self.vec.iter().filter(|x| !x.success).count()
    }

    // TODO memoize
    pub fn success_rate(&self) -> f64 {
        if self.attempt_count() == 0 { return f64::NAN }
        self.success_count() as f64 / self.attempt_count() as f64
    }

    pub fn success_round_trip_statistics(&self) -> Vec<RoundTripStatistics> {
        self
            .vec
            .iter()
            .filter(|x| x.success)
            .filter_map(|x| x.round_trip_statistics )
            .collect::<Vec<_>>()
    }

}

use std::fmt;
impl fmt::Display for Events {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f, 
            concat!(
                "attempt count: {}\n",
                "success count: {}\n",
                "success rate: {}",
            ),
            self.attempt_count(),
            self.success_count(),
            self.success_rate(),
        )
    }
}