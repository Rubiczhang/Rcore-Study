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

​					![Pseudo](NotePic\CSR.png)

#### 	现代操作系统中的监管者模式