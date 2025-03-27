fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    (3..=sqrt_n)
        .step_by(2)
        .all(|i| n % i != 0)
}

fn main() {
    println!("2 is prime: {}", is_prime(2));
    println!("17 is prime: {}", is_prime(17));
    println!("4 is prime: {}", is_prime(4));
    println!("97 is prime: {}", is_prime(97));
    println!("100 is prime: {}", is_prime(100));
}
#[test]
fn test_is_prime() {
    let test_cases = [
        (2, true),
        (3, true),
        (4, false),
        (17, true),
        (25, false),
        (97, true),
        (100, false),
        (0, false),
        (1, false),
    ];
    test_cases
        .iter()
        .for_each(|&(n, expected)| {
            assert_eq!(is_prime(n), expected, "Failed for number {}", n);
        });
}
