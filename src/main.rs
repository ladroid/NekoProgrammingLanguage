use std::{env, fs};

use my_project::interpreter::Interpreter;

static ERROR_MISSING_INPUT_FILE: &str = "Missing input file";
static ERROR_FILE_READ: &str = "Could not read file";
static ERROR_BAD_EXTENSION: &str = "Source files must have the .nek file extension";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let source_file = args.iter().skip(1).next().ok_or(ERROR_MISSING_INPUT_FILE)?;
    
    if ! source_file.ends_with(".nek"){
        return Err(ERROR_BAD_EXTENSION.into());
    }

    let code = fs::read_to_string(source_file).map_err(|e| format!("{} {}: {}", ERROR_FILE_READ, source_file, e))?;

    let mut interpreter = Interpreter::new();
    interpreter.run(code.as_str())?;

    Ok(())
}
