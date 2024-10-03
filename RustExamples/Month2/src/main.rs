/*
 * Example Solution for Fall 2024 CS Club Coding Competition Problems Month 2
 * ICPC Mid-Atlantic USA 2022 Division 2
 * Problem A:
 * 
 * Espresso!
 * 
 */

use std::io::stdin;

fn main(){

    //================Step 0================
    //Input and Variables

    println!("enter input:");

    //rust input is done with stdin().readline(), with a mutable buffer string passed as an argument
    let mut input_line = String::new();

    //expect() gets the string out of the Result, crashing (with a message) if error
    stdin().read_line(&mut input_line).expect("read_line should have worked");

    //trim() to get rid of trailing newline, split() at the space, and collect() into a Vector (dynamic array)
    let input_line: Vec<&str> = input_line.trim().split(" ").collect();

    //get r and f (as i32s) by parsing the two elements in the input_line vec
    let n: i32 = input_line[0].parse().expect("make sure its two numbers separated by a space");
    let s: i32 = input_line[1].parse().expect("make sure its two numbers separated by a space");

    let mut water = s;
    let mut refills = 0;
    
    //================Step 1================
    //Iteration and Parsing

    //Loop n times
    for _i in 0..n {

        println!("water: {}", water);

        //Grab the next line
        let mut order = String::new();
        stdin().read_line(&mut order).expect("read_line should have worked");

        //to get the digit out of the line, I use
        //the same method as above for getting a digit out of a String, using
        //Rust's nice dynamic parse()
        let mut water_used: i32 = order[0..1].parse::<i32>().expect("espresso shot count should be a number");
        
        //now, I 'could' check if the second character is *specifically*
        //the target 'L', but I trust that competition rules guarantee
        //valid input test data
        //therefor, if there exists a second character *at all*, I assume
        //that the order is a latte
        if order.trim().chars().count() > 1 { water_used += 1; }

        //We now know how much water is used for the current order!

        //================Step 2================
        //Order Fulfillment

        //first check if we have enough water
        //if not, refill it!
        if water < water_used {
            water = s;
            refills += 1;
        }

        //and then go ahead and make the order! (use the water)
        water -= water_used;

    }

    //================Step 3================
    //Output

    //well, after every order we should know how many refills
    //John needed! print it!

    println!("{}",refills);

}

/* * * Notes/Thoughts: * * *
 * 
 * Remember, this is *my* interpretation/solution, you may come at this in a different way
 * with a different programming style, thats totally okay! This is also *very* verbose,
 * to emphasize the problem-solving. Feel free to hyper-optimize your code.
 * 
 */
/* * * Reading and understanding the problem:
 * 
 * Alright, in this problem, we need to help John make some espresso for students.
 * Specifically, we need to figure out how many times John needs to refill
 * the water tank on his espresso machine to serve all n students.
 * 
 * The espresso machine has a tank of water that holds s ounces. 
 * The tank of water starts the shift full. Making coffee takes
 * 1 ounce of water per 1 shot of espresso, plus one additional ounce if it's
 * a latte. Student's orders tell us how many shots of espresso they want,
 * and whether they want a latte or not. We have to go through every order and
 * refill the water tank only when there isn't enough water to make the next order.
 * 
 */
/* * * Input Structure:
 * 
 * Multiple lines:
 * 
 * n s
 * el       |
 * el       |
 * el       |--n times
 * .        |
 * .        |
 * 
 * where n (1 <= n <= 100) is how many orders,
 * and s (10 <= s <= 200) is how many ounces of water the tank holds.
 * Every line after the first has either one or two characters:
 * e - a number between 1 and 4 (inclusive) for how many shots of espresso
 * l - either an uppercase L for a latte, or *nothing* (empty, no character)
 * 
 */
/* * * Output Structure:
 *  
 * One line:
 * 
 * The number of times the water tank needs to be refilled
 * 
 */
/* * * Solving the problem:
 * 
 * We need to:
 * 0) Gather input and store values, 1) loop through the incoming orders
 * and compute how much water they use, 2) calculate the water level for
 * the tank after each order, refilling as needed, and 
 * 3) output how many times the water was refilled.
 * 
 * Step 0:  Input and Variables
 *          In Java, we use a java.util.Scanner object to take input.
 *          We need two variables, n and s, so we just take those as ints
 *          from the input and store them. We also need to keep track of
 *          how many times the water was refilled, so we can initialize that
 *          here.
 * 
 * Step 1:  Iteration and Parsing
 *          We know how many lines to expect, so we can just loop that
 *          many times and take a line at a time. We do need to parse 
 *          the line for each order, since it's taken in as a String. 
 *          The first character is always a digit, so we can parse 
 *          that as an integer, and if the L is there,
 *          we just need to account for the one extra ounce of water.
 *          
 * Step 2:  Order Fulfillment
 *          Once we have the amount of water used for an order,
 *          we can subtract it from our tank! If we don't have enough
 *          water to do so, refill the tank first!
 * 
 * Step 3:  Output
 *          Finishing up is as easy as outputting our refill count variable!
 * 
 * * * * * * * * * * * * * *
 */