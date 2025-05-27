//! Ping command round trip metrics.
//! 
//! Example:
//! 
//! ```
//! use ping_command::round_trip_statistics::RoundTripStatistics;
//! let round_trip_statistics = RoundTripStatistics {
//!     min: 12.445,
//!     max: 61.365,
//!     average: 26.791,
//!     standard_deviation: 20.049,
//! };
//! assert_eq!(round_trip_statistics.min, 12.445);
//! assert_eq!(round_trip_statistics.max, 61.365);
//! assert_eq!(round_trip_statistics.average, 26.791);
//! assert_eq!(round_trip_statistics.standard_deviation, 20.049);
//! ```

#[derive(Debug, Clone, Copy)]
pub struct RoundTripStatistics {
    pub min: f64,
    pub max: f64,
    pub average: f64,
    pub standard_deviation: f64,
}

impl std::fmt::Display for RoundTripStatistics {

    /// Format the RoundTripStatistics for Display.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            concat!(
                "min: {:.3}\n",
                "max: {:.3}\n",
                "average: {:.3}\n",
                "standard deviation: {:.3}",
            ),
            self.min,
            self.max,
            self.average,
            self.standard_deviation,
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseError;

impl std::str::FromStr for RoundTripStatistics {
    type Err = ParseError;

    /// Parse the ping standard output string round trip metrics into milliseconds as f64 floats.
    /// 
    /// Example:
    /// 
    /// ```nodoc
    /// use std::str::FromStr;
    /// use ping_command::round_trip_statistics::RoundTripStatistics;
    /// let str = "round-trip min/avg/max/stddev = 12.445/26.791/61.365/20.049 ms";
    /// let round_trip_statistics = RoundTripStatistics::from_str(&str),unwrap();
    /// assert_eq!(round_trip_statistics.min, 12.445);
    /// assert_eq!(round_trip_statistics.max, 61.365);
    /// assert_eq!(round_trip_statistics.average, 26.791);
    /// assert_eq!(round_trip_statistics.standard_deviation, 20.049);
    /// ```
    ///
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"(?ms)^round-trip min/avg/max/stddev = (?<min>[\d\.]+)/(?<average>[\d\.]+)/(?<max>[\d\.]+)/(?<standard_deviation>[\d\.]+) ms").unwrap();
        if let Some(captures) = r.captures(s) {
            let min = captures["min"].parse::<f64>().unwrap_or(f64::NAN);
            let max: f64 = captures["max"].parse::<f64>().unwrap_or(f64::NAN);
            let average = captures["average"].parse::<f64>().unwrap_or(f64::NAN);
            let standard_deviation = captures["standard_deviation"].parse::<f64>().unwrap_or(f64::NAN);
            return Ok(
                Self {
                    min, 
                    max, 
                    average, 
                    standard_deviation,
                }
            )
        }
        return Err(ParseError)
    }
}

#[cfg(test)]
mod tests {
   use super::*;

   mod display {
       use super::*;

        #[test]
        fn test_display() {
            let x = RoundTripStatistics {
                min: 12.445,
                max: 61.365,
                average: 26.791,
                standard_deviation: 20.049,
            };
            let actual = x.to_string();
            let expect = concat!(
                "min: 12.445\n",
                "max: 61.365\n",
                "average: 26.791\n",
                "standard deviation: 20.049",
            );
            assert_eq!(actual, expect);
        }

    }

    mod from_str {
       use super::*;
       use std::str::FromStr;

        #[test]
        fn test_present() {
            let str = "round-trip min/avg/max/stddev = 12.445/26.791/61.365/20.049 ms";
            let result = RoundTripStatistics::from_str(&str);
            assert!(result.is_ok());
            let x = result.unwrap();
            assert_eq!(x.min, 12.445);
            assert_eq!(x.max, 61.365);
            assert_eq!(x.average, 26.791);
            assert_eq!(x.standard_deviation, 20.049);
        }

        #[test]
        fn test_absent() {
            let str = "hello world";
            let result = RoundTripStatistics::from_str(&str);
            assert!(result.is_err());
        }

    }

}

