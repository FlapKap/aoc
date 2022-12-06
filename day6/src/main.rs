use std::{io, collections::{HashSet, VecDeque}};

fn main() {
    let chars: Vec<char> = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .collect();

    let mut count = 1;
    let mut found1 = false;
    let mut found2 = false;
    let mut buf1 = VecDeque::new();
    let mut buf2 = VecDeque::new();

    for c in chars.iter() {
        if buf1.len() >= 4 {
            buf1.remove(0);
        }
        if buf2.len() >= 14 {
            buf2.remove(0);
        }
        buf1.push_back(c);
        buf2.push_back(c);

        if HashSet::<_>::from_iter(buf1.iter()).len() == 4 && !found1 {
            found1 = true;
            println!("solution to part one is {}", count);
        }
        if HashSet::<_>::from_iter(buf2.iter()).len() == 14 && !found2 {
            found2 = true;
            println!("solution to part two is {}", count);
        }
        count += 1;
    }
}
