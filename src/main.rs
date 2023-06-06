use std::ops::{Add, Sub};
struct Vector{
    x:f64,
    y:f64
}

impl Vector {
    fn new(x:f64, y:f64)->Vector{
        Vector{x,y}
    }
    fn mag(&self)->f64{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl Add<Vector> for Vector{
    type Output = Vector;
    fn add(self, other: Vector)->Vector{
        Vector::new(self.x+other.x, self.y+other.y)
    }
}
impl Sub<Vector> for Vector{
    type Output = Vector;
    fn sub(self, other: Vector)->Vector{
        Vector::new(self.x-other.x, self.y-other.y)
    }
}

fn main(){
    println!("HELLO WORLD");
}