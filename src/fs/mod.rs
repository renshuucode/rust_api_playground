use std::io;

pub mod file;

pub fn run(action: &str, path: &str, content: &str) -> io::Result<()> {
    match action {
        "read" => {
            println!("Reading file: {}", path);
            file::read_file(path)?;
        }
        "write" => {
            println!("Writing to file: {}", path);
            file::write_file(path, content)?;
        }
        "metadata" => {
            println!("Getting metadata for file: {}", path);
            file::get_metadata(path)?;
        }
        _ => {
            println!("Please specify a valid action (e.g., --action read)");
        }
    }
    Ok(())
}