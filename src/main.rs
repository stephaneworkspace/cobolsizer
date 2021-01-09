/*
 * Cobol sizer
 * ===========
 *
 * Rust Cobol sizer by Stéphane (https://github.com/stephaneworkspace)
 *
 * Compatible Microfocus RM Cobol
 *
 * L'idée serrait une sélection mode visual dans vim et une touche pour donner
 * la taille dans la quickfix list
 *
 * Et il y a aussi un default à RM Cobol c'est la taille maximum de 65k, un
 * script qui vérifie les sources serrait interessant
 */
extern crate regex;
mod cfg;
use cfg::parse;
use num::Integer;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let clap = parse();
    println!("Compute Cobol Pic");
    //println!("Size: {}", compute("TODO"));
    println!("-----------------");
    //let filename = "examples/sample1.cpy";
    let filename = clap.file;
    println!("In file {}", filename);
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push("examples");
    file_path.push("sample1.cpy");
    let file = File::open(file_path.as_path())?;
    let mut contents;

    // BEFORE SPLIT REMOVE COMMENT * IN COPY FILE read by line
    let reader = BufReader::new(&file);

    let mut reader_mod: Vec<String> = Vec::new();
    for l in reader.lines() {
        let mut sw_ignore = false;
        let line = l?;
        if &line.chars().count() >= &7 {
            let mut chars = line.chars();
            let mut x: char = ' ';
            for _ in 0..7 {
                x = chars.next().unwrap_or(' ');
            }
            if x == '*' {
                sw_ignore = true;
            }
        } else {
            sw_ignore = true;
        }
        if !sw_ignore {
            reader_mod.push(line.to_string())
        }
    }
    contents = reader_mod
        .iter()
        .fold(String::new(), |a, b| format!("{}{}\n", a, b.trim_end()));
    contents = contents.trim_end().to_string();

    // SPLIT END OF LINE
    //file.read_to_string(&mut contents)?;
    println!("With text:\n{}", &contents);
    println!("------------------");

    let contents_split: Vec<&str> = contents
        .split(".\n")
        .filter(|&x| {
            if x.contains(" VALUE ") {
                let mut count_parentese: u32 = 0;
                let c: Vec<&str> = x.split("\"").collect();
                count_parentese += c.len() as u32;
                let d: Vec<&str> = x.split("'").collect();
                count_parentese += d.len() as u32;
                if count_parentese > 0 {
                    count_parentese.is_even()
                } else {
                    true
                }
            } else {
                true
            }
        })
        .collect();

    let mut vector_debug: Vec<LineDebug> = Vec::new();
    for c in contents_split.iter() {
        let re = Regex::new(r"PIC|OCCURS").unwrap();
        let v_type: Vec<&str> = c.match_indices(&re).map(|(_, x)| x).collect();
        let mut field_pos = "";
        let mut field_size: String = "".to_string();
        if v_type.len() > 0 {
            let v: Vec<&str> = re.splitn(c, 2).collect();

            for (i, vv) in v.iter().enumerate() {
                match i {
                    0 => field_pos = vv,
                    1 => {
                        let re_value = Regex::new(r"VALUE").unwrap(); // TODO BINARY
                        let splitn_value: Vec<&str> =
                            re_value.splitn(vv, 2).collect();
                        field_size = splitn_value
                            .iter()
                            .next()
                            .unwrap_or(&"")
                            .replace(" ", "")
                    },
                    _ => break,
                }
            }
        } else {
            field_pos = &c;
        }

        let mut sw_occurs = false;
        for (i, vv) in v_type.iter().enumerate() {
            match i {
                0 => {
                    sw_occurs = vv == &"OCCURS";
                    break;
                },
                _ => {
                    sw_occurs = false;
                    break;
                },
            }
        }

        let occurs: u32 = if sw_occurs {
            field_size.to_string().parse().unwrap_or(0)
        } else {
            0
        };

        let mut iter_name = field_pos.clone().split_ascii_whitespace();
        let _ = iter_name.next().unwrap_or("?").to_string();
        let name: String = iter_name.next().unwrap_or("?").to_string();

        let line_debug = LineDebug {
            pos: field_pos
                .clone()
                .split_ascii_whitespace()
                .next()
                .unwrap_or("0")
                .to_string()
                .parse()
                .unwrap_or(0),
            name,
            occurs,
            sw_occurs: false, // Init value
            field_pos: field_pos.to_string(),
            field_size: field_size.to_string(),
        };
        vector_debug.push(line_debug);
    }
    let _: Vec<&LineDebug> = vector_debug
        .iter()
        .map(|x| x)
        // .inspect(|x| println!("{:?}", x))
        .collect();

    let mut pos = 0;
    let mut sw_occurs = false;
    let mut occurs_temp = 0;

    let iter: Vec<LineDebug> = vector_debug
        .into_iter()
        .filter(|x| {
            x.field_pos != "".to_string()
              //  && x.field_size != "".to_string()
                && x.field_pos.parse().unwrap_or(0) == 0
        })
        .map(|x| {
            let ld = if sw_occurs {
                if &pos < &x.pos {
                    LineDebug {
                        pos: x.pos,
                        name: x.name,
                        occurs: occurs_temp,
                        sw_occurs: false,
                        field_pos: x.field_pos,
                        field_size: x.field_size,
                    }
                } else {
                    sw_occurs = false;
                    pos = 0;
                    occurs_temp = 0;
                    LineDebug {
                        pos: x.pos,
                        name: x.name,
                        occurs: x.occurs,
                        sw_occurs: false,
                        field_pos: x.field_pos,
                        field_size: x.field_size,
                    }
                }
            } else {
                if x.occurs > 0 {
                    sw_occurs = true;
                    pos = x.pos;
                    occurs_temp = x.occurs;
                }
                LineDebug {
                    pos: x.pos,
                    name: x.name,
                    occurs: x.occurs,
                    sw_occurs: x.occurs > 0,
                    field_pos: x.field_pos,
                    field_size: x.field_size,
                }
            };
            ld
        })
        //.filter(|x| !x.sw_occurs)
        .collect();

    let iter_proper: Vec<LineCobol> = iter
        .into_iter()
        .map(|x| {
            let field_type: Type = if x.sw_occurs {
                Type::OCCURS
            } else if x.field_size.contains("X") {
                Type::PICX(x.field_size.clone())
            } else if x.field_size.contains("9")
                || x.field_size.contains("Z")
                || x.field_size.contains("-")
                || x.field_size.contains(".")
                || x.field_size.contains("$")
                || x.field_size.contains("*")
            {
                Type::PIC9(x.field_size.clone())
            } else if x.pos > 0 {
                Type::STRUCT
            } else {
                Type::UNKNOWN
            };
            LineCobol {
                pos: x.pos,
                name: x.name,
                occurs: x.occurs,
                field_type,
                field_type_original: x.field_size,
            }
        })
        .collect();

    let mut min: Vec<u32> = Vec::new();
    for i in iter_proper.iter() {
        let mut occ = i.occurs;
        if occ == 0 {
            occ = 1
        };
        // Check if increment pos or decrement
        let position: u32 = match min.iter().max() {
            Some(max) => {
                if &i.pos <= max {
                    min = min.into_iter().filter(|&x| x < i.pos).collect();
                }
                if min.iter().find(|&x| x == &i.pos) != Some(&i.pos) {
                    min.push(i.pos);
                }
                min.len() as u32
            },
            None => {
                min.push(i.pos);
                min.len() as u32
            },
        };

        let compute_size = match i.field_type.size() {
            Some(size) => size * occ,
            None => 0,
        };
        let space = "    ";
        let begin: String = match position {
            1 => format!("{:6} {:02} {}", compute_size, i.pos, i.name)
                .to_string(),
            2 => format!("{:6} {}{:02} {}", compute_size, space, i.pos, i.name)
                .to_string(),
            3 => format!(
                "{:6} {}{}{:02} {}",
                compute_size, space, space, i.pos, i.name
            )
            .to_string(),
            4 => format!(
                "{:6} {}{}{}{:02} {}",
                compute_size, space, space, space, i.pos, i.name
            )
            .to_string(),
            5 => format!(
                "{:6} {}{}{}{}{:02} {}",
                compute_size, space, space, space, space, i.pos, i.name
            )
            .to_string(),
            6 => format!(
                "{:6} {}{}{}{}{}{:02} {}",
                compute_size, space, space, space, space, space, i.pos, i.name
            )
            .to_string(),
            7 => format!(
                "{:6} {}{}{}{}{}{}{:02} {}",
                compute_size,
                space,
                space,
                space,
                space,
                space,
                space,
                i.pos,
                i.name
            )
            .to_string(),
            _ => format!(
                "{:6} {}{}{}{}{}{}{}{:02} {}",
                compute_size,
                space,
                space,
                space,
                space,
                space,
                space,
                space,
                i.pos,
                i.name
            )
            .to_string(),
        };
        match i.field_type {
            Type::PICX(_) => {
                println!("{:<49}PIC {}.", begin, i.field_type_original);
            },
            Type::PIC9(_) => {
                println!("{:<49}PIC {}.", begin, i.field_type_original);
            },
            Type::STRUCT => {
                println!("{}.", begin);
            },
            Type::OCCURS => {
                println!("{} OCCURS {}.", begin, i.field_type_original);
            },
            _ => {},
        }
    }
    println!("------------------");
    println!(
        "Size: {}",
        iter_proper.iter().fold(0, |acc, x| {
            let mut occ = x.occurs;
            if occ == 0 {
                occ = 1;
            }
            acc + (x.field_type.size().unwrap_or(0) * occ)
        })
    );
    Ok(())
}

#[derive(Debug)]
struct LineDebug {
    pos: u32,
    name: String,
    occurs: u32,
    sw_occurs: bool,
    field_pos: String,
    field_size: String,
}

#[derive(Debug)]
struct LineCobol {
    pos: u32,
    name: String,
    occurs: u32,
    field_type: Type,
    field_type_original: String,
}

#[derive(Debug)]
enum Type {
    PICX(String),
    PIC9(String),
    STRUCT,
    OCCURS,
    UNKNOWN,
}

impl Type {
    fn size(&self) -> Option<u32> {
        use Type::*;
        match self {
            PICX(val) => {
                let re = Regex::new(r"X\((\d{1,})\)|X").unwrap();
                let v_type: Vec<&str> =
                    val.match_indices(&re).map(|(_, x)| x).collect();
                let result: u32 = v_type.iter().cloned().fold(0, |acc, x| {
                    let xx: String = if x.contains("X(") {
                        let mut temp_xx: String =
                            x.replace("X(", "").to_string();
                        temp_xx = temp_xx.replace(")", "").to_string();
                        temp_xx
                    } else {
                        x.to_string()
                    };
                    if xx.contains("X") {
                        acc + 1
                    } else {
                        acc + xx.parse().unwrap_or(0)
                    }
                });
                Some(result)
            },
            PIC9(val) => {
                let re =
                    Regex::new(r"9\((\d{1,})\)|Z\((\d{1,})\)|9|Z|\-|.|V|S")
                        .unwrap();
                let v_type: Vec<&str> =
                    val.match_indices(&re).map(|(_, x)| x).collect();
                let result: u32 = v_type.iter().cloned().fold(0, |acc, x| {
                    let xx: String = if x.contains("9(") {
                        let mut temp_xx: String =
                            x.replace("9(", "").to_string();
                        temp_xx = temp_xx.replace(")", "").to_string();
                        temp_xx
                    } else if x.contains("Z(") {
                        let mut temp_xx: String =
                            x.replace("Z(", "").to_string();
                        temp_xx = temp_xx.replace(")", "").to_string();
                        temp_xx
                    } else {
                        x.to_string()
                    };
                    if xx.contains("9")
                        || xx.contains("Z")
                        || xx.contains("-")
                        || xx.contains(".")
                        || xx.contains("V")
                        || xx.contains("S")
                        || xx.contains("$")
                        || xx.contains("*")
                    {
                        acc + 1
                    } else {
                        acc + xx.parse().unwrap_or(0)
                    }
                });
                Some(result)
            },
            STRUCT => None,
            OCCURS => None,
            UNKNOWN => None,
        }
    }
}
