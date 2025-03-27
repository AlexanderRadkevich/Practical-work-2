fn count_permutation(shipments: &Vec<u32>) -> usize {
    if shipments.is_empty() {
        return 0;
    }
    let total: u32 = shipments.iter().sum();
    let count = shipments.len() as u32;
    if total % count != 0 {
        return usize::MAX;
    }
    let target = total / count;
    let mut moves = 0;
    let mut excess = 0i64;
    for &weight in shipments {
        let diff = weight as i64 - target as i64;
        excess += diff;
        moves += diff.abs() as usize;
    }
    moves / 2
}
fn gen_shipments(n: usize) -> Vec<u32> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    if n == 0 {
        return Vec::new();
    }
    let base = rng.gen_range(1..10);
    let total = base * n as u32;
    let mut result = Vec::with_capacity(n);
    let mut remaining = total;
    for _ in 0..n-1 {
        let max_possible = remaining - (n - result.len() - 1) as u32;
        let value = rng.gen_range(1..=max_possible.min(total / n as u32 + 1));
        result.push(value);
        remaining -= value;
    }
    result.push(remaining);
    result
}
fn main() {
    let examples = vec![
        vec![8, 2, 2, 4, 4],
        vec![9, 3, 7, 2, 9],
    ];
    for shipments in examples {
        println!("{:?}", shipments);
        let moves = count_permutation(&shipments);
        println!("answer = {}\n", moves);
    }
    let generated = gen_shipments(5);
    println!("Generated: {:?}", generated);
    println!("answer = {}\n", count_permutation(&generated));
}
#[test]
fn test_count_permutation() {
    let test_cases = [
        (vec![8, 2, 2, 4, 4], 4),
        (vec![9, 3, 7, 2, 9], 7),
        (vec![1, 2, 3], usize::MAX),
    ];
    for (shipments, expected) in test_cases.iter() {
        assert_eq!(count_permutation(shipments), *expected);
    }
}
