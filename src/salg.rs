#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x, y, z
        }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
    pub fn trunc(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y
        }
    }
    pub fn dot(a: &Self, b: &Self) -> f64 {
        a.x*b.x + a.y*b.y + a.z*b.z
    }
}
impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x, y,
        }
    }
    pub fn magnitude(&self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
    pub fn norm(&self) -> Self {
        let scaler = 1.0/self.magnitude();
        self.scale(scaler)
    }
    pub fn scale(&self, factor: f64) -> Self {
        Self {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
    pub fn expand(&self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: other,
        }
    }
    pub fn dot(a: Self, b: Self) -> f64 {
        a.x*b.x + a.y*b.y
    }
}


