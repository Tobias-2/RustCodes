//graphic: write two methods that calculate the area and circumference of a graphical object, 
//which can either be a rectangle, a right triangle with sides of equal length, or a circle.

pub struct Width(pub f64);
pub struct Height(pub f64);
pub struct Side(pub f64);
pub struct Radius(pub f64);

pub enum Geometry {
	Rectangle(Width, Height), 
	Triangle(Side),
	Circle(Radius),
}

impl Geometry {
	pub fn area(&self) -> f64 {
		use crate::graphic::Geometry::*;
		match self {
			Rectangle(Width(w), Height(h)) => w * h,
			Triangle(Side(s)) => (s * s) / 2.0,
			Circle(Radius(r)) => r * r * 3.14,
		}
	}
	
	pub fn circumference(&self) -> f64 {
		use crate::graphic::Geometry::*;
		match self {
			Rectangle(Width(w), Height(h)) => (w + h) * 2.0,
			Triangle(Side(s)) => s + s + 1.41 * s,
			Circle(Radius(r)) => 2.0 * r * 3.14,
		}
	}
}

