use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let path = "aoc3pt1.txt";
    let file = File::open(path).expect("to no kaboom");

    let reader = BufReader::new(file);

    let mut prio = 0;

    for gnome in reader.lines() {
        let mut rucksack = gnome.expect("contains items");
        let right_compartment = rucksack.split_off(rucksack.len() / 2);

        let mut items: HashMap<char, u8> = HashMap::new();

        for item in rucksack.chars() {
            match items.get(&item) {
                Some(&count) => {
                    // println!("{:?}{:?}", item, count);
                    items.insert(item, count + 1);
                }
                None => {
                    items.insert(item, 1);
                }
            }
        }

        for item in right_compartment.chars() {
            match items.get(&item) {
                Some(_) => {
                    let ascii = item as u32;
                    let rity = if item.is_lowercase() {
                        ascii - 96
                    } else {
                        ascii - 64 + 26
                        // ascii - 38 ;)?
                    };
                    prio += rity;
                    // println!("got em! {:?} = {:?}", item, ascii);
                    break;
                }
                None => (),
            }
        }

        println!("{:?}, {:?} {:#?}", rucksack, right_compartment, items);
    }
    println!("{:?}", prio);
}
