fn main() {
    const WIDTH: i32 = 11;
    const HEIGHT: i32 = 11;
    let mut output = String::new();
    for i in 0..HEIGHT {
        let spaces = ((HEIGHT - 1) / 2 - i).abs();
        let stars = if i <= HEIGHT / 2 {
            1 + 2 * i
        } else {
            WIDTH - 2 * (i - HEIGHT / 2)
        };
        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        output.push('\n');
    }
    print!("{}", output);
    println!();
}
