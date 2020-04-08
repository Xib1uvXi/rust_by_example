# Rust

## Pre 

### rustup

rustup 是rust官方的版本管理工具。应当作为安装 Rust 的首选

安装命令:

```bash
curl https://sh.rustup.rs -sSf | sh
```

国内rustup镜像配置:

```bash
# 需要配置在 zshrc or bashrc

#  RUSTUP_DIST_SERVER(用于更新 toolchain)
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static

# RUSTUP_UPDATE_ROOT(用于更新 rustup)
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
```

关键命令:

```bash
# 更新工具链
rustup update

# 更新 rustup 本身
rustup self update

# 安装最新的 nightly 版本
rustup install nightly

# 指定版本
rustup install nightly-2020-03-18

# 指定默认使用的版本
rustup default nightly
```

例子：
```bash
rustup default stable
rustup default nightly
rustc --version
# output: 
#➜  ~ rustc --version
#rustc 1.44.0-nightly (f509b26a7 2020-03-18)

# rust 有三种Release Channels:
# stable
# nightly
# beta

# nightly 是最激进的版本，包含了大量（可能不稳定）的新/高级特性。stable 版本目前可能还不支持一些高级特性。beta 介于两者之间

# 对于 stable nightly 的解释可以详细阅读官方教程（The Rust Programming Language）中相关内容：
# https://doc.rust-lang.org/book/appendix-07-nightly-rust.html
```

### Cargo

国内镜像配置:

```bash
cd ~/.cargo/
ls | grep config  #如果没有任何output

vim config

#### ------- vim 写入下面配置
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

## Hello World

使用``cargo`` 创建项目:

```bash
cargo new helloworld

## output:
#     Created binary (application) `helloworld` package
```

运行rust hello world:

```bash

cd helloworld

cargo run


# output:

#   Compiling helloworld v0.1.0 (/Users/xib/Desktop/workspaces/rust/mastering-rust/rust_by_example/helloworld)
#    Finished dev [unoptimized + debuginfo] target(s) in 1.04s
#     Running `target/debug/helloworld`
# Hello, world!
```

相关源码：

```rust
fn main() {
    println!("Hello, world!");
}

//  cargo，自动产生的主函数
//  注释//，和c++一样，没有块注释，多行的每行//
//  每个程序有main.rs, main()
//  fn 声明函数
//  println!，!表示这是一个宏
//  使用4空格缩进，大括号标志程序块，分号结尾         
```

在看看官方的另个例子:

```rust
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
```

尝试去运行下，会发现报错：

```text
➜  helloworld git:(master) ✗ cargo run
   Compiling helloworld v0.1.0 (/Users/xib/Desktop/workspaces/rust/mastering-rust/rust_by_example/helloworld)
error[E0463]: can't find crate for `rand`
 --> src/main.rs:1:1
  |
1 | extern crate rand;  //引入外部库
  | ^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: could not compile `helloworld`.
```

在代码中开头有声明引入外部库：

```rust
extern crate rand; 
```

但是在项目中的Cargo.toml没有声明dependencies所以会导致报错，这里不得不吐槽下，这么不智能的吗...
修改 ``Cargo.toml``, 在dependencies下添加库：

```toml
[package]
name = "helloworld"
version = "0.1.0"
authors = ["wangxi <xib1102@icloud.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand="0.7.3"
```

可以使用``cargo search`` 搜索依赖:

```bash
cargo search rand

#output:
#rand = "0.7.3"              # Random number generators and other randomness functionality.
#random_derive = "0.0.0"     # Procedurally defined macro for automatically deriving rand::Rand for structs and enums
#ndarray-rand = "0.11.0"     # Constructors for randomized arrays. `rand` integration for `ndarray`.
#fake-rand-test = "0.0.0"    # Random number generators and other randomness functionality.
#rand_derive = "0.5.0"       # `#[derive(Rand)]` macro (deprecated).
#rand_core = "0.5.1"         # Core random number generator traits and tools for implementation.
#rand_mt = "3.0.0"           # Reference Mersenne Twister random number generators.
#pcg_rand = "0.11.1"         # An implementation of the PCG family of random number generators in pure Rust
#derive_rand = "0.0.0"       # `#[derive]`-like functionality for the `rand::Rand` trait.
#rand_macros = "0.1.10"      # `#[derive]`-like functionality for the `rand::Rand` trait.
#... and 307 crates more (use --limit N to see more)
```

这里还有一个坑，如果声明了``rand="0.4.6"```，执行:

```bash
cargo update -p rand --precise 0.7.3  ## --precise 指定确定版本
````

会报错，并不会像go mod 一样自动帮你升级，错误信息：

```text
➜  helloworld git:(master) ✗ cargo update -p rand --precise 0.7.3
    Updating `git://mirrors.ustc.edu.cn/crates.io-index` index
error: failed to select a version for the requirement `rand = "^0.4.6"`
  candidate versions found which didn't match: 0.7.3
  location searched: `git://mirrors.ustc.edu.cn/crates.io-index` index (which is replacing registry `https://github.com/rust-lang/crates.io-index`)
required by package `helloworld v0.1.0 (/Users/xib/Desktop/workspaces/rust/mastering-rust/rust_by_example/helloworld)`
perhaps a crate was updated and forgotten to be re-vendored?
```

如果是指定：``rand="~0.7"`` 这样的形式，就可以指定指定升级成``0.7.3``

```text
~1.2.3    <=>    [1.2.3, 1.3.0)
~1.2      <=>    [1.2.0, 1.3.0)
~1        <=>    [1.0.0, 2.0.0)
```

还有一种骚气的 ``*`` 任意版本:

```text
*            <=>    [0.0.0, ..)
1.*         <=>    [1.0.0, 2.0.0)
1.2.*      <=>    [1.2.0, 1.3.0)
```

还可以通过git去指定依赖:

```toml

#Cargo将从这个位置去适配git仓库，然后到git仓库任意位置(不一定需要在git仓库的根目录下)中查找Cargo.toml文件，从中查找到所有需要的crates
#因为我们没有指定任何其他的信息，Cargo会假定我们使用的是master分支的最近一次提交的内容
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand" }

#我们可以把git关键字与rev，tag，或者branch关键字结合来指定其他信息。下面是一个指定使用next分支上最近内容的例子
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand", branch = "next" }
```

更加可以使用本地路径去加载依赖:

```toml
#这将告诉Cargo我们的helloworld工程依赖一个叫hello_utils的crate，这个crate可以在hello_utils目录下找到(依据我们在Cargo.toml中写入的内容)
[dependencies]
hello_utils = { path = "hello_utils" }

#在crates.io中，不允许使用仅指定路径的依赖项。如果我们想要发布我们的hello_world crate，我们需要先发布一个hello_utils的版本到crates.io上(或者指定一个git仓库位置)，并且也要在依赖行中指定它的版本
[dependencies]
hello_utils = { path = "hello_utils", version = "0.1.0" }
```

关于 cargo 依赖的相关文档可以查看：[The Cargo Book Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)

## Rust let

