use rust_api_playground::fs::file; // 引入被测试的模块
use std::fs;
use std::io;

#[test]
fn test_read_file() -> io::Result<()> {
    // 创建一个临时文件
    let test_path = "test_files/test_read.txt";
    fs::write(test_path, "Hello, test!")?;

    // 调用 read_file 函数
    let content = file::read_file(test_path)?;

    // 验证读取的内容是否正确
    assert_eq!(content, "Hello, test!");

    // 清理临时文件
    fs::remove_file(test_path)?;
    Ok(())
}