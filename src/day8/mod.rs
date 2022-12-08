pub fn main() {
    let input = include_str!("./input");

    let total_chars: usize = input.lines().map(|line| line.chars().count()).sum();
    let unescaped_chars: usize = input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let mut count = 0;
            loop {
                match iter.next() {
                    Some('"') => {}
                    Some('\\') => {
                        match iter.next() {
                            Some('\\') => {}
                            Some('"') => {}
                            Some('x') => {
                                iter.next();
                                iter.next();
                            }
                            None | Some(_) => unreachable!("escaped char unterminated"),
                        }
                        count += 1;
                    }
                    None => break,
                    _ => count += 1,
                }
            }
            count
        })
        .sum();

    //part 2
    let escaped_chars: usize = input
        .lines()
        .map(|line| {
            let mut iter = line.chars();
            let mut count = 0;
            loop {
                match iter.next() {
                    Some('"') => count += 2,
                    Some('\\') => match iter.next() {
                        Some('\\') => count += 4,
                        Some('"') => count += 4,
                        Some('x') => {
                            iter.next();
                            iter.next();
                            count += 5;
                        }
                        None | Some(_) => unreachable!("escaped char unterminated"),
                    },
                    None => break,
                    _ => count += 1,
                }
            }
            count + 2
        })
        .sum();

    println!("\nday 8");
    println!(
        "number of unescaped characters: {}",
        total_chars - unescaped_chars
    );
    println!(
        "number of escaped characters: {}",
        escaped_chars - total_chars
    );
}
