use rand::{thread_rng, Rng};

const LOWER_CASE: &str = "abcdefghijklmnopqrstuvwxyzç";
const UPPER_CASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZÇ";
const NUMBERS: &str = "0123456789";
const SPECIAL: &str = "!@#$%&*()[]{}:;?/\\=+-_";

pub fn generate_pw(level: &str, char_count: usize) -> Result<(), String> {
    match level {
        "lowest" => {
            for _ in 1..char_count {
                print!("{}", random_char(LOWER_CASE)?);
            }
        }
        "low" => {
            for _ in 1..char_count {
                match thread_rng().gen_range(0..2) {
                    0 => print!("{}", random_char(LOWER_CASE)?),
                    1 => print!("{}", random_char(NUMBERS)?),
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
                    0 => print!("{}", random_char(LOWER_CASE)?),
                    1 => print!("{}", random_char(NUMBERS)?),
                    2 => print!("{}", random_char(UPPER_CASE)?),
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
                    0 => print!("{}", random_char(LOWER_CASE)?),
                    1 => print!("{}", random_char(NUMBERS)?),
                    2 => print!("{}", random_char(UPPER_CASE)?),
                    3 => print!("{}", random_char(SPECIAL)?),
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
