fn draw_tree() {
    let star_counts = vec![1, 3, 5, 7, 9];

    for stars_in_row in &star_counts {
        let width = star_counts[4]; 
        for row in 0..*stars_in_row / 2 + 1 {
            let stars = 2 * row + 1;
            let spaces = (width - stars) / 2;
            println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
        }
    }
}

fn main() {
    draw_tree();
}
