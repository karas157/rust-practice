fn find_variables() -> (char, char, char, char, char, char, char, char) {
    let m = 'm';
    let u = 'u';
    let x = 'x';
    let a = 'a';
    let s = 's';
    let l = 'l';
    let o = 'o';
    let n = 'n';

    (m, u, x, a, s, l, o, n)
}

fn main() {
    let (m, u, x, a, s, l, o, n) = find_variables();

    println!("{}{}{}", m, u, x);
    println!("{:<8}{}", x, a);
    println!("{:-<6}", "");
    println!("    {}{}", s, l);
    println!("    {}{}", o, n);
}
