//! This crate provides a utility for counting updates, for instance frame rates.
//!
//! Implementors of the `RateCounter` trait have a
//! method, `.update()`, which is meant to be called every time your system
//! updates (e.g. every frame, every physics update, etc).
//!
//! The trait `RateCounterImmut` adds an immutable update method which consumes
//! the rate counter and returns an updated one.
//!
//! This can also be done immutably using shadowing and `.update_immut()`.
//!
//! # Examples
//! The one important thing to remember is to call your Counter's `update()`
//! (or `update_immut()`) at the beginning of your cycles.
//!
//! ```
//! use update_rate::{RateCounter, DiscreteRateCounter};
//! // Create a new DiscreteRateCounter with a sample rate of 10 updates
//! let mut c = DiscreteRateCounter::new(10);
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
mod base;
pub use base::DiscreteRateCounter;

//mod rolling;
//pub use rolling::RollingRateCounter;

pub trait RateCounter {
    /// Return the current rate at which the UpdateRateCounter is measuring, in
    /// updates.
    fn sample_rate(&self) -> u64;

    /// Set the number of updates which the UpdateRateCounter waits before
    /// updating its status.
    fn set_sample_rate(&mut self, sample_rate: u64);

    /// Updates the struct in place, but requires a mutable binding.
    /// Call this at the beginning of each cycle of the periodic activity being
    /// measured.
    fn update(&mut self);

    /// Return the last calculated rate of operation, in Hertz (updates per
    /// second).
    fn rate(&self) -> f64;
}

pub trait RateCounterImmut: RateCounter {    
    /// Consumes the struct and returns an updated version.
    /// Call this at the beginning of each cycle of the periodic activity being
    /// measured.
    /// Especially useful in applications like FPS counters, where shadowing can
    /// be used to avoid a mutable binding.
    fn update_immut(self) -> Self;
}