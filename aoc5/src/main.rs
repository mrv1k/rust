use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "example.txt";

    let file = File::open(path).expect("to no js");
    let reader = BufReader::new(file);

    let mut stacks = String::new();
    let mut moves = false;

    for row in reader.lines() {
        let row = row.unwrap();

        if moves {
            println!("i like u move it move it: {:?}", row);
        } else {
            stacks += &row;
            stacks += "\n";
        }

        if row.is_empty() {
            moves = true
        }
    }

    println!("Hello, world! {}", "a");
}
