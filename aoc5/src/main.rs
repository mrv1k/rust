use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "example.txt";

    let file = File::open(path).expect("to no js");
    let reader = BufReader::new(file);

    let mut original_stacks = String::new();
    let mut vec_stack: Vec<Vec<char>> = vec![];
    let mut moves = false;

    for row in reader.lines() {
        let row = row.unwrap();

        if row.is_empty() {
            let mut lines = original_stacks.lines().rev();

            for char in lines.next().unwrap().chars() {
                if char.is_numeric() {
                    vec_stack.push(vec![]);
                }
            }

            for stack in lines {
                for char_ind in stack.char_indices() {
                    let char = char_ind.1;

                    if char.is_alphabetic() {
                        let ind = char_ind.0 - 1;
                        let vec_ind = if ind > 0 { ind / 4 } else { ind };
                        vec_stack[vec_ind].push(char);
                        // print!("{:?}", vec_ind);
                    }
                }
                // println!();
            }

            println!("{:?}", vec_stack);
            moves = true
        }

        if moves {
            let k = row.split_ascii_whitespace();
            println!("{:?}", row);

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
            println!("i like u move it move it: {:#?}{:?}{:?}", amount, from, to);
        } else {
            original_stacks += &row;
            original_stacks += "\n";
        }
    }

    println!("Hello, world! {}", "a");
}
