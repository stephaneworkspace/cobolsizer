use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub file: String,
}

const FILE: &str = "path";

pub fn parse() -> Config {
    let matches = App::new("Cobol sizer")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Stéphane Bressani")
        .about("Size of cobol structure")
        .arg(
            Arg::with_name(FILE)
                .long("file")
                .value_name("FILE")
                .help("File to analyse")
                .multiple(false)
                .required(true),
        )
        .get_matches();
    Config {
        file: matches.value_of(FILE).unwrap().to_string(),
    }
}
