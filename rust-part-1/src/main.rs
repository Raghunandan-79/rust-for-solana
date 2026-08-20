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
}
