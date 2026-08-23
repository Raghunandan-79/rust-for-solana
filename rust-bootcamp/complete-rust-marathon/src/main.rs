/*
    Part-2 Advanced Rust APIs
        1. Collections, Vectors
        2. Iterators
        3. Hashmaps
        4. Strings, &str and slices
        5. Generics
        6. Traits
        7. Multithreading
        8. Macros
        10. Lifetimes
*/

use std::{collections::HashMap, sync::mpsc, thread, time::Duration};

fn main() {
    vectors();
    hashmaps();
    iterators();
    strings_vs_slices();
    traits();
    multithreading();
    message_passing();
}

fn vectors() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);
    vec.push(3);
    println!("{:?}", vec);
    println!("{:?}", even_filter(&vec));
    println!("{:?}", vec);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = Vec::new();

    for val in vec.iter() {
        if *val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    return new_vec;
}

fn hashmaps() {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Raghunandan"), 21);
    users.insert(String::from("Harkirat"), 32);
    let user1: Option<&i32> = users.get("Raghunandan");
    println!("{}", user1.unwrap());
    println!("{:?}", users);

    let second_user_age: Option<&i32> = users.get("Harkirat");
    match second_user_age {
        Some(age) => println!("age is {}", age),
        None => println!("User not found in the DB"),
    }

    let pairs: Vec<(String, i32)> =
        vec![(String::from("harkirat"), 21), (String::from("raman"), 31)];

    let grouped_pairs = group_values_by_keys(pairs);
    println!("{:?}", grouped_pairs);
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm: HashMap<String, i32> = HashMap::new();

    for (key, value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn iterators() {
    let nums: Vec<i32> = vec![1, 2, 3];
    for num in &nums {
        print!("{} ", num);
    }
    println!();

    let iter = nums.iter();
    for value in iter {
        print!("{} ", value);
    }
    println!();

    let mut nums: Vec<i32> = vec![4, 5, 6];
    let iter = nums.iter_mut();

    for value in iter {
        *value = *value + 1;
    }
    println!("{:?}", nums);

    let mut iter = nums.iter();
    while let Some(val) = iter.next() {
        print!("{} ", val);
    }
    println!();

    let iter = nums.into_iter();
    for value in iter {
        print!("{} ", value);
    }
    println!();

    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    println!("{}", total);
    assert_eq!(total, 6);
}

fn strings_vs_slices() {
    let mut name: String = String::from("Harkirat");
    name.push_str(" Singh");
    println!("name is {}", name);
    name.replace_range(8..name.len(), "");
    println!("name is {}", name);

    name.push_str(" Singh");
    println!("name is {}", name);
    let ans: &str = first_word(&name);
    println!("ans is {}", ans);

    let word: String = String::from("Hello world");
    let word2 = &word[0..5];
    println!("{}", word2);
}

fn first_word(str: &String) -> &str {
    let mut space_index = 0;

    for i in str.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }

    return &str[0..space_index];
}

fn traits() {
    let user: User = User {
        name: String::from("Raghunandan"),
        age: 21,
    };

    println!("{}", user.summarize());
    notify(&user);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn multithreading() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }
}

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val: String = String::from("hi");
        tx.send(val).unwrap();
    });

    let received: String = rx.recv().unwrap();
    println!("Got: {received}")
}
