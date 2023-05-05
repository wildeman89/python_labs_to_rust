/* Lab 3 - Baseball stats
This lab reads from a data file, and uses functions to do some of the processing.
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").expect("Failed to open the file!");
    let reader = BufReader::new(file);

    println!("Player\tBatting Average\t\tSlugging Average");
    for line in reader.lines() {
        let temp = line.unwrap();
        let temp_list: Vec<&str> = temp.split(",").collect();
        let player = temp_list[0];
        let s1 = temp_list[1].trim().parse::<i32>().expect("Could not parse str to i32");
        let d1 = temp_list[2].trim().parse::<i32>().expect("Could not parse &str to i32");
        let t1 = temp_list[3].trim().parse::<i32>().expect("Could not parse &str to i32");
        let hr1 = temp_list[4].trim().parse::<i32>().expect("Could not parse &str to i32");
        let atbat = temp_list[5].trim().parse::<i32>().expect("Could not parse &str to i32");

        let s2 = single(s1);
        let d2 = double(d1);
        let t2 = triple(t1);
        let hr2 = home_run(hr1);
        let bat_avg: f64 = (s1 + d1 + t1 + hr1) as f64 / atbat as f64;
        let slug_avg: f64 = (s2 + d2 + t2 + hr2) / atbat as f64;

        println!("{:^6}\t{:^15.4}\t\t{:^15.4}", player, bat_avg, slug_avg);
    }
}

// functions
fn single(num: i32) -> f64 {
    1.0 * num as f64
}

fn double(num: i32) -> f64 {
    2.0 * num as f64
}

fn triple(num: i32) -> f64 {
    3.0 * num as f64
}

fn home_run(num: i32) -> f64 {
    4.0 * num as f64
}