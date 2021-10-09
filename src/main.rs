use clap::{App, Arg};
use passwords::PasswordGenerator;

fn main() -> Result<(), String> {
    let mut pass_gen = PasswordGenerator::new();
    pass_gen.exclude_similar_characters = true;

    let matches = App::new("Rugen")
        .version(format!("v{}", env!("CARGO_PKG_VERSION")).as_str())
        .author("by S0raWasTaken")
        .about("\nA simple cli app to generate passwords")
        .arg(
            Arg::with_name("type")
                .short("t")
                .help("Types: lowest, low, medium, high")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("size")
                .short("s")
                .help("Password size")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    let size: usize = matches
        .value_of("size")
        .ok_or("No size")?
        .parse()
        .map_err(|_| String::from("That size was not a number"))?;

    if let Some(t) = matches.value_of("type") {
        match t.to_ascii_lowercase().as_str() {
            "lowest" => {
                pass_gen.numbers = false;
                pass_gen.length = size;
                println!("{}", pass_gen.generate_one()?);
                Ok(())
            }
            "low" => {
                pass_gen.length = size;
                println!("{}", pass_gen.generate_one()?);
                Ok(())
            }
            "medium" => {
                pass_gen.uppercase_letters = true;
                pass_gen.length = size;
                println!("{}", pass_gen.generate_one()?);
                Ok(())
            }
            "high" => {
                pass_gen.uppercase_letters = true;
                pass_gen.symbols = true;
                pass_gen.length = size;
                println!("{}", pass_gen.generate_one()?);
                Ok(())
            }
            _ => Err(String::from("Password type unknown")),
        }
    } else {
        Err(String::from("How the hell did we get here?"))
    }
}
