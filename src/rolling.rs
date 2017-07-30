use std::time::Instant;
use std::collections::VecDeque;
use super::RateCounter;

/// A rolling update counter. It records as many updates as the given sample rate
/// and re-calculates the average update time on each call to update.
///
/// Generally, this is to be preferred over the discrete version. However, for very
/// high values of `sample`, this can be quite inefficient, especially if the rate
/// value isn't needed during each cycle.
///
/// # Usage
///
/// Call `.update()` every time your system starts a new update/cycle; for
/// instance, an FPS counter would call this at the beginning of every frame.
/// The sample rate (set with `set_sample_rate()` and in the first argument to
/// `new()`) governs how many `.update()` calls are required before a
/// meaningful result is produced.
///
/// You can also use .update_immut() for this to avoid a mutable binding.
#[derive(Clone)]
pub struct RollingRateCounter {
    updates: VecDeque<Instant>,
    rate: f64,
    samples: u64,
}

impl RollingRateCounter {
    /// Create a new RollingRateCounter which calculates the update rate every
    /// update, averaging over a window of `update_rate` cycles.
    ///
    /// # Panics
    /// This function will panic if given a value of `samples` equal to 0.
    pub fn new(samples: u64) -> Self {
        if samples == 0 {
            panic!("Tried to build a RollingRateCounter with a sample_rate of 0")
        }
        RollingRateCounter {
            updates: VecDeque::with_capacity(samples as usize),
            rate: 0.0,
            samples: samples,
        }
    }
}

impl RateCounter for RollingRateCounter {
    fn samples(&self) -> u64 {
        self.samples
    }

    fn set_samples(&mut self, samples: u64) {
        if samples == 0 {
            panic!("Tried to set sample_rate of a RollingRateCounter to 0");
        }
        self.samples = samples;

        // Remove the oldest updates until the window
        // is the correct length
        while self.updates.len() > self.samples as usize {
            self.updates.remove(0);
        }
    }

    fn update(&mut self) {
        // Remove the element at the top of the queue until it's cut down to size
        while self.updates.len() >= self.samples as usize {
            self.updates.pop_front();
        }

        self.updates.push_back(Instant::now());

        self.rate = 0.0;
        for (index, _) in self.updates.iter().enumerate() {
            if index == 0 {
                continue;
            }
            // Get the time elapsed during the update interval being considered
            let delta_t = self.updates[index].duration_since(self.updates[index - 1]);
            let delta_t = delta_t.as_secs() as f64 + delta_t.subsec_nanos() as f64 * 1e-9;

            // Average it with the rate
            let avg_delta_t = (self.rate + delta_t) / 2.0;
            self.rate = self.samples as f64 / avg_delta_t;
        }
    }

    fn rate(&self) -> f64 {
        self.rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_rolling_rate_counter() {
        let mut c = RollingRateCounter::new(10);
        assert!(
            c.rate() == 0.0,
            "Counter should have no data before it gets enough samples (has {}).",
            c.rate()
        );

        let sample_period = ::std::time::Duration::from_millis(10);
        for _ in 1..11 {
            // Use busy-wait because sleeping is extremely inaccurate
            let start = Instant::now();
            while start.elapsed() < sample_period {}

            c.update();
        }

        // Rate should be 100 Hz with 10 ms/update
        let difference = 100.0 - c.rate();
        assert!(
            difference < 10.0,
            "Counter rate {} should be closer to actual rate 100.0.",
            c.rate()
        );
    }
}
