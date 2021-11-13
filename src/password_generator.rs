use rand::{thread_rng, Rng};

pub fn generate_pw(level: &str, char_count: usize) -> Result<(), String> {
    let lower_case = "abcdefghijklmnopqrstuvwxyzç";
    let upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZÇ";
    let numbers = "0123456789";
    let special = "!@#$%&*()[]{}:;?/\\=+-_";

    match level {
        "lowest" => {
            for _ in 1..char_count {
                print!("{}", random_char(lower_case)?);
            }
        }
        "low" => {
            for _ in 1..char_count {
                match thread_rng().gen_range(0..2) {
                    0 => print!("{}", random_char(lower_case)?),
                    1 => print!("{}", random_char(numbers)?),
                    _ => {
                        return Err(String::from(
                            "Program malfunction: gen_range() index out of bounds",
                        ))
                    }
                }
            }
        }
        "medium" => {
            for _ in 1..char_count {
                match thread_rng().gen_range(0..3) {
                    0 => print!("{}", random_char(lower_case)?),
                    1 => print!("{}", random_char(numbers)?),
                    2 => print!("{}", random_char(upper_case)?),
                    _ => {
                        return Err(String::from(
                            "Program malfunction: gen_range() index out of bounds",
                        ))
                    }
                }
            }
        }
        "high" => {
            for _ in 1..char_count {
                match thread_rng().gen_range(0..4) {
                    0 => print!("{}", random_char(lower_case)?),
                    1 => print!("{}", random_char(numbers)?),
                    2 => print!("{}", random_char(upper_case)?),
                    3 => print!("{}", random_char(special)?),
                    _ => {
                        return Err(String::from(
                            "Program malfunction: gen_range() index out of bounds",
                        ))
                    }
                }
            }
        }
        _ => return Err(format!("Unknown type: '{}'", level)),
    }
    print!("\n");
    Ok(())
}

fn random_char(charset: &str) -> Result<char, String> {
    let index = thread_rng().gen_range(0..charset.len() - 1);
    let ch = charset
        .chars()
        .nth(index)
        .ok_or(format!("Couldn't fetch char from index {}", index))?;
    Ok(ch)
}
