fn main() {
    let input_string = "1abc2";
    let chrs: Vec<char> = input_string.chars().filter(|char| char.is_digit(10)).collect();
    println!("{:?}",chrs);
}