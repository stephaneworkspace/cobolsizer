use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub filtered_src: bool,
    pub compute_src: bool,
}

const FILE: &str = "path";
const SHOW_FILTERED_SRC: &str = "show_filtered_src";
const SHOW_DETAIL_COMPUTE: &str = "show_detail_compute";

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
        .arg(
            Arg::with_name(SHOW_FILTERED_SRC)
                .long("detail-src")
                .value_name("DETAIL_FILTERED_SOURCE")
                .help("Show detail filtered source")
                .multiple(false)
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name(SHOW_DETAIL_COMPUTE)
                .long("detail-compute")
                .value_name("SHOW_DETAIL_COMPUTE")
                .help("Show detail computed source")
                .multiple(false)
                .required(false)
                .takes_value(false),
        )
        .get_matches();
    Config {
        file: matches.value_of(FILE).unwrap().to_string(),
        filtered_src: matches.is_present(SHOW_FILTERED_SRC),
        compute_src: matches.is_present(SHOW_DETAIL_COMPUTE),
    }
}
