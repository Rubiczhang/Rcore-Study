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

使用 `return` 关键字和指定值，可从函数中提前返回；但大部分函数隐式的返回最后的表达式。这是一个有返回值的函数的例子

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

### Chap4.

​	对于存储方式跟堆有关的变量，在它对应的大括号结束时，会调用`drop`函数（相当于c++中的析构函数）。

​	**注意，在rust中，字符串有两种类型。

```rust
let mut s = "string";	//这里s是一个字符串引用，虽然有mut但是不能从中改变“string”的值，mut的作用是可以改变s的指向 比如：
s.push_str("xxx");		//报错
s = "hello";			//可以

let mut s = String::from("hello");	//这里s就是一个字符串
```



#### 	移动：

​		对于string类型的变量，其`ptr len capacity`都在栈中存储。`ptr`指向的字符数组在堆中存储

​		并且在复制的时候，直接将被复制的那个变量无效化（移动）

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);	//报错	
```

​	移动也可以发生在函数的参数传递以及返回值里，如：

```rust
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
```

​		这种写法过于繁杂，所以rust中可以使用”引用“



#### 	克隆：

​		如果需要确确实实地复制的话，应该使用`clone`函数。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```



#### 	对于只在栈上的数据而言，直接拷贝



#### 	引用:	

​	对于一个变量，要么只能有一个可变引用，要么只能有数个不可变引用。

```rust

	//普通引用的作用及传参形式
	fn main() {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }
	
	//mut引用的作用 及传参形式
	fn main() {
	    let mut s = String::from("hello");

    	change(&mut s);
	}

	fn change(some_string: &mut String) {
    	some_string.push_str(", world");
	}

	//mut引用和普通引用的区别 注意看下面mut的位置
	fn main(){
        let mut b = String::from("world");
        let mut b2 = String::from("here");
        let b1 = &mut b;
    	b1.push_str("hello");	//可以编译通过
        b1 = &mut b2;			//编译不通过
        
	}

	fn main(){
        let mut b = String::from("world");
    	let mut b2 = String::from("here");
    	let mut b1 = &b;
    	b1.push_str("hello");	//编译不通过
    	b1 = & mut b2;			//编译通过
	}
```



#### 	slice

```rust
let s = String::from("hello world")
let slice = &s[0..2];
let slice = &s[..2];
let world = &s[6..];
```



<img src="https://kaisery.github.io/trpl-zh-cn/img/trpl04-06.svg" alt="world containing a pointer to the 6th byte of String s and a length 5" style="zoom: 25%;" />













## Rust by Example

### 	Chap1.

​			

​	