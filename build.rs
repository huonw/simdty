use std::os;
use std::io::{Writer, File};

fn simd_type(w: &mut Writer, t: &str, width: u32, length: u32) {
    assert!(length >= 2);
    assert!(t == "f" || t == "u" || t == "i");

    let ty = format!("{}{}", t, width);
    let mut contents = String::new();
    for _ in (0..length) {
        if !contents.is_empty() { contents.push_str(", ") }

        contents.push_str("pub ");
        contents.push_str(&ty[]);
    }
    writeln!(w, "\
#[simd]
#[derive(Copy, Show)]
/// {length} values of type {ty} in a single SIMD vector.
pub struct {ty}x{length}({contents});", ty=ty, length=length, contents=contents).unwrap()
}

fn main() {
    let dst = Path::new(os::getenv("OUT_DIR").unwrap());
    let mut out = File::create(&dst.join("types.rs")).unwrap();
    for length in [2, 4, 8, 16, 32, 64].iter().cloned() {
        for &int in ["i", "u"].iter() {
            for &int_width in [8, 16, 32, 64].iter() {
                simd_type(&mut out, int, int_width, length)
            }
        }

        let float = "f";
        for &float_width in [32, 64].iter() {
            simd_type(&mut out, float, float_width, length)
        }
    }
}
