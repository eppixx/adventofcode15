#[derive(PartialEq, Eq)]
enum Lights {
    On,
    Off,
    Toggle,
}

pub fn main() {
    let input = include_str!("./input");
    let mut light_grid = vec![vec![false; 1000]; 1000];

    let split_coords = |line: &str| -> (usize, usize) {
        let mut iter = line.split(',');
        (
            str::parse::<usize>(iter.next().unwrap()).unwrap(),
            str::parse::<usize>(iter.next().unwrap()).unwrap(),
        )
    };

    type Point = (usize, usize);
    let instructions: Vec<(Lights, Point, Point)> = input
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            if iter.next() == Some("toggle") {
                let start = split_coords(iter.next().unwrap());
                let end = split_coords(iter.nth(1).unwrap());
                (Lights::Toggle, start, end)
            } else {
                let lights = match iter.next() {
                    Some("off") => Lights::Off,
                    Some("on") => Lights::On,
                    input => unreachable!("unexpected input: {:?}", input),
                };
                let start = split_coords(iter.next().unwrap());
                let end = split_coords(iter.nth(1).unwrap());
                (lights, start, end)
            }
        })
        .collect();

    // change grid according to instructions
    instructions.iter().for_each(|(inst, start, end)| {
        for line in light_grid.iter_mut().take(end.0 + 1).skip(start.0) {
            for light in line.iter_mut().take(end.1 + 1).skip(start.1) {
                match inst {
                    Lights::Toggle => *light = !*light,
                    Lights::On => *light = true,
                    Lights::Off => *light = false,
                }
            }
        }
    });

    let lights_on: usize = light_grid
        .iter()
        .map(|line| line.iter().map(|light| usize::from(*light)).sum::<usize>())
        .sum();

    //part 2
    let mut light_grid = vec![vec![0; 1000]; 1000];
    instructions.iter().for_each(|(inst, start, end)| {
        for line in light_grid.iter_mut().take(end.0 + 1).skip(start.0) {
            for light in line.iter_mut().take(end.1 + 1).skip(start.1) {
                match inst {
                    Lights::Toggle => *light += 2,
                    Lights::On => *light += 1,
                    Lights::Off => *light -= if *light == 0 { 0 } else { 1 },
                }
            }
        }
    });

    let total_brightness: i32 = light_grid.iter().map(|line| line.iter().sum::<i32>()).sum();

    println!("\nday6");
    println!("count of turned on lights: {}", lights_on);
    println!("total brightness of lights: {}", total_brightness);
}
