use rand::distributions::{Distribution, Uniform};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    pub fn x(&self) ->f32 {
        self.x
    }

    pub fn y(&self) ->f32 {
        self.y
    }

    pub fn z(&self) ->f32 {
        self.z
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.x = self.x * scale;
        self.y = self.y * scale;
        self.z = self.z * scale;
        self
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn length_squared(&self) -> f32{
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn random() -> Vec3 {
        let step = Uniform::new(0.0, 1.0);
        let mut rng = rand::thread_rng();
        Vec3 { 
            x: step.sample(&mut rng),
            y: step.sample(&mut rng), 
            z: step.sample(&mut rng) 
        }
    }
    
    fn random_bounded(min: f32, max: f32) -> Vec3{
        let step = Uniform::new(min, max);
        let mut rng = rand::thread_rng();
        Vec3 { 
            x: step.sample(&mut rng),
            y: step.sample(&mut rng), 
            z: step.sample(&mut rng) 
        }
    }
}

impl std::convert::From<f32> for Vec3 {
    fn from(value: f32) -> Self {
        Self {
            x: value,
            y: value,
            z: value,
        }
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x, 
            y: self.y + other.y, 
            z: self.z + other.z
        }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Vec3 {
        Vec3 { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl std::ops::Div for Vec3 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x/other.x,
            y: self.y/other.y,
            z: self.z/other.z
        }
    }
    
}
impl std::ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self { 
            x: self.x * other.x, 
            y: self.y * other.y, 
            z: self.z * other.z 
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{x} {y} {z}", x = self.x as i32, y = self.y as i32, z = self.z as i32)
    }
}

fn unit_vector(v: Vec3) -> Vec3 {
    let length = v.length_squared().sqrt();
    Vec3 { x: v.x/length, y: v.y/length, z: v.z/length }
}

fn random_in_unit_sqhere() -> Vec3 {
    loop {
        let p = Vec3::random_bounded(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sqhere())
}

pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_unit_vector();
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        in_unit_sphere.scale(-1.0)
    }
}
