use test4::graphic::Geometry;
use test4::graphic::Width;
use test4::graphic::Height;
use test4::graphic::Radius;
use test4::graphic::Side;

fn main() {
	let a: f64 = 6.7;
	let b: f64 = 4.2;
	
	let rectangle = Geometry::Rectangle(Width(a), Height(b));
	println!("A rectangle with width {a} and height {b} has area {} and circumference {}",
	rectangle.area(), rectangle.circumference());
	
	let triangle = Geometry::Triangle(Side(a));
	println!("A triangle with side {a} has area {} and circumference {}",
	triangle.area(), triangle.circumference());
	
	let circle = Geometry::Circle(Radius(b));
	println!("A circle with radius {b} has area {} and circumference {}",
	circle.area(), circle.circumference());
}