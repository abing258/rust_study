# rust的整体架构
* **包:** Cargo 的一个功能，它允许你构建、测试和分享 crate。
* **Crates:** 一个模块的树形结构，它形成了库或二进制项目。
* **模块** and **use:** 允许你控制作用域和路径的私有性。
* **路径:** 一个命名例如结构体、函数或模块等项的方式。
* **作用域** 代码所在的嵌套上下文有一组定义为 “in scope” 的名称。

## Crate
**crate** 是 Rust 在编译时最小的代码单位。  
1. 二进制型。可以被编译为可执行程序。必须有一个 main 函数来定义当程序被执行的时候所需要做的事情。 `cargo new **`创建的就是典型的二进制
2. 库（library）。它们提供一些诸如函数之类的东西    

**crate root**是一个源文件，Rust 编译器以它为起始点，并构成你的 crate 的根模块  

## Package
**包(package)** 是提供一系列功能的一个或者多个 crate。一个包会包含一个 Cargo.toml 文件，阐述如何去构建这些 crate。  
包中可以包含至多一个库 crate(library crate)。包中可以包含任意多个二进制 crate(binary crate)，但是必须至少包含一个 crate（无论是库的还是二进制的）。  
1. src/main.rs 就是一个与包同名的二进制 crate 的 crate 根
2. src/lib.rs，就是一个与包同名的库 crate 根  

通过将文件放在 src/bin 目录下，一个包可以拥有多个二进制 crate：每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。  

## 模块
模块是如何工作的:
1. 从crate根节点开始
2. **声明模块**。在crate根文件中，声明一个新模块 `mod garden` 编译器会在以下路径寻找模块代码
   1. 内联。在大括号中，当mod garden后方不是一个分号而是一个大括号
   2. 在文件 src/garden.rs
   3. 在文件 src/garden/mod.rs
3. **声明子模块**。在除了crate根节点以外的其他文件中，可以定义子模块。比如：src/garden.rs中定义了`mod vegetables`;编译器会在以下路径寻找模块代码。  
   1. 内联, 在大括号中，当mod vegetables后方不是一个分号而是一个大括号
   2. 在文件 src/garden/vegetables.rs
   3. 在文件 src/garden/vegetables/mod.rs
4. **模块中的代码路径**。在同一个crate中，隐私规则的允许下，可以使用路径引用代码模块。比如：一个garden vegetables模块下的Asparagus类型，可以使用 `crate::garden::vegetables::Asparagus`被找到  
5. **私有 vs 公用**。一个模块里的代码默认对其父模块私有。为了使一个模块公用，应当在声明时使用`pub mod 替代 mod`。为了使一个公用模块内部的成员公用，应当在声明前使用`pub`  
6. **use 关键字**。use关键字创建了一个成员的快捷方式，用来减少长路径的重复。 `crate::garden::vegetables::Asparagus` 可以通过 `use crate::garden::vegetables::Asparagus` 创建快捷方式来使用。

## 路径
1. **绝对路径（absolute path）** 从 crate 根开始，以 crate 名或者字面值 crate 开头。
2. **相对路径（relative path）** 从当前模块开始，以 self、super（构建从父模块开始的相对路径） 或当前模块的标识符开头。  

可以引用不一定可以编译，rust默认所有项都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。    
**私有性规则**不但应用于模块，还应用于结构体（结构体和字段是分开计算的）、枚举、函数和方法。  

## use
**use**引用也会检查私有性  
1. use 将函数的父模块引入作用域  
2. use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径  

### 使用 as 关键字提供新的名称
`use std::io::Result as IoResult;`  
### 使用 pub use 重导出名称
使外部代码，也可以使用use的项  
### 使用外部包
1. 在Cargo.toml中添加依赖，如 `rand = "0.8.3"`  
2. 在项目包的作用域引入 `use rand::Rng;`  
标准库(std)相对于自己的包也是外部依赖，需要引入 `use std::collections::HashMap;`  
### 嵌套路径来消除大量的 use 行
```rust
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};
```

### 通过 glob 运算符将所有的公有定义引入作用域
`use std::collections::*;`  
