use std::{
    borrow::Borrow,
    env::args_os,
    io::{stdin, stdout},
    process::ExitCode,
};
use std::io::Write;

use pancake::Interpreter;

// We use ExitCode to prevent the implicit Error: printout when using a Result<T, E>
fn main() -> ExitCode {
    // Read the CLI arguments
    let Some(filepath) = args_os().nth(1) else {
        println!(include_str!("help.txt"));
        return ExitCode::SUCCESS;
    };
    match filepath.to_string_lossy().borrow() {
        "--docs" => println!(include_str!("../README.txt")),
        "--license" => println!(include_str!("../LICENSE.txt")),
        _ => {
            // Grab the input and output
            let mut input = stdin().lock();
            let mut output = stdout().lock();
            // Read the file
            // This could be read line by line, but it would require a complex
            // system of keeping track of which instructions need labels,
            // and that seems more complicated than I care to do for a simple project like this.
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
            let mut interpreter = Interpreter::default();
            // We need to jump around, so we store the index externally
            let mut index = 0;
            while let Some(instr) = program.get(index) {
                eprintln!("Instruction: {instr:?}");
                if let Some(new_index) =
                    match interpreter.execute(index, *instr, &mut input, &mut output) {
                        Ok(v) => v,
                        Err(err) => {
                            eprintln!("Runtime error: {err} at instruction #{index} ({instr:?})");
                            return ExitCode::FAILURE;
                        }
                    }
                {
                    index = new_index;
                } else {
                    index += 1;
                }
                let _ = output.flush();
            }
        }
    }
    ExitCode::SUCCESS
}
