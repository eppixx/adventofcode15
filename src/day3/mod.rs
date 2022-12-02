use std::collections::HashMap;

pub fn main() {
    let input = include_str!("./input");
    let mut map = HashMap::new();

    //starting house
    map.insert((0, 0), 1);

    input.chars().fold((0, 0), |acc, c| {
        let new_pos = match c {
            '^' => (acc.0, acc.1 + 1),
            'v' => (acc.0, acc.1 - 1),
            '<' => (acc.0 - 1, acc.1),
            '>' => (acc.0 + 1, acc.1),
            input => unreachable!("unexpected input: {}", input),
        };

        //check if house has been visited yet
        match map.get(&new_pos) {
            Some(presents) => map.insert(new_pos, presents + 1),
            None => map.insert(new_pos, 1),
        };

        //return accumulator
        new_pos
    });

    //part 2
    let mut next_year = HashMap::new();
    next_year.insert((0, 0), 2);
    input
        .chars()
        .enumerate()
        .fold(((0, 0), (0, 0)), |acc, (i, c)| {
            let calc_new_pos = |pos: (i32, i32), c| match c {
                '^' => (pos.0, pos.1 + 1),
                'v' => (pos.0, pos.1 - 1),
                '<' => (pos.0 - 1, pos.1),
                '>' => (pos.0 + 1, pos.1),
                input => unreachable!("unexpected input: {}", input),
            };

            let update_map =
                |map: &mut HashMap<(i32, i32), i32>, pos: (i32, i32)| match map.get(&pos) {
                    Some(presents) => map.insert(pos, presents + 1),
                    None => map.insert(pos, 1),
                };

            if i % 2 == 0 {
                let santa_pos = calc_new_pos(acc.0, c);
                update_map(&mut next_year, santa_pos);
                (santa_pos, acc.1)
            } else {
                let robot_pos = calc_new_pos(acc.1, c);
                update_map(&mut next_year, robot_pos);
                (acc.0, robot_pos)
            }
        });

    println!("\nday3");
    println!("households with at least 1 present: {}", map.len());
    println!(
        "households with at least 1 present next year: {}",
        next_year.len()
    );
}
