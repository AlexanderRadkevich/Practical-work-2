use itertools::Itertools;
fn solve_cryptarithm() -> Vec<(i32, i32, i32, i32, i32, i32, i32, i32)> {
    let mut solutions = Vec::new();
    for perm in (0..10).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];
        if m == 0 || s == 0 {
            continue;
        }
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;
        if muxa * a == slon {
            solutions.push((m, u, x, a, s, l, o, n));
        }
    }

    solutions
}
fn print_solution(solution: &(i32, i32, i32, i32, i32, i32, i32, i32)) {
    let (m, u, x, a, s, l, o, n) = *solution;
    let muxa = 1000 * m + 100 * u + 10 * x + a;
    let slon = 1000 * s + 100 * l + 10 * o + n;
    println!("  {}{}{}{}", m, u, x, a);
    println!("x     {}", a);
    println!("------");
    println!("  {}{}{}{}", s, l, o, n);
    println!("m = {}, u = {}, x = {}, a = {}, s = {}, l = {}, o = {}, n = {}", m, u, x, a, s, l, o, n);
    println!();
}
fn main() {
    let solutions = solve_cryptarithm();
    for solution in &solutions {
        print_solution(solution);
    }
    println!("Кількість рішень: {}", solutions.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cryptarithm() {
        let solutions = solve_cryptarithm();
        for &(m, u, x, a, s, l, o, n) in &solutions {
            let muxa = 1000 * m + 100 * u + 10 * x + a;
            let slon = 1000 * s + 100 * l + 10 * o + n;
            assert_eq!(muxa * a, slon);
            let digits: Vec<i32> = vec![m, u, x, a, s, l, o, n];
            let unique_digits: std::collections::HashSet<i32> = digits.into_iter().collect();
            assert_eq!(unique_digits.len(), 8);
            assert_ne!(m, 0);
            assert_ne!(s, 0);
        }
    }
}
