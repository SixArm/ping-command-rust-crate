use std::time::Instant;
use crate::round_trip_statistics::RoundTripStatistics;

#[derive(Debug, Clone)]
pub struct Event {
    pub timestamp: Instant,
    pub host: String,
    pub success: bool,
    pub round_trip_statistics: Option<RoundTripStatistics>,
}
