use std::env;
use std::fs;
use std::io::{Read, Write};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    // let mut contents = fs::read_to_string(input_fname)
    // .expect("Something went wrong reading the file");
    let mut file = fs::File::open(input_fname)
    .map_err(|err| format!("error opening {}: {}", input_fname, err))?;
    let mut contents =  String::new();
    file.read_to_string(&mut contents)
    .map_err(|err| format!("error reading {}: {}", input_fname, err))?;
   

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("error opening output {}: {}", output_fname, err))?;
    output_file
        .write_all(contents.as_bytes())
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = process(&args[1], &args[2]) {
        eprintln!("{}", err)
    }
}