use std::fs;
use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <source_file>", args[0]);
        std::process::exit(1);
    }

    let mut output = "output_binary";

    if args.len() >= 3 {
        output = &args[2];
    }

    let filename = &args[1];
    let source_code = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    match titanium_compiler::compile_source(&source_code) {
        Ok(compiled_output) => {
            let temp_source = "temp_output.rs";
            fs::write(temp_source, compiled_output)
                .expect("Failed to write temporary target code file");

            println!("Transpilation successful. Invoking rustc...");

            let status = Command::new("rustc")
                .arg(temp_source)
                .arg("-o")
                .arg(output)
                .status()
                .expect("Failed to execute rustc compiler process");

            let _ = fs::remove_file(temp_source);

            if status.success() {
                println!("Compilation successful! Created executable '{}'", output);
            } else {
                eprintln!("rustc failed to compile the generated code.");
            }
        }    
        Err(e) => eprintln!("Compilation failed: {}", e),
    }
}
