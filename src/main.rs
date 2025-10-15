use std::{env::{current_dir}, fs, path::PathBuf};

use dotsunda::{parse, ParseOptionBuilder};
fn main() {
    let dir = "playground";
    let mut input_path = PathBuf::from(current_dir().unwrap());
        input_path.push(dir);
        input_path.push("input.sunda");
    let mut output_path = PathBuf::from(current_dir().unwrap());
        output_path.push(dir);
        output_path.push("output.txt");
    let opt = &ParseOptionBuilder::default();
    let result = parse(str::from_utf8(&fs::read(input_path.to_str().unwrap()).unwrap()).unwrap(), opt).unwrap();
    let _ = fs::write(output_path.to_str().unwrap(), result.clone());
}
