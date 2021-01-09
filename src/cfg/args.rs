use clap::{App, Arg};

#[derive(Debug)]
pub struct Config {
    pub file: String,
    pub result: bool,
    pub rm_cobol_limit_65280: bool,
    pub filtered_src: bool,
    pub compute_src: bool,
    pub compute_struct_and_occurs: bool,
    pub separator: bool,
}

const FILE: &str = "path";
const RESULT: &str = "result";
const CHECK_RM_COBOL_LIMIT: &str = "rm_cobol_limit";
const SHOW_FILTERED_SRC: &str = "show_filtered_src";
const SHOW_DETAIL_COMPUTE: &str = "show_detail_compute";
const SHOW_COMPUTE_STRUCT_AND_OCCURS: &str = "compute_struct_and_occurs";
const SEPARATOR: &str = "separator";

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
            Arg::with_name(RESULT)
                .long("result")
                .value_name("RESULT")
                .help("Show result")
                .multiple(false)
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name(CHECK_RM_COBOL_LIMIT)
                .long("rm-cobol-limit")
                .value_name("CHECK_RM_COBOL_LIMIT")
                .help("Show only data structure/occurs > 65280 bytes")
                .multiple(false)
                .required(false)
                .takes_value(false),
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
        .arg(
            Arg::with_name(SHOW_COMPUTE_STRUCT_AND_OCCURS)
                .long("compute-struct-occurs")
                .value_name("SHOW_COMPUTE_STRUCT_AND_OCCURS")
                .help("Show detail of structure and occurs")
                .multiple(false)
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name(SEPARATOR)
                .long("separator")
                .value_name("SEPARATOR")
                .help("Display separator -------")
                .multiple(false)
                .required(false)
                .takes_value(false),
        )
        .get_matches();
    // Set result if nothing selected
    let result = if !matches.is_present(SHOW_COMPUTE_STRUCT_AND_OCCURS)
        && !matches.is_present(SHOW_DETAIL_COMPUTE)
        && !matches.is_present(SHOW_FILTERED_SRC)
        && !matches.is_present(CHECK_RM_COBOL_LIMIT)
    {
        true
    } else {
        matches.is_present(RESULT)
    };
    Config {
        file: matches.value_of(FILE).unwrap().to_string(),
        result: result,
        rm_cobol_limit_65280: matches.is_present(CHECK_RM_COBOL_LIMIT),
        filtered_src: matches.is_present(SHOW_FILTERED_SRC),
        compute_src: matches.is_present(SHOW_DETAIL_COMPUTE),
        compute_struct_and_occurs: matches
            .is_present(SHOW_COMPUTE_STRUCT_AND_OCCURS),
        separator: matches.is_present(SEPARATOR),
    }
}
