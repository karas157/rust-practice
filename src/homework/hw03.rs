fn print_border(size: usize) {
    for _ in 0..=size {
        print!("* ");
    }
    println!();
}

fn print_middle(size: usize) {
    for i in 1..size {
        for j in 0..=size {
            if j == 0 || j == size {
                print!("* ");
            } else if i == j || i + j == size {
                print!("* ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

fn main() {
    let size = 20;
    print_border(size);
    print_middle(size);
    print_border(size);
}
