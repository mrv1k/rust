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

            println!("i like u move it move it: {:#?}{:?}{:?}", amount, from, to);
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
