use std::ops::{Add, Sub, Mul, Div};
use std::fmt;
use std::fs::File;
#[warn(non_snake_case)]
// use termcolor::{Color, ColorChoice, ColorSpec, StandartStream, WriteColor};
struct Vector{
    x:f64,
    y:f64
}

impl Vector {
    fn new(x:f64, y:f64)->Vector{
        Vector{x,y}
    }
}

impl fmt::Display for Vector{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "<{}, {}>",self.x, self.y)
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
impl Mul<Vector> for Vector{
    type Output = Vector;
    fn mul(self, other: Vector)->Vector{
        Vector::new(self.x*other.x, self.y*other.y)
    }
}
impl Div<Vector> for Vector{
    type Output = Vector;
    fn div(self, other:Vector)->Vector{
        Vector::new(self.x/other.x, self.y/other.y)
    }
}
fn mag(v: &Vector) -> f64{
    (v.x*v.x + v.y*v.y).sqrt()
}
fn main(){
    let mut file = File::create("output.txt").expect("File Wasnt Created");
}