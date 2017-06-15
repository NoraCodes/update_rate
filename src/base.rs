use std::time::{Duration, Instant};
use super::{RateCounter, RateCounterImmut};

/// A generic non-rolling update counter, suitable for rapidly-changing rate
/// counting, such as FPS counters in games. It counts n updates, averages, and
/// then resets (where n is the sample rate), which means that it takes at least
/// n updates to react to a change in rate appropriately.
///
/// # Usage
///
/// Call `.update()` every time your system starts a new update/cycle; for
/// instance, an FPS counter would call this at the beginning of every frame.
/// The sample rate (set with `set_sample_rate()` and in the first argument to
/// `new()`) governs how many `.update()` calls are required before a
/// meaningful result is produced.
///
/// You can also use .update_immut() for this. Since DiscreteRateCounter is 
/// small and easily copyable, this is negligibly less efficient.
#[derive(Clone, Copy)]
pub struct DiscreteRateCounter {
    updates_since_clear: u64,
    time_at_last_clear: Instant,
    rate: f64,
    sample_rate: u64,
}

impl DiscreteRateCounter {
    /// Create a new DiscreteRateCounter which calculates the update rate every
    /// `sample_rate` cycles.  Until that many cycles occur, `rate()` will
    /// return a useless value, typically 0.0.
    ///
    /// If this isn't acceptable, one strategy is to start the sample rate at 0
    /// and keep ramping it up until it reaches your target sample rate;
    /// however, the data near the beginning will not be useful.
    pub fn new(sample_rate: u64) -> Self {
       DiscreteRateCounter {
            updates_since_clear: 0,
            time_at_last_clear: Instant::now(),
            rate: 0.0,
            sample_rate: sample_rate,
        }
    }

    /// Return the number of cycles since the rate was last recalculated.
    pub fn rate_age_cycles(&self) -> u64 {
        self.updates_since_clear
    }

    /// Return the amount of time since the rate was last recalculated. This
    /// requires examining the system clock and is thus relatively expensive.
    pub fn rate_age_duration(&self) -> Duration {
        self.time_at_last_clear.elapsed()
    }
}

impl super::RateCounter for DiscreteRateCounter {
    /// Return the current rate at which the UpdateRateCounter is measuring, in
    /// updates.
    fn sample_rate(&self) -> u64 {
        self.sample_rate
    }

    /// Set the number of updates which the UpdateRateCounter waits before
    /// updating its status.
    fn set_sample_rate(&mut self, sample_rate: u64) {
        self.sample_rate = sample_rate
    }

    /// Updates the struct in place, but requires a mutable binding.
    /// Call this at the beginning of each cycle of the periodic activity being
    /// measured.
    fn update(&mut self) {
        self.updates_since_clear += 1;

        if self.updates_since_clear >= self.sample_rate {
            let elapsed = self.time_at_last_clear.elapsed();
            // Compose a f64 of the amount of time elapsed since the last
            // update; that's seconds plus nanos
            let real_time_since_clear = elapsed.as_secs() as f64 +
                                        elapsed.subsec_nanos() as f64 * 1.0e-9;
            // The rate is the number of updates, over the amount of time it's
            // taken to do them
            self.rate = self.updates_since_clear as f64 / real_time_since_clear;

            // Reset the structure
            self.time_at_last_clear = Instant::now();
            self.updates_since_clear = 0;
        }
    }

    /// Return the last calculated rate of operation, in Hertz (updates per
    /// second).
    fn rate(&self) -> f64 {
        self.rate
    }
}

impl RateCounterImmut for DiscreteRateCounter {
    /// Consumes the struct and returns an updated version.
    /// Call this at the beginning of each cycle of the periodic activity being
    /// measured.
    /// Especially useful in applications like FPS counters, where shadowing can
    /// be used to avoid a mutable binding.
    /// # Examples
    ///
    /// ```
    /// use update_rate::DiscreteRateCounter;
    /// use update_rate::{RateCounter, RateCounterImmut};
    /// let c = DiscreteRateCounter::new(5);
    /// for i in 1..101 {
    ///     let c = c.update_immut();
    ///     if i % 10 == 0 {println!("Rate: {}", c.rate())}
    ///     // Do work here
    /// }
    /// ```
    fn update_immut(self) -> Self {
        let mut new = self.clone();
        new.update();
        new
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update_rate_counter() {
        let mut c = DiscreteRateCounter::new(10);
        assert!(c.rate() == 0.0,
                "Counter should have no data before it gets enough samples.");

        let sample_period = Duration::from_millis(10);
        for i in 1..11 {
            // Use busy-wait because sleeping is extremely inaccurate
            let start = Instant::now();
            while start.elapsed() < sample_period {}

            c.update();

            // Mod 10 because rate_age_cycles will go back to 0 at sample_rate which is 10
            assert!(c.rate_age_cycles() == i % 10,
                    "Rate age not in sync with cycle loop! {} loops but ras = {}",
                    i,
                    c.rate_age_cycles());
        }

        // Rate should be 100 Hz with 10 ms/update
        let difference = 100.0 - c.rate();
        println!("Rate was {}", c.rate());
        assert!(difference < 10.0,
                "Counter rate should be closer to actual rate.");
    }
    #[test]
    fn test_update_rate_counter_immut() {
        let mut c = DiscreteRateCounter::new(10);
        assert!(c.rate() == 0.0,
                "Counter should have no data before it gets enough samples.");

        let sample_period = Duration::from_millis(10);
        for i in 1..11 {
            // Use busy-wait because sleeping is extremely inaccurate
            let start = Instant::now();
            while start.elapsed() < sample_period {}

            c = c.update_immut();

            // Mod 10 because rate_age_cycles will go back to 0 at sample_rate which is 10
            assert!(c.rate_age_cycles() == i % 10,
                    "Rate age not in sync with cycle loop! {} loops but ras = {}",
                    i,
                    c.rate_age_cycles());
        }

        // Rate should be 100 Hz with 10 ms/update
        let difference = 100.0 - c.rate();
        println!("Rate was {}", c.rate());
        assert!(difference < 10.0,
                "Counter rate should be closer to actual rate.");
    }
}
