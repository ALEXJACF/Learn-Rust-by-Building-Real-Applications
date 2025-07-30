use std::io;

const EARTH_GRAVITY: f32 = 9.81;
const MARS_GRAVITY: f32 = 3.711;

// Macro example in Rust to calculate weight on Mars.
macro_rules! calculate_weight_on_mars {
    ($weight:expr) => {
        ($weight / EARTH_GRAVITY) * MARS_GRAVITY
    };
}

fn main() {
    println!("Enter your weight (kg): ");
    // Mutable string to hold user input.
    let mut input = String::new();

    // If result was not ok, unwrap will terminate the program.
    // If result was ok, unwrap will return the value.
    io::stdin().read_line(&mut input).unwrap();

    //Remove whitespace and convert to f32.
    let weight: f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weight_on_mars(weight);

    // Needs mut keyword to modify the variable (Kg to grams).
    //mars_weight = mars_weight * 1000.0;  // ERROR: cannot assign twice to immutable variable `mars_weight`

    // Floating-point number to 2 decimal places.
    println!("Function: Weight on Mars: {{{:.2} kg}}", mars_weight);

    println!("Macro: Weight on Mars: {{{:.2} kg}}", calculate_weight_on_mars!(weight));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / EARTH_GRAVITY) * MARS_GRAVITY
}
