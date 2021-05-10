use std::time::Instant;

pub struct Timer {
    name: String,
    start: Instant,
}

impl Timer {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            start: Instant::now(),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        let end = Instant::now();
        let duration = end - self.start;
        let n_millis = (duration.as_micros() as f64) / 1000.0;

        println!("PROFILER [{}]: {} ms", self.name, n_millis);
    }
}

#[macro_export]
macro_rules! profile_scope {
    ($name:expr) => {
        #[allow(unused_variables)]
        let timer = crate::timer::Timer::new($name);
    };
}
