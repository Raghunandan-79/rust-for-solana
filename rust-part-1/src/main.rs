fn main() {
    println!("Hello, world!");

    println!();
    println!("Data Types");
    // Data types
    // numbers
    let mut x: i32 = 1;
    println!("{}", x);
    x = 10;
    println!("{}", x);

    // booleans
    let is_male = false;
    let is_above_18: bool = true;

    if is_male {
        println!("You are a male");
    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        print!("You are a legal male");
    }

    // strings
    let greeting: String = String::from("Hello world");
    println!("{}", greeting);
    let mut greeting2: String = String::from("Good morning, ");
    greeting2.push_str(&String::from("Raghu"));
    println!("{}", greeting2);

    // arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", arr.len());

    // vectors
    let mut xs: Vec<i32> = vec![1, 2, 3];
    println!("{}", xs.len());
    xs.push(4);
    println!("{}", xs.len());
    println!("{:?}", xs);

    println!();
    
    // Conditionals and Loops
    println!("Conditionals and Loops");
    let x: i32 = 99;
    if is_even(x) {
        println!("{} is even", x);
    } else {
        println!("{} is odd", x);
    }

    let str: String = String::from("Raghunadan Sharma");
    println!("First name: {}", get_first_name(str));

    println!();

    // Borrowing and References
    println!("Borrowing and References");
    let str: String = String::from("Raghunandan");
    let len = get_length(&str);
    println!("{} {}", str, len);
}

pub fn is_even(x: i32) -> bool {
    return x % 2 == 0;
}

pub fn get_first_name(str: String) -> String {
    let mut first_name: String = String::from("");

    for c in str.chars() {
        if c == ' ' {
            break;
        }
        first_name.push(c);
    }

    return first_name;
}

fn get_length(str: &String) -> usize {
    let len = str.len();
    return len;
}