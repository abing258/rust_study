# 编写自动化测试
讨论 Rust 测试功能的机制。我们会讲到编写测试时会用到的注解和宏，运行测试的默认行为和选项，以及如何将测试组织成单元测试和集成测试。  

## 如何编写测试
Rust 中的测试函数是用来验证非测试代码是否按照期望的方式运行的。测试函数体通常执行如下三种操作：
1. 设置任何所需的数据或状态
2. 运行需要测试的代码 
3. 断言其结果是我们所期望的

Rust 中的测试就是一个带有 test 属性注解的函数。  
属性（attribute）是关于 Rust 代码片段的元数据；  
为了将一个函数变成测试函数，需要在 fn 行之前加上 #[test]。  
当使用 cargo test 命令运行测试时，Rust 会构建一个测试执行程序用来调用标记了 test 属性的函数，并报告每一个测试是通过还是失败。  
注意事项：
1. 当测试函数中出现 panic 时测试就失败了
2. 每一个测试都在一个新线程中运行
3. 当主线程发现测试线程异常了，就将对应测试标记为失败

```rust
#[cfg(test)]
mod tests {
    //#[test]：这个属性表明这是一个测试函数，这样测试执行者就知道将其作为测试处理
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//$ cargo test
//Compiling test_exercise v0.1.0 (file:///projects/test_exercise)
//Finished test [unoptimized + debuginfo] target(s) in 0.57s
//Running unittests (target/debug/deps/test_exercise-92948b65e88960b4)

//running 1 test
//test tests::it_works ... ok
//test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
//Doc-tests test_exercise
//running 0 tests
//test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

//0 ignored表示 没有将任何测试标记为忽略
//0 filtered out表示 没有过滤需要运行的测试
//0 measured 统计是针对性能测试的
//Doc-tests test_exercise 开头的这一部分是所有文档测试的结果。
```

## 在测试中不同的宏
**panic!** 主要是造成 panic  
**assert!** 宏来检查结果，由标准库提供，在希望确保测试中一些条件为 true 时非常有用。需要向 assert! 宏提供一个求值为布尔值的参数。如果值是 true，assert! 什么也不做，同时测试会通过。如果值为 false，assert! 调用 panic! 宏，这会导致测试失败。  
**assert_eq!** 比较两个值是相等
**assert_ne!** 比较两个值不相等
这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 PartialEq 和 Debug trait。所有的基本类型和大部分标准库类型都实现了这些 trait。对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。需要实现 Debug 才能在断言失败时打印他们的值。因为这两个 trait 都是派生 trait，如第五章示例 5-12 所提到的，通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解。
都可以自定义失败信息  `assert!(
result.contains("Carol"),
"Greeting did not contain name, value was `{}`",
result
);`  

### 使用 should_panic 检查 panic
除了检查代码是否返回期望的正确的值之外，检查代码是否按照期望处理错误也是很重要的。  
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
//属性中 expected 参数提供的值是 Guess::new 函数 panic 信息的子串。
```

### 将 Result<T, E> 用于测试
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

## 控制测试如何运行
`cargo test` 在测试模式下编译代码并运行生成的测试二进制文件。可以使用命令行参数改变默认行为。默认是`并行`运行的。  
`cargo test <cargo test 的参数> -- <生成的测试二进制文件参数>`  

### 并行或连续的运行测试
避免测试数据的互相影响，采用以下方式解决：
1. 每个测试读写不同的文件
2. 一次运行一个测试 `cargo test -- --test-threads=1` --test-threads 参数和希望使用线程的数量给测试二进制文件

### 现实函数的输出
`cargo test -- --show-output` --show-output 告诉 Rust 显示成功测试的输出。  

### 通过指定名字来运行部分测试
1. 运行单个测试 cargo test 传递任意测试的名称来只运行这个测试
2. 运行单个多个 cargo test 后面传递部分函数名，会模糊匹配所有的函数
3. 忽略某些测试 对测试方法使用 #[ignore] 可以忽略掉该测试
   1. 只希望运行被忽略的测试 `cargo test -- --ignored`
   2. 不管是否忽略都要运行全部测试 `cargo test -- --include-ignored` 

## 测试的组织结构
**单元测试**倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口  
**集成测试**对于你的库来说则完全是外部的。只测试公有接口  

## 单元测试
单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期  
单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中。  
规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块。

### 测试模块和 #[cfg(test)]
`#[cfg(test)]` 注解告诉Rust只在执行cargo test时才编译和运行测试代码  
这在只希望构建库的时候可以节省编译时间，并且因为它们并没有包含测试，所以能减少编译产生的文件的大小。  
`#[cfg(test)]`解释： cfg属性代表 configuration 它告诉 Rust 其之后的项只应该被包含进特定配置选项中。  
### 测试私有函数
子模块的项可以使用其上级模块的项。在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域

## 集成测试
集成测试的目的是测试库的多个部分能否一起正常工作。  
1. 在项目根目录创建一个 tests 目录，与 src 同级
   1. 在目录下可以创建任意多的文件，每个文件都是会被当作单独的crate执行
   2. 使用`use crate_name`将不同的crate引入 
   3. 使用`mod mod_name`将公共的mod引入使用
   4. 不需要将任何代码标注为 #[cfg(test)]。tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行 cargo test 时编译这个目录中的文件。  
```rust
use adder;

#[test]
fn it_adds_two() {
assert_eq!(4, adder::add_two(2));
}
```
运行集成测试
1. cargo test 
2. 通过 --test 执行文件名运行 `cargo test --test file_name`
无论文件中是否有函数，只要文件存在就在测试结果中看到文件的运行结果(仅限tests目录下的一级文件)  

### 二进制 crate 的集成测试
项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。  
