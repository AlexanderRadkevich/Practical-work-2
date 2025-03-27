const WIDTH: usize = 11; // Ширина конверта
const HEIGHT: usize = 7; // Висота конверта

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 || x == y || x == WIDTH - 1 - y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output); // print! використовується один раз
}
