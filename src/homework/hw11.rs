use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    (0..n).map(|_| rng.random_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (usize, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, data[min_index], data[min_index + 1])
}

fn print_result(data: &[i32], min_index: usize, a: i32, b: i32) {
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:    [");
    for (i, val) in data.iter().enumerate() {
        print!("{:>2}", val);
        if i != data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    print!("indexes: ");
    for i in 0..data.len() {
        if i == min_index {
            print!("{:>width$}", "\\__", width = 4);
        } else if i == min_index + 1 {
            print!("{:>width$}", "__/", width = 4);
        } else {
            print!("{:>4}", "");
        }
    }
    println!();

    println!(
        "min adjacent sum = {} + {} = {} at indexes: {}, {}",
        a,
        b,
        a + b,
        min_index,
        min_index + 1
    );
}

fn main() {
    let data = gen_random_vector(20);
    let (min_index, a, b) = min_adjacent_sum(&data);
    print_result(&data, min_index, a, b);
}
