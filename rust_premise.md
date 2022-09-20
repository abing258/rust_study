# rust的前提
rust 是一种 预编译静态类型（ahead-of-time compiled）语言，这意味着你可以编译程序，并将可执行文件送给其他人，他们甚至不需要安装 Rust 就可以运行。
## 工具介绍
**rustc** 是编译器  
````
rustc main.rs   #编译一个.rs文件
````
**rustup** 是管理编译器的工具  
````
rustup update   #更新rust
````
**cargo** 是包管理和工程管理工具  
````
cargo new hello_cargo   #创建一个rust项目

cargo build     #构建一个cargo项目

cargo run      #构建并运行一个cargo项目

cargo check     #快速检查代码确保其可以编译，但并不产生可执行文件
````

## 项目结构介绍
一个新的cargo项目包含如下结构
```
hello_cargo
|--src
    |--main.rs
|--Cargo.toml
```
````Cargo.toml
[package]   #标题，表明下面的语句用来配置一个包
name = "hello_cargo"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
edition = "2018"

[dependencies]  #罗列项目依赖的片段 rust中的代码包被称为crate
````