use std::ops;
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {e: [x, y, z]}
    }

    pub fn _zero() -> Self {
        Vec3{e: [0.0, 0.0, 0.0]}
    }

    pub fn _random() -> Self {
        Vec3{e: [rand::random::<f64>(), rand::random::<f64>(), rand::random::<f64>()]}
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        Vec3{e: [rand::random_range(min..max), rand::random_range(min..max), rand::random_range(min..max)]}
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            let lengthsq = p.length_squared();
            if 1e-160 < lengthsq && lengthsq <= 1.0 {
                let n = p / lengthsq.sqrt();
                return n;
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let p = Vec3::random_unit();
        if dot(&p, normal) > 0.0 {
            p 
        } else {
            -p 
        }
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0]+self.e[1]*self.e[1]+self.e[2]*self.e[2]
    }
}

pub type Point3 = Vec3;

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x()+rhs.x(), self.y()+rhs.y(), self.z()+rhs.z())
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x()-rhs.x(), self.y()-rhs.y(), self.z()-rhs.z())
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new(self.x()*rhs, self.y()*rhs, self.z()*rhs)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs:f64) -> Vec3 {
        Vec3::new(self.x()/rhs, self.y()/rhs, self.z()/rhs)
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    return a.x()*b.x()+a.y()*b.y()+a.z()*b.z();
}

pub fn normalise(a: &Vec3) -> Vec3 {
    let length = a.length();
    if length == 1.0 { return *a };

    let factor = 1.0 / length;

    let x = a.x() * factor;
    let y = a.y() * factor;
    let z = a.z() * factor;

    Vec3::new(x, y, z)
}

// Colours
pub type Colour = Vec3;

