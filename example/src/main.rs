use std::convert::TryFrom;
use std::collections::HashMap;

#[derive(Debug)]
struct PositiveNumber(u32);

impl PositiveNumber {
    fn get(&self) -> u32 {
        self.0
    }
}

impl TryFrom<i32> for PositiveNumber {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value >= 0 {
            Ok(PositiveNumber(value as u32))
        } else {
            Err(format!("{} is not positive", value))
        }
    }
}

// - - - - - - - - - -

fn print_sum(arr: &[i32]) {
    let sum: i32 = arr.iter().sum();
    println!("Sum of array: {}", sum);
}

// - - - - - - - - - -

// This function takes two string slices and returns the longer one.
// We use the same lifetime 'a for both inputs and the return value.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// - - - - - - - - - -    

/// A value can either be a single value or multiple values
#[derive(Debug)]
enum Value<'a> {
    Single(&'a str),
    Multiple(Vec<&'a str>),
}

/// QueryString holds key-value pairs with potential multiple values
#[derive(Debug)]
#[allow(dead_code)]
struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

/// Implement parsing logic for query strings like `a=1&b=2&d=7&d=abc`
impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        // Split query string by '&'
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            // If there's an '=' sign, split into key and value
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];         // everything before '='
                val = &sub_str[i + 1..];     // everything after '='
            }

            // Insert into hashmap or update existing entry
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    // If key already has a single value, convert it to a vec
                    Value::Single(prev_val) => {
                        *existing = Value::Multiple(vec![*prev_val, val]);
                    }
                    // If key already has multiple values, just push the new one
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val)); // If key is new, insert as Single
        }

        QueryString { data }
    }
}

fn main() {
    println!("Hello, world!");

    // - - - - - - - - - -

    let string = String::from("127.0.0.1:8080");
    let string_slice= &string[10..]; //------ borrow of `string` occurs here ------ bad example
    let string_borrowed: &str = &string; //------ borrow of `string` occurs here
    let string_literal = "1234"; // string_literal is a string slice

    // cannot move out of `string` because it is borrowed - Fix by using reference &string
    dbg!(&string);          // [server\src\main.rs:20:5] &string = "127.0.0.1:8080"
    dbg!(string_slice);     // [server\src\main.rs:21:5] string_slice = "8080"
    dbg!(string_borrowed);  // [server\src\main.rs:22:5] string_borrowed = "
    dbg!(string_literal);   // [server\src\main.rs:23:5] string_literal = "1234"

    // - - - - - - - - - -

    // Call print_sum to avoid dead_code warning
    let numbers = [1, 2, 3, 4, 5];
    print_sum(&numbers);

    // - - - - - - - - - -

    // for loop examples in Rust
    for i in 0..=5 {
        println!("i is: {}", i);
    }

    let fruits = ["apple", "banana", "cherry"];

    for fruit in fruits.iter() {
        println!("{}", fruit);
    }

    // - - - - - - - - - -

    // match statement example
    let number: u8 = 3;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Something else"),
    }

    // - - - - - - - - - -

    // Array example
    let n = PositiveNumber::try_from(10);
    println!("{:?}", n); // Ok(PositiveNumber(10))
    if let Ok(pn) = n {
        println!("PositiveNumber value: {}", pn.get());
    }

    let n = PositiveNumber::try_from(-3);
    println!("{:?}", n); // Err("-3 is not positive")
    // Example of type conversion and traits in rust
    let n = PositiveNumber::try_from(10);
    println!("{:?}", n); // Ok(PositiveNumber(10))

    let n = PositiveNumber::try_from(-3);
    println!("{:?}", n); // Err("-3 is not positive")

    // - - - - - - - - - -

    // if let example
    let sub_str = "username=password";
    let (key, val);

    if let Some(i) = sub_str.find('=') {
        // If '=' is found, split the string into key and value
        key = &sub_str[..i];
        // We use `&sub_str[..i]` to get the substring before '='
        // and `&sub_str[i + 1..]` to get the substring after
        val = &sub_str[i + 1..];

        println!("Key: {}", key);   // "username"
        println!("Value: {}", val); // "password"
    } else {
        println!("No '=' found in the string.");
    }

    // - - - - - - - - - -

    // Lifetime example
    let string1 = String::from("hello");
    let string2 = String::from("world!!!");

    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
    // 'a is a lifetime parameter — it says:
    // "The returned reference will live at least as long as both x and y."
    let result = longest(&string1, &string2);

    println!("The longest string is: {}", result);

    // - - - - - - - - -

    // Example of using QueryString
    let input = "a=1&b=2&c&d=&e===&d=7&d=abc";
    let qs: QueryString = input.into();

    println!("{:#?}", qs);

}
