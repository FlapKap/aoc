use std::io;
fn main() {
    let stdin = io::stdin();
    let mut group = Vec::new();
    let mut totals = Vec::new();
    for line in stdin.lines(){
        let yo = line.unwrap().clone();
        if yo.is_empty(){
            //regn sum
            totals.push(group.iter().sum::<i32>());
            group.clear();
            continue;
        }
        group.push(yo.parse::<i32>().unwrap());
    }
    totals.push(group.iter().sum::<i32>());
    println!("part one solution: {:?}", totals.iter().max().unwrap());

    totals.sort();
    totals.reverse();
    println!("part two solution: {:?}",(&totals[0..3]).iter().sum::<i32>());
}
