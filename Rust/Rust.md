### 1. 熟悉GitHub的使用

​	目前只会图形化交互，用鼠标点击上传/下载按钮



## 《Rust程序设计语言》

### Chap1.

#### 1.rust环境配置

​	在[这个](https://www.rust-lang.org/tools/install)链接里面下载rust-up 然后跟着引导就安装好了rust的编译器

​	**意料之外的问题**：

```cmd
$ rustc --version
rustc 1.44.1 (c7087fe00 2020-06-17)
$ rustc hello.rs
error: linking with `link.exe` failed: exit code: 1
```

​	经过排查并验证，问题出在了Visual Studio上面

​	rustc在编译程序的时候需要有VS的C++环境（即使原先安装了g++也是这样）



#### 2.cargo

​	cargo 是一个“包管理工具”，对我而言，他就像一个没有gui的ide。

​	```cargo check``` 是检查语法错误的一个命令，而不是检查文件有没有改动。

### Chap2.

#### crate：

​	代码包被称作crate

​	crate也分  ```二进制crate（可执行文件）``` 和 ```库crate（代码crate）``` 

#### match：

​	类似于C语言中的swtich

#### except：

​	Result类型的一个方法。`Result` 的成员是 `Ok` 和 `Err`，`Ok` 成员表示操作成功，内部包含成功时产生的值。`Err` 成员则意味着操作失败，并且包含失败的前因后果。`io::Result` 的实例拥有 expect方法。如果 `io::Result` 实例的值是 `Err`，`expect` 会导致程序崩溃，并显示当做参数传递给 `expect` 的信息。如果 如果 `io::Result` 实例的值是 `Ok`，`expect` 会获取 `Ok` 中的值并原样返回。

### Chap3.

#### 	显式声明类型：

​		`let x :type =...` 

#### 	元组

​	  rust中的元组可以包含不同种元素，在声明之后，可以改变任意位置上的值，但相应位置的数据类型就不能改变了，并且长度也不能改变。在给元组整个复制的时候是对每个元素依次复制的，并不是直接把一个新的元组实体赋值给tup。

​	元组元素的访问格式是

```rust
let mut tup = (1, "a", "c");
let x = tup.1;			//x = "a"
let (x, y, z) = tup;	//x = 1, y = "a", z = "c"
tup = (1, 2, 3)			//会报错，
```

#### 数组

​	老朋友，有以下几种声明方法。

```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
let a = [3; 5];		//a = [3,3,3,3,3]
let a = [1, 2, 3, 4, 5];
```

​	不同于元组，数组中只能存一种元素，要想对数组中的元素进行更改，也需要在声明的时候加上mut

#### 	函数

```rust
fn plus_one(x: i32) -> i32 {	//参数要指定类型，返回值类型用箭头指定
    let y = 2;	//语句，没有值
    x + 1		//没有分号，表达式，类似于return
}
```

#### 	控制流

#### 		if

​			语法如下

​			其中判断条件必须为布尔值

```rust
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

#### 		loop

​			其中break可以返回值

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

#### 		while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}
```

#### 		for

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```





## Rust by Example

### 	Chap1.

​			

​	