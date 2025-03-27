fn is_palindrome(x: u32) -> bool {
    let s = x.to_string();
    s.chars()
        .zip(s.chars().rev())
        .all(|(a, b)| a == b)
}
#[test]
fn test() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];
    data
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
}
fn main() {
    println!("{}", is_palindrome(123));
    println!("{}", is_palindrome(121));
    println!("{}", is_palindrome(1221));
}
