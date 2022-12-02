pub fn main() {
    let input = "bgvyzdsv";
    let mut target_number = 0;

    for i in 0.. {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash = format!("{:x}", digest);
        if hash.starts_with("00000") {
            target_number = i;
            break;
        }
    }

    let mut second_number = 0;
    for i in target_number.. {
        let digest = md5::compute(format!("{}{}", input, i));
        let hash = format!("{:x}", digest);
        if hash.starts_with("000000") {
            second_number = i;
            break;
        }
    }

    println!("\nday4");
    println!("the first number with 00000.. hash is: {}", target_number);
    println!("the first number with 000000.. hash is: {}", second_number);
}
