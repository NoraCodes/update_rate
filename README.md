# update_rate

A generic, low-overhead rate counter for FPS counters and the like. 
Usage is simple; pick the number of cycles (frames, updates, etc.) you'd like to average over, 
construct an `UpdateRateCounter`, and call `update()` on it every cycle.
Now you can call `rate()` to find the average rate over the last interval.

## Example

The one important thing to remember is to call your Counter's `update()` at the beginning of your cycles.

```
use update_rate::UpdateRateCounter;

// Create a new UpdateRateCounter with a sample rate of 10 updates
let mut c = UpdateRateCounter::new(10);

for _ in 1..11 {
    c.update();
    // Rate should be 100 Hz with 10 ms/update
    std::thread::sleep(std::time::Duration::from_millis(10));
}

println!("Loop ran at {} cycles per second.", c.rate());
let difference = 100.0 - c.rate();
assert!(difference < 10.0, "Counter rate should be closer to actual rate.");
```