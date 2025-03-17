fn is_palindrome(n: i32) -> bool {
    let s = n.to_string();
    s.chars().eq(s.chars().rev())
}

fn main() {
    let numbers = [121, -121, 10, 1221, 12321, 123321, 0];

    for &num in &numbers {
        println!("{} is palindrome: {}", num, is_palindrome(num));
    }
}
