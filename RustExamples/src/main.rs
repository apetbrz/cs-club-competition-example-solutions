/*
 * Example Solution for Fall 2024 CS Club Coding Competition Problems Month 1
 * ICPC Mid-Atlantic USA 2022 Division 2
 * Problem A:
 * 
 * Blueberry Waffle
 * 
 */

use std::io::stdin;

fn main() {

    //================Step 0================
    //Input and Variables

    println!("enter input:");

    //rust input is done with stdin().readline(), with a mutable buffer string passed as an argument
    let mut input_line = String::new();

    //unwrap() gets the string out of the Result, crashing if error
    stdin().read_line(&mut input_line).unwrap();

    //trim() to get rid of trailing newline, split at the space, and collect into a Vector (dynamic array)
    let input_line: Vec<&str> = input_line.trim().split(" ").collect();

    //get r and f (as i32s) by parsing the two elements in the input_line vec
    let r: i32 = input_line[0].parse().unwrap();
    let f: i32 = input_line[1].parse().unwrap();

    //================Step 1================
    //Calculating Rotation

    //if we just did f/r, it would be integer division!
    //so make sure to tell the code to treat f/r as a floating 
    //point number, not an integer!
    let flips: f64 = f as f64 / r as f64;

    //================Step 2================
    //Horizontal Snap

    let flips = (flips + 0.5) as i32;
    //This line rounds by adding 0.5 to the decimal value,
    //then cutting off the decimal! (by converting to integer and back)
    //you can use whatever rounding method you want

    //================Step 3================
    //Upfacing Side & Output

    if flips % 2 == 0 {
        println!("up");
    }
    else{
        println!("down");
    }

}