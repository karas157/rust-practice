fn print_diamond(size: usize) {
    let mid = size / 2;

    for i in 0..=mid {
        for _ in 0..(mid - i) {
            print!("  ");
        }
        for _ in 0..(2 * i + 1) {
            print!("* ");
        }
        println!();
    }

    for i in (0..mid).rev() {
        for _ in 0..(mid - i) {
            print!("  ");
        }
        for _ in 0..(2 * i + 1) {
            print!("* ");
        }
        println!();
    }
}

fn main() {
    let size = 21; 
    print_diamond(size);
}
