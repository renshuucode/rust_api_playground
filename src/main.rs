use std::io::{self, Read};
use std::fs::{File, remove_dir};

fn read_file_content(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let file_path = "test.txt";
    let contents = read_file_content(file_path).unwrap();
    println!("{}", contents);
    Ok(())
}