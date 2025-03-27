struct Point {
    x: i32,
    y: i32,
}
struct Rectangle {
    a: Point,
    b: Point,
}
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for rect in xs {
        min_x = min_x.min(rect.a.x);
        max_x = max_x.max(rect.b.x);
        min_y = min_y.min(rect.b.y);
        max_y = max_y.max(rect.a.y);
    }
    let mut occupied_points = std::collections::HashSet::new();
    for rect in xs {
        for x in rect.a.x..rect.b.x {
            for y in rect.b.y..rect.a.y {
                occupied_points.insert((x, y));
            }
        }
    }
    occupied_points.len() as i32
}
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}
fn main() {
    area_occupied_test();
    println!("Test passed!");
}
