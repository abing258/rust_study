# rust的结构体和枚举
## 结构体
结构体与元组有些类似，但是结构体中，可以为每条数据命名 

1. 直接赋值实例化
2. 使用Field Init速记实例化
3. 使用结构更新语法从其他实例创建实例
4. 使用没有命名字段的元组结构来创建不同的类型
5. 没有任何字段的单位类结构

### 定义和实例化
```rust
//定义结构体
#[derive(Debug)]    //添加改宏方便打印输出
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {

    //实例化
    let mut user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user.email = String::from("anotheremail@example.com");
    println!("user is {:?}", user); // user is User { email: someone@example.com, username: someusername123, active: true, sign_in_count: 1 }
    println!("user is {:#?}", user); 
    // user is User { 
    //  email: someone@example.com,
    //  username: someusername123,
    //  active: true,
    //  sign_in_count: 1,
    // }
    
    //使用结构体更新语法从其他实例创建实例
    let user1 = User {
        email: String::from("another@example.com"),
        ..user      //..user 必须排在最后，以指定任何剩余字段应从user中的相应字段中获取其值
    };              //此时user已经失效，因为username是移动赋值，
    
    
    // 使用没有命名字段的元组结构来创建不同的类型
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); //black和origin值是不同类型的，因为它们是不同元组结构的实例

    //没有任何字段的单位类结构
    let subject = AlwaysEqual;
    
    
    //dbg!的使用
    let scale = 2;
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: dbg!(30 * scale),
    };
    dbg!(&user2);
    //[src/main.rs:10] 30 * scale = 60
    //[src/main.rs:14] &user2 = User {
    //  email: someone@example.com,
    //  username: someusername123,
    //  active: true,
    //  sign_in_count: 60,
    //}
    
}

//当参数名称和结构字段名称相同时，可以使用字段init速记语法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

```
**打印**  调用dbg!宏（拥有表达式的所有权）会打印到标准错误控制台流 (stderr)，而不是println!（只是宏的引用）打印到标准输出控制台流 (stdout)。  
### 结构数据的所有权
1. 每个字段都是真实数据（比如 String类型），拥有该类型的结构体，其中一个数据失效后，整个结构失效
2. 也可以使用引用类型（比如 &str类型），但是必须配合生命周期一起使用

### 结构体方法
它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。不过方法与函数是不同的，因为它们在结构体的上下文中，被定义并且它们第一个参数总是 self，它代表调用该方法的结构体实例  
1. 为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写）
2. 每个结构体都允许拥有多个 impl 块
3. 使用 &self 来替代 rectangle: &Rectangle，&self 实际上是 self: &Self 的缩写（不需要所有权的写法，需要的话用 &mut self）
4. 通常，但并不总是如此，与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 getters （rust不自动实现）

由于rust会 **自动引用和解引用** 所以可以直接使用 **.**运算符来使用方法，当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。  

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
//为了使函数定义于 Rectangle 的上下文中，我们开始了一个 impl 块（impl 是 implementation 的缩写）
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    //多个参数使用
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    //我们可以定义不以 self 为第一参数的关联函数（因此不是方法），因为它们并不作用于一个结构体的实例
    //不是方法的关联函数经常被用作返回一个结构体新实例的构造函数
    //
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    //使用结构体名和 :: 语法来调用这个关联函数
    //这个函数位于结构体的命名空间中：:: 语法用于关联函数和模块创建的命名空间
    let sq = Rectangle::square(3);
}
```

## 枚举
**enum** 枚举给予了将一个值成为一个集合的方法 
枚举的成员存在于其标识符的命名空间中使用 **::** 两个冒号获取  
```rust
//枚举的定义 这么设计的益处是现在 IpAddrKind::V4 和 IpAddrKind::V6 都是 IpAddrKind 类型的
enum IpAddrKind {
    V4,         //  使用::获取，如：IpAddrKind::V4;
    V6,
}
//接将数据附加到枚举的每个成员上
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```
**一个难点** 如果我们使用不同的结构体，由于它们都有不同的类型，我们将不能像使用示例 6-2 中定义的 Message 枚举那样，轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型。  
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//枚举也可以定义方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}


struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## Option 枚举
Rust中没有空值，**空值**（Null）是一个值，它代表没有值  
Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。  
另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。  
```rust
fn main() {
    //some_number 的类型是 Option<i32>
    let some_number = Some(5);
    //some_char 的类型是 Option<char>,这（与 some_number）是一个不同的类型
    //因为我们在 Some 成员中指定了值，Rust 可以推断其类型
    let some_char = Some('e');
    //absent_number， Rust 需要我们指定 Option 整体的类型，因为编译器只通过 None 值无法推断出 Some 成员保存的值的类型
    let absent_number: Option<i32> = None;
}
```
当有一个 Some 值时，我们就知道存在一个值，而这个值保存在 Some 中。  
当有个 None 值时，在某种意义上，它跟空值具有相同的意义：并没有一个有效的值。  
那么，Option<T> 为什么就比空值要好呢？
1. 当在 Rust 中拥有一个像 i8 这样类型的值时，编译器确保它总是有一个有效的值。我们可以自信使用而无需做空值检查。
2. 只有当使用 Option<i8>（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况
3. 能帮助我们捕获到 假设某值不为空但实际上为空的情况
4. 消除了错误地假设一个非空值的风险

## 控制流运算符
**match** 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应代码(按顺序执行分支,匹配到了，其他的分支将不再比较)
match的匹配是穷尽的，必须涵盖所有可能的情况  
模式可由字面值、变量、通配符和许多其他内容构成；

    match 表达式 {
        //分支
        模式 + 一些代码
    }
    

**match** 和 **if**的区别：if 后面的表达式必须返回一个布尔值，而match后面的表达式可以是任何类型  

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match + 表达式（coin）
    match coin {
        //模式（Coin::Quarter） => 代码（25）
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}

```
### 绑定值的模式
```rust
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn main() {}

```
### 匹配 Option<T>
```rust
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

```
### 通配模式和 _ 占位符
```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        //匹配模式是字面值 3 和 7，最后一个分支则涵盖了所有其他可能的值，
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        //模式是我们命名为 other 的一个变量。other 分支的代码通过将其传递给 move_player 函数来使用这个变量
        other => move_player(other),
        //当不想使用通配模式获取的值时
        //_ => reroll(),
        //_ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

```

### if let 简洁控制流
if let 语法让我们以一种不那么冗长的方式结合 if 和 let，来处理只匹配一个模式的值而忽略其他模式的情况  
```rust
fn main() {
    let config_max = Some(3u8);
    
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {    //相当于match中的 _ => ()
        println!("The");
    }
}

```