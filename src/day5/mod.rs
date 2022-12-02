pub fn main() {
    let input = include_str!("./input");

    let has_3_vowels = |line: &str| -> bool {
        line.chars()
            .filter(|c| c == &'a' || c == &'e' || c == &'i' || c == &'o' || c == &'u')
            .count()
            >= 3
    };
    let has_double_letter = |line: &str| -> bool {
        line.chars()
            .fold((false, ' '), |acc, c| {
                if acc.1 == c {
                    (true, c)
                } else {
                    (acc.0, c)
                }
            })
            .0
    };
    let does_not_contain = |line: &str| -> bool {
        !line.contains("ab") && !line.contains("cd") && !line.contains("pq") && !line.contains("xy")
    };

    let nice_kids = input
        .lines()
        .filter(|line| has_3_vowels(line) && has_double_letter(line) && does_not_contain(line))
        .count();

    //part 2
    let has_two_pairs = |line: &str| -> bool {
        for (i, c) in line.chars().skip(1).enumerate() {
            if line
                .matches(&format!("{}{}", line.chars().nth(i).unwrap(), c))
                .count()
                >= 2
            {
                return true;
            }
        }
        false
    };
    let double_char_with_buffer = |line: &str| -> bool {
        line.chars()
            .fold((false, ' ', ' '), |acc, c| {
                if c == acc.1 {
                    (true, acc.2, c)
                } else {
                    (acc.0, acc.2, c)
                }
            })
            .0
    };

    let new_nice_kids = input
        .lines()
        .filter(|line| has_two_pairs(line) && double_char_with_buffer(line))
        .count();

    println!("\nday 5");
    println!("number of nice kids on list: {}", nice_kids);
    println!("nice kids by new rules: {}", new_nice_kids);
}
