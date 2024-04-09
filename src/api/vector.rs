use std::ops::{Add, Mul, Sub, Div};
/*
Implementation of Vector3
 */
#[derive(Debug, Clone)]
pub struct Vector3 {
    pub x : f64,
    pub y : f64,
    pub z : f64
}


pub static E1 : Vector3 = Vector3{x:1.0, y:0.0, z:0.0};
pub static E2: Vector3 = Vector3{x:0.0, y:1.0, z:0.0};
// pub const DIVABLE_DELTA : f64 = 1.0; 
pub const INF_DISTANCE : f64 = 1e+8;

impl Vector3 {
    pub fn cross(&self, rhs : &Self)->Self{
       Self{x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x} 
    }

    pub fn normalize(&self)->Self {
        self / self.length()
    }

    pub fn dot(&self, rhs : &Self)->f64{
          self.x * rhs.x
        + self.y * rhs.y
        + self.z * rhs.z
    } 

    pub fn length2(&self) -> f64{
          self.x * self.x 
        + self.y * self.y
        + self.z * self.z
    }

    pub fn length(&self) -> f64{
          f64::sqrt( self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn onb(&self) -> (Self, Self) {
        let mut u = self.cross(&E1);
        if u.length() < 1e-6 {
           u = self.cross(&E2);
        };        
        let other_vec = self.cross(&u);
        (u, other_vec)
    }
}

impl Mul for &Vector3{
    type Output = Vector3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl Mul for Vector3{
    type Output = Vector3;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl Add for &Vector3{
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add for Vector3{
    type Output = Vector3;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


impl Sub for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
       Self::Output{x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z} 
    }
}

impl Sub for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Self) -> Self::Output {
       Self::Output{x : self.x - rhs.x, y : self.y - rhs.y, z : self.z - rhs.z} 
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3; 
    fn div(self, rhs: f64) -> Self::Output {
        Vector3{x: self.x/rhs, y: self.y/rhs, z: self.z/rhs}
    }
}

impl Mul<f64> for &Vector3 {
   type Output = Vector3; 
   fn mul(self, rhs: f64) -> Self::Output {
      Vector3{x: self.x * rhs, y : self.y * rhs, z : self.z * rhs} 
   }
}

impl Mul<f64> for Vector3 {
   type Output = Vector3; 
   fn mul(self, rhs: f64) -> Self::Output {
      Vector3{x: self.x * rhs, y : self.y * rhs, z : self.z * rhs} 
   }
}

impl Mul<&Vector3> for f64{
   type Output = Vector3; 
   fn mul(self, rhs: &Vector3) -> Self::Output {
      Self::Output{x: self * rhs.x, y : self * rhs.y, z : self * rhs.z} 
   }
}

impl Mul<Vector3> for f64{
   type Output = Vector3; 
   fn mul(self, rhs: Vector3) -> Self::Output {
      Self::Output{x: self * rhs.x, y : self * rhs.y, z : self * rhs.z} 
   }
}