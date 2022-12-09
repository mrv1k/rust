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
    let mut group = 0;

    let mut items: HashMap<char, u8> = HashMap::new();

    for gnome in reader.lines() {
        if group == 3 {
            // println!("GNOME RESET!");
            items = HashMap::new();
            group = 0;
        }
        group += 1;

        let rucksack = gnome.expect("contains items");

        for item in rucksack.chars() {
            match items.get(&item) {
                Some(&count) => {
                    // println!("{:?}{:?},{:?}", item, count, group);
                    let next_count = count + 1;
                    if group == next_count {
                        items.insert(item, next_count);

                        if next_count == 3 {
                            let ascii = item as u32;
                            let rity = if item.is_lowercase() {
                                ascii - 96
                            } else {
                                ascii - 64 + 26
                            };
                            prio += rity;
                            // println!("got em! {:?} ascii:{:?} prio:{:?}", item, ascii, rity);
                            break;
                        }
                    }
                }
                None => {
                    if group == 1 {
                        items.insert(item, 1);
                    }
                }
            }
        }

        // println!("{:?}, {:?} {:#?}", rucksack, prio, items);
    }
    println!("{:?}", prio);
}
