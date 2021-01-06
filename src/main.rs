/*
 * Cobol sizer
 * ===========
 *
 * Rust Cobol sizer by Stéphane (https://github.com/stephaneworkspace)
 *
 * Compatible Microfocus RM Cobol
 */
use num::Integer;
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
    let mut contents_split: Vec<&str> = contents
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
    for c in contents_split.iter() {
        println!("Split by .:\n{}", &c);

        /*let v: Vec<&str> = c.splitn(2, "OCCURS").collect();
        for (i, vv) in v.iter().enumerate() {
            if i == 1 {
                println!("OK: {}", vv);
            }
        }*/

        let v: Vec<&str> = c.splitn(2, "PIC").collect();
        for (i, vv) in v.iter().enumerate() {
            if i == 1 {
                println!("OK: {}", vv);
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Line {
    field_type: Type,
}

#[derive(Debug)]
enum Type {
    PICX(u32),
    PIC9(f32),
    PIC9Binary(f32),
}

impl Type {
    fn size(&self) -> u32 {
        use Type::*;
        match self {
            PICX(val) => *val,
            PIC9(_) => {
                // TODO more in depth
                0 as u32
            },
            PIC9Binary(_) => {
                // TODO more in depth
                0 as u32
            },
        }
    }
}

fn compute(line: &str) -> u32 {
    use Type::*;
    let l: Line = Line {
        field_type: PICX(10),
    };
    l.field_type.size()
}
