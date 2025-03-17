fn rotate2(s: &str, shift: isize) -> String {
    let len = s.len() as isize;
    if len == 0 {
        return s.to_string();
    }
    let shift = (shift % len + len) % len; // Нормалізація зсуву
    let (left, right) = s.split_at(shift as usize);
    format!("{}{}", right, left)
}

fn main() {
    let s = "abcdefgh";
    let shift_values = [0, 8, -8, 1, 2, 10, -1, -2, -10];

    for &shift in &shift_values {
        let shifted = rotate2(s, shift);
        println!("Shift {}: {}", shift, shifted);
    }
}
