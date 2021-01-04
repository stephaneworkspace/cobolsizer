/*
 * Cobol sizer
 * ===========
 *
 * Rust Cobol sizer by Stéphane (https://github.com/stephaneworkspace)
 *
 * Compatible Microfocus RM Cobol
 */

fn main() {
    println!("Compute Cobol Pic");
    println!("Size: {}", compute("TODO"));
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
