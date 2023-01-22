#[derive(Default, Debug)]
struct AuntFeatures {
    children: Option<i32>,
    cats: Option<i32>,
    samoyeds: Option<i32>,
    pomeranians: Option<i32>,
    akitas: Option<i32>,
    vizslas: Option<i32>,
    goldfish: Option<i32>,
    trees: Option<i32>,
    cars: Option<i32>,
    perfumes: Option<i32>,
}

fn parse(input: &str) -> nom::IResult<&str, Vec<AuntFeatures>> {
    use nom::bytes::complete::{tag, take, take_while};
    use nom::multi::{separated_list0, separated_list1};
    use nom::sequence::separated_pair;
    use nom::AsChar;

    fn features(input: &str) -> nom::IResult<&str, Vec<(&str, i32)>> {
        let (rest, feature) = separated_list1(
            tag(", "),
            separated_pair(
                take_while(AsChar::is_alpha),
                tag(": "),
                nom::character::complete::i32,
            ),
        )(input)?;
        Ok((rest, feature))
    }

    fn sue(input: &str) -> nom::IResult<&str, i32> {
        let (rest, number) =
            separated_pair(take(3usize), tag(" "), nom::character::complete::i32)(input)?;
        Ok((rest, number.1))
    }

    fn line(input: &str) -> nom::IResult<&str, AuntFeatures> {
        let (rest, features) = separated_pair(sue, tag(": "), features)(input)?;

        let mut aunt = AuntFeatures::default();
        for feature in features.1 {
            match feature {
                ("children", num) => aunt.children = Some(num),
                ("cats", num) => aunt.cats = Some(num),
                ("samoyeds", num) => aunt.samoyeds = Some(num),
                ("pomeranians", num) => aunt.pomeranians = Some(num),
                ("akitas", num) => aunt.akitas = Some(num),
                ("vizslas", num) => aunt.vizslas = Some(num),
                ("goldfish", num) => aunt.goldfish = Some(num),
                ("trees", num) => aunt.trees = Some(num),
                ("cars", num) => aunt.cars = Some(num),
                ("perfumes", num) => aunt.perfumes = Some(num),
                _ => unimplemented!(),
            }
        }

        Ok((rest, aunt))
    }

    let (rest, list) = separated_list0(tag("\n"), line)(input)?;
    Ok((rest, list))
}

pub fn main() {
    println!("\nday 16");
    let input = include_str!("./input");
    //     let input = "Sue 0: cats: 66, cars: 4, samoyeds: 1
    // Sue 1: goldfish: 5, cars: 2, samoyeds: 2";
    let target_aunt = AuntFeatures {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let aunts = parse(input).unwrap().1;
    let pos = aunts.iter().position(|a| {
        let mut matches = vec![];
        if a.children.is_some() {
            matches.push(target_aunt.children == a.children);
        }
        if a.cats.is_some() {
            matches.push(target_aunt.cats == a.cats);
        }
        if a.samoyeds.is_some() {
            matches.push(target_aunt.samoyeds == a.samoyeds)
        }
        if a.pomeranians.is_some() {
            matches.push(target_aunt.pomeranians == a.pomeranians)
        }
        if a.akitas.is_some() {
            matches.push(target_aunt.akitas == a.akitas)
        }
        if a.vizslas.is_some() {
            matches.push(target_aunt.vizslas == a.vizslas)
        }
        if a.goldfish.is_some() {
            matches.push(target_aunt.goldfish == a.goldfish)
        }
        if a.trees.is_some() {
            matches.push(target_aunt.trees == a.trees)
        }
        if a.cars.is_some() {
            matches.push(target_aunt.cars == a.cars)
        }
        if a.perfumes.is_some() {
            matches.push(target_aunt.perfumes == a.perfumes)
        }

        matches.iter().all(|a| a == &true)
    });
    println!("The Aunt your looking for is {:?}", pos.unwrap() + 1);

    //part 2
    let pos = aunts.iter().position(|a| {
        let mut matches = vec![];
        if a.children.is_some() {
            matches.push(target_aunt.children == a.children);
        }
        if a.cats.is_some() {
            matches.push(target_aunt.cats < a.cats);
        }
        if a.samoyeds.is_some() {
            matches.push(target_aunt.samoyeds == a.samoyeds)
        }
        if a.pomeranians.is_some() {
            matches.push(target_aunt.pomeranians > a.pomeranians)
        }
        if a.akitas.is_some() {
            matches.push(target_aunt.akitas == a.akitas)
        }
        if a.vizslas.is_some() {
            matches.push(target_aunt.vizslas == a.vizslas)
        }
        if a.goldfish.is_some() {
            matches.push(target_aunt.goldfish > a.goldfish)
        }
        if a.trees.is_some() {
            matches.push(target_aunt.trees < a.trees)
        }
        if a.cars.is_some() {
            matches.push(target_aunt.cars == a.cars)
        }
        if a.perfumes.is_some() {
            matches.push(target_aunt.perfumes == a.perfumes)
        }

        matches.iter().all(|a| a == &true)
    });
    println!("The real Aunt your looking for is {:?}", pos.unwrap() + 1);
}
