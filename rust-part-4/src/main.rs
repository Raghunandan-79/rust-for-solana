use std::{f32::consts::PI, fmt::Display};

use serde::{Deserialize, Serialize};

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

macro_rules! say_hello {
    () => {
        println!("Hello world!");
    };
}

// Defining a create_funtion macro
macro_rules! create_funtion {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_funtion!(hello);

// procedural macros
#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rectangle has a width of {}, and height of {}", self.width, self.height)
    }
}

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    username: String,

    #[serde(rename = "pass_word")]
    password: String,

    #[serde(rename = "user_age")]
    age: u32
}

fn macros_applied_to_attributes() {
    let user: User = User { 
        username: String::from("Alice"), 
        password: String::from("password123"), 
        age: 30, 
    };

    let json: String = serde_json::to_string(&user).unwrap();
    println!("{}", json);
}

fn macros() {
    say_hello!();
    hello();

    let rect: Rectangle = Rectangle { 
        width: 10, 
        height: 30 
    };
    println!("{:?}", rect);
    println!("Width: {}", rect.width);
    println!("Height: {}", rect.height);

    let rect2: Rectangle = Rectangle {
        width: 10, 
        height: 40 
    };
    
    if rect == rect2 {
        println!("They are equal");
    } else {
        println!("They are not equal");
    }

    println!("{}", rect);

    macros_applied_to_attributes();
}

fn main() {
    traits();
    macros();
}
