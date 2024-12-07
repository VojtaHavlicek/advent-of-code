// PROBLEM 1 
use std::fs::File; // Import the fs module
use std::io::{self, BufRead}; // Import the io module

fn main() -> std::io::Result<()> 
{
    let file = File::open("input.txt")?; // Read the contents of the file into a string
    let reader = io::BufReader::new(file); // Create a new BufReader
    
    let mut col1 = Vec::new();
    let mut col2 = Vec::new();

    // Parse the file
    for line in reader.lines() // Iterate over the lines in the file
    {
        let line = line?; // Unwrap the line

        let numbers:Vec<i32> = line
        .split_whitespace()
        .filter_map(|m|m.parse().ok()) // Parse the numbers
        .collect(); // Check this? 

        if numbers.len() == 2 {
            col1.push(numbers[0]);
            col2.push(numbers[1]);
        } else {
            eprintln!("Invalid line: {}", line);
        }
    }

    println!("Column 1: {:?}", col1);
    println!("Column 2: {:?}", col2);
   
    // Sort the vectors 
    col1.sort();
    col2.sort();

    // Compute the absolute difference
    let mut diff = 0;
    for i in 0..col1.len() {
        diff += (col1[i] - col2[i]).abs();
    }

    println!("The sum of the absolute differences is: {}", diff);

    Ok(()) // Return an empty Result
}

// PROBLEM 2? 
