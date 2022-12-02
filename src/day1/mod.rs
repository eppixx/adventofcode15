pub fn main() {
    let input = include_str!("./input");

    let floor: i32 = input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => unreachable!("invalid char: {}", c),
        })
        .sum();
    let pos_of_first_cellar: usize = input
        .chars()
        .enumerate()
        .fold((0, 0), |(acci, accv), (i, c)| {
            if accv >= 0 {
                match c {
                    '(' => (i, accv + 1),
                    ')' => (i, accv - 1),
                    c => unreachable!("invalid char: {}", c),
                }
            } else {
                (acci, accv)
            }
        })
        .0;

    println!("\nday 1");
    println!("target floor: {}", floor);
    println!(
        "position of first cellar floor: {}",
        pos_of_first_cellar + 1
    );
}
