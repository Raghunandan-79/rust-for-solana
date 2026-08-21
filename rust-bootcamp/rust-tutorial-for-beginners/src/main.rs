use std::{fs, io::Error};

fn main() {
    println!("Hello, world!");

    // Variables in rust
    let x: i32 = 1;
    println!("x: {}", x);

    let y: f32 = 1000.001;
    println!("y: {}", y);

    let x: bool = true;
    if x {
        println!("true");
    } else {
        println!("false")
    }

    let greeting: String = String::from("Good morning!");
    println!("{}", greeting);
    println!("{:?}", greeting.chars().nth(9));

    // conditionals
    let is_even: bool = true;

    if is_even {
        println!("The number is even");
    } else {
        println!("The number is odd");
    }

    // loops
    for i in 0..4 {
        println!("{}", i);
    }

    let sentence: String = String::from("Hello World!");
    let first_word: String = get_first_word(&sentence);
    println!("First word of sentence {} is: {}", sentence, first_word);

    // Structs
    let user1: User = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("User 1 username: {:?}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign in count: {}", user1.sign_in_count);

    let rect: Rect = Rect { 
        width: 30, 
        height: 50
    };
    println!("The area of the rectangle is {}", rect.area());

    let my_direction: Direction = Direction::North;
    let new_direction: Direction = my_direction;
    move_around(new_direction);

    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(3.0, 6.0);

    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));

    let res: Result<String, Error> = fs::read_to_string("example.txt");

    match res {
        Ok(content) => {
            println!("File content: {}", content);
        },
        Err(err) => {
            println!("Error: {}", err);
        }
    }

    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Option::Some(index) => println!("The letter 'a' is found at index: {}", index),
        Option::None => println!("The letter 'a' is not found in the string."),
    }
}

fn get_first_word(sentence: &String) -> String {
    let mut ans: String = String::from("");

    for char in sentence.chars() {
        ans.push_str(&char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }

    return ans;
}

// structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "the rectangle prints like this{}", self.width * self.height)
    }
}

// enums
#[derive(PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn move_around(direction: Direction) {
    if direction == Direction::North {
        println!("Moving North");
    } else if direction == Direction::East {
        println!("Moving East");
    } else if direction == Direction::South {
        println!("Moving South");
    } else if direction == Direction::West {
        println!("Moving West");
    }
}

// Pattern matching
enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height ) => width * height,
    }
}

// option enum
pub enum Option<T> {
    None,
    Some(T),
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Option::Some(index as i32);
        }
    }

    return Option::None;
}
