use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    let mut xmat: [[i32; 3]; 5] = [[0; 3]; 5];
    let mut ymat: [[i32; 7]; 3] = [[0; 7]; 3];
    // zmat is xmat * ymat. So it has the rows of xmat, cols of ymat
    let mut zmat: [[i32; 7]; 5] = [[0; 7]; 5];

    load_xmat(&mut xmat);
    load_ymat(&mut ymat);
    load_zmat(&xmat, &ymat, &mut zmat);
    let (xsum, ysum) = compute_sums(&xmat, &ymat);
    let small = get_smallest(&ymat);
    display_matrix(&xmat, &ymat, &zmat);
    println!("Xmat Sum: {}", xsum);
    println!("Ymat Sum: {}", ysum);
    println!("Sum: {}", xsum+ysum);
    println!("Smallest: {}", small);
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

fn load_zmat(amat: &[[i32;3];5], bmat: &[[i32;7]; 3], cmat: &mut [[i32;7];5]) {
    for row in 0..5 {
        for col in 0..7 {
            let mut val: i32 = 0;
            for i in 0..3 {
                val += amat[row][i] * bmat[i][col];
            }
            cmat[row][col] = val;
        }
    }
}

fn compute_sums(amat: &[[i32;3];5], bmat: &[[i32;7];3]) -> (i32, i32) {
    let mut amat_sum: i32 = 0;
    let mut bmat_sum: i32 = 0;

    for row in amat {
        amat_sum += row[2];
    }

    for col in bmat[2] {
        bmat_sum += col;
    }
    (amat_sum, bmat_sum)
}

fn get_smallest(matrix: &[[i32;7];3]) -> i32 {
    let mut small: i32 = 0;
    for col in matrix[1] {
        if small > col {
            small = col;
        }
    }
    small
}

fn display_matrix(mat1: &[[i32;3];5], mat2: &[[i32;7];3], mat3: &[[i32;7];5]) {
    println!("X Matrix");
    for row in 0..mat1.len() {
        for col in mat1[row] {
            print!("{:^5}", col);
        }
        println!("");
    }
    println!("");

    println!("Y Matrix");
    for row in 0..mat2.len() {
        for col in mat2[row] {
            print!("{:^5}", col);
        }
        println!("");
    }
    println!("");

    println!("Z Matrix");
    for row in 0..mat3.len() {
        for col in mat3[row] {
            print!("{:^7}", col);
        }
        println!("");
    }
    println!("");
}