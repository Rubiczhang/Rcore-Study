## 总览

![总览](NotePic\OverView.png)

###   寄存器集

​		![寄存器](NotePic\Registors.png)

### 寄存器命名

​	![RegistName](NotePic\RegistName.png)

## RISC-V汇编

​		   汇编语言跟指令集不同，汇编语言包括伪指令。

### 		伪指令：

​				![Pseudo](NotePic\PseudoIns1.png)

​				![Pseudo](NotePic\PseudoIns2.png)

### 两种特权模式：

#### 	简单嵌入式系统的机器模式（M模式）

​			RISC-V中的异常分为两类

​				同步异常（mcause最高有效位为0

​					指令执行期间发生，如访问无效地址

​				中断（mcause最高有效位为1

​					外部事件：软件、时钟、外部来源

​		机器模式下的控制状态寄存器

​					![异常处理器](NotePic\CSR.png)

##### 	中断处理（1-4 原子性

  1. 异常指令的 PC 被保存在 mepc 中，PC 被设置为 mtvec。（对于同步异常，mepc 指向导致异常的指令；对于中断，它指向中断处理后应该恢复执行的位置。）

  2. 根据异常来源设置 mcause，并将 mtval 设置为出错的地址或者其它适用于特定异常的信息字。

  3. 把控制状态寄存器 mstatus 中的 MIE 位置零以禁用中断，并把先前的 MIE 值保 留到 MPIE 中。

  4. 发生异常之前的权限模式保留在 mstatus 的 MPP 域中，再把权限模式更改为 M。（如果处理器仅实现 M 模式，则有效地跳过这 个步骤）。

  5. 通用寄存器由软件保存

  6. 可抢占的中断处理程序在中断处理前把寄存器保存到栈中，并在退出之前禁用中断并恢复寄存器

  7. mret 将 PC 设置为 mepc，通过将 mstatus 的 MPIE 域复制到 MIE 来恢复之前的中断使能设置，并将权限模式设置为 mstatus 的 MPP 域中的值。 这基本是前一段中描述的逆操作。

     ##### 物理内存保护

     ​	PMP 包括几个地址寄存器（通常为 8 到 16 个）和相 应的配置寄存器。这些配置寄存器可以授予或拒绝读、写和执行权限。当处于 U 模式的处 理器尝试取指或执行 load 或 store 操作时，将地址和所有的 PMP 地址寄存器比较。如果地 址大于等于 PMP 地址 i，但小于 PMP 地址 i+1，则 PMP i+1 的配置寄存器决定该访问是否 可以继续，如果不能将会引发访问异常。

​	![物理内存保护](NotePic\PMP.png)

#### 现代操作系统的监管者模式

##### 	mideleg 

​	（Machine Interrupt Delegation，机器中断委托）CSR 控制将哪些**中断**委托给 S 模式。与 mip 和 mie 一样，mideleg 中的每个位对应于一个异常。例如， mideleg[5]对应于 S 模式的时钟中断，如果把它置位，S 模式的时钟**中断**将会移交 S 模式 的异常处理程序，而不是 M 模式的异常处理程序。

​	M 模式还可以通过 mideleg  CSR 将**同步异常**委托给 S 模式。该机制类似于刚才提到 的中断委托，但 mideleg  中的位对应的不再是中断，而是图 10.3 中的**同步异常**编码。例 如，置上 mideleg [15]便会把 store page fault（store 过程中出现的缺页）委托给 S 模式。

#####   sip sie

​		sie（Supervisor Interrupt Enable，监管者中断使能）和 sip（Supervisor Interrupt Pending，监管者中断待处理）CSR 是 S 模式的控制状态寄存器，**他们是 mie 和 mip 的子集** （mie和mip寄存器中的一部分位构成sie和sip）。它们有着和 M 模式下相同的布 局，但在 sie 和 sip 中只有与由 mideleg 委托的中断对应的位才能读写。那些没有被委派 的中断对应的位始终为零。

##### 	异常处理是由权限更高的代码处理的

##### 		

#### 基于页面的虚拟内存

​	