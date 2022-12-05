use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines().flatten().collect();
    let mut count1 = 0;
    let mut count2 = 0;
    let h: Vec<i32> = lines
        .iter()
        .flat_map(|s: &String| {
            s.split(',')
                .flat_map(|n| n.split('-').map(|m| m.parse::<i32>()))
        })
        .flatten()
        .collect();

    for i in h.chunks_exact(4) {
        let [a1, a2, b1, b2]: [i32; 4] = i.try_into().unwrap();
        // a inside b
        if a1 >= b1 && a2 <= b2 {
            count1 += 1;
        } else if b1 >= a1 && b2 <= a2 {
            count1 += 1;
        }

        if a1 <= b2 && a2 >= b1 {
            count2 += 1;
        }
    }
    println!("part 1 solution {}", count1);
    println!("part 2 solution {}", count2);
}
