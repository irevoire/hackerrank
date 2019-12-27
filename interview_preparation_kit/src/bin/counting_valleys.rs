/// README: the rust version does not exist for this challenge
///
fn main() {
    let stdin = std::io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap(); // number of elements
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();

    let mut height = 0;
    let mut sum = 0;

    line.trim().chars().for_each(|direction| {
        match direction {
            'U' => height += 1,
            'D' => height -= 1,
            _ => panic!("Can’t happens"),
        }
        if height == 0 {
            sum += 1;
        }
    });

    println!("{}", sum / 2);
}
