pub fn main() {
    println!("\nday 12");
    let input = include_str!("./input");

    let input: serde_json::Value = serde_json::from_str(input).unwrap();

    println!("The sum of numbers in the document is {}", iterate(&input));
    println!(
        "The sum of numbers with Value red in the document is {}",
        iterate_for_red(&input)
    );
}

fn iterate(input: &serde_json::Value) -> i64 {
    use serde_json::Value;
    match input {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(iterate).sum(),
        Value::Object(map) => map.iter().map(|(_key, value)| iterate(value)).sum(),
        _ => 0,
    }
}

fn iterate_for_red(input: &serde_json::Value) -> i64 {
    println!("{:?}", input);
    use serde_json::Value;
    match input {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(arr) => arr.iter().map(iterate_for_red).sum(),
        Value::Object(map) => {
            if map
                .iter()
                .any(|(_key, value)| matches!(value, Value::String(value) if value == "red"))
            {
                0
            } else {
                map.iter().map(|(_key, value)| iterate_for_red(value)).sum()
            }
        }
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1() {
        let input = "[1,2,3]";
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 6);
        let input = r#"{"a":2,"b":4}"#;
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 6);
        let input = r#"{"a":{"b":4},"c":-1}"#;
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 3);
        let input = "[[[3]]]";
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 3);
        let input = r#"{"a":[-1,1]}"#;
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 0);
        let input = r#"[-1,{"a":1}]"#;
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 0);
        let input = "[]";
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 0);
        let input = "{}";
        assert_eq!(super::iterate(&serde_json::from_str(input).unwrap()), 0);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            ("[1,2,3]", 6),
            (r#"[1,{"c":"red","b":2},3]"#, 4),
            (r#"{"d":"red","e":[1,2,3,4],"f":5}"#, 0),
            (r#"[1,"red",5]"#, 6),
        ];
        for test in input {
            assert_eq!(
                super::iterate_for_red(&serde_json::from_str(test.0).unwrap()),
                test.1
            );
        }
    }
}
