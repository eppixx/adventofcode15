pub fn main() {
    println!("\nday 14");
    let input = include_str!("./input");

    #[derive(PartialEq, Eq, Hash, Debug)]
    struct Deer<'a> {
        name: &'a str,
        speed: i32,
        fly_time: i32,
        rest_time: i32,
    }
    let time = 2503;

    let data: Vec<Deer> = input
        .lines()
        .map(|line| {
            let mut split = line.split_terminator(&[' ', '.']);
            Deer {
                name: split.next().unwrap(),
                speed: str::parse::<i32>(split.nth(2).unwrap()).unwrap(),
                fly_time: str::parse::<i32>(split.nth(2).unwrap()).unwrap(),
                rest_time: str::parse::<i32>(split.nth(6).unwrap()).unwrap(),
            }
        })
        .collect();

    let distances: Vec<i32> = data
        .iter()
        .map(|d| {
            let mut delta = 0;
            let mut distance = 0;
            let mut flying = true;
            while delta < time {
                match (flying, time - delta) {
                    (true, dt) if dt >= d.fly_time => {
                        delta += d.fly_time;
                        distance += d.speed * d.fly_time;
                        flying = false;
                    }
                    (false, dt) if dt >= d.rest_time => {
                        delta += d.rest_time;
                        flying = true;
                    }
                    (true, dt) => {
                        delta += dt;
                        distance += dt * d.speed;
                        flying = false;
                    }
                    _ => break,
                }
                // println!("{delta}, {} {} {}", time - delta, flying, distance);
            }
            distance
        })
        .collect();

    println!(
        "The fastest Reindeer flew {} km",
        distances.iter().max().unwrap()
    );

    //part 2
    #[derive(PartialEq, Debug)]
    enum State {
        Flying(i32),
        Resting(i32),
    }
    #[derive(Debug)]
    struct DeerState {
        state: State,
        distance: i32,
        score: i32,
    }
    let mut deer_map = std::collections::HashMap::new();
    data.iter().for_each(|d| {
        let _ = deer_map.insert(
            d,
            DeerState {
                state: State::Flying(d.fly_time),
                distance: 0,
                score: 0,
            },
        );
    });
    for _i in 1..time {
        // update deers
        for deer in &data {
            if let Some(deer_state) = deer_map.get_mut(deer) {
                match deer_state {
                    DeerState {
                        state,
                        distance,
                        score: _,
                    } if *state == State::Flying(1) => {
                        *state = State::Resting(deer.rest_time);
                        *distance += deer.speed;
                    }
                    DeerState {
                        state,
                        distance: _,
                        score: _,
                    } if *state == State::Resting(1) => {
                        *state = State::Flying(deer.fly_time);
                    }
                    DeerState {
                        state: State::Flying(time),
                        distance,
                        score: _,
                    } => {
                        *time -= 1;
                        *distance += deer.speed;
                    }
                    DeerState {
                        state: State::Resting(time),
                        distance: _,
                        score: _,
                    } => {
                        *time -= 1;
                    }
                }
            }
        }
        // scoring
        let max_distance = deer_map.iter().map(|d| d.1.distance).max().unwrap();
        deer_map
            .iter_mut()
            .filter(|d| d.1.distance == max_distance)
            .for_each(|d| d.1.score += 1);
    }

    println!(
        "Points of the winning reindeer: {}",
        deer_map.iter().map(|d| d.1.score).max().unwrap()
    );
}
