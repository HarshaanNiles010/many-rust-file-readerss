#[allow(unused_imports)]
use std::{fs::{self, File}, io::{self, BufRead}, path::Path};
fn main() {
    let input_path = "src/bin/input2.txt";
    let file_contents = parse_file(input_path);
    let file_numbers = convert(file_contents.clone());
    let mut fib_terms: Vec<i64> = Vec::new();
    for i in file_numbers.clone(){
        fib_terms.push(fibonacci(i));
    }
    dbg!(fib_terms);
    dbg!(file_numbers);
    dbg!(file_contents);
}

fn parse_file(input: &str) -> Vec<String>{
    let path = Path::new(input);
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut res = Vec::new();
    for line in reader.lines(){
        let line = line.expect("Failed to read the file");
        res.push(line);
    }
    return res;
}

fn convert(input: Vec<String>) -> Vec<i32>{
    let mut res: Vec<i32> = Vec::new();
    for i in input{
        res.push(i.parse::<i32>().unwrap());
    }
    return res;
}

// This is an example for the use of a file to find a number

fn fibonacci(term: i32) -> i64{
    let mut a: Vec<i64> = vec![1,1];
    for i in  2..term as usize{
        a.push(a[i-1] + a[i-2]);
    }
    return a[a.len()-1];
}