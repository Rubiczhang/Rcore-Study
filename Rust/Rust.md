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
let slice = &s[0..2];//（左闭右开
let slice = &s[..2];
let world = &s[6..];
```



<img src="https://kaisery.github.io/trpl-zh-cn/img/trpl04-06.svg" alt="world containing a pointer to the 6th byte of String s and a length 5" style="zoom: 25%;" />







### Chap5. 结构体

```rust
//***********普通结构体***************************
#[derive(Debug)]
struct Rectangle {	//定义语法
    width: u32,
    height: u32,
}

impl Rectangle {	//方法定义语法
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
			//赋值语法
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

//******************元组结构体*************************
//每个位置的类型都一样
struct Color(i32, i32, i32);

let black = Color(0, 0, 0)
```

#### 	方法

当使用 `object.something()` 调用**方法**时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 `object` 与方法签名匹配。也就是说，这些代码是等价的：

#### 	函数

​		函数定义的时候**不**以self为第一个参数。

​		调用的时候应该用`::`这个符号 如：

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main(){
    let sq = Rectangle::square(16);
}
```



### Chap6. 枚举和模式匹配

```rust
//***********最简单的枚举*************
enum IpAddrKind {
    V4,
    V6,
}


let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

//************为枚举加上类型****************
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

跟结构体的语法相同，枚举也可以用`impl`关键字来定义自己的函数

​	 

####   match

```rust
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### 包含了枚举的枚举

```rust
#[derive(Debug)] // 这样可以可以立刻看到州的名称
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
        },
    }
}
```

####  Option:

​	`option`也是一种枚举，定义在标准库中，定义如下：

```rust
enum Option<T>{
    Some(T),
    None,
}
```



### Chap7. 包 crate 模块

​	包   ----(包含)----->  一个**库crate** | 多个**二进制crate** 

   包至少包含一个crate

   每个crate都有一个根。

  一个包有一个Cargo.toml

####    模块

​		**模块** 让我们可以将一个**crate** 中的代码进行分组。

​		如果在一个库（restaurant）的restaurant/src/lib.rs 中定义了下面一段代码

```rust
    mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}

            fn seat_at_table() {}
        }

        mod serving {
            fn take_order() {}

            fn server_order() {}

            fn take_payment() {}
        }
    }
```

在前面我们提到了，`src/main.rs` 和 `src/lib.rs` 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 `crate` 的模块，该结构被称为 *模块树*（*module tree*）。上述代码的模块树如下：

示例 7-2 展示了示例 7-1 中的模块树的结构。

```text
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

#### 模块的私有与公有：

```rust
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
    }

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        front_of_house::hosting::add_to_waitlist();
    }
```

要在`eat_at_restaurant()`中访问add_to_waitlist()必须要将其路径上的所有东西都设置成`pub`的（枚举成员默认是私有的）。在上面一段代码中显然front_of_house不是pub的，这是因为eat_at_restaurant()跟front_of_house在同一个create里（他们是兄弟节点）

#### super

​	super相当于文件路径中的`..`即访问父节点

#### 公有的结构体

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

#### use

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;//这里一般引用到函数的父模块，而不是函数

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

`use xxx as xxx`跟python中的`import xxx as xxx`是一样的

`pub use`



在用**外部包**中定义的代码时，应该在`Cargo.toml`文件中加入**包**名，并且在源文件中`use`声明相应的函数/模块



#### 嵌套路径

```rust
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};	//这一行相当于上面两行

use std::io;
use std::io::Write;

use std::io::{self, Write};		//这一行相当于上面两行
```



### Chap.8 常见集合

####  Vector

```rust
let v: Vec<i32> = Vec::new();	//初始化语法1
let v = vec![1,2,3];	//初始化语法2
v.push(5); 				
let third: &i32 = &v[2];	//v[2]返回的是值，&v[2]返回的是引用
match v.get(2) {			//返回一个Option<&T>
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

不能在对`vector`中的某项进行引用时对vector整体进行操作，如：

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

#### 遍历vector中的元素

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;		//解引用
}
```



#### 字符串

```rust
let mut s = String::new();
let data = "inital contents"
let s = data.to_string();//实现了Display trait的类型都实现了这个方法
let s = String::from("inital contents");

//字符串的拼接
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用

let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);  //把字符串添加进去

println!("s2 is {}", s2);//报错

let mut s = String::from("lo");
s.push('l');	//把字符添加进去
```

#### hash map

​	使用的时候要添加`use std::collections::hashMap`

​	hash map 是**同质**的

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect(); //用数组构建hash map

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);	//向表中添加元素，这里参数是借用
// 这里 field_name 和 field_value 不再有效，
```



#### 访问hash map里面的值

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

### Chap9. 泛型 trait   生命周期

#### 结构体定义中的泛型

```rust
//单泛型结构体定义及使用
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
//泛型相关的方法定义
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

//双泛型结构体定义及使用
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

#### trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

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
```

#### trait的动态分发和静态分发

```rust
#[derive(Debug)]
struct Foo;
trait Bar{
    fn baz(&self);
}
impl Bar for Foo{
    fn baz(&self){ println!("{:?}", self)}
}

fn static_dispatch<T>(t: &T) where T:Bar{   //静态分发 &后面跟的是一种具体类型，而不是trait
    t.baz();
}

fn dynamic_dispatch(t: &Bar){               //动态分发 &后面跟的是一种trait
    t.baz();
}

fn main(){
    let foo = Foo;
    static_dispatch(&foo);  //rust调用这个函数的时候并没有调用第10行的代码
                            //而是调用了下面这个单态化函数（实际实现的时候函数名可能有所出入）Rust编程之道 P61
                            //fn static_dispatch_Foo(t:Foo){
                            //   Foo.baz(t); 
                            //}
                            //该函数是rust编译器在编译时额外添加的


    dynamic_dispatch(&foo); //rust编译器并没有为动态分发的函数添加每种可能情况的代码
                            //而是将参数以TraitObject的形式传入到函数中（rust编程之道P72）
                            //并且从TraitObejtc指向的相应位置来调用其中的函数(Foo.baz(t))
}
```



### Chap10.智能指针

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

#### 	Box\<T>

​		Box\<T>的类型为`usize`

