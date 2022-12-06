use std::{collections::VecDeque, io, vec};

fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let num_col = (lines.get(0).unwrap().len() + 1) / 4;

    let mut state1: Vec<VecDeque<char>> = vec![VecDeque::new(); num_col];
    let mut state2: Vec<VecDeque<char>> = vec![VecDeque::new(); num_col];
    for line in &lines {
        if !line.starts_with("move") {
            for i in (1..line.len()).step_by(4) {
                let c = line.as_bytes()[i] as char;
                if c.is_alphabetic() {
                    //part1
                    state1[i / 4].push_front(c);
                    //part2
                    state2[i / 4].push_front(c);
                } else if c.is_numeric() {
                    break;
                }
            }
        } else {
            let cmd: Vec<_> = line.split(' ').collect();
            let amount: usize = cmd.get(1).unwrap().parse::<usize>().unwrap();
            let from: usize = cmd.get(3).unwrap().parse::<usize>().unwrap() - 1;
            let to: usize = cmd.get(5).unwrap().parse::<usize>().unwrap() - 1;
            //part1
            for _ in 0..amount {
                let e = state1[from].pop_back().unwrap();
                state1[to].push_back(e);
            }
            //part2
            let len = state2[from].len();
            let es: Vec<_> = state2[from].drain((len - amount)..len).collect();
            state2[to].extend(es);
        }
    }

    print!("part 1 solution ");
    for s in state1 {
        if s.is_empty() {
            break;
        }
        print!("{}", s.back().unwrap());
    }
    println!();

    print!("part 2 solution ");
    for s in state2 {
        if s.is_empty() {
            break;
        }
        print!("{}", s.back().unwrap());
    }
    println!()
}
