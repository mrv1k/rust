use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "aoc4pt1.txt";
    let file = File::open(path).expect("to no kerblam");
    let reader = BufReader::new(file);

    let mut fully_contain_each_other = 0;

    fn mek(n: Option<&&str>) -> Vec<u32> {
        n.unwrap()
            .split("-")
            .map(|n| n.parse::<u32>().unwrap_or(0))
            .collect()
    };

    for gnome in reader.lines() {
        // gnome shape: "2-4,6-8"
        let gnome = gnome.expect("to gnome");

        let pairs: Vec<&str> = gnome.split(",").collect();

        let pair_a: Vec<u32> = mek(pairs.first());
        let pair_b: Vec<u32> = mek(pairs.last());

        let ax = pair_a.first().unwrap();
        let ay = pair_a.last().unwrap();

        let bx = pair_b.first().unwrap();
        let by = pair_b.last().unwrap();

        if (ax <= bx && ay >= by) || (ax >= bx && ay <= by) {
            fully_contain_each_other += 1;
            // print!("a {:#?} v {:#?} | ", ax, ay);
            // println!("b {:#?} v {:#?}", bx, by);
        }
        // else {
        //     println!("no {:#?}", pairs);
        // }
    }

    println!("Hello, world! {}", fully_contain_each_other);
}
