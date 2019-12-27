fn main() {
    let stdin = std::io::stdin();
    let mut base = String::new();
    stdin.read_line(&mut base).unwrap(); // number of elements
    let mut nb = String::new();
    stdin.read_line(&mut nb).unwrap();
    let nb = nb.trim().parse().unwrap();

    let base_len = base.trim().len();
    let mut nb_a = base.trim().chars().take(nb).filter(|c| *c == 'a').count();

    let factor = nb / base_len;
    nb_a *= factor;
    nb_a += base
        .trim()
        .chars()
        .take(nb % base_len)
        .filter(|c| *c == 'a')
        .count();

    println!("{}", nb_a);
}
