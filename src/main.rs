use std::{
    env::args_os,
    fs::File,
    io::{
        BufRead,
        stdin,
        stdout,
        Read,
        self
    },
    process::ExitCode, collections::HashMap, borrow::Borrow
};

// We use ExitCode to prevent the implicit Error: printout when using a Result<T, E>
fn main() -> ExitCode {
    // Read the CLI arguments
    let Some(filepath) = args_os().skip(1).next() else {
        repl();
        return ExitCode::SUCCESS;
    };
    match filepath.to_string_lossy().borrow() {
        "--docs" => println!(include_str!("../README.txt")),
        "--license" => println!(include_str!("../LICENSE.txt")),
        "--help" => println!("Usage:\n\tpancake\tInitiates an interactive session.\n\tpancake <filepath>\tExecutes a program.\n\tpancake --docs\tPrints the documentation and exits.\n\tpancake --license\tPrints the license (MIT, with commercial clause removed) and exits.\n\tpancake --help\tPrints this message and exits."),
        _ => {
            // Grab the input and output
            let input = stdin().lock();
            let output = stdout().lock();
            // Read the file
            // This could be read line by line, but it would require a complex
            // system of keeping track of which instructions need labels,
            // and that seems more complicated than I need.
            let program = match std::fs::read_to_string(filepath) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("Failed to read file: {err}");
                    return ExitCode::FAILURE;
                }
            };
            let program = match pancake::parse_file(program) {
                Ok(v) => v,
                Err((location, why)) => {
                    eprintln!("Parsing error: {why} at line {location}");
                    return ExitCode::FAILURE;
                }
            };
            todo!("Execute line-by-line")
        }
    }
    ExitCode::SUCCESS
}

/// Initiates an interactive REPL.
fn repl() {
    
}