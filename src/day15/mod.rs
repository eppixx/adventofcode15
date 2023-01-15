use itertools::Itertools;

fn parse(input: &str) -> nom::IResult<&str, Vec<Ingredient>> {
    use nom::bytes::complete::{tag, take_while};
    use nom::multi::separated_list0;
    use nom::sequence::separated_pair;
    use nom::AsChar;

    fn ingreds(input: &str) -> nom::IResult<&str, Vec<(&str, i32)>> {
        let (rest, ingreds) = separated_list0(
            tag(", "),
            separated_pair(
                take_while(AsChar::is_alpha),
                tag(" "),
                nom::character::complete::i32,
            ),
        )(input)?;
        Ok((rest, ingreds))
    }

    fn line(input: &str) -> nom::IResult<&str, Ingredient> {
        let (rest, result) =
            separated_pair(take_while(AsChar::is_alpha), tag(": "), ingreds)(input)?;
        Ok((
            rest,
            Ingredient {
                capacity: result.1[0].1,
                durability: result.1[1].1,
                flavor: result.1[2].1,
                texture: result.1[3].1,
                calories: result.1[4].1,
            },
        ))
    }
    let (rest, list) = separated_list0(tag("\n"), line)(input)?;
    Ok((rest, list))
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

pub fn main() {
    let input = include_str!("./input");
    println!("\nday 15");

    let ingreds: Vec<Ingredient> = parse(input).unwrap().1;

    let combies = itertools::repeat_n(0..=100, ingreds.len())
        .multi_cartesian_product()
        .filter(|c| c.iter().sum::<i32>() == 100);

    let mut results = vec![];
    for combi in combies.clone() {
        let mut capa = 0;
        let mut dura = 0;
        let mut flavor = 0;
        let mut texture = 0;
        for (i, v) in combi.iter().enumerate() {
            capa += ingreds[i].capacity * v;
            dura += ingreds[i].durability * v;
            flavor += ingreds[i].flavor * v;
            texture += ingreds[i].texture * v;
        }
        capa = capa.max(0);
        dura = dura.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);
        results.push((combi, capa * dura * flavor * texture));
    }

    println!(
        "Score of the highest scoring cookie: {}",
        results.iter().map(|r| r.1).max().unwrap()
    );

    //part 2
    let mut results = vec![];
    for combi in combies {
        let mut capa = 0;
        let mut dura = 0;
        let mut flavor = 0;
        let mut texture = 0;
        let mut calo = 0;
        for (i, v) in combi.iter().enumerate() {
            capa += ingreds[i].capacity * v;
            dura += ingreds[i].durability * v;
            flavor += ingreds[i].flavor * v;
            texture += ingreds[i].texture * v;
            calo += ingreds[i].calories * v;
        }
        capa = capa.max(0);
        dura = dura.max(0);
        flavor = flavor.max(0);
        texture = texture.max(0);
        results.push((calo, capa * dura * flavor * texture));
    }

    println!(
        "Score of the highest scoring cookie with 500 calories: {}",
        results
            .iter()
            .filter(|r| r.0 == 500)
            .map(|r| r.1)
            .max()
            .unwrap()
    );
}
