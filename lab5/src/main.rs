use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut xmat: [[i32; 3]; 5] = [[0; 3]; 5];
    let mut ymat: [[i32; 7]; 3] = [[0; 7]; 3];

    println!("{:?}", xmat);
    load_xmat(&mut xmat);
    println!("{:?}", xmat);
    println!("{:?}", ymat);
    load_ymat(&mut ymat);
    println!("{:?}", ymat);
}

fn read_file(file_name: &str) -> Vec<i32> {
    let file = File::open(file_name).expect("Failed to read file");
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let temp_list: Vec<_> = line.split(" ").collect();
    let mut int_vec: Vec<i32> = Vec::new();
    for i in 0..temp_list.len() {
        let val: i32 = temp_list[i].trim().parse::<i32>().expect("could not parse to i32");
        int_vec.push(val);
    }
    int_vec
}

fn load_xmat(matrix: &mut [[i32; 3]; 5]) {
    let vec: Vec<i32> = read_file("xdata.txt");
    let mut index: usize = 0;
    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            matrix[row][col] = vec[index];
            index += 1;
        }
    }
}

fn load_ymat(matrix: &mut [[i32;7];3]) {
    let vec: Vec<i32> = read_file("ydata.txt");
    let mut index: usize = 0;
    for col in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            matrix[row][col] = vec[index];
            index += 1;
        }
    }
}