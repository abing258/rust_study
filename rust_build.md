# RUST 构建和发布
在 Rust 中 发布配置（release profiles）是预定义的、可定制的带有不同选项的配置，他们允许程序员更灵活地控制代码编译的多种选项。每一个配置都彼此相互独立。  

## 构建
1. `cargo build` 采用dev的环境配置。 dev 配置被定义为开发时的好的默认配置  
2. `cargo build --release` 采用release 配置。 release 配置则有着良好的发布构建的默认配置  

配置的设置地方在 `Cargo,toml` 文件的 `[profile.*]` 部分。
```yml
[profile.dev]
#opt-level 设置控制 Rust 会对代码进行何种程度的优化.数值越高优化程度越高
opt-level = 0

[profile.release]
opt-level = 3
```

## 将 crate 发布到 Crates.io
[crates.io](https://crates.io) 用来分发包的源代码，所以它主要托管开源代码。  

### 编写有用的文档注释
`//` 是代码注释  
Rust 也有特定的用于文档的注释类型 `///`，他们会生成html文档。 意在让对库感兴趣的程序员理解如何 使用 这个 crate。  
```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
运行 `cargo doc` 生成这个文档注释的 HTML 文档，这个命令运行由 Rust 分发的工具 rustdoc 并将生成的 HTML 文档放入 target/doc 目录。  
开发时方便起见，运行 `cargo doc --open` 会构建当前 crate 文档  

常用（文档注释）部分
1. `# Examples` Markdown 标题在 HTML 中创建了一个以 “Examples” 为标题的部分
2. Panics：这个函数可能会 panic! 的场景。
3. Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
4. Safety：如果这个函数使用 unsafe 代码,期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件  

`//!` 这为包含注释的项，而不是位于注释之后的项增加文档。通常用于 crate 根文件（通常是 src/lib.rs）或模块的根文件为 crate 或模块整体提供文档  
`//!` 最后一行没有任何代码。

### 发布crate
1. 创建账号并登陆
2. `Cargo.toml `中的 `[package]` 添加必要的信息
3. `cargo publish` 发布  
4. 发布现存的新版本 
   1. 改变 Cargo.toml 中 [version](http://semver.org) 所指定的值
   2. 直接使用 `cargo publish`
   3. 撤回一个旧版本(已经使用的不受影响，新的项目不能使用) `cargo yank --vers 1.0.1`
   4. `cargo yank --vers 1.0.1 --undo` 可以撤销之前的撤回
   5. 撤回并没有删除代码

```yaml
[package]
#唯一的名称
name = "guessing_game"
version = "0.1.0"
edition = "2021"
description = "A fun game where you guess what number the computer has chosen."
# license 标识符值 (https://spdx.org/licenses/)
# 如果你希望使用不存在于 SPDX 的 license，则需要将 license 文本放入一个文件，将该文件包含进项目中，接着使用 license-file 来指定文件名而不是使用 license 字段
license = "MIT OR Apache-2.0"
```
[cargo](https://doc.rust-lang.org/cargo/)  

## Cargo 工作空间
项目结构：
1. 最外层的文件夹 创建 `Cargo.toml` 
2. 写入 
`[workspace] 
members = [
"adder",
]`
3. 在文件夹创建 `cargo new addr`
4. 需要多个包就重复 2 3步骤
5. 通过共享一个 target 目录，工作空间可以避免其他 crate 多余的重复构建。
6. cargo并不假定工作空间中的Crates会相互依赖，所以需要明确表明工作空间中 crate 的依赖关系。
7. 可以通过 -p 参数和包名称来运行 `cargo run` 指定工作空间中我们希望使用的包 --> `cargo run -p adder`

## 使用 cargo install 从 Crates.io 安装二进制文件
`cargo install` 用于在本地安装和使用二进制 crate。  
二进制文件都安装到 Rust 安装根目录的 bin 文件夹中。  
你使用 rustup.rs 安装的 Rust 且没有自定义任何配置，这将是 $HOME/.cargo/bin。  

## Cargo 自定义扩展命令
新的子命令来对 Cargo 进行扩展，而无需修改 Cargo 本身  
如果 $PATH 中有类似 cargo-something 的二进制文件，就可以通过 cargo something 来像 Cargo 子命令一样运行它。  
使用 `cargo install` 安装的扩展可以使用 `cargo --list`来展示出来
