fn main() {
    println!("Hello, world!");
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let a = five();
    println!("The value of a is: {}", a);

    //字符串的理解
    let a = "hello";                    //字符串字面值，不可变
    println!("a is{}", a);

    let mut a = "hello";                    //字符串字面值，不可变
    // no method named `push_str` found for reference `&str` in the current scope
    // a.push_str(", world!");
    // println!("a change is {}", a);

    let b = String::from("hello");      //String类型，被分配到堆上
    println!("b  is {}", b);
    let mut c = String::from("hello");
    c.push_str(", world!"); // push_str() 在字符串后追加字面值
    println!("c change is{}", c); // 将打印 `hello, world!`

    let my_string = String::from("hello world");
    let slice = &my_string[0..2];
    println!("{}", slice);

    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word = first_word(my_string_literal);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        aaa => move_player(aaa),
    }

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn five() -> i32 {
    5
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

