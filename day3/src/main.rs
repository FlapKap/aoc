use std::collections::HashSet;
use std::io;

fn main() {
    let stdin = io::stdin();
    let mut score = 0;
    let lines: Vec<String> = stdin.lines().flatten().collect();
    for line in &lines {
        // part 1
        let (l1, l2) = line.split_at(line.len() / 2);
        let s1: HashSet<char> = HashSet::from_iter(l1.chars());
        let s2: HashSet<char> = HashSet::from_iter(l2.chars());

        for c in s1.intersection(&s2) {
            if *c as u32 <= 96 {
                score += *c as u32 - (64 - 26);
            } else {
                score += (*c as u32) - 96;
            }
        }
    }
    println!("solution for part 1: {}", score);

    // part 2
    let m = lines.chunks_exact(3).map(|x| -> char {
        let mut xiter = x.iter();
        let a: HashSet<char> = HashSet::from_iter(xiter.next().unwrap().chars());
        let b: HashSet<char> = HashSet::from_iter(xiter.next().unwrap().chars());
        let c: HashSet<char> = HashSet::from_iter(xiter.next().unwrap().chars());

        let v = *(&(&a & &b) & &c).iter().next().unwrap();
        return v;
    });
    let mut score2 = 0;
    for c in m.collect::<Vec<char>>() {
        if c as u32 <= 96 {
            score2 += c as u32 - (64 - 26);
        } else {
            score2 += (c as u32) - 96;
        }
    }
    println!("part 2 solution {:?}", score2);
}
