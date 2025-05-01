use rand::Rng;

fn count_permutation(shipments: &[u32]) -> usize {
    if shipments.len() <= 1 {
        return 0;
    }

    let total: u32 = shipments.iter().sum();
    let count = shipments.len() as u32;

    if total % count != 0 {
        return usize::MAX;
    }

    let target = total / count;
    let mut moves = 0;
    let mut excess: i64 = 0;

    for &weight in shipments {
        let diff = weight as i64 - target as i64;
        excess += diff;
        moves += diff.abs() as usize;
    }

    moves / 2
}

fn gen_shipments(n: usize) -> Vec<u32> {
    if n == 0 {
        return Vec::new();
    }

    let mut rng = rand::thread_rng();
    let base = rng.gen_range(1..10);
    let total = base * n as u32;
    let mut result = Vec::with_capacity(n);
    let mut remaining = total;

    for i in 0..n - 1 {
        let min_remaining = (n - 1 - i) as u32;
        let max_possible = remaining - min_remaining;
        let value = rng.gen_range(1..=max_possible.min(total / n as u32 + 1));
        result.push(value);
        remaining -= value;
    }

    result.push(remaining);
    result
}

fn print_shipments_and_result(shipments: &[u32]) {
    println!("{:?}", shipments);
    let moves = count_permutation(shipments);
    println!("answer = {}\n", moves);
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
