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
}
