fn print_christmas_tree(triangle_count: usize) {
    let max_width = triangle_count * 2 + 1;
    for triangle in 0..triangle_count {
        let height = triangle + 2;
        let spaces = " ".repeat(max_width / 2);
        println!("{}*", spaces);
        (1..height).for_each(|row| {
            let star_count = row * 2 + 1;
            let space_count = (max_width - star_count) / 2;
            let spaces = " ".repeat(space_count);
            let stars = "*".repeat(star_count);
            println!("{}{}", spaces, stars);
        });
    }
}
fn main() {
    print_christmas_tree(3);
}
