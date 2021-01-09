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
use num::Integer;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    println!("Compute Cobol Pic");
    //println!("Size: {}", compute("TODO"));
    println!("-----------------");
    let filename = "examples/sample1.cpy";
    println!("In file {}", filename);
    let mut file_path = PathBuf::new();
    file_path.push(env::current_dir().unwrap().as_path());
    file_path.push("examples");
    file_path.push("sample1.cpy");
    let file = File::open(file_path.as_path())?;
    let mut contents = String::new();
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
    contents = reader_mod.iter().fold(String::new(), |a, b| a + b + "\n");
    contents = contents.trim_end().to_string();

    // SPLIT END OF LINE
    //file.read_to_string(&mut contents)?;
    println!("With text:\n{}", &contents);
    println!("------------------");
    let contents_split: Vec<&str> = contents
        .split(".")
        .filter(|&x| {
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
        })
        .collect(); // WARNING "." SHOULD WORK
    let mut vector_debug: Vec<LineDebug> = Vec::new();
    for c in contents_split.iter() {
        let re = Regex::new(r"PIC|OCCURS").unwrap();
        let v_type: Vec<&str> = c.match_indices(&re).map(|(_, x)| x).collect();
        let v: Vec<&str> = re.splitn(c, 2).collect();
        let mut field_pos = "";
        let mut field_size: String = "".to_string();

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
    let mut pos = 0;
    let mut sw_occurs = false;
    let mut occurs_temp = 0;

    let iter: Vec<LineDebug> = vector_debug
        .into_iter()
        .filter(|x| {
            x.field_pos != "".to_string()
                && x.field_size != "".to_string()
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
        .collect();

    let iter_proper: Vec<LineCobol> = iter
        .into_iter()
        .map(|x| {
            let field_type: Type = if x.sw_occurs {
                Type::OCCURS
            } else if x.field_size.contains("X") {
                Type::PICX(x.field_size)
            } else if x.field_size.contains("9")
                || x.field_size.contains("Z")
                || x.field_size.contains("-")
                || x.field_size.contains(".")
            {
                Type::PIC9(x.field_size)
            } else {
                Type::UNKNOWN
            };
            LineCobol {
                pos: x.pos,
                name: x.name,
                occurs: x.occurs,
                sw_occurs: x.sw_occurs,
                field_type,
            }
        })
        .collect();

    for i in iter_proper {
        let mut occ = i.occurs;
        if occ == 0 {
            occ = 1
        };
        println!("Debug: {:?}, size: {}", &i, i.field_type.size() * occ);
    }
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
    sw_occurs: bool,
    field_type: Type,
}

#[derive(Debug)]
enum Type {
    PICX(String),
    PIC9(String),
    OCCURS,
    UNKNOWN,
}

impl Type {
    fn size(&self) -> u32 {
        use Type::*;
        match self {
            PICX(val) => {
                let re = Regex::new(r"X\((\d{1,})\)|X").unwrap();
                let v_type: Vec<&str> =
                    val.match_indices(&re).map(|(_, x)| x).collect();
                v_type.iter().cloned().fold(0, |acc, x| {
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
                })
            },
            PIC9(val) => {
                let re =
                    Regex::new(r"9\((\d{1,})\)|Z\((\d{1,})\)|9|Z|-|.").unwrap();
                let v_type: Vec<&str> =
                    val.match_indices(&re).map(|(_, x)| x).collect();
                v_type.iter().cloned().fold(0, |acc, x| {
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
                    if xx.contains("9") | xx.contains("Z")
                        || xx.contains("-")
                        || xx.contains(".")
                    {
                        acc + 1
                    } else {
                        acc + xx.parse().unwrap_or(0)
                    }
                })
            },
            OCCURS => 0 as u32,
            UNKNOWN => 0 as u32,
        }
    }
}
