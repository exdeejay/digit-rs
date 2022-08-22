use digit::Digit;
use std::time::Instant;

const TIMESTEP: f32 = 1.0 / 30.0;

fn main() -> windows::Result<()> {
    // Instantiate main Digit class to initialize basically everything
    let mut digit = Digit::new();

    // Initialize time tracking for update deltas
    let mut last = Instant::now();
    loop {
        // Track elapsed time since last frame
        let current = Instant::now();
        let mut elapsed = (current - last).as_micros() as f32 / 1000000.0;
        last = current;

        // Fixed timestep updating
        while elapsed.abs() > 0.00001 {
            let dt = elapsed.min(TIMESTEP);
            digit.update(dt);
            elapsed -= dt;
        }
        digit.render();

        //TODO: implement sleeping here to decrease CPU usage
    }
}
