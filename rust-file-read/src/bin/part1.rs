use std::{fs::{self, File}, io::{self, BufRead}, path::Path};

fn main(){
    let input1 = "src/bin/input1.txt";
    let del = " ";
    let output1 = file_parse_result_as_string(input1);
    let output2 = file_parse_as_vector_string(input1);
    let res = parse_by_deli(output2.clone(), del);
    dbg!(output1);
    dbg!(output2);
    dbg!(res);
}
fn file_parse_result_as_string(input: &str) -> String{
    let content = fs::read_to_string(input).expect("Could not read");
    return content;
}
fn file_parse_as_vector_string<'a>(input: &'a str) -> Vec<String>{
    let path = Path::new(input);
    let file = File::open(&path).expect("File not found");
    let reader = io::BufReader::new(file);
    let mut temp:Vec<String> = Vec::new();
    for line in reader.lines(){
        let line = line.expect("Could not read the contents of the file");
        temp.push(line);
    }
    return temp;
}
fn parse_by_deli(input: Vec<String>, del: &str) -> Vec<String>{
    let mut temp: Vec<_> = Vec::new();
    for line in input{
        temp.push(line.as_str().lines()
        .map(|line| line.trim().split(del).map(|s| String::from(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>());
    }
    let mut novo_temp: Vec<_> = Vec::new();
    for part in temp{
        novo_temp.push(part[0].clone());
    }
    let mut res = Vec::new();
    for n in novo_temp{
        for i in n{
            res.push(i);
        }
    }
    return res;

}