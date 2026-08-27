use std::f32::consts::PI;

trait Shape {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
}

struct Rect {
    width: f32,
    height: f32,
}

struct Circle {
    radius: f32,
}

impl Shape for Rect {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> f32 {
        return 2.0 * (self.width + self.height);
    }
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        return PI * self.radius * self.radius;
    }

    fn perimeter(&self) -> f32 {
        return 2.0 * PI * self.radius;
    }
}

fn get_perimeter_and_area<T: Shape>(shape: &T) -> (f32, f32) {
    return (shape.area(), shape.perimeter());
}

fn traits() {
    let rect: Rect = Rect {
        width: 10.0,
        height: 20.0,
    };

    let circle: Circle = Circle { radius: 10.25 };

    println!("Area of rectangle is: {}", get_perimeter_and_area(&rect).0);
    println!("Perimeter of rectangle is: {}", get_perimeter_and_area(&rect).1);

    println!("Area of circle is: {}", get_perimeter_and_area(&circle).0);
    println!("Perimeter of circle is: {}", get_perimeter_and_area(&circle).1);
}

fn macros() {

}

fn main() {
    traits();
    macros();
}
