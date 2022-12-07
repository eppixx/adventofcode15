use std::collections::HashMap;

pub fn main() {
    let input = include_str!("./input");

    #[derive(Hash, Eq, PartialEq, Debug, Clone)]
    enum Gate {
        Value(u16),
        Gate(String),
    }

    impl Gate {
        fn new(value: &str) -> Self {
            match value {
                value if str::parse::<u16>(value).is_ok() => {
                    Self::Value(str::parse::<u16>(value).unwrap())
                }
                s => Self::Gate(s.to_string()),
            }
        }
    }

    #[derive(Clone)]
    enum Instruction {
        Set(Gate),
        Not(Gate),
        And(Gate, Gate),
        Or(Gate, Gate),
        Rshift(Gate, u8),
        Lshift(Gate, u8),
    }

    let mut instructions: Vec<(Instruction, String)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let mut iter = split.next().unwrap().split(' ');
            let instruction = match iter.next().unwrap() {
                "NOT" => Instruction::Not(Gate::new(iter.next().unwrap())),
                src => match iter.next() {
                    Some("OR") => Instruction::Or(Gate::new(src), Gate::new(iter.next().unwrap())),
                    Some("AND") => {
                        Instruction::And(Gate::new(src), Gate::new(iter.next().unwrap()))
                    }
                    Some("RSHIFT") => Instruction::Rshift(
                        Gate::new(src),
                        str::parse::<u8>(iter.next().unwrap()).unwrap(),
                    ),
                    Some("LSHIFT") => Instruction::Lshift(
                        Gate::new(src),
                        str::parse::<u8>(iter.next().unwrap()).unwrap(),
                    ),
                    None => Instruction::Set(Gate::new(src)),
                    Some(input) => unreachable!("unexpected input: {} in line {}", input, line),
                },
            };
            (instruction, String::from(split.next().unwrap()))
        })
        .collect();

    // consumes instructions and caches its values till there are no instrucions left
    let reduce_instructions =
        |mut instructions: Vec<(Instruction, String)>| -> HashMap<String, u16> {
            let mut cache: HashMap<String, u16> = HashMap::new();
            loop {
                if instructions.is_empty() {
                    return cache;
                }
                instructions.retain(|(ins, target)| match ins {
                    Instruction::Set(Gate::Value(value)) => {
                        cache.insert(target.clone(), *value);
                        false
                    }
                    Instruction::Set(Gate::Gate(src)) => {
                        if cache.contains_key(src) {
                            cache.insert(target.clone(), cache[src]);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Not(Gate::Value(value)) => {
                        cache.insert(target.clone(), !*value);
                        false
                    }
                    Instruction::Not(Gate::Gate(src)) => {
                        if cache.contains_key(src) {
                            cache.insert(target.clone(), !cache[src]);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Lshift(Gate::Value(value), amount) => {
                        cache.insert(target.clone(), value << amount);
                        false
                    }
                    Instruction::Lshift(Gate::Gate(src), amount) => {
                        if cache.contains_key(src) {
                            cache.insert(target.clone(), cache[src] << amount);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Rshift(Gate::Value(value), amount) => {
                        cache.insert(target.clone(), value >> amount);
                        false
                    }
                    Instruction::Rshift(Gate::Gate(src), amount) => {
                        if cache.contains_key(src) {
                            cache.insert(target.clone(), cache[src] >> amount);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::And(Gate::Value(left), Gate::Value(right)) => {
                        cache.insert(target.clone(), left & right);
                        false
                    }
                    Instruction::And(Gate::Gate(left), Gate::Value(right)) => {
                        if cache.contains_key(left) {
                            cache.insert(target.clone(), cache[left] & right);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::And(Gate::Value(left), Gate::Gate(right)) => {
                        if cache.contains_key(right) {
                            cache.insert(target.clone(), left & cache[right]);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::And(Gate::Gate(left), Gate::Gate(right)) => {
                        if cache.contains_key(left) && cache.contains_key(right) {
                            cache.insert(target.clone(), cache[left] & cache[right]);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Or(Gate::Value(left), Gate::Value(right)) => {
                        cache.insert(target.clone(), left | right);
                        false
                    }
                    Instruction::Or(Gate::Gate(left), Gate::Value(right)) => {
                        if cache.contains_key(left) {
                            cache.insert(target.clone(), cache[left] | right);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Or(Gate::Value(left), Gate::Gate(right)) => {
                        if cache.contains_key(right) {
                            cache.insert(target.clone(), left | cache[right]);
                            false
                        } else {
                            true
                        }
                    }
                    Instruction::Or(Gate::Gate(left), Gate::Gate(right)) => {
                        if cache.contains_key(left) && cache.contains_key(right) {
                            cache.insert(target.clone(), cache[left] | cache[right]);
                            false
                        } else {
                            true
                        }
                    }
                });
            }
        };

    let cache = reduce_instructions(instructions.clone());

    //part 2
    instructions.push((Instruction::Set(Gate::Value(cache["a"])), String::from("b")));
    let cache2 = reduce_instructions(instructions);

    println!("\nday 7");
    println!("value at wire a is: {}", cache["a"]);
    println!("now value at wire a is: {}", cache2["a"]);
}
