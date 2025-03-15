fn draw_tree(parts: usize) {
    let star_counts = vec![1, 3, 5, 7, 9];

    for i in 0..parts {
        let stars_in_row = star_counts[i];

        for row in 0..stars_in_row {
            let stars = 2 * row + 1;
            let spaces = (star_counts[parts - 1] - stars) / 2;
            println!("{:width$}{}", "", "*".repeat(stars), width = spaces);
        }
    }
}

fn main() {
    let parts = 5;
    draw_tree(parts);
}
