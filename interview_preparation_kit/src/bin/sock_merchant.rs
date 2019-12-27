use std::collections::HashSet;

fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap(); // number of elements
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let mut socks: HashSet<&str> = HashSet::new();

    let pair: usize = line
        .split(' ')
        .map(str::trim)
        .map(|sock| {
            if let Some(_) = socks.take(sock) {
                1
            } else {
                socks.insert(sock);
                0
            }
        })
        .sum();

    println!("{}", pair);
}
