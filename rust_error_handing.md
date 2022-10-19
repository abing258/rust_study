# 错误处理
分为可恢复的（recoverable）和 不可恢复的（unrecoverable）错误。  
Rust 没有异常。相反，它有 Result<T, E> 类型，用于处理可恢复的错误，还有 panic! 宏，在程序遇到不可恢复的错误时停止执行。  

## panic!宏
当执行这个宏时，程序会打印出一个错误信息，展开并清理栈数据，然后接着退出。  
panic!宏默认会进行展开操作，这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。 
另一个操作是终止，（abort），这会不清理数据就退出程序。那么程序所使用的内存需要由操作系统来清理。  
如果想项目的二进制文件越小越好，可以在 `Cargo.toml 的 [profile] 部分增加 panic = 'abort'`，
如果你想要在release模式中 panic 时直接终止：   
`[profile.release]`  
`panic = 'abort'`  

```rust
fn main() {
    panic!("crash and burn");
}
```
### panic! 的 backtrace
backtrace 是一个执行到目前位置所有被调用的函数的列表  
让我们将 RUST_BACKTRACE 环境变量设置为任何不是 0 的值来获取 backtrace `RUST_BACKTRACE=1 cargo run`
为了获取带有这些信息的 backtrace，必须启用 debug 标识。当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用  

### Result 处理可恢复的错误
```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
    
    //使用闭包和unwrap_or_else
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

### 失败时 panic 的简写：unwrap 和 expect
unwrap --> 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!  
expect --> 和unwrap一样的作用，多了一个自定的panic的信息  
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

```
### 传播错误
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

### 传播错误的简写：? 运算符
**? 运算符**所使用的错误值被传递给了 from 函数，它定义于标准库的 From trait 中，其用来将错误从一种类型转换为另一种类型。  
当 ? 运算符调用 from 函数时，收到的错误类型被转换为由当前函数返回类型所指定的错误类型。  
只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。  
```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
？运算符可以直接使用链式调用  
```rust
use std::fs::File;
use std::fs;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_fs() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```
? 运算符只能被用于返回值与 ? 作用的值相兼容的函数。除了Result，？运算符也可用在Option<T>上，当结果是None，函数会提前返回。  
main 函数也可以返回任何实现了 std::process::Termination trait 的类型。  

## 何时panic!
1. 示例、代码原型和测试都非常适合 panic
2. 错误处理指导原则
   1. 有害状态是非预期的行为，与偶尔会发生的行为相对，比如用户输入了错误格式的数据
   2. 在此之后代码的运行依赖于不处于这种有害状态，而不是在每一步都检查是否有问题
   3. 没有可行的手段来将有害状态信息编码进所使用的类型中的情况。
3. 创建自定义类型进行有效性验证  