use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, (usize, usize)) {
    if data.len() < 2 {
        return (0, (0, 0));
    }

    let mut min_sum = data[0] + data[1];
    let mut min_indices = (0, 1);

    for i in 0..(data.len() - 1) {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_indices = (i, i + 1);
        }
    }

    (min_sum, min_indices)
}

fn print_vector_with_min(data: &[i32], min_indices: (usize, usize), min_sum: i32) {

    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2}.", i);
    }
    println!();

    print!("data:    [");
    for i in 0..data.len() {
        if i == min_indices.0 || i == min_indices.1 {
            print!("{:2},", data[i]);
        } else {
            print!("{:2}, ", data[i]);
        }
    }
    println!("]");

    print!("         ");
    for i in 0..data.len() {
        if i == min_indices.0 || i == min_indices.1 {
            if i == min_indices.0 {
                print!("____");
            } else {
                print!("__");
            }
        } else {
            print!("    ");
        }
    }
    println!();

    println!(
        "min_adjacent_sum={}+{}={} at indexes:{}.{}",
        data[min_indices.0], data[min_indices.1], min_sum, min_indices.0, min_indices.1
    );
}

fn main() {
    let data = gen_random_vector(20);

    let (min_sum, min_indices) = min_adjacent_sum(&data);
    
    print_vector_with_min(&data, min_indices, min_sum);
}
