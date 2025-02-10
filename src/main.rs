use std::io::{self};
use std::fs;

// fn read_file_content(path: &str) -> io::Result<String> {
//     let mut file = File::open(path)?;
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;
//     Ok(contents)
// }

// fn main() -> io::Result<()> {
//     let file_path = "test.txt";
//     let contents = read_file_content(file_path).unwrap();
//     println!("{}", contents);
//     Ok(())
// }

// 1. 读取文件
// 1.1 读取整个文件为字符串
// fn main() -> io::Result<()> {
//     let content = fs::read_to_string("src/test.txt")?;
//     println!("文件内容:\n{}", content);
//     Ok(())
// }

// 1.2 读取文件为 `Vec<u8>`
fn main() -> io::Result<()> {
    let content = fs::read("src/duolingo.jpeg")?;
    println!("文件内容:\n{:?}", content);
    Ok(())
}

