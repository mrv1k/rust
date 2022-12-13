use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "example.txt";
    let file = File::open(path).expect("to no js");
    let reader = BufReader::new(file);

    let mut stacks = String::new();

    // let t = reader.lines();

    // let cursor = std::io::Cursor::new(b"lorem\nipsum\r\ndolor");
    // let mut lines_iter = cursor.lines().map(|l| l.unwrap());
    // lines_iter.
    // assert_eq!(lines_iter.next(), Some(String::from("lorem")));

    for row in reader.lines() {
        let row = row.unwrap();
        if row.is_empty() {
            // println!("EMPTY");
            break;
        }

        stacks += &row;
        stacks += "\n";
    }

    for row in stacks.lines() {
        println!("{}", row);
    }

    println!("Hello, world! {}", "a");
}
