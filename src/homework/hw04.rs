const HEIGHT: usize = 11;
fn main() {
    let mut output = String::new();
    let mid = HEIGHT / 2;

    for y in 0..HEIGHT {
        for x in 0..HEIGHT {
            if x == mid - y || x == mid + y || x == y - mid || x == HEIGHT + mid - 1 - y {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n'); 
    }
    print!("{}", output); 
}
