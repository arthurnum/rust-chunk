use time;

pub struct Timer {
    start: time::SteadyTime,
    last_frame: time::SteadyTime
}

pub fn new() -> Box<Timer> {
    Box::new(Timer { start: time::SteadyTime::now(),
                     last_frame: time::SteadyTime::now()})
}

impl Timer {
    pub fn elapsed(&self) -> i64 {
        let dur = time::SteadyTime::now() - self.start;
        dur.num_milliseconds()
    }

    pub fn frame_time(&mut self) -> i64 {
        let origin = self.last_frame;
        self.last_frame = time::SteadyTime::now();
        (self.last_frame - origin).num_milliseconds()
    }
}
