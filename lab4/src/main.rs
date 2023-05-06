/* Lab 4 - Standard Deviation
This lab is about using lists to store information and using that data
to calculate the standard deviation of test scores. */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let s: i32;
    let sdev: f64;
    let sum_sd1: f64;
    let mut scores: Vec<i32> = Vec::new();
    let mut dev: Vec<f64> = Vec::new();
    let mut dev1: Vec<f64> = Vec::new();
    let mut sd1: Vec<f64> = Vec::new();
    s = load_scores(&mut scores);
    let xbar: f64 = s as f64 / scores.len() as f64;

    let tdev = deviation(xbar, &scores, &mut dev, &mut dev1);
    sdev = (tdev / dev.len() as f64).sqrt();
    sum_sd1 = standard_score(sdev, &dev, &mut sd1);

    println!("\t\t\tStatistical Analysis");
    display_lists(&scores, &dev, &dev1, &sd1);
    println!("Sum={}\t\tAverage={}\t\tStandard Deviation={:.4}", s, xbar, sdev);
    println!("Sum of Standard Score = {:.2}", sum_sd1);
}

// This function will read a file and load the values from the
// file into the vector and then return the sum.
fn load_scores(vec: &mut Vec<i32>) -> i32 {
    let mut s = 0;
    let file = File::open("data.txt").expect("Failed to open file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let temp_list: Vec<&str> = line.split(",").collect();
    for t in temp_list {
        let v = t.trim().parse::<i32>().expect("could not parse to i32");
        s += v;
        vec.push(v);
    }
    s

}

fn deviation(xb: f64, scrs: &Vec<i32>, dev: &mut Vec<f64>, dev1: &mut Vec<f64>) -> f64 {
    let mut s: f64 = 0.0;
    for score in scrs {
        let value = xb - *score as f64;
        let val2 = value.powf(2.0);
        dev.push(value);
        s += val2;
        dev1.push(val2);
    }
    s
}

fn standard_score(sd: f64, dv1: &Vec<f64>, sdl: &mut Vec<f64>) -> f64 {
    let mut s = 0.0;
    for v in dv1 {
        let val = *v / sd;
        s += val;
        sdl.push(val);
    }
    s
}

fn display_lists(scrs: &Vec<i32>, dv: &Vec<f64>, dv1: &Vec<f64>, sd: &Vec<f64>) {
    println!("SCORES\t\t  DEV\t\t\t  DEV1\t\t\t  SD1");
    let length = scrs.len();
    for i in 0..length {
        println!("{:^6}\t\t{:^8.4}\t\t{:^8.4}\t\t{:^8.4}", scrs[i], dv[i], dv1[i], sd[i]);
    }
}