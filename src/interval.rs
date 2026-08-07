

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(a: f64, b: f64) -> Self {
        Self {
            min: a,
            max: b,
        }
    }

    pub fn infinity() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, value: f64) -> bool {
        self.min <= value && value <= self.max
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.min < value && value < self.max
    }

    pub fn max(&self) -> f64 {
        self.max
    }

    pub fn min(&self) -> f64 {
        self.min
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value >= self.max { return self.max; }
        if value <= self.min { return self.min; }

        value
    }
}
