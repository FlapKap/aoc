use std::io;

fn main() {
    let stdin = io::stdin();
    let mut score1 = 0;
    let mut score2 = 0;
    for line in stdin.lines() {
        // part 1
        let l = line.unwrap();
        let opponent = l.chars().next().unwrap();
        let you = l.chars().nth(2).unwrap();
        match you {
            'X' => score1 += 1,
            'Y' => score1 += 2,
            'Z' => score1 += 3,
            _ => {}
        }
        match opponent {
            'A' => if you == 'Y' { score1 += 6 } else if you == 'X' { score1 += 3 },
            'B' => if you == 'Z' { score1 += 6 } else if you == 'Y' { score1 += 3 },
            'C' => if you == 'X' { score1 += 6 } else if you == 'Z' { score1 += 3 },
            _ => {}
        }
        // part 2
        let outcome = you;
        match outcome {
            'Y' => score2 += 3,
            'Z' => score2 += 6,
            _ => {}
        }
        match opponent {
            //rock
            'A' => match outcome {
                'X' => score2 += 3, //lose-scissors
                'Y' => score2 += 1, //draw-rock
                'Z' => score2 += 2, //win-paper
                _ => {}
            }
            //paper
            'B' => match outcome {
                'X' => score2 += 1, //lose-rock
                'Y' => score2 += 2, //draw-paper
                'Z' => score2 += 3, //win-scissors
                _ => {}
            }
            //scissors
            'C' => match outcome {
                'X' => score2 += 2, //lose-paper
                'Y' => score2 += 3, //draw-scissors
                'Z' => score2 += 1, //win-rock
                _ => {}
            }
            _ => {}
        }
    }
    println!("solution for part 1: {:?}", score1);
    println!("solution for part 2: {:?}", score2);
}
