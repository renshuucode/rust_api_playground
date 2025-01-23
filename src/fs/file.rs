use std::fs;
use std::io;

// 读取文件内容
pub fn read_file(path: &str) -> io::Result<()> {
    let content = fs::read_to_string(path)?;
    println!("File content: {}", content);
    Ok(())
}

// 写入文件内容
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    println!("File written successfully!");
    Ok(())
}

// 获取文件元数据
pub fn get_metadata(path: &str) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    println!("File size: {} bytes", metadata.len());
    println!("Is directory: {}", metadata.is_dir());
    Ok(())
}