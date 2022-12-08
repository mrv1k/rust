use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "aoc3pt1.txt";
    let file = File::open(path).expect("to no kaboom");

    let reader = BufReader::new(file);

    for rucksack in reader.lines() {
        let compartments = rucksack.expect("contains items");
        for item in compartments.chars() {
            print!("{:?}", item);
        }

        println!("{:?}", compartments);
    }
}
