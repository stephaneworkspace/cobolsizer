/*
 * Cobol sizer
 * ===========
 *
 * Rust Cobol sizer by Stéphane (https://github.com/stephaneworkspace)
 *
 * Compatible Microfocus RM Cobol
 */
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
    let mut file = File::open(file_path.as_path())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("With text:\n{}", contents);
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
