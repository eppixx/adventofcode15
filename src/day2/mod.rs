pub fn main() {
    let input = include_str!("./input");
    let paper: i32 = input
        .lines()
        .map(|line| {
            let sides: Vec<i32> = line
                .split('x')
                .map(|c| str::parse::<i32>(c).unwrap())
                .collect();
            let (xy, yz, xz) = (
                sides[0] * sides[1],
                sides[1] * sides[2],
                sides[0] * sides[2],
            );
            let paper = 2 * xy + 2 * yz + 2 * xz;
            paper + [xy, yz, xz].iter().min().unwrap()
        })
        .sum();
    let ribbon: i32 = input
        .lines()
        .map(|line| -> i32 {
            let mut sides: Vec<i32> = line
                .split('x')
                .map(|c| str::parse::<i32>(c).unwrap())
                .collect();
            sides.sort();
            let side_ribbon: i32 = sides.iter().take(2).map(|s| s * 2).sum();
            let bow: i32 = sides.iter().product();
            side_ribbon + bow
        })
        .sum();

    println!("\nday 2");
    println!("needed paper: {}", paper);
    println!("ribbon length needed: {}", ribbon);
}
