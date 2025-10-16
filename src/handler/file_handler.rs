use owo_colors::OwoColorize;
use std::fs;

/// Handle file operations
pub struct FileHandler;

impl FileHandler {
    /// Read and validate the source file
    pub fn read_source_file(path: &str) -> Result<String, ()> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(e) => {
                eprintln!();
                eprintln!(
                    "{} {}",
                    format!("Failed to read '{}':", path).bright_red(),
                    e
                );
                std::process::exit(1);
            }
        }
    }

    /// Save LLVM IR to file
    pub fn save_ir_to_file(ir_string: &str) {
        let ir_path = "Sample.ll";
        match fs::write(ir_path, ir_string) {
            Ok(_) => println!("{} '{}'", "LLVM IR saved to".green(), ir_path),
            Err(e) => eprintln!("{} {}", "Failed to write IR file:".red(), e),
        }
    }
}
