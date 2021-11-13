use clap::{App, Arg};

use crate::password_generator::generate_pw;

mod password_generator;

fn main() -> Result<(), String> {
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
        generate_pw(t, size)?;
        //println!("{}", pw);
        Ok(())
    } else {
        Err(String::from("How the hell did we get here?"))
    }
}
