use std::fs;
use std::io;
use std::path::Path;
// 读取文件内容
pub fn read_file(path: &str) -> io::Result<String> {
    let content = fs::read_to_string(path)?;
    println!("File content: {}", content);
    Ok(content)
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

pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}

// 测试模块
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;

    // 测试读取文件
    #[test]
    fn test_read_file() -> io::Result<()> {
        // 创建一个临时文件
        let test_path = "test_files/test_read.txt";
        fs::write(test_path, "Hello, test!")?;

        // 调用 read_file 函数
        let content = read_file(test_path)?;

        // 验证读取的内容是否正确
        assert_eq!(content, "Hello, test!");

        // 清理临时文件
        fs::remove_file(test_path)?;
        Ok(())
    }

    // 测试写入文件
    #[test]
    fn test_write_file() -> io::Result<()> {
        // 创建一个临时文件路径
        let test_path = "test_files/test_write.txt";

        // 调用 write_file 函数
        write_file(test_path, "Test content")?;

        // 验证文件内容是否正确
        let content = fs::read_to_string(test_path)?;
        assert_eq!(content, "Test content");

        // 清理临时文件
        fs::remove_file(test_path)?;
        Ok(())
    }

    // 测试获取文件元数据
    #[test]
    fn test_get_metadata() -> io::Result<()> {
        // 创建一个临时文件
        let test_path = "test_files/test_metadata.txt";
        fs::write(test_path, "Test metadata")?;

        // 调用 get_metadata 函数
        get_metadata(test_path)?;

        // 清理临时文件
        fs::remove_file(test_path)?;
        Ok(())
    }

    #[test]
    fn test_exists() {
        let path = "test_files/hello.txt";
        assert!(exists(path));
    }
}
