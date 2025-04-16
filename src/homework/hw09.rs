fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }
    let len = s.len() as isize;
    let n = ((n % len) + len) % len;
    if n == 0 {
        return s;
    }
    let mut chars: Vec<char> = s.chars().collect();
    chars.rotate_left(n as usize);
    chars.into_iter().collect()
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.to_string(), *n),
                exp.to_string()
            );
        });
}

fn main() {
    test();
    println!("{}", rotate("abcdefgh".to_string(), 2));
    println!("{}", rotate("abcdefgh".to_string(), -1));
}
