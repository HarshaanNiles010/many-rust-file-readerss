#[allow(unused_imports)]
use std::{fs::{self, File}, io::{self, BufRead}, path::Path};

fn main(){
    let input = "src/bin/input3.txt";
    let file_content = parse_file(input);
    let numbers = get_numbers(file_content.clone());
    let results = gen_numbers(numbers.clone());
    dbg!(file_content);
    dbg!(numbers);
    dbg!(results);
}

fn parse_file(input: &str) -> Vec<String>{
    let path = Path::new(input);
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut res: Vec<String> = Vec::new();
    for line in reader.lines(){
        let line = line.expect("Failed to read the file");
        res.push(line);
    }
    return res;
}
// This is a function that takes an alphanumeric string i.e 1abc2 and gets all the numbers 
// Example "1abc2" -> [1,2]
fn get_numbers(file_content: Vec<String>) -> Vec<Vec<i32>>{
    // This will be the resultant vector containing all the sub vectors of numbers in each string
    let mut res: Vec<Vec<i32>> = Vec::new();
    // Iterate over each alphanumeric strings
    for content in file_content{
    // Convert each string to a vector of Chars
        let new_temp: Vec<char> = content
            .chars()
    // Filter out all the non digit charachters from the string
            .filter(|c| c.is_digit(10)).collect();
    // Convert the output from previous step into a iterator and use map to convert into digit and cast it from an u8 to i32 for general purpose use
    // Then use the collect function to collect the output into a vector which then will be stored into the result vector
        res.push(new_temp.iter().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>());
    }
    // Finally return the vector to be used later
    return res;
    // ***IMPORTANT*** There is no error handling present that checks this against strings devoid of any numbers.
}
// This function will take the given vector of vector of numbers and convert them to two digit numbers
// Example [1,2] -> 12
// Example [1,2,3,4,5] -> 15
// If only one number is given then [1] -> 11
fn gen_numbers(numbers: Vec<Vec<i32>>) -> Vec<i32> {
    // Declaring an empty result vector
    let mut result: Vec<i32> = Vec::new();
    // Iterate through the input vector to check for each of the vectors
    for number in numbers{
        // If length is 2 then just combine the numbers
        if number.len() == 2{
            let temp = number[0]*10 + number[1];
            result.push(temp);
        }
        // If the length is 1 then just concatenate with itself
        if number.len() == 1{
            let temp = number[0]*10 + number[0];
            result.push(temp);
        }
        // If the length is more than 2 concatenate the first and the last number
        if number.len() > 2{
            let temp = number[0]*10 + number[number.len() - 1];
            result.push(temp);
        }
    }
    // Return the result back to the main function
    return result;
}