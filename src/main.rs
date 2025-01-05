use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let n: i32 = rng.gen_range(000_000_000..1_000_000_000);
    let first_vd: u32;
    let second_vd: u32;

    first_vd = calculate_vd(n.to_string().as_str());

    let mut new_n: String = "".to_owned();

    new_n.push_str(n.to_string().as_str());
    new_n.push_str(&first_vd.to_string());

    second_vd = calculate_vd(new_n.to_string().as_str());

    let mut result: String = "".to_owned();
    result.push_str(new_n.to_string().as_str());
    result.push_str(&second_vd.to_string());

    if result.len() < 11 {
        result = format!("{:0>11}", result);
    }

    println!("{}", result);
}

fn calculate_vd(number: &str) -> u32 {
    let result: u32;
    let mut sum: u32 = 0;
    let mut multiplier: u32 = (number.chars().count() as u32) + 1;

    for c in number.chars() {
        let current_string = c.to_string();
        let current_number: u32 = current_string.parse().unwrap();
        sum += current_number * multiplier;
        multiplier -= 1;
    }

    let remainder: u32 = sum % 11;

    if remainder < 2 {
        result = 0
    } else {
        result = 11 - remainder
    }

    return result;
}
