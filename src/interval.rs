#[derive(Debug,Clone,Default)]
pub struct Interval {
    min: f32,
    max: f32
}

struct Empty(Interval);
struct Universe(Interval);

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval {
            min, max
        }
    }
    
    pub fn min(&self) -> f32 {
        self.min
    }

    pub fn max(&self) -> f32 {
        self.max
    }

    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && self.max > x
    }

    pub fn clamp(&self, x: f32) -> f32{
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Empty {
    fn new() -> Interval {
        Interval { min: f32::INFINITY, max: -f32::INFINITY }
    }
}

impl Universe {
    fn new() -> Interval {
        Interval { min: -f32::INFINITY, max: f32::INFINITY }
    }
}
