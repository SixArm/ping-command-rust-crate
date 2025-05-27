//! Ping utilties. 
//! 
//! These are deliberately implemeted as functions, not structs, 
//! because they are intended to bseful for writing your own code.

use std::process::Command;
use regex::Regex;
use crate::round_trip_statistics::RoundTripStatistics;

/// Call the ping command, then return the output stream.
/// 
/// Example:
/// 
/// ```
/// let args = ["-c", "1", "-W", "5", "localhost"]
/// ping_command(&args);
/// ```
/// 
pub fn ping_command(args: &[&str]) ->  std::io::Result<String> {
    let output = Command::new("ping")
    .args(args)
    .output()?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).into())
    }
    return Err(
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "Ping command output is not a success." 
        )
    )
}

/// Call the ping command, then parse the result into round trip metrics.
/// 
/// Return four floats:
/// 
/// * min a.k.a. minimum
/// * avg a.k.a. average
/// * max a.k.a. maximum
/// * stddev a.k.a. standard deviation
/// 
/// Example:
/// 
/// ```
/// let args = ["-c", "1", "-W", "5", "localhost"]
/// let (min, avg, max, stddev) = ping_command_into_round_trip_statistics(&args).unwrap();
/// ```
/// 
pub fn ping_command_into_round_trip_statistics(args: &[&str]) -> std::io::Result<RoundTripStatistics> {
    let result = ping_command(args)?;
    return Ok(RoundTripStatistics.from_str(&result))
}

/// Create ping command args, given a host string, and using heuristics.
/// 
/// This function sets heuristic arguments that depend on the operating system
/// target family, because different operating systems have different ping
/// command options.
/// 
/// Example when target family = "unix":
/// 
/// ```
/// ping_command_args("localhost"); //=> ["-c", "1", "-W", "5", "localhost"]
/// ```
/// 
/// Example when target family = "windows":
/// 
/// ```
/// ping_command_args("localhost"); //=> ["-n", "1", "-w", "5000", "localhost"]
/// ```
/// 
pub fn ping_command_args(host: &str) -> Vec<&str> {
    if cfg!(target_family = "unix") {
        vec!["-c", "1", "-W", "5", host]
    } 
    else 
    if cfg!(target_family = "windows") {
        vec!["-n", "1", "-w", "5000", host]
    }
    else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ping_command_args {
        use super::*;

        #[test]
        fn test_ping_command_args() {
                let x = ping_command_args("localhost");
                if cfg!(target_family = "unix") {
                    assert_eq!(x, ["-c", "1", "-W", "5", host]);
                }
                else 
                if cfg!(target_family = "windows") {
                    assert_eq!(x, "-n", "1", "-w", "5000", host);
                }
                else {
                    unreachable!()
                }
        }

    }

}