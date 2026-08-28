use std::vec::IntoIter;

use borsh::{BorshDeserialize, BorshSerialize};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32
}

fn serde() {
    let person: Person = Person { 
        name: String::from("John Doe"), 
        age: 30 
    };

    // serialize to JSON
    let json_str: String = serde_json::to_string(&person).unwrap();
    println!("Serialized json: {}", json_str);

    // deserialize from JSON
    let deserialzed_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized prson: {:?}", deserialzed_person);

    // serialize to toml
    let toml_str: String = toml::to_string(&person).unwrap();
    println!("Serialized toml: {}", toml_str);

    // deserialize from toml
    let deserialized_toml_person: Person = toml::from_str(&toml_str).unwrap();
    println!("Deserialized toml person: {:?}", deserialized_toml_person); 
}

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>
}

fn borsh() {
    let original: MyStruct = MyStruct { 
        id: 42, 
        data: "Hello Borsh!".into(), 
        v: vec![1, 2, 3] 
    };

    let mut buffer: Vec<u8> = Vec::new();
    original.serialize(&mut buffer).unwrap();

    let deserialized = MyStruct::try_from_slice(&mut buffer).unwrap();
    
    assert_eq!(original, deserialized);

    println!("Successfully serialized and deserialized: {:?}", deserialized);
}

fn iterators() {
    let v: Vec<i32> = vec![1, 2, 3];

    let mut v_iter: IntoIter<i32> = v.into_iter();

    while let Some(i) = v_iter.next() {
        print!("{} ", i);
    }
    println!();

    let first_element: Option<i32> = v_iter.next();
    if let Some(element) = first_element {
        println!("{}", element);
    } else {
        println!("Vector ended");
    }
}

fn main() {
    serde();
    borsh();
    iterators();
}
