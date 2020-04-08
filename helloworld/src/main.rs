extern crate rand;  //引入外部库

use std::io;
//使用标准库
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("hello world");

    println!("Guess the number!");  //字符串，Rust里面叫做字符串文本

    let secret_number = rand::thread_rng().gen_range(1, 101);  //let创建不变量

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        //let mut, 创建变量， String， 字符类型， ::调用关联函数

        io::stdin().read_line(&mut guess)  //&， 参考，即地址， ::调用方法，“.”调用函数
            .expect("Failed to read line");  //错误处理

        let guess: u32 = match guess.trim().parse() {  //match类似switch
            Ok(num) => num,  //=>返回分支结果
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);   //{}匹配字符串

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}