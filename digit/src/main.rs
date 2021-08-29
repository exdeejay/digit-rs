use digit::Digit;
use std::time::Instant;

const TIMESTEP: f32 = 1.0 / 30.0;

fn main() -> windows::Result<()> {
    let mut digit = Digit::new();

    let mut last = Instant::now();
    loop {
        let current = Instant::now();
        let mut elapsed = (current - last).as_micros() as f32 / 1000000.0;
        last = current;

        while elapsed.abs() > 0.00001 {
            let dt = elapsed.min(TIMESTEP);
            digit.update(dt);
            elapsed -= dt;
        }
        digit.render();
    }
}
