use digit::Digit;
use std::time::Instant;

fn main() -> windows::Result<()> {
    let mut digit = Digit::new();
    let mut last = Instant::now();
    loop {
        let current = Instant::now();
        let delta = (current - last).as_micros() as f32 / 1000000.0;
        last = current;
        digit.update(delta);
        digit.render();
    }
}
