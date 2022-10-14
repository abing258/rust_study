# rust的常见集合
不同于内建的数组和元组类型，这些集合指向的数据是储存在**堆**上的，这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小。  

## Vector | Vec<T>
vector可以存储多个相同类型的值，且在内存中彼此相邻的排列。  
vector 是用泛型实现的。  
### 新建
```rust
fn main() {
   //新建一个空的 vector 来储存 i32 类型的值
   let v: Vec<i32> = Vec::new();
   //新建一个包含初值的 vector。 由于有初始数据，编译器会可以判断出v 的类型是 Vec<i32>
   let v = vec![1, 2, 3];
}
```
### 更新
vector增加元素，可以使用 **push** 方法。  
```rust
fn main() {
   let mut v = Vec::new();

   v.push(5);
   v.push(6);
   v.push(7);
   v.push(8);
}
```
### 访问
1. 使用 & 和 [] 返回一个引用  
2. 使用 get 方法以索引作为参数来返回一个 Option<&T>  
方法一当超出vec长度时，会panic。get会得到一个None
```rust
fn main() {
   let v = vec![1, 2, 3, 4, 5];

   let third: &i32 = &v[2];
   println!("The third element is {}", third);

   match v.get(2) {
      Some(third) => println!("The third element is {}", third),
      None => println!("There is no third element."),
   }
}
```
**难点**
当我们获取了 vector 的第一个元素的不可变引用并尝试在 vector 末尾增加一个元素的时候，如果尝试在函数的后面引用这个元素是行不通的。  
vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。  

### 遍历
```rust
fn main() {
   let v = vec![100, 32, 57];
   for i in &v {
      println!("{}", i);
   }

   let mut v = vec![100, 32, 57];
   for i in &mut v {
      //为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值。
      *i += 50;
   }
}
```
### 枚举来储存多种类型
```rust
fn main() {
   enum SpreadsheetCell {
      Int(i32),
      Float(f64),
      Text(String),
   }

   let row = vec![
      SpreadsheetCell::Int(3),
      SpreadsheetCell::Text(String::from("blue")),
      SpreadsheetCell::Float(10.12),
   ];
}
```
Rust 在编译时就必须准确的知道 vector 中类型的原因:  
1. 需要知道储存每个元素到底需要多少内存
2. 第二个好处是可以准确的知道这个 vector 中允许什么类型
[Vec API](https://doc.rust-lang.org/std/vec/struct.Vec.html) 可以查看具体的方法  

## String
String 是一个 Vec<u8> 的封装  
字符串就是作为字节的集合外加一些方法实现的。  
Rust 的核心语言中只有一种字符串类型：字符串slice str，它通常以被借用的形式出现，&str。字符串 slices：它们是一些对储存在别处的 UTF-8 编码字符串数据的引用。  
由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串slices。  
称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型。  
String 和字符串 slices 都是 UTF-8 编码的。  

### 新建
1. String::new();
2. initial contents".to_string();
3. String::from("initial contents");
```rust
fn main() {
   //新建一个空的 String
   let mut s = String::new();
   //to_string 方法，可以从字符串字面值创建 String
   //它能用于任何实现了 Display trait 的类型，字符串字面值也实现了它
   let data = "initial contents";
   let s = data.to_string();
   // 该方法也可直接用于字符串字面值：
   let s = "initial contents".to_string();
   //String::from 函数来从字符串字面值创建 String
   let s = String::from("initial contents");
}
```
由于字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据，下面的列子在字符串中储存不同语言的问候语。  
```rust
fn main() {
   let hello = String::from("السلام عليكم");
   let hello = String::from("Dobrý den");
   let hello = String::from("Hello");
   let hello = String::from("שָׁלוֹם");
   let hello = String::from("नमस्ते");
   let hello = String::from("こんにちは");
   let hello = String::from("안녕하세요");
   let hello = String::from("你好");
   let hello = String::from("Olá");
   let hello = String::from("Здравствуйте");
   let hello = String::from("Hola");
}
```

### 更新字符串
1. push_str 方法来附加字符串 slice
2. push_str 方法来附加字符
3. +运算符拼接字符串（fn add(self, s: &str) -> String {}）  
4. format! 宏拼接字符串  
```rust
fn main() {
   let mut s1 = String::from("foo");
   let s2 = "bar";
   s1.push_str(s2);
   //push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权
   println!("s2 is {}", s2);


   let mut s = String::from("lo");
   s.push('l');


   let s1 = String::from("Hello, ");
   let s2 = String::from("world!");
   let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
   //+ 运算法可以被理解为 fn add(self, s: &str) -> String {}
   //&String 可以被 强转（coerced）成 &str --> Deref 强制转换

   let s1 = String::from("tic");
   let s2 = String::from("tac");
   let s3 = String::from("toe");
   let s = format!("{}-{}-{}", s1, s2, s3);//format! 生成的代码使用引用所以不会获取任何参数的所有权
}
```
### 索引字符串
字母的UTF-8编码占用一个字节。 `Здравствуйте` Unicode 标量值需要两个字节存储。  
`&hello[0]` 返回的是字节值，104 而不是 h  
**字节** **标量值** **字形簇**  
梵文书写的印度语单词 “नमस्ते”,  
最终它储存在 vector 中的 u8 值--> [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]  
从 Unicode 标量值的角度理解它们--> ['न', 'म', 'स', '्', 'त', 'े']  
以字形簇的角度理解--> ["न", "म", "स्", "ते"]  
Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。  
### 字符串 slice
索引字符串通常是一个坏点子，因为字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice。   
可以使用 [] 和一个 range 来创建含特定字节的字符串 slice：  
```rust
fn main() {
   let hello = "Здравствуйте";
   let s = &hello[0..4];
   let a = &hello[0..1];//Rust 在运行时会 panic，就跟访问 vector 中的无效索引时一样
}
```
### 遍历字符串的方法
遍历字符串每一部分必须明确表示需要是字节（.bytes()）/字符（.chars()）/字形簇（需要自己找crate）  
```rust
fn main() {
   for c in "नमस्ते".chars() {
      println!("{}", c);
   }
}
```

## Hash Map
**HashMap<K, V>** 类型储存了一个键类型 K 对应一个值类型 V 的映射。它通过一个 哈希函数（hashing function）来实现映射，决定如何将键和值放入内存中  

### 新建Map
1. 可以使用 new 创建一个空的 HashMap，并使用 insert 增加元素。  
2. 在一个元组的 vector 上使用迭代器（iterator）和 collect 方法，其中每个元组包含一个键值对
所有的键必须是相同类型，值也必须都是相同类型。  
```rust
use std::collections::HashMap;

fn main() {
   //方法一
   let mut scores = HashMap::new();
   scores.insert(String::from("Blue"), 10);//insert会转移所有权
   scores.insert(String::from("Yellow"), 50);
    //访问
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //方法二
   let teams = vec![String::from("Blue"), String::from("Yellow")];
   let initial_scores = vec![10, 50];
   let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
}
```
### 更新
1. 再次insert会覆盖掉原先的值
2. 只在键没有对应值时插入`.entry(key)).or_insert(50);`
```rust
use std::collections::HashMap;
fn main() {
    //覆盖
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    
    //entry
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
    
    //根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
```
### 哈希函数
HashMap 默认使用一种叫做 [SipHash](https://en.wikipedia.org/wiki/SipHash) 的哈希函数，它可以抵御涉及哈希表（hash table）1 的拒绝服务（Denial of Service, DoS）攻击。然而这并不是可用的最快的算法。  
可以指定一个不同的 hasher 来切换为其它函数。hasher 是一个实现了 BuildHasher trait 的类型。 