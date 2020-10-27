---
description: 本文档旨在帮助同学们在被实验题折磨时有个最终归宿以及是Lab的补充内容汇总。内容是总结了前人的项目仓库记录。
---

# Lab实现及参考答案

## Lab 0

### 讨论

#### SKTT1Ryze:

+ 本次实验作为rCore系列实验的预备实验，不禁让人惊讶于其内容之丰富，设计之巧妙，做着做着就沉浸其中，回过神来已经花了一天的时间。
+ 这个实验涉及了很多我之前没有接触到过的知识，比如OpenSBI，比如qemu的使用方法，整个实验下来，收益良多。
+ 预备实验就已经有一定的难度并且给我带来了许多挑战，这不禁让我对后面的实验期待起来。

#### wfly1998:

+ 感觉Rust这门语言是真的严谨，标准库里带着对panic的处理以及堆栈展开的处理，很贴心了，所以禁用掉标准库之后要自己实现一遍。
+ `#[no_mangle]`可以使编译的时候编译器不会为了实现重载而把函数名字改得乱七八糟
+ `objdump -x`可以查看程序的元信息，`objdump -d`可以对程序进行反汇编----`objcopy`在uCore中也有用到过，转换成`binary`格式一方面可以直接运行，另一方面可以有效缩小程序尺寸，这样才能使uCore的bootloader缩小到512字节以内的。

#### stellarkey:

+ C语言：内存不安全（SegmentFault），缺少现代语言特性和好用的工具链。

  而Rust：内存+线程安全，高层语言特性，友好的工具链，蓬勃发展的社区生态。

+ 在此项目（rCore）开始时，开源界已经有不少RustOS的项目：

  - Redox：这是目前完成度最高的RustOS，微内核架构，平台x86_64
  - 《Writing an OS in Rust》& blog_os：这是一个从零开始写RustOS的教程，平台x86_64
  - CS140e：这是斯坦福2018年新开的实验性课程，用Rust写的教学OS，平台arm/RaspberryPi

#### shiweiwww

+ 从去年入坑os，当初的目的单纯想提高下自己技能，经历了拿起放下再拿起，从刚开始啥都不懂开始慢慢入门找到有感觉了。一直以来想自己写一个简单的os可以运行在Qemu或者bochs等虚拟机上，奈何功力尚浅，迟迟不敢动手，刚好有这次学习机会，决定从零搭建一个os项目，尽管参考实验指导书和rCore-Tutorial不少代码，就把这次实习当做是一个初版的os，然后一步一步的迭代完善

+ 以前用c写OS，如linux 0.11等，整体经过三步才`bootloader-->setup-->Os`, opensbi帮我们完成了前面的两步骤，现在只需要关心如何写好OS就可以，极大的简化了代码的复杂度

+ rcore实验指导书lab0中要是能介绍如何编译运行调试riscv汇编的例子就更好了，这样对riscv有直观的感受，要是不懂有什么指令，特权级相关的，可以简单写个demo在看看，毕竟纸上得来终觉浅，光看书还是不知道怎么用

+ 对rust了解的还是很浅，看来抽时间还得刷下官网的rust的[Embedded devices](https://www.rust-lang.org/what/embedded)教程



## Lab 1

### 讨论

#### SKTT1Ryze:

+ 终于做完 Lab1 了，比想像中还要花费精力。对 Lab1 的修改版本将会在另外的报告中说明。从这个实验中不仅加深了对`Rust`语言的理解，还亲身感受到了如何用Rust语言编写操作系统的，收益良多。剩下的实验继续加油。
+ [学习笔记](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/rCore_Labs/Lab1/Report/lab_1_report.md#lab1)

#### wfly1998

+ [分析中断的处理流程](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab1.md#%E5%88%86%E6%9E%90%E4%B8%AD%E6%96%AD%E7%9A%84%E5%A4%84%E7%90%86%E6%B5%81%E7%A8%8B)

#### JohnWestonNull

+ scause 与 stval 之所以不放到上下文中是因为这两者只是临时变量，不需要特意存储到栈上，只需要在分派时放到对应的参数位置即可。

#### shiweiwww

+ debug汇编花了一天时间，主要是犯了一个低级错误，刚开始没设置sp的值，导致ld一直失败，解决了这bug，感觉riscv汇编不是什么大问题了，同时在看lab1的代码，理解更深了一点，明白为啥这么做，和x86实现中断没什么本质区别



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab1_practice.md#%E5%AE%9E%E9%AA%8C%E9%A2%98)
+ [stellarkey](https://github.com/stellarkey/os_summer_project/tree/master/rcore_project#%E5%AE%9E%E9%AA%8C%E4%B8%80%E5%AE%9E%E9%AA%8C%E9%A2%98)
+ [chibinz](https://github.com/chibinz/rCoreLab/blob/ce554c15c2e27e732b285803085d36d3da8f4c20/doc/lab1.md#lab-1-%E5%AD%A6%E4%B9%A0%E8%AE%B0%E5%BD%95)

