

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
// fn main() -> io::Result<()> {
//     let content = fs::read("src/duolingo.jpeg")?;
//     println!("文件内容:\n{:?}", content);
//     Ok(())
// }

// 1.3 逐行读取文件
// use std::fs::File;
// use std::io::{self, BufRead};
// fn main() -> io::Result<()> {
//     let file = File::open("src/test.txt")?;
//     let reader = io::BufReader::new(file);
//     for line in reader.lines() {
//         println!("{}", line?);
//     }
//     Ok(())
// }

// 2. 写入文件
// 2.1 覆盖写入
// use std::fs;
// fn main() -> std::io::Result<()> {
//     fs::write("src/test.txt", "Hello, world!")?;
//     println!("文件写入成功");
//     Ok(())
// }

// 2.2 逐步写入
// use std::fs::File;
// use std::io::Write;
// fn main() -> std::io::Result<()> {
//     let mut file = File::create("src/test.txt")?;
//     file.write_all(b"Hello! Rust")?;
//     file.write_all("第二行内容".as_bytes())?;
//     println!("文件写入成功");
//     Ok(())
// }

// 2.3 追加写入
// use std::fs::OpenOptions;
// use std::io::Write;
// fn main() -> std::io::Result<()> {
//     let mut file = OpenOptions::new()
//         .write(true)
//         .append(true)
//         .open("src/test.txt")?;
//     file.write_all(b"Hello, Beatles!")?;
//     println!("文件写入成功");
//     Ok(())
// }

// 3. 文件与目录操作
// 3.1 创建目录
// use std::fs;
// fn main() -> std::io::Result<()> {
//     fs::create_dir("src/sub_dir")?;
//     Ok(())
// }

// 3.2 递归创建目录
// use std::fs;
// fn main() -> std::io::Result<()> {
//     fs::create_dir_all("src/sub_dir/sub_sub_dir")?;
//     Ok(())
// }

// 3.3 删除目录
// use std::fs;
// fn main() -> std::io::Result<()> {
//     fs::remove_dir("src/sub_dir")?;
//     Ok(())
// }

// 5. 遍历目录
use std::fs;
fn main() -> std::io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        println!("{:?}", entry.path().display());
    }
    Ok(())
}