#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub const fn zero_to_inf() -> Self {
        Self {
            min: 0.0,
            max: f32::INFINITY,
        }
    }

    pub const fn empty() -> Self {
        Self {
            min: f32::INFINITY,
            max: f32::NEG_INFINITY,
        }
    }

    pub const fn universe() -> Self {
        Self {
            min: 0.0,
            max: f32::INFINITY,
        }
    }

    pub fn merge(interval_1: Self, interval_2: Self) -> Self {
        let min = if interval_1.min <= interval_2.min {
            interval_1.min
        } else {
            interval_2.min
        };
        let max = if interval_1.max >= interval_2.max {
            interval_1.max
        } else {
            interval_2.max
        };
        Self { min, max }
    }
}

impl Interval {
    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contain(&self, value: f32) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f32) -> bool {
        self.min < value && value < self.max
    }

    pub fn clamp(&self, value: f32) -> f32 {
        if value < self.min {
            self.min
        } else if value > self.max {
            self.max
        } else {
            value
        }
    }

    pub fn expand(&self, delta: f32) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}
