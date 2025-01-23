use clap::{Arg, Command};
mod fs;

fn main() {
    let matches = Command::new("rust_api_playground")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("A playground to practice Rust APIs")
        .arg(
            Arg::new("module")
                .short('m')
                .long("module")
                .value_name("MODULE")
                .help("Select the module to run (e.g., fs)"),
        )
        .arg(
            Arg::new("action")
                .short('a')
                .long("action")
                .value_name("ACTION")
                .help("Select the action to run (e.g., read, write)"),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Path to the file or directory"),
        )
        .arg(
            Arg::new("content")
                .short('c')
                .long("content")
                .value_name("CONTENT")
                .help("Content to write to the file"),
        )
        .get_matches();

    // 获取命令行参数
    let module = matches.get_one::<String>("module").map(String::as_str).unwrap_or("default");
    let action = matches.get_one::<String>("action").map(String::as_str).unwrap_or("default");
    let path = matches.get_one::<String>("path").map(String::as_str).unwrap_or("default.txt");
    let content = matches.get_one::<String>("content").map(String::as_str).unwrap_or("default");

    match module {
        "fs" => {
            println!("Running fs module exercises...");
            fs::run(action, path, content).unwrap();
        }
        _ => {
            println!("Please specify a module to run (e.g., --module fs)");
        }
    }
}