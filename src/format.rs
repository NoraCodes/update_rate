//! Provides implementations of format traits

use std::fmt;
use crate::RateCounter;
use crate::rolling::RollingRateCounter;
use crate::DiscreteRateCounter;

/// Creates an implementation of fmt::Display and fmt::Debug for the given type implementing RateCounter
macro_rules! format_impl_for_RateCounter {
    ($($type:tt)*) => {
        impl fmt::Display for $($type)* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{} Hz", self.rate())
            }
        }

        impl fmt::Debug for $($type)* {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{{ samples: {}, rate: {} }}", self.samples(), self.rate())
            }
        }
    };
}

format_impl_for_RateCounter!(RollingRateCounter);

format_impl_for_RateCounter!(DiscreteRateCounter);

#[cfg(test)]
mod tests {
    use {RateCounter, RollingRateCounter, DiscreteRateCounter};
    use std::time::Duration;
    use std::thread::sleep;
    #[test]
    fn test_rolling_rate_counter_formatting() {
        let mut c = RollingRateCounter::new(10);

        let sample_period = Duration::from_millis(10);
        for _ in 1..5 {
            // Accuracy doesn't matter here so we can just sleep
            sleep(sample_period);
            c.update();
        }

        // Rate should be 100 Hz with 10 ms/update
        let rate = c.rate();
        let samples = c.samples();
        let expected = format!("{} Hz", rate);
        assert_eq!(expected, format!("{}", c), "Display output should be of the form \"<rate> Hz\"");
        let expected = format!("{{ samples: {}, rate: {} }}", samples, rate);
        assert_eq!(expected, format!("{:?}", c), "Debug output should be of the form \"{{ samples: <samp>, rate: <rate> }}\"");
    }

    #[test]
    fn test_discrete_rate_counter_formatting() {
        let mut c = DiscreteRateCounter::new(10);

        let sample_period = ::std::time::Duration::from_millis(10);
        for _ in 1..5 {
            // Accuracy doesn't matter here so we can just sleep
            sleep(sample_period);
            c.update();
        }

        // Rate should be 100 Hz with 10 ms/update
        let rate = c.rate();
        let samples = c.samples();
        let expected = format!("{} Hz", rate);
        assert_eq!(expected, format!("{}", c), "Display output should be of the form \"<rate> Hz\"");
        let expected = format!("{{ samples: {}, rate: {} }}", samples, rate);
        assert_eq!(expected, format!("{:?}", c), "Debug output should be of the form \"{{ samples: <samp>, rate: <rate> }}\"");
    }
}
