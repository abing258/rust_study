use std::{env, process};

use mini_grep::Config;

//CASE_INSENSITIVE=1 cargo run to poem.txt
//带着环境变量运行 cargo run
fn main() {
    //获取命令行参数的迭代器
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

