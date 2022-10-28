# 泛型、Trait 和生命周期
**泛型**是具体类型或其他属性的抽象替代  
**trait**这是一个定义泛型行为的方法，可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型  
**生命周期（lifetimes）** 它是一类允许我们向编译器提供引用如何相互关联的泛型。  

## 泛型数据类型
使用泛型为像函数签名或结构体这样的项创建定义，这样它们就可以用于多种不同的具体数据类型。  
### 在函数定义中使用泛型
当使用泛型定义函数时，本来在函数签名中指定参数和返回值的类型的地方，会改用泛型来表示。  
当在函数签名中使用一个类型参数时，必须在使用它之前就声明它。
```rust
fn largest<T>(list: &[T]) -> T {
    list[0]
}
//可以这样理解这个定义：函数 largest 有泛型类型 T。它有个参数 list，其类型是元素为 T 的 slice。largest 函数的返回值类型也是 T。
```
**完整代码**
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
//注意事项：i32 和 char 这样的类型是已知大小的并可以储存在栈上，所以他们实现了 Copy trait
//当我们将 largest 函数改成使用泛型后，现在 list 参数的类型就有可能是没有实现 Copy trait 
//在 T 的 trait bounds 中增加 Copy！
```
### 结构体定义中的泛型
同样也可以用 <> 语法来定义结构体，它包含一个或多个泛型参数类型字段。  
```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

### 枚举定义中的泛型
```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### 方法定义中的泛型
必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。  
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。  
```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```
另一个选择是定义方法适用于某些有限制（constraint）的泛型类型。  
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
//这段代码意味着 Point<f32> 类型会有一个方法 distance_from_origin，而其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法。


//方法使用了与结构体定义中不同类型的泛型
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

### 泛型代码的性能
Rust 实现了泛型，使得使用泛型类型参数的代码相比使用具体类型并没有任何速度上的损失。  
Rust 通过在编译时进行泛型代码的 单态化（monomorphization）来保证效率。单态化是一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。  

## Trait：定义共同行为
trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。  

### 定义trait
一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。  
```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
//trait 体中可以有多个方法：一行一个方法签名且都以分号结尾。
```

### 为类型实现 trait 
```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl 关键字之后，我们提供需要实现 trait 的名称，接着是 for 和需要实现 trait 的类型的名称。
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
```
只有当至少一个 trait 或者要实现 trait 的类型位于 crate 的本地作用域时，才能为该类型实现 trait。不能为外部类型实现外部 trait。这个限制是被称为 相干性（coherence） 的程序属性的一部分，或者更具体的说是 孤儿规则（orphan rule），其得名于不存在父类型。  

### 默认实现
```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
//如果想要对 NewsArticle 实例使用这个默认实现，而不是定义一个自己的实现
impl Summary for NewsArticle {}
```

### trait 作为参数
```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.notify());
}
```
**Trait Bound 语法**  
```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_two(item1: &impl Summary, item2: &impl Summary) {}
pub fn notify_two_bound<T: Summary>(item1: &T, item2: &T) {}
```
**通过 + 指定多个 trait bound**  
```rust
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify_bound<T: Summary + Display>(item: &T) {}
```
**通过 + 指定多个 trait bound**  
```rust
pub fn notify(item: &(impl Summary + Display)) {}

pub fn notify_bound<T: Summary + Display>(item: &T) {}
```
**通过 where 简化 trait bound**  
```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
```

### 返回实现了 trait 的类型
在返回值中使用 impl Trait 语法，来返回实现了某个 trait 的类型：  
```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
//不过这只适用于返回单一类型的情况。例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就行不通
```
### 使用 trait bound 有条件地实现方法
```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
也可以对任何实现了特定 trait 的类型有条件地实现 trait。对任何满足特定 trait bound 的类型实现 trait 被称为 blanket implementations  

## 生命周期确保引用有效
**生命周期**也是一种泛型。Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域。  
### 生命周期避免了悬垂引用
生命周期的主要目标是避免悬垂引用，后者会导致程序引用了非预期引用的数据。  
**悬垂引用**是生命周期 'b 比生命周期 'a 要小：'b引用'a,被引用的对象比它的引用者存在的时间更短。

### 函数中的泛型生命周期
```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 文本揭示了返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y。事实上我们也不知道，
// 因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！
// 我们也不知道传入的引用的具体生命周期
```
**生命周期注解语法**  
生命周期注解并不改变任何引用的生命周期的长短。与当函数签名中指定了泛型类型参数后就可以接受任何类型一样，当指定了泛型生命周期后函数也能接受任何生命周期的引用。  
生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。  
**语法**
1. 生命周期参数名称必须以撇号（'）开头
2. 其名称通常全是小写
3. 类似于泛型其名称非常短。
4. 'a 是大多数人默认使用的名称
5. 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开。
````
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
````
单个的生命周期注解本身没有多少意义，因为生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的。  
例如如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。  
这两个生命周期注解意味着引用 first 和 second 必须与这泛型生命周期存在得一样久。  

### 函数签名中的生命周期注解
就像泛型类型参数，泛型生命周期参数需要声明在函数名和参数列表间的尖括号中。
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//在这个签名中我们想要表达的限制是所有（两个）参数和返回的引用的生命周期是相关的，也就是这两个参数和返回的引用存活的一样久。
//它的实际含义是 longest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。


//当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分。
//泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
//因为我们用相同的生命周期参数 'a 标注了返回的引用值，所以返回的引用值就能保证在 x 和 y 中较短的那个生命周期结束之前保持有效。
```
**注解**通过在函数签名中指定生命周期参数时，我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝。  
函数并不需要知道 x 和 y 具体会存在多久，而只需要知道有某个可以被 'a 替代的作用域将会满足这个签名。  

### 深入理解生命周期
生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。  

### 结构体定义中的生命周期注解
这需要为结构体定义中的每一个引用添加生命周期注解。  
```rust
//必须在结构体名称后面的尖括号中声明泛型生命周期参数，以便在结构体定义中使用生命周期参数
struct ImportantExcerpt<'a> {
    //它存放了一个字符串 slice，这是一个引用
    part: &'a str,
}
//impl 之后和类型名称之后的生命周期参数是必要的
impl<'a> ImportantExcerpt<'a> {
    //因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
    fn level(&self) -> i32 {
        3
    }
    //这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和 announcement 他们各自的生命周期。
    //接着，因为其中一个参数是 &self，返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
//ImportantExcerpt 的实例，它存放了变量 novel 所拥有的 String 的第一个句子的引用
//novel 的数据在 ImportantExcerpt 实例创建之前就存在。
//ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的。
```

### 生命周期省略（Lifetime Elision）
```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
//在早期版本（pre-1.0）的 Rust 中，这的确是不能编译的,应该写成
//fn first_word<'a>(s: &'a str) -> &'a str {}

```
**生命周期省略规则**Rust 引用分析的模式，这些规则是一系列特定的场景，此时编译器会考虑，如果代码符合这些场景，就无需明确指定生命周期。 符合这些场景就不需要显示的指定生命周期，否则就需要指定。  
函数或方法的参数的生命周期被称为 输入**生命周期（input lifetimes）**，而返回值的生命周期被称为 **输出生命周期（output lifetimes）**。  
编译器采用三条规则来判断引用何时不需要明确的注解：
1. 第一条规则是每一个是引用的参数都有它自己的生命周期参数(适用于输入生命周期)。
2. 第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：`fn foo<'a>(x: &'a i32) -> &'a i32`。(输出生命周期参数)
3. 第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法,那么所有输出生命周期参数被赋予 self 的生命周期。  

### 静态生命周期
**'static**，其生命周期能够存活于整个程序期间。  
所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
```rust
fn main() {
    let s: &'static str = "I have a static lifetime.";
    //这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面值都是 'static 的。
}
```
不过将引用指定为 'static 之前，思考一下这个引用是否真的在整个程序的生命周期里都有效。不太建议手动指定这个引用  

### 结合泛型类型参数、trait bounds 和生命周期
```rust
use std::fmt::Display;

//因为生命周期也是泛型，所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
