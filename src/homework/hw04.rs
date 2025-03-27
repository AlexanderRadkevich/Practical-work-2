const WIDTH: usize = 11;
const HEIGHT: usize = 11; 
fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if x == WIDTH / 2 - y || x == WIDTH / 2 + y || x == y - WIDTH / 2 || x == WIDTH + WIDTH / 2 - 1 - y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }
    print!("{}", output); 
}
