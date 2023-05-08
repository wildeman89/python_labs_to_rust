// This lab originally uses more lists to store information, however,
// I wrote it to use a dictionary to store the restaurant information. As such,
// I will use a struct to hold the information of the restaurants.

use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Record {
    name: String,
    food_type: String,
    reservation: bool,
    rating: u8,
    credit_card: bool,
}

fn main() {
    let mut restaurants: Vec<Record> = Vec::new();
    load_restaurants("data.txt", &mut restaurants);

    accept_credit_cards(&restaurants);
    three_or_more_stars(&restaurants);
    chinese_or_thai_three_stars(&restaurants);
    health_food(&restaurants);
    reservations(&restaurants);
    ascending_ratings(&restaurants);
}

fn load_restaurants(file_name: &str, rest_list: &mut Vec<Record>) {
    let file = File::open(file_name).expect("Failed to read file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let temp_line = line.unwrap();
        let temp_list: Vec<&str> = temp_line.split(",").collect();
        let name = temp_list[0].to_string();
        let food_type = temp_list[1].to_string();
        let reservation: bool;
        if temp_list[2].eq("yes") {
            reservation = true;
        } else {
            reservation = false;
        }
        let rating = temp_list[3]
            .trim()
            .parse::<u8>()
            .expect("could not parse to u8");
        let credit_card: bool;
        if temp_list[4].eq("yes") {
            credit_card = true;
        } else {
            credit_card = false;
        }

        let rec: Record = Record {
            name,
            food_type,
            reservation,
            rating,
            credit_card,
        };
        rest_list.push(rec);
    }
}

fn accept_credit_cards(records: &Vec<Record>) {
    println!("The following Restaurants accept Credit Cards:");
    for record in records {
        if record.credit_card {
            println!("\t{}", record.name);
        }
    }
    println!("");
}

fn three_or_more_stars(records: &Vec<Record>) {
    println!("The following Restaurants have a 3 star rating or higher:");
    for record in records {
        if record.rating >= 3 {
            println!("\t{}", record.name);
        }
    }
    println!("");
}

fn chinese_or_thai_three_stars(records: &Vec<Record>) {
    println!("The following Restaurants serve either Chinese or Thai, and have 3 or more stars:");
    for record in records {
        if (record.food_type.eq("Chinese") || record.food_type.eq("Thai")) && record.rating >= 3 {
            println!("\t{}", record.name);
        }
    }
    println!("");
}

fn health_food(records: &Vec<Record>) {
    println!("The following Restaurants serve Health Food:");
    for record in records {
        if record.food_type.eq("Health") {
            println!("\t{}", record.name);
        }
    }
    println!("");
}

fn reservations(records: &Vec<Record>) {
    println!("The following Restaurants accept Reservations:");
    for record in records {
        if record.reservation {
            println!("\t{}", record.name);
        }
    }
    println!("");
}

fn ascending_ratings(records: &Vec<Record>) {
    let mut count = 1;
    while count <= 5 {
        println!("The following Restaurants have a {} star rating:", count);
        for record in records {
            if record.rating == count {
                println!("\t{}", record.name);
            }
        }
        println!("");
        count += 1;
    }
}
