use std::fs;

fn main() {
    error_handling();
    option_enum();
    generics_and_trait_bounds();
}

fn error_handling() {
    let file_content: String = fs::read_to_string("a.txt").unwrap_or(String::from("Empty file"));
    println!("{}", file_content);

    let file_content2 = fs::read_to_string("hello.txt").unwrap();
    println!("{}", file_content2);
}

fn option_enum() {
    let value: Option<u32> = find_first_a(String::from("harkirat"));
    
    match value {
        Some(value) => println!("First a was found at {}", value),
        None => println!("Couldn't find an a inside the string")
    }
}

fn find_first_a(s: String) -> Option<u32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u32);
        }
    }

    return None;
}

fn generics_and_trait_bounds() {
    println!("{}", mul_generic(10, 20));

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<String> = vec![String::from("Harkirat"), String::from("Singh")];
    let v3: Vec<f64> = vec![1.0, 2.0, 3.0];

    println!("{}", first_element(v1).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());

    let r1: Rect<i32> = Rect {
        width: 20,
        length: 20,
    };
    println!("Area of first rectangle: {}", get_area(r1));

    let r2: Rect<f64> = Rect { 
        width: 10.254, 
        length: 20.254, 
    };
    println!("Area of second rectangle: {}", get_area(r2));
}

struct Rect<T> {
    width: T,
    length: T,
}

fn get_area<T: std::ops::Mul<Output = T>>(v: Rect<T>) -> T {
    return v.length * v.width;
}

fn mul_generic<T: std::ops::Mul<Output = T>>(a: T, b: T) -> T {
    return a * b;
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0);
}
