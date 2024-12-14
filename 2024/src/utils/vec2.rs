#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    pub fn cross(&self, other: &Self) -> f64 {
        self.x * other.y - other.x * self.y
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn div_n(&self, n: f64) -> Self {
        Self {
            x: self.x / n,
            y: self.y / n,
        }
    }

    pub fn mul_n(&self, n: f64) -> Self {
        Self {
            x: self.x * n,
            y: self.y * n,
        }
    }
}