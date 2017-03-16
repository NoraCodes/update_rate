//! This crate provides a utility for counting updates, for instance frame rates. 
//! The struct [`UpdateRateCounter`](./struct.UpdateRateCounter.html) has a method, `.update()`, which is meant to be called 
//! every time your system updates (e.g. every frame, every physics update, etc).
//!
//! # Examples
//! The one important thing to remember is to call your Counter's `update()` at the beginning of your cycles.
//!
//! ```
//! use update_rate::UpdateRateCounter;
//! 
//! // Create a new UpdateRateCounter with a sample rate of 10 updates
//! let mut c = UpdateRateCounter::new(10);
//!
//! for _ in 1..11 {
//!     c.update();
//!     // Rate should be 100 Hz with 10 ms/update
//!     std::thread::sleep(std::time::Duration::from_millis(10));
//! }
//!
//! let difference = 100.0 - c.rate();
//! println!("Rate was {}", c.rate());
//! assert!(difference < 10.0, "Counter rate should be closer to actual rate.");
//! ```
use std::time::{Duration, Instant};

/// A generic non-rolling update counter, suitable for rapidly-changing rate counting, such as FPS counters in games.
/// # Usage
/// Call `.update()` every time your system starts a new update/cycle; for instance, 
/// an FPS counter would call this at the beginning of every frame. 
/// The sample rate (set with `set_sample_rate()` and in the first argument to `new()`)
/// governs how many `.update()` calls are required before a meaningful result is produced.
pub struct UpdateRateCounter {
    updates_since_clear: u64,
    time_at_last_clear: Instant,
    rate: f64,
    sample_rate: u64
}

impl UpdateRateCounter {
    /// Create a new UpdateRateCounter which calculates the update rate every `sample_rate` cycles.
    /// Until that many cycles occur, `rate()` will return a useless value, typically 0.0.
    /// 
    /// If this isn't acceptable, one strategy is to start the sample rate at 0 and
    /// keep ramping it up until it reaches your target sample rate; however, the data near the beginning will be utter trash.
    pub fn new(sample_rate: u64) -> Self {
        UpdateRateCounter {
            updates_since_clear: 0,
            time_at_last_clear: Instant::now(),
            rate: 0.0,
            sample_rate: sample_rate
        }
    }

    /// Return the current rate at which the UpdateRateCounter is measuring, in updates.
    pub fn sample_rate(&self) -> u64 { self.sample_rate }

    /// Set the number of updates which the UpdateRateCounter waits before updating its status.
    pub fn set_sample_rate(&mut self, sample_rate: u64) { self.sample_rate = sample_rate }

    /// Call this at the beginning of each cycle of the periodic activity being measured.
    pub fn update(&mut self) {
        self.updates_since_clear += 1;

        if self.updates_since_clear >= self.sample_rate {
            let elapsed = self.time_at_last_clear.elapsed();
            // Compose a f64 of the amount of time elapsed since the last update; that's seconds plus nanos
            let real_time_since_clear = elapsed.as_secs() as f64 + elapsed.subsec_nanos() as f64 * (1.0/1000000000.0);
            // The rate is the number of updates, over the amount of time it's taken to do them
            self.rate = self.updates_since_clear as f64 / real_time_since_clear;

            // Reset the structure
            self.updates_since_clear = 0;
            self.time_at_last_clear = Instant::now();
        }
    }

    /// Return the last calculated rate of operation, in Hertz (updates per second).
    pub fn rate(&self) -> f64 { self.rate }

    /// Return the number of cycles since the rate was last recalculated.
    pub fn rate_age_cycles(&self) -> u64 { self.updates_since_clear }

    /// Return the amount of time since the rate was last recalculated. This requires examining the system clock
    /// and is thus relatively expensive.
    pub fn rate_age_duration(&self) -> Duration { self.time_at_last_clear.elapsed() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_rate_counter() {
        let mut c = UpdateRateCounter::new(10);
        assert!(c.rate() == 0.0, "Counter should have no data before it gets enough samples.");
        for i in 1..11 {
            // Rate should be 100 Hz with 10 ms/update
            std::thread::sleep(std::time::Duration::from_millis(10));
            c.update();
            assert!(c.rate_age_cycles() == i % 10, // Mod 10 because rate_age_cycles will go back to 0 at sample_rate which is 10
                "Rate age not in sync with cycle loop! {} loops but ras = {} ", i, c.rate_age_cycles());
        }
        let difference = 100.0 - c.rate();
        println!("Rate was {}", c.rate());
        assert!(difference < 10.0, "Counter rate should be closer to actual rate.");
    }
}