// This code is about the use of multiplication of two matrices in Rust the user will input the size of the matrix and the elements of the matrix and the program will output the result of the multiplication of the two matrices.

use std::io;

fn main() {
    println!("Enter the size of the matrix");
    let mut size = String::new();
    io::stdin().read_line(&mut size).expect("Failed to read line");
    let size: usize = size.trim().parse().expect("Please type a number!");

    let mut matrix1 = vec![vec![0; size]; size];
    let mut matrix2 = vec![vec![0; size]; size];
    let mut result = vec![vec![0; size]; size];

    println!("Enter the elements of the first matrix");
    for i in 0..size {
        for j in 0..size {
            let mut element = String::new();
            io::stdin().read_line(&mut element).expect("Failed to read line");
            let element: i32 = element.trim().parse().expect("Please type a number!");
            matrix1[i][j] = element;
        }
    }

    println!("Enter the elements of the second matrix");
    for i in 0..size {
        for j in 0..size {
            let mut element = String::new();
            io::stdin().read_line(&mut element).expect("Failed to read line");
            let element: i32 = element.trim().parse().expect("Please type a number!");
            matrix2[i][j] = element;
        }
    }

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix1[i][k] * matrix2[k][j];
            }
        }
    }

    println!("The result of the multiplication of the two matrices is:");
    for i in 0..size {
        for j in 0..size {
            print!("{} ", result[i][j]);
        }
        println!();
    }
}




