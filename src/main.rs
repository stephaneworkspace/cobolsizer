/*
 * Cobol sizer
 * ===========
 *
 * Rust Cobol sizer by Stéphane (https://github.com/stephaneworkspace)
 *
 * Compatible Microfocus RM Cobol
 *
 */
extern crate regex;
mod cfg;
use cfg::parse;
use num::Integer;
use regex::Regex;
//use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    let clap = parse();
    let current_file: String = clap.file.clone();
    let sw_separator = clap.separator;
    let filename = clap.file;
    let mut file_path = PathBuf::new();
    //file_path.push(env::current_dir().unwrap().as_path());
    //file_path.push("examples");
    //file_path.push("sample1.cpy");
    file_path.push(&filename);
    let file = File::open(file_path.as_path())?;
    let reader = BufReader::new(&file);

    // BEFORE SPLIT REMOVE COMMENT * IN COPY FILE read by line
    let mut contents;

    let mut reader_mod: Vec<String> = Vec::new();
    for l in reader.lines() {
        let mut sw_ignore = false;
        let line = l.unwrap_or("".to_string());
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
            let mut temp = line.to_string();
            // Remove before * comment
            if temp.chars().count() > 6 {
                for _ in 0..6 {
                    temp.remove(0);
                }
            }
            reader_mod.push(temp)
        }
    }
    contents = reader_mod
        .iter()
        .fold(String::new(), |a, b| format!("{}{}\n", a, b));
    contents = contents.trim_end().to_string();

    if clap.filtered_src {
        println!("{}", &contents);
        if sw_separator.clone() {
            println!("------");
        }
    }

    // SPLIT END OF LINE
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
    // Bug sometime no \n at end
    let mut sw_last = true;
    let contents_split_2: Vec<String> = contents_split
        .iter()
        .rev()
        .map(|&x| {
            if sw_last {
                let mut temp: String = x.to_string();
                match temp.trim_end().chars().rev().next() {
                    Some('.') => {
                        temp.pop();
                    },
                    _ => {},
                }
                sw_last = false;
                temp
            } else {
                x.to_string()
            }
        })
        .collect();

    let mut vector_debug: Vec<LineDebug> = Vec::new();
    for c in contents_split_2.iter().rev() {
        let re = Regex::new(r" PIC| OCCURS").unwrap();
        let v_type: Vec<&str> = c.match_indices(&re).map(|(_, x)| x).collect();
        let mut field_pos = "";
        let mut field_size: String = "".to_string();
        if v_type.len() > 0 {
            let v: Vec<&str> = re.splitn(c, 2).collect();
            for (i, vv) in v.iter().enumerate() {
                match i {
                    0 => field_pos = vv,
                    1 => {
                        let re_value = Regex::new(r"VALUE|BLANK ZERO").unwrap();
                        let splitn_value: Vec<&str> =
                            re_value.splitn(vv, 2).collect();
                        field_size = splitn_value
                            .iter()
                            .next()
                            .unwrap_or(&"")
                            .replace(" ", "")
                            .trim_end()
                            .to_string()
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
                    sw_occurs = vv == &" OCCURS";
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
            sw_occurs: false,        // Init value
            sw_occurs_inside: false, // Init value
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
                        sw_occurs_inside: false,
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
                        sw_occurs_inside: false,
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
                    sw_occurs_inside: false,
                    field_pos: x.field_pos,
                    field_size: x.field_size,
                }
            };
            ld
        })
        // Detect OCCURS inside PIC
        .map(|x| {
            if x.field_size.contains("PIC") {
                let re_value = Regex::new(r"PIC").unwrap();
                let splitn_value: Vec<&str> =
                    re_value.splitn(&x.field_size, 2).collect();
                let mut iter = splitn_value.iter();
                let occurs = iter
                    .next()
                    .unwrap_or(&"")
                    .replace(" ", "")
                    .parse()
                    .unwrap_or(0);
                let field_size = iter.next().unwrap_or(&"").replace(" ", "");
                LineDebug {
                    pos: x.pos,
                    name: x.name,
                    occurs: occurs,
                    sw_occurs: false,
                    sw_occurs_inside: true,
                    field_pos: x.field_pos,
                    field_size,
                }
            } else {
                LineDebug {
                    pos: x.pos,
                    name: x.name,
                    occurs: x.occurs,
                    sw_occurs: x.sw_occurs,
                    sw_occurs_inside: false,
                    field_pos: x.field_pos,
                    field_size: x.field_size,
                }
            }
        })
        //.filter(|x| !x.sw_occurs)
        .collect();
    let mut old_pos: u32 = 0;
    let mut sw_redefines = false;
    let iter_proper: Vec<LineCobol> = iter
        .into_iter()
        .map(|x| {
            let mut field_size: String = x.field_size.clone();
            let field_type: Type = if x.sw_occurs {
                Type::OCCURS
            } else if field_size.contains("X") {
                if x.field_pos.contains(" REDEFINES ") {
                    Type::PICXREDEFINES
                } else {
                    // Page 14
                    // https://www.microfocus.com/documentation/rm-cobol/1214/RMC-LRM.pdf
                    if field_size.clone().contains("COMP-1") {
                        field_size = field_size.replace("COMP-1", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP1,
                        ))
                    } else if field_size.clone().contains("COMP-3") {
                        field_size = field_size.replace("COMP-3", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP3,
                        ))
                    } else if field_size.clone().contains("COMP-4") {
                        field_size = field_size.replace("COMP-4", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP4,
                        ))
                    } else if field_size.clone().contains("COMP-5") {
                        field_size = field_size.replace("COMP-5", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP5,
                        ))
                    } else if field_size.clone().contains("COMP-6") {
                        field_size = field_size.replace("COMP-6", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP6,
                        ))
                    } else if field_size.clone().contains("PACKED-DECIMAL") {
                        field_size = field_size.replace("PACKED-DECIMAL", "");
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::PACKEDDECIMAL,
                        ))
                    } else if field_size.clone().contains("BINARY(") {
                        let re_value =
                            Regex::new(r"BINARY\((\d{1,})\)Y").unwrap();
                        let splitn_value: Vec<&str> =
                            re_value.splitn(&field_size, 2).collect();
                        let mut iter = splitn_value.iter();
                        let pic = iter.next().unwrap_or(&"").replace(" ", "");
                        let binary = iter
                            .next()
                            .unwrap_or(&"")
                            .replace(" ", "")
                            .parse()
                            .unwrap_or(0);
                        field_size = pic.clone();
                        Type::PICX((
                            pic,
                            x.sw_occurs_inside,
                            Binary::BINARY(binary),
                        ))
                    } else {
                        Type::PICX((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::None,
                        ))
                    }
                }
            } else if field_size.contains("9")
                || field_size.contains("Z")
                || field_size.contains("-")
                || field_size.contains(".")
                || field_size.contains("$")
                || field_size.contains("*")
            {
                if x.field_pos.contains(" REDEFINES ") {
                    Type::PIC9REDEFINES
                } else {
                    // Page 14
                    // https://www.microfocus.com/documentation/rm-cobol/1214/RMC-LRM.pdf
                    if field_size.clone().contains("COMP-1") {
                        field_size = field_size.replace("COMP-1", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP1,
                        ))
                    } else if field_size.clone().contains("COMP-3") {
                        field_size = field_size.replace("COMP-3", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP3,
                        ))
                    } else if field_size.clone().contains("COMP-4") {
                        field_size = field_size.replace("COMP-4", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP4,
                        ))
                    } else if field_size.clone().contains("COMP-5") {
                        field_size = field_size.replace("COMP-5", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP5,
                        ))
                    } else if field_size.clone().contains("COMP-6") {
                        field_size = field_size.replace("COMP-6", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::COMP6,
                        ))
                    } else if field_size.clone().contains("PACKED-DECIMAL") {
                        field_size = field_size.replace("PACKED-DECIMAL", "");
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::PACKEDDECIMAL,
                        ))
                    } else if field_size.clone().contains("BINARY(") {
                        let re_value =
                            Regex::new(r"BINARY\((\d{1,})\)Y").unwrap();
                        let splitn_value: Vec<&str> =
                            re_value.splitn(&x.field_size, 2).collect();
                        let mut iter = splitn_value.iter();
                        let pic = iter.next().unwrap_or(&"").replace(" ", "");
                        let binary = iter
                            .next()
                            .unwrap_or(&"")
                            .replace(" ", "")
                            .parse()
                            .unwrap_or(0);
                        field_size = pic.clone();
                        Type::PIC9((
                            pic,
                            x.sw_occurs_inside,
                            Binary::BINARY(binary),
                        ))
                    } else {
                        Type::PIC9((
                            field_size.clone(),
                            x.sw_occurs_inside,
                            Binary::None,
                        ))
                    }
                }
            } else if x.pos > 0 {
                if x.field_pos.contains(" REDEFINES ") {
                    Type::STRUCTREDEFINES
                } else {
                    Type::STRUCT
                }
            } else {
                Type::UNKNOWN
            };
            LineCobol {
                pos: x.pos,
                name: x.name,
                occurs: x.occurs,
                field_type,
                field_type_original: field_size,
            }
        })
        // ignore REDEFINES + niveau 88 + niveau 66
        .filter(|x| {
            if x.pos == 88 || x.pos == 66 {
                false
            } else {
                match x.field_type {
                    Type::PICXREDEFINES => false,
                    Type::PIC9REDEFINES => false,
                    Type::STRUCTREDEFINES => {
                        sw_redefines = true;
                        old_pos = x.pos;
                        false
                    },
                    _ => {
                        if sw_redefines {
                            if x.pos > old_pos {
                                false
                            } else {
                                sw_redefines = false;
                                old_pos = 0;
                                true
                            }
                        } else {
                            true
                        }
                    },
                }
            }
        })
        //.inspect(|x| println!("{:?}", x))
        .collect();
    let mut iter_struct_and_occurs: Vec<LineCobol> = Vec::new();
    for (i, record) in iter_proper.iter().enumerate() {
        match &record.field_type {
            Type::STRUCT | Type::OCCURS => {
                let current_pos = record.pos;
                let mut sw_stop = false;
                let size: u32 = iter_proper
                    .iter()
                    .enumerate()
                    .filter(|(j, x)| {
                        if sw_stop {
                            false
                        } else {
                            if *j > i {
                                if x.pos > current_pos.clone() {
                                    true
                                } else {
                                    sw_stop = true;
                                    false
                                }
                            } else {
                                false
                            }
                        }
                    })
                    .fold(0, |acc, (_, x)| {
                        let mut occ: u32 = x.occurs;
                        if occ == 0 {
                            occ = 1;
                        }
                        acc + (x.field_type.size().unwrap_or(0) * occ)
                    });
                let field_type = match &record.field_type {
                    Type::STRUCT => Type::STRUCTSIZED(size),
                    Type::OCCURS => Type::OCCURSSIZED(size),
                    _ => Type::UNKNOWN,
                };
                iter_struct_and_occurs.push(LineCobol {
                    pos: record.pos,
                    name: (*record.name).to_string(),
                    occurs: record.occurs,
                    field_type,
                    field_type_original: (*record.field_type_original)
                        .to_string(),
                });
            },
            _ => {},
        }
    }

    let sw_compute_struct_and_occurs = clap.compute_struct_and_occurs;
    if sw_compute_struct_and_occurs.clone() {
        display(
            iter_struct_and_occurs.iter().collect(),
            sw_separator.clone(),
            false,
            "".to_string(),
        );
    }

    if clap.rm_cobol_limit_65280 {
        display(
            iter_struct_and_occurs
                .iter()
                .filter(|x| match x.field_type {
                    Type::STRUCTSIZED(val) => {
                        if val > 65280 {
                            true
                        } else {
                            false
                        }
                    },
                    Type::OCCURSSIZED(val) => {
                        if val > 65280 {
                            true
                        } else {
                            false
                        }
                    },
                    _ => false,
                })
                .collect(),
            sw_separator.clone(),
            true,
            current_file,
        );
    }

    let sw_compute_src = clap.compute_src;
    if sw_compute_src.clone() {
        display(
            iter_proper.iter().collect(),
            sw_separator.clone(),
            false,
            "".to_string(),
        );
    }
    let error: u32 = iter_proper.iter().fold(0, |acc, x| match x.field_type {
        Type::UNKNOWN => acc + 1,
        _ => acc,
    });
    if clap.result {
        if sw_compute_src {
            println!(
                "{:6}",
                iter_proper.iter().fold(0, |acc, x| {
                    let mut occ = x.occurs;
                    if occ == 0 {
                        occ = 1;
                    }
                    acc + (x.field_type.size().unwrap_or(0) * occ)
                })
            );
        } else {
            println!(
                "{}",
                iter_proper.iter().fold(0, |acc, x| {
                    let mut occ = x.occurs;
                    if occ == 0 {
                        occ = 1;
                    }
                    acc + (x.field_type.size().unwrap_or(0) * occ)
                })
            );
        }
    }
    // That can be normal COBOL code
    if clap.error && error > 0 {
        eprintln!("{} structure COBOL error line found !", error);
    }
    Ok(())
}

#[derive(Debug)]
struct LineDebug {
    pos: u32,
    name: String,
    occurs: u32,
    sw_occurs: bool,
    sw_occurs_inside: bool,
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

fn display(
    vec: Vec<&LineCobol>,
    sw_separator: bool,
    sw_file_name_error: bool,
    file_name: String,
) {
    let mut min: Vec<u32> = Vec::new();
    for i in vec.iter() {
        let mut occ = i.occurs;
        if occ == 0 {
            occ = 1
        };
        // Security
        match &i.field_type {
            Type::OCCURSSIZED(_) => {
                occ = 1;
            },
            _ => {},
        }
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
        let text_occurs: String = match i.field_type {
            Type::PICX((_, sw_occurs_inside, _)) => {
                if sw_occurs_inside {
                    format!("OCCURS {}", i.occurs)
                } else {
                    "".to_string()
                }
            },
            Type::PIC9((_, sw_occurs_inside, _)) => {
                if sw_occurs_inside {
                    format!("OCCURS {}", i.occurs)
                } else {
                    "".to_string()
                }
            },
            _ => "".to_string(),
        };
        let begin: String = match position {
            1 => format!(
                "{:6} {:02} {} {}",
                compute_size, i.pos, i.name, text_occurs
            )
            .trim_end()
            .to_string(),
            2 => format!(
                "{:6} {}{:02} {} {}",
                compute_size, space, i.pos, i.name, text_occurs
            )
            .trim_end()
            .to_string(),
            3 => format!(
                "{:6} {}{}{:02} {} {}",
                compute_size, space, space, i.pos, i.name, text_occurs
            )
            .trim_end()
            .to_string(),
            4 => format!(
                "{:6} {}{}{}{:02} {} {}",
                compute_size, space, space, space, i.pos, i.name, text_occurs
            )
            .trim_end()
            .to_string(),
            5 => format!(
                "{:6} {}{}{}{}{:02} {} {}",
                compute_size,
                space,
                space,
                space,
                space,
                i.pos,
                i.name,
                text_occurs
            )
            .trim_end()
            .to_string(),
            6 => format!(
                "{:6} {}{}{}{}{}{:02} {} {}",
                compute_size,
                space,
                space,
                space,
                space,
                space,
                i.pos,
                i.name,
                text_occurs
            )
            .trim_end()
            .to_string(),
            7 => format!(
                "{:6} {}{}{}{}{}{}{:02} {} {}",
                compute_size,
                space,
                space,
                space,
                space,
                space,
                space,
                i.pos,
                i.name,
                text_occurs
            )
            .trim_end()
            .to_string(),
            _ => format!(
                "{:6} {}{}{}{}{}{}{}{:02} {} {}",
                compute_size,
                space,
                space,
                space,
                space,
                space,
                space,
                space,
                i.pos,
                i.name,
                text_occurs
            )
            .trim_end()
            .to_string(),
        };
        let print: String = match &i.field_type {
            Type::PICX((_, _, bin)) | Type::PIC9((_, _, bin)) => {
                match bin {
                    Binary::None => {
                        format!("{:<49}PIC {}.", begin, i.field_type_original)
                    },
                    _ => {
                        format!(
                            "{:<49}PIC {} {}.",
                            begin,
                            i.field_type_original,
                            bin.text()
                        )
                    },
                }
            },
            Type::STRUCT => {
                format!("{}.", begin)
            },
            Type::OCCURS => {
                format!("{} OCCURS {}.", begin, i.field_type_original)
            },
            Type::STRUCTSIZED(_) => {
                format!("{}.", begin)
            },
            Type::OCCURSSIZED(_) => {
                format!("{} OCCURS {}.", begin, i.field_type_original)
            },
            _ => "".to_string(),
        };
        if print != "".to_string() {
            if sw_file_name_error {
                println!("{:<80}{}", print, file_name);
            } else {
                println!("{}", print);
            }
        }
    }
    if sw_separator.clone() {
        println!("------");
    }
}

#[derive(Debug)]
enum Type {
    PICX((String, bool, Binary)), // value + sw_occurs_inside + Optional Binary
    PICXREDEFINES,
    PIC9((String, bool, Binary)), // value + sw_occurs_inside + Optional Binary
    PIC9REDEFINES,
    STRUCT,
    STRUCTREDEFINES,
    OCCURS,
    UNKNOWN,
    STRUCTSIZED(u32),
    OCCURSSIZED(u32),
}

impl Type {
    fn size(&self) -> Option<u32> {
        use Type::*;
        match self {
            PICX((val, _, _)) => {
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
            PICXREDEFINES => None,
            PIC9((val, _, _)) => {
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
            PIC9REDEFINES => None,
            STRUCT => None,
            STRUCTREDEFINES => None,
            OCCURS => None,
            UNKNOWN => None,
            STRUCTSIZED(val) => Some(*val),
            OCCURSSIZED(val) => Some(*val),
        }
    }
}

#[derive(Debug)]
enum Binary {
    COMP1,
    COMP3,
    COMP4,
    COMP5,
    COMP6,
    PACKEDDECIMAL,
    BINARY(u32),
    None,
}

impl Binary {
    fn text(&self) -> String {
        use Binary::*;
        match &self {
            COMP1 => "COMP-1".to_string(),
            COMP3 => "COMP-3".to_string(),
            COMP4 => "COMP-4".to_string(),
            COMP5 => "COMP-5".to_string(),
            COMP6 => "COMP-6".to_string(),
            PACKEDDECIMAL => "PACKED-DECIMAL".to_string(),
            BINARY(val) => format!("BINARY({})", val),
            None => "".to_string(),
        }
    }
}
