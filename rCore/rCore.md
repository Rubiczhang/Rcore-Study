## lab0

`https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html`

1. 简便起见，将panic的处理策略改成`abort` , 直接调用panic_handler而不是先堆栈展开再调用

   在Cargo.toml 中加上两行代码：

   ​	os/Cargo.toml

   ```toml
   ...
   
   # panic 时直接终止，因为我们没有实现堆栈展开的功能
   [profile.dev]
   panic = "abort"
   
   [profile.release]
   panic = "abort"
   ```

2. main.rs中要加的两个标签如下

   ```rust
	#![no_std]	//禁用std
	//为何要禁用std？裸机环境，没有std
	#[panic_handler]	//声明panic处理函数 如：
	fn panic(_info: &PanicInfo)-> !{
	    loop{}
	}

   ```

3. 为何不用main函数？而是_start()函数？

   main函数不是第一个运行的函数，_strat函数是的。

   ```rust
   /// 覆盖 crt0 中的 _start 函数
   /// 我们暂时将它的实现为一个死循环
   #[no_mangle]
   pub extern "C" fn _start() -> ! {
       loop ={}
   }
   ```

4. 如何编译成裸机目标（用不了std

   ```cargo build --target riscv64imac-unknown-none-elf```  其中"none"表示没有操作系统（裸机）

   也可以在.cargo/config中加入

   os/.cargo/config

   ```toml
   # 编译的目标平台
   [build]
   target = "riscv64imac-unknown-none-elf"
   ```

5. 查看汇编码：

   ```rust
   rust-objdump target/riscv64imac-unknown-none-elf/debug/os -x --arch-name=riscv64
   -x查看源信息
   -d查看反汇编
   ```

   生成镜像：

   `rust-objcopy target/riscv64imac-unknown-none-elf/debug/os --strip-all -O binary target/riscv64imac-unknown-none-elf/debug/kernel.bin`

   


## lab1

​	`https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-1/guide/intro.html`

### 	习题：

#### 		执行ebreak指令的时候，sp是如何变化的？

​			硬件指令并没有对sp进行操作

​			观察代码可知，sp首先向下生长，给context留下足够的空间。

​			这个时候sp的值可以作为context的地址

> ​			虽然栈是从上往下生长，但是数组/结构体寻址则是基地址加上偏移量，所以sp可以作为context的地址。

​			从`_interrupt`里跳到`handle_interrupt`里时并没有改变sp的值。（jal指令）

​			在`handle_interrupt`里调用了`breakpoint`函数，这时有新的参数入栈和出栈

​			`handle_interrupt`函数调用完成之后继续执行interrupt.asm的代码

> `rust`函数不带`return`语句，也没有看到ret指令，这里这个函数调用过程有些不懂

​			然后pc回到`jal handle_interrupt`的下一行，继续执行，这里不加restore标签应该也是可以的，在resotre中，sp恢复到来之前的值。

#### 		main.rs去除panic!会如何？

​			在我的代码中运行的结果是出现了一个非ebreak并且非time_break的中断

​			看了答案了解了代码会继续往下运行，很容易就出线非法访问。

#### 		代码题：

​			第一问，显然，见代码

​			第二问，经过查文档，stval可以在地址例外中保存出错的地址，所以只需要读取这个寄存器的值就可以判断了。要读这个寄存器，只能用csrr指令。

> 我看漏了一个东西，这里handle_interrupt已经获取了stval并且把它当成参数传入了

​			第三问，可以在entry.asm `sret`之前把执行`ld t1 (x0)`指令

​	目前存在的问题

​				1. 为何handler.rs没有use console就可以使用println!

## lab2

​	这个lab最终目的是为了完成一个物理页帧分配系统，由于内核在进行这些工作时需要一个堆，但是OS并没有提前实现这个堆，所以我们手动实现它。并且这个堆是用的现有的轮子。剩下的部分都是为了完成一个物理内存管理的模块这个模块的管理器是FRAME_ALLOCTOR。

 	1.  .bss字段是存放初始值为0的值的内存空间。并且这里实现的是**内核堆**，它仅仅需要一块空间来存放。我认为，如果用户进程需要堆的话，可能会对其单独分配一块空间作为堆。

2. 经过验证，这个trait在`StackedAllocator` 中具体实现，其时间复杂度为O(1)  （执行了一次 `vec.push()`  其空间复杂度为O(n)