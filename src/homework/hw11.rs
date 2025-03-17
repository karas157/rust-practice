use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::rng(); 
    (0..n).map(|_| rng.random_range(10..100)).collect() 
}

fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index1 = 0;
    let mut min_index2 = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index1 = i;
            min_index2 = i + 1;
        }
    }

    (min_index1, min_index2, min_sum)
}

fn print_vector_and_result(data: &[i32]) {
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:<2}", i)).collect::<Vec<String>>().join(". "));
    println!("data:   {:?}", data);

    let (index1, index2, sum) = min_adjacent_sum(data);
    println!(
        "indexes: {}{:>2}  \\",
        " ".repeat(index1 * 3),
        " ".repeat((index2 - index1 - 1) * 3)
    );
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[index1], data[index2], sum, index1, index2);
}

fn main() {
    let vec = gen_random_vector(20);
    print_vector_and_result(&vec);
}
