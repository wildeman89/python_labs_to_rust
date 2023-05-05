// This program focuses on getting user input and then processing that input
// and returning out the results. Very simple program.

use std::io;

fn main() {
    let movie_name: String = get_movie_name();
    let adult_tickets_sold = get_number_of_tickets("How many adult tickets were sold?");
    let child_tickets_sold = get_number_of_tickets("How many childrens tickets were sold?");

    // local variables for processing
    let adult_price: f64 = 6.0;
    let child_price: f64 = 3.0;
    let kept: f64 = 0.20;
    let gross_profit = (adult_price * adult_tickets_sold) + (child_price * child_tickets_sold);
    let net_profit = gross_profit * kept;
    let movie_co_profit = gross_profit - net_profit;

    // Output all the information
    println!("\n\t\tMovie Theater Information");
    println!("Movie Title: {}", movie_name.trim());
    println!("Number of adult tickets sold: {}", adult_tickets_sold);
    println!("Number of children tickets sold: {}", child_tickets_sold);
    println!("Gross Profit   : $ {}", gross_profit);
    println!("Net Profit     : $ {}", net_profit);
    println!("Movie Co Profit: $ {}", movie_co_profit);
}

fn get_movie_name() -> String {
    println!("Enter Movie Title:");
    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read user input");

    name
}

fn get_number_of_tickets(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number = input
        .trim()
        .parse::<f64>()
        .expect("Could not parse input to i32");
    number
}
