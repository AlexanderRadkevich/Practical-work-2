use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| rng.gen_range(10..100))
        .collect()
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize) {
    if data.len() < 2 {
        return (0, 0);
    }
    data.windows(2)
        .enumerate()
        .map(|(i, window)| (window[0] + window[1], i))
        .min_by_key(|&(sum, _)| sum)
        .unwrap_or((0, 0))
}
fn print_result(data: &[i32], min_sum: i32, min_index: usize) {
    print!("indexes:");
    for i in 0..data.len() {
        print!(" {:2}.", i);
    }
    println!();
    print!("data:   [");
    for (i, &num) in data.iter().enumerate() {
        if i > 0 {
            print!(", ");
        }
        print!("{:2}", num);
    }
    println!("]");
    print!("indexes:");
    for i in 0..data.len() {
        if i == min_index {
            print!(" \\__");
        } else if i == min_index + 1 {
            print!(" __/");
        } else {
            print!("    ");
        }
    }
    println!();
    println!(
        "min adjacent sum={} at indexes:{},{}", 
        min_sum, 
        min_index, 
        min_index + 1
    );
    println!();
}
fn main() {
    let vec = gen_random_vector(20);
    let (min_sum, min_index) = min_adjacent_sum(&vec);
    print_result(&vec, min_sum, min_index);
}
#[test]
fn test_min_adjacent_sum() {
    let test_cases = [
        (
            vec![45, 87, 49, 64, 50, 37, 45, 72, 55, 64, 90, 86, 60, 54, 78, 72, 83, 44, 89, 22],
            (82, 5)
        ),
        (
            vec![29, 92, 14, 65, 57, 98, 10, 45, 11, 48, 69, 21, 12, 75, 51, 69, 72, 36, 47, 45],
            (33, 11)
        ),
        (
            vec![19, 86, 66, 95, 40, 24, 90, 74, 98, 37, 26, 44, 76, 86, 48, 63, 11, 38, 29, 40],
            (49, 16)
        ),
        (
            vec![30, 18, 68, 87, 99, 81, 88, 45, 34, 79, 81, 79, 93, 55, 26, 24, 32, 55, 59, 97],
            (48, 0)
        ),
    ];
    for (data, expected) in test_cases.iter() {
        let result = min_adjacent_sum(data);
        assert_eq!(result, *expected);
    }
}
