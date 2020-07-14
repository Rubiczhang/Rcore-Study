#### 7.2

​	看了一下《rust程序设计》的前几章，对rust有了一个大体的了解。预计明后两天就可以完成语法的学习了。

​	github还不是很会用，今天就先传一个README上去



#### 7.3

 1. 勉强会用github了。经过一番操作，更新github只需要以下三行命令就可以把本地文件夹push到github上。其他的操作到以后再继续深入学习。

    `git add.`

    `git commit -m "comment"`

    `git push origin master`

 2. 继续学习了《rust程序设计》

    明白了mut的各种用法

#### 7.4

​	了解了rust中的结构体和枚举以及模块化的概念

​    其中模块化的概念还不是很清楚，等到后期再深入了解一下

#### 7.5&7.6

​	考了驾照，这两天没有学习

​	

#### 7.7

​	基本了解了rust的语法

​	对生命周期还是有些困惑

​	完成了四分之一的rustlings

​	明天把rustling 完成并且把生命周期弄明白

#### 7.8 & 7.9

做了大部分rustlings 

也大概明白了生命周期是个什么东西

//*这两天事情比较多所以进度慢了点，后面就可以全心搞rcore了* 0 . 0

//*明天可以把rustlings做完并且完成部分习题*



#### 7.10

​	完成了rustlings

​	编程题完成了一个，主要还是对rust的语法不熟悉

​	*明天把编程题完成*



#### 7.11

​	今天在配置rust的debug环境花了一天
​	环境调试好的话debug很简单：

​	1. 用编译的时候用gnu工具,命令如下：

`rustc --target=x86_64-pc-windows-gnu xxx.rs -g`

2. 调试：

`gdb xxx.exe`

​	这里说一下自己遇到的~~环境~~问题及解决方法：（*windows下*）

```
rustc --target=x86_64-pc-windows-gnu xxx.rs -g
    //以上的"x86_64-pc-windows-gnu"或许可以换成其他版本的gnu 但是我用的是x86_64的（也导致了我给自己在最后一步挖了个坑
    //如果执行这一步的时候提醒：
    //error: linker `xxxx` may not be installed
    //则需要将你想使用的gnu工具包添加到rust里面
rustup target add x86_64-pc-windows-gnu
    //这里我仍然用的是x86_64-pc-windows-gnu
    //当我再次尝试第一行的命令时，又一次报错
    //error: linker `x86-w64-mingw32-gcc` not found	//大概是这个报错方式，我记不清了
	//这是因为我电脑上面没有这个gcc版本，理论上直接安装就可以了，但是我不会
	//我采用了一种复杂度很高的解决方案
        //1. 下载并 安装msys2
        //2. 在msys2 里执行
        pacman -S mingw-w64-x86_64-gcc
        //3. 打开安装msys2的文件夹，并将mingw64的bin文件夹的路径添加到系统变量中
	//这个时候我可以执行第一行的编译代码了，但是我并不能成功地用gdb打开
	//又经过很久我才发现我原本的gdb是32位的，不能调试64位的exe，所以我又在
	//msys2里面安装了gdb
	pacman -Ss gdb
	//仍然把gdb.exe（9.2版本的）所在的bin路径放到系统变量中
	//这个时候我再编译和调试就一路通畅了
```

#### 7.14

​	学习了rust的trait，了解了risc-v的基本指令