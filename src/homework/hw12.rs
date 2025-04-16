fn all_different(digits: &[u32]) -> bool {
    let mut used = [false; 9];
    for &d in digits {
        if d == 0 || used[d as usize] {
            return false;
        }
        used[d as usize] = true;
    }
    true
}
fn solve_cryptarithm() -> usize {
    let mut solutions = 0;
    for m in 1..=8 {
        for u in 1..=8 {
            for x in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    let digits = [m, u, x, a, s, l, o, n];
                                    if !all_different(&digits) {
                                        continue;
                                    }
                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;
                                    if muxa as u64 * a as u64 == slon as u64 {
                                        println!("{} (muxa = {}{}{}{})", muxa, m, u, x, a);
                                        println!("{}        {} (x = {}, a = {})", x, a, x, a);
                                        println!("------");
                                        println!("{} (slon = {}{}{}{})", slon, s, l, o, n);
                                        println!();
                                        solutions += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    solutions
}

fn main() {
    let solution_count = solve_cryptarithm();
    println!("Скільки рішень має задача? {}", solution_count);
}
