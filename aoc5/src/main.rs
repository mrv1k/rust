use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // let path = "example.txt";
    let path = "input.txt";

    let file = File::open(path).expect("to no js");
    let reader = BufReader::new(file);

    let mut original_stacks = String::new();
    let mut vec_stack: Vec<Vec<char>> = vec![];
    let mut moves = false;

    for row in reader.lines() {
        let row = row.expect("to blow");

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

        if moves && !row.is_empty() {
            let k = row.split_ascii_whitespace();

            let mut amount = 0;
            let mut from = 0;
            let mut to = 0;

            for o in k {
                let u = o.parse::<usize>().unwrap_or(0);
                if u != 0 {
                    if amount == 0 {
                        amount = u;
                        continue;
                    }
                    if from == 0 {
                        from = u;
                        continue;
                    }
                    if to == 0 {
                        to = u;
                        break;
                    }
                }
            }

            println!("amount {:?} from {:?} to {:?}", amount, from, to);
            println!("before {:?}", vec_stack);

            let mut fml: Vec<char> = vec![];
            for _ in 1..=amount {
                let pop = &vec_stack[from - 1].pop().expect("anarchy");
                let _ = fml.push(*pop);
            }
            fml.reverse();
            let _ = &vec_stack[to - 1].append(&mut fml);

            println!("after {:?}", vec_stack);
            // println!(
            //     "after {:?} from {:?} to {:?}",
            //     amount, &vec_stack[from], &vec_stack[to],
            // );
            println!();
        } else {
            original_stacks += &row;
            original_stacks += "\n";
        }
    }

    let mut end = String::new();
    for stack in vec_stack {
        let riot = stack.last().expect("riot");
        end.push(*riot);
    }
    println!("Hello, world! {}", end);
}
