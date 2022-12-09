pub fn main() {
    let mut input = String::from("cqjxjnds");

    loop {
        input = increment(&input);
        if let true = validate(&input) {
            break;
        }
    }

    // part 2
    let mut next_input = input.clone();
    loop {
        next_input = increment(&next_input);
        if let true = validate(&next_input) {
            break;
        }
    }

    println!("\nday 11");
    println!("Santas new password is: {}", input);
    println!("Santas new new password is: {}", next_input);
}

pub fn increment(input: &str) -> String {
    let output: String = input
        .chars()
        .rev()
        .fold(
            // bool states if increament is done
            (false, String::new()),
            |mut acc: (bool, String), c| match (acc.0, c) {
                (false, c) if c == 'z' => {
                    acc.1.insert(0, 'a');
                    (false, acc.1)
                }
                (false, c) => {
                    acc.1.insert(0, (c as u8 + 1) as char);
                    (true, acc.1)
                }
                (true, c) => {
                    acc.1.insert(0, c);
                    (true, acc.1)
                }
            },
        )
        .1;

    output
}

pub fn validate(input: &str) -> bool {
    let ascend_3_chars = |input: &str| -> bool {
        input
            .as_bytes()
            .windows(3)
            .any(|s| s[0] + 2 == s[2] && s[1] + 1 == s[2])
    };
    let only_valid_chars = |input: &str| -> bool {
        input
            .find(|s| !(s == 'i' || s == 'o' || s == 'l'))
            .is_some()
    };
    let two_unique_pairs = |input: &str| -> bool {
        let search = input
            .as_bytes()
            .windows(2)
            .enumerate()
            .find(|(_, s)| s[0] == s[1]);
        match search {
            Some((i, _)) => input
                .as_bytes()
                .windows(2)
                .skip(i + 2)
                .any(|s| s[0] == s[1]),
            _ => false,
        }
    };

    ascend_3_chars(input) && only_valid_chars(input) && two_unique_pairs(input)
}
