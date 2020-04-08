fn main() {
    // ! 这个是说明是一个宏
    println!("Hello, world!");
    println!("Hello world!");

    println!("{} day", 31);

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
