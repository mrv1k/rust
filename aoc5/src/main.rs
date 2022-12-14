use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "example.txt";

    let file = File::open(path).expect("to no js");
    let reader = BufReader::new(file);

    let mut original_stacks = String::new();
    let mut vec_stack: Vec<Vec<&str>> = vec![];
    let mut moves = false;

    for row in reader.lines() {
        let row = row.unwrap();

        if row.is_empty() {
            let count = original_stacks.lines().count() - 1;
            println!("{}", count);

            let everything = original_stacks.as_bytes();
            let digit_row = original_stacks.lines().rev().next().expect("yolo");

            for char_ind in digit_row.char_indices() {
                // print!("{:?}", everything[char_ind.0] as char);
                if char_ind.1.is_numeric() {
                    let index = char_ind.0;
                    let num = char_ind.1.to_digit(10).unwrap().to_be_bytes();
                    print!("{:?}{:?} ", num, char_ind.1);
                }
            }
            println!("{:?}", digit_row);

            // vec_stack
            moves = true
        }

        if moves {
            let k = row.split_ascii_whitespace();

            let mut amount = 0;
            let mut from = 0;
            let mut to = 0;

            for o in k {
                let int = o.parse::<u32>().unwrap_or(0);
                if int != 0 {
                    if amount == 0 {
                        amount = int;
                        continue;
                    }
                    if from == 0 {
                        from = int;
                        continue;
                    }
                    if to == 0 {
                        to = int;
                    }
                }
            }
            // println!("i like u move it move it: {:#?}{:?}{:?}", amount, from, to);
        } else {
            original_stacks += &row;
            original_stacks += "\n";
        }
    }

    println!("Hello, world! {}", "a");
}
