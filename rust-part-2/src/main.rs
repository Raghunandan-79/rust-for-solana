struct Address {
    city: String,
    pincode: String,
    country: String,
}

struct User {
    name: String,
    age: u32,
    address: Vec<Address>,
}

impl User {
    fn is_allowed_to_vote(&self, legal_age: u32) -> bool {
        if self.age >= legal_age {
            return true;
        }

        return false;
    }

    fn who_am_i() -> String {
        return String::from("User struct");
    }
}

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}

impl Shape {
    fn calculate_area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Square(side_length) => side_length * side_length,
            Shape::Rectangle(width, height) => {
                width * height
            },
        }
    }
}

fn main() {
    let user1: User = User {
        name: String::from("Harkirat"),
        age: 18,
        address: vec![Address {
            city: String::from("Chandigarh"),
            pincode: String::from("123123"),
            country: String::from("India"),
        }],
    };

    let user2: User = User {
        name: String::from("Raman"),
        age: 13,
        address: vec![],
    };

    println!("User1 name: {}", user1.name);
    println!("User1 city: {}", user1.address[0].city.as_str());
    println!("User1 pincode: {}", user1.address[0].pincode.as_str());
    println!("User1 country: {}", user1.address[0].country.as_str());
    println!("{}", user1.is_allowed_to_vote(18));
    println!("{}", user2.is_allowed_to_vote(20));
    println!("{}", User::who_am_i());

    move_away(Direction::UP);
    move_away(Direction::DOWN);

    let circle: Shape = Shape::Circle(5.0);
    let square: Shape = Shape::Square(4.0);
    let rectangle: Shape = Shape::Rectangle(3.0, 6.0);
    println!("Area of Circle: {}", circle.calculate_area());
    println!("Area of Square: {}", square.calculate_area());
    println!("Area of Rectangle: {}", rectangle.calculate_area());
}

fn move_away(direction: Direction) {
    if direction == Direction::UP {
        println!("Moved in up direction");
    } else if direction == Direction::DOWN {
        println!("Moved in down direction");
    } else if direction == Direction::LEFT {
        println!("Moved in left direction");
    } else if direction == Direction::RIGHT {
        println!("Moved in right direction");
    }
}
