# RUST闭包和迭代器

## 闭包
闭包（closures）是可以保存在一个变量中或作为参数传递给其他函数的匿名函数。  
可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获被定义时所在作用域中的值。  

定义：
1. 以一对竖线（|）开始，在竖线中指定闭包的参数；
2. 参数之后是存放闭包体的大括号 —— 如果闭包体只有一行则大括号是可以省略的
3. 大括号之后闭包的结尾，需要用于 let 语句的分号。
4. 因为闭包体的最后一行没有分号（正如函数体一样），所以闭包体（num）最后一行的返回值作为调用闭包时的返回值 。
```rust
fn main() {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}
//这个 let 语句意味着 expensive_closure 包含一个匿名函数的 定义，不是调用匿名函数的 返回值。
```
### 闭包类型推断和注解
闭包不要求像 fn 函数那样在参数和返回值上注明类型。函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分。当然也可以注明类型  
每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。  
```rust
fn main() {
    //函数定义
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    //完整标注的闭包定义
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    //闭包定义中省略了类型注解
    let add_one_v3 = |x|             { x + 1 };
    //去掉了可选的大括号
    let add_one_v4 = |x|               x + 1  ;
}
```
虽然闭包不要求注明参数类型，但是一旦推断出参数类型，再次调用就不能传其他类型了，编译器会报错。  

### 使用带有泛型和 Fn trait 的闭包
惰性求值：创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。  
```rust
//存放闭包的结构体
struct Cacher<T>//T 的 trait bound 指定了 T 是一个使用 Fn 的闭包
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = self.calculation(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn main() {
    
}
```
### 闭包会捕获其环境
```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}
//equal_to_x 闭包也被允许使用变量 x，因为它与 equal_to_x 定义于相同的作用域。
```
闭包可以通过三种方式捕获其环境(直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用)  
这三种捕获值的方式被编码为如下三个 Fn trait：  
1. FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。  
2. FnMut 获取可变的借用值所以可以改变其环境
3. Fn 从其环境获取不可变的借用值  

如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。  
注意：即使其捕获的值已经被移动了，move 闭包仍需要实现 Fn 或 FnMut。这是因为闭包所实现的 trait 是由闭包所捕获了什么值而不是如何捕获所决定的。而 move 关键字仅代表了后者。  
```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;
    
    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```


## 使用迭代器（惰性 lazy）处理元素序列
迭代器模式允许你对一个序列的项进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑  
迭代器只有在遍历时才有用处。单纯创建的迭代器没有任何意义  
迭代器是 Rust 的 零成本抽象（zero-cost abstractions）之一，它意味着抽象并不会引入运行时开销  
```rust
fn main() {
    let v1 = vec![1, 2, 3];
    //v1_iter 需要是可变的
    //使用 for 循环时无需使 v1_iter 可变因为 for 循环会获取 v1_iter 的所有权并在后台使 v1_iter 可变。
    //iter 方法生成一个不可变引用的迭代器。如果我们需要一个获取 v1 所有权并返回拥有所有权的迭代器，则可以调用 into_iter
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}
```
迭代器的实现方式：
1. Iterator trait 和 next 方法
```rust
pub trait Iterator {
    //关联类型。实现 Iterator trait 要求同时定义一个 Item 类型，这个 Item 类型被用作 next 方法的返回值类型。
    type Item;
    //next 是 Iterator 实现者被要求定义的唯一方法。next 一次返回迭代器中的一个项，封装在 Some 中，当迭代器结束时，它返回 None。
    //迭代器上调用 next 方法改变了迭代器中用来记录序列位置的状态。（消费）
    //从 next 调用中得到的值是 vector 的不可变引用
    fn next(&mut self) -> Option<Self::Item>;
    // 此处省略了方法的默认实现
}
```
产生其他迭代器的方法：  
Iterator trait 中定义了另一类方法，被称为 迭代器适配器（iterator adaptors），他们允许我们将当前迭代器变为不同类型的迭代器。

```rust
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    //调用迭代器适配器方法 map 对每个元素都加一
    let x1 = v1.iter().map(|x| x + 1);  //此时该迭代器没有调用，是毫无用处
    
    let v2 = x1.collect();
}
```

使用闭包获取环境：  
使用 filter 迭代器适配器和捕获环境的闭包的常规用例
```rust
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

实现 Iterator trait 来创建自定义迭代器：
```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 迭代器和循环
迭代器属于Rust的高级的抽象，零成本抽象之一。性能优于循环
