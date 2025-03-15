fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (100, false),
        (10007, true),
    ];

    for (n, expected) in test_data.iter() {
        let result = is_prime(*n);
        if result == *expected {
            println!("{} passed the test", n);
        } else {
            println!("{} failed the test (expected: {}, got: {})", n, expected, result);
        }
    }
}

fn main() {
    test_is_prime();
}
