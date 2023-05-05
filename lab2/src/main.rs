// This is the rewriting of lab2. Now, in the python course, I don't think lists
// have been covered yet, let alone a dictionary or classes. In the first rewrite,
// even though functions weren't covered by that point in the class, I still used
// them to help clean up the program. I will use some functions.
use std::io;

fn main() {
    println!("\nThis program will calculate the total cost of a house for");
    println!("5 years of ownership. It will ask for information on 3 houses.");

    // local variables
    let period: u8 = 5;
    for i in 0..3 {
        let init_cost = get_input(format!("Enter Initial Cost for House #{}", i + 1));
        let annual_fuel_cost = get_input(format!("Enter Annuel Fuel Cost for House #{}", i + 1));
        let tax_rate = get_input(format!("Enter Tax Rate for House #{}", i + 1));

        let tax_cost = (init_cost * tax_rate) * period as f64;
        let total_cost = init_cost + (annual_fuel_cost * period as f64) + tax_cost;
        println!(
            "Initial Cost: {}\t\tAnnual Fuel Cost: {}\t\tTax Rate: {}",
            init_cost, annual_fuel_cost, tax_rate
        );
        println!(
            "Total House #{} Cost for {}years: $ {}",
            i + 1,
            period,
            total_cost
        );
    }
}

fn get_input(prompt: String) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let number = input
        .trim()
        .parse::<f64>()
        .expect("couldn't parse input to f64");
    number
}
