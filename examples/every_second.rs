use update_rate::{RateCounter, RollingRateCounter};

fn main() {
    let mut c = RollingRateCounter::new(10);

    loop {
        c.update();
        std::thread::sleep(std::time::Duration::from_secs(1)); // Perform the slow operation
        println!("Updating at {}", c);
    }
}
