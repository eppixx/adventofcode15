pub fn main() {
    let input = String::from("1113222113");

    let part1 = look_and_seek(input.clone(), 40);
    let part2 = look_and_seek(input, 50);

    println!("\nday 10");
    println!("applying look-and-say 40 times: {}", part1.len());
    println!("applying look-and-say 50 times: {}", part2.len());
}

fn look_and_seek(mut input: String, times: usize) -> String {
    for _ in 0..times {
        let mut iter = input.chars().peekable();
        let mut new_input = String::new();
        loop {
            // set start
            let previous = iter.next();
            if previous.is_none() {
                break;
            }
            let mut times = 1;

            // check how many times previous repeats
            loop {
                match iter.peek() {
                    c if c == previous.as_ref() => {
                        times += 1;
                        iter.next();
                    }
                    _ => break,
                }
            }
            // note: i used format!() before and it was a lot slower than this
            // part 2 with 50 iterations took more than 15 min instead of a fraction of a second
            new_input.push_str(&times.to_string());
            new_input.push(previous.unwrap());
        }
        input = new_input;
    }
    input
}
