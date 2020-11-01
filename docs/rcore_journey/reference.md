---
description: 本文档旨在帮助同学们在被实验题折磨时有个最终归宿以及是Lab的补充内容汇总。内容来源前人的项目仓库记录。
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
+ 现在稍稍有点理解RISC-V的哲学了：“保持简洁，保持功能单一”
+ 总而言之，扩展性是为了可以将处理器做到很小、功能很少，也可以做到很大、功能很多

#### stellarkey:

+ 挨个看了一遍。有的同学一开始就已经是rust、OS的老手，每天做的工作早就开始写操作系统；有的同学刚入门；有的同学做lab快要完成；有的同学进度一般；还有的同学中途放弃。
+ 一些我遇到的问题和没遇到的问题，都在其他同学的Daily找到了一些映证。
+ 心血来潮看了一点Go语言，Go语言也是系统编程级语言，也是现代语言，也许跟Rust有一些相似性。至少从目前看来，它们的创新特点有很多类似的地方。

#### JohnWestonNull

+ scause 与 stval 之所以不放到上下文中是因为这两者只是临时变量，不需要特意存储到栈上，只需要在分派时放到对应的参数位置即可。

#### shiweiwww:

+ debug汇编花了一天时间，主要是犯了一个低级错误，刚开始没设置sp的值，导致ld一直失败，解决了这bug，感觉riscv汇编不是什么大问题了，同时在看lab1的代码，理解更深了一点，明白为啥这么做，和x86实现中断没什么本质区别

#### chibinz:

+ 今天给rCore-Tutorial提交了第一个pull requst，写了一个`dbg!`macro。这个宏还是蛮有用的，可以显示行号，文件名，同时返回变量的值，自己平时用的蛮多的。



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab1_practice.md#%E5%AE%9E%E9%AA%8C%E9%A2%98)
+ [stellarkey](https://github.com/stellarkey/os_summer_project/tree/master/rcore_project#%E5%AE%9E%E9%AA%8C%E4%B8%80%E5%AE%9E%E9%AA%8C%E9%A2%98)
+ [chibinz](https://github.com/chibinz/rCoreLab/blob/ce554c15c2e27e732b285803085d36d3da8f4c20/doc/lab1.md#lab-1-%E5%AD%A6%E4%B9%A0%E8%AE%B0%E5%BD%95)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E4%B8%80%E4%B8%AD%E6%96%AD)



## Lab 2

### 讨论

#### SKTT1Ryze:

+ 我们之前在 C/C++ 语言等中使用过`malloc/free`等动态内存分配方法，与在编译期就已完成的静态内存分配相比，动态的内存分配可以根据程序运行时状态修改内存申请的时机及大小，显得更为灵活，但是这是需要操作系统的支持的，同时也会带来一些开销。
  在`rCore`中也需要动态内存分配，比如`Box<T>`，`Rc<T>`和`Vec`等等。
+ Lab2 主要是实现了操作系统的内存管理模块，通过这次实验，最大的收获是了解了如何在 Rust 中使用 trait 对某个结构体的行为进行抽象。

#### wfly1998：

+ （前面几个Lab的代码还可以直接用的，最多加几句`use`语句就好，这里的缺少好多

#### stellarkey:

+ 涉及到的代码太多了，一步一步手写代码实现实在是难以实现。因此开始大量copy里面的板子，比如algorithms以及各个模板。先跑通再说。如果这里不符合要求，那么也只能如此了。
+ 好了，分析了一下已有的线段树实现，发现并没有想象中那么难，自己重新实现了一下，效果还可以。



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab2.md#lab-2-%E5%AE%9E%E9%AA%8C%E8%AE%B0%E5%BD%95)
+ [chibinz](https://github.com/chibinz/rCoreLab/blob/ce554c15c2e27e732b285803085d36d3da8f4c20/doc/lab2.md#lab-2-%E5%AD%A6%E4%B9%A0%E8%AE%B0%E5%BD%95)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E4%BA%8C%E5%86%85%E5%AD%98%E5%88%86%E9%85%8D)
+ [shiweiwww](https://github.com/shiweiwww/rcore/tree/master/labs/lab2#lab2%E5%AE%9E%E9%AA%8C%E6%8A%A5%E5%91%8A)



## Lab 3

### 讨论

#### SKTT1Ryze:

+ 到目前为止，我们简易的操作系统还只是一个内核在执行，还没有多任务的概念。在现代的操作系统中，为了让其他的程序能方便的运行在操作系统上，需要完成的一个很重要的抽象是「每个程序有自己的地址空间，且地址空间范围是一样的」，这将会减少了上层程序的大量麻烦，否则程序本身要维护自己需要的物理内存，这也会导致极大程度的不安全。
  操作系统为了解决这个问题，使用了虚拟地址。
+ 本次实验我们利用页表实现了虚拟地址到物理地址的映射和内核空间段的重映射。



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab3_practice.md#%E5%AE%9E%E9%AA%8C%E4%B8%89%E8%99%9A%E5%AE%9E%E5%9C%B0%E5%9D%80%E8%BD%AC%E6%8D%A2)
+ [chibinz](https://github.com/chibinz/rCoreLab/blob/ce554c15c2e27e732b285803085d36d3da8f4c20/doc/lab3.md#lab-3-%E5%AD%A6%E4%B9%A0%E8%AE%B0%E5%BD%95)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E4%B8%89%E8%99%9A%E5%AE%9E%E5%9C%B0%E5%9D%80%E8%BD%AC%E6%8D%A2)
+ [shiweiwww](https://github.com/shiweiwww/rcore/tree/master/labs/lab3#lab3%E5%AE%9E%E9%AA%8C%E6%8A%A5%E5%91%8A)



## Lab 4 (上)

### 讨论

#### SKTT1Ryze:

+ 进程是资源的分配单位，线程是CPU的基本调度单位。
  出于OS对计算机系统精细管理的目的，我们通常将“正在运行”的动态特性从进程中剥离出来，这样的一个借助 CPU 和栈的执行流，我们称之为线程 (Thread) 。一个进程可以有多个线程，也可以如传统进程一样只有一个线程。
+ 本次实验主要是理清线程和进程的概念，通过设置`Context`构造一个线程的状态抽象描述，实现内核栈和调度器。

#### wfly1998:

+ 另外，Lab4的坑好多，我不知道踩哪去了，我整的代码老是报错，线程启动不起来。最后我直接把 `rCore-Tutorial` 的代码复制过来，把 `drivers`、`fs`、`kernel` 这几个目录删掉，然后把耦合的代码也全部删掉，再修修补补，`make run`，线程就启动起来了。
+ 正在排查到底是哪出的问题……一个个文件 `diff` 好麻烦

#### chibinz:

+ 今天尝试着在做lab 4，不是很顺利。tutorial里面的代码和仓库里面的代码还是有很多不一样的。自己修好编译错误之后操作系统就一直提示StoreFault，有很大的可能是给线程初始化页表的时候出了一些问题。
+ 下午提交了一个pr，并在issue中回答了一个关于lab2内存分配的问题。当时自己在这里也思考了很久，希望对提问题的同学有所帮助。



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab4_practice_1.md#%E5%AE%9E%E9%AA%8C%E5%9B%9B%E4%B8%8A%E7%BA%BF%E7%A8%8B)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E5%9B%9B%E4%B8%8A%E7%BA%BF%E7%A8%8B)
+ [shiweiwww](https://github.com/shiweiwww/rcore/tree/master/labs/lab4#lab4%E5%AE%9E%E9%AA%8C%E6%8A%A5%E5%91%8A)



## Lab 4 (下)

### 讨论

#### chibinz:

+ OS设计的初衷是为了实现multitasking，最早的雏形是time sharing system。简单的来说，就是把cpu时间进行分割，分配给不同的程序。

+ 最容易想到的multiplexing算法是round robin，把cpu时间均分，但是这显然不够灵活，不能满足日记增长复杂的需求。并且同时运行的两个程序，他们在运行过程中不能访问重叠的地址，这意味着说为了实现多任务需要重新编译用户程序。

+ 为了应对第一个问题，OS提出了scheduler，和thread的概念，把运行中的程序抽象成thread，方便scheduler进行调度。thread随时都能够被中断，中断后能够恢复原来的状态，这里存储的状态就是Context（上下文）。

+ 为了应对第二个问题，OS提出了虚拟地址的概念，不同程序的的虚拟地址可以有重叠，但是虚拟地址最后映射到的物理地址一定是没有冲突的。每次访问内存的话都做一次翻译软件上实现会十分耗时，因此虚拟地址到物理地址的翻译就交给了硬件（mmu）。

#### JohnWestonNull:

+ 线程是 OS 中**程序运行**的最小单位. rCore 的线程其实也挺简单的, 实现了 创建-准备-停止-启动 的生命周期循环.
+ 准备线程的过程很简单, 将存储的上下文取出, 放入内核栈中, 这样在 `__restore` 时会直接将该线程上下文还原。停止线程的方式正好相反, `__interrupt` 会把上下文作为参数传入, 线程直接存起来就好。



### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab4_practice_2.md#%E5%AE%9E%E9%AA%8C%E5%9B%9B%E4%B8%8B%E7%BA%BF%E7%A8%8B%E8%B0%83%E5%BA%A6)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E5%9B%9B%E4%B8%8B%E7%BA%BF%E7%A8%8B%E8%B0%83%E5%BA%A6)





## Lab 5

### 讨论

#### SKTT1Ryze:

+ 文件系统是操作系统用于明确存储设备（常见的是磁盘，也有基于NAND Flash的固态硬盘）或分区上的文件的方法和数据结构；即在存储设备上组织文件的方法。
+ 本次实验主要是在 QEMU 上挂载了存储设备，并实现了 virtio 驱动和一个简单的文件系统。

#### chibinz:

+ Lab 5的代码量不是很大，今天也已经完成了一部分，文件系统因为是调用外部的crate，只有寥寥几行，重点是理解virtio以及设备树的遍历。
+ 相比于做Lab 4时的艰难，Lab 5实现的整个过程就很顺畅，基本上没有遇到什么问题。因为很快就把实验内容和学习记录写完了，剩下的时间回顾了一下这三个周学习rCore的成果，还是比较有收获的。在DailySchedule里面添加了日历和汇总，方便别的同学查看。



## Lab 6

### 讨论

#### SKTT1Ryze:

+ 本次实验将为用户搭建程序开发框架。
+ 这次实验主要是成功单独生成了 ELF 格式的用户程序，并打包进文件系统中，同创建并运行了用户进程。另外，我们还实现了一些系统调用为用户程序提供服务。

#### wfly1998:

+ 六个 Lab 就这样完成了，感觉并没有很好地理解，因为很多重要部分都被封装在库里了
+ 折腾线段树折腾了一整天，挺累的，但是对内存分配有了更深的理解，也学会了如何使用 Rust 写复杂的数据结构

#### stellarkey: 

+ 今天是截止日期了。昨天肝了一晚上，睡眼朦胧。不过已经尽力去做了。也没有什么遗憾。期望以后还有机会继续学自己想学的东西。比如，心心念念的 Go 语言，却没有系统地抽时间学。。

#### chibinz:

+ Lab 6其实做了3件事情：一是用户程序的环境搭建；二是操作系统调用的实现；三是标准输入和输出流。要说三者是独立的也不不完全准确。用户程序与操作系统沟通的桥梁是系统调用。标准输入和标准输出在*nix系统中被抽象成一个文件，在read/write的时候需要特殊处理。但是如果能够把他们放在单独的章节讲的话可能会更清晰一些。

  

### 实验参考答案

+ [wfly1998](https://github.com/wfly1998/DailySchedule/blob/master/06-rcore-lab-notes/lab6_practice.md#%E5%AE%9E%E9%AA%8C%E5%85%AD%E7%B3%BB%E7%BB%9F%E8%B0%83%E7%94%A8)
+ [yunwei37](https://github.com/yunwei37/os-summer-of-code-daily/blob/master/labs/report.md#%E5%AE%9E%E9%AA%8C%E5%85%AD%E7%B3%BB%E7%BB%9F%E8%B0%83%E7%94%A8)
+ [shiweiwww](https://github.com/shiweiwww/rcore/tree/master/labs/lab6#lab6%E5%AE%9E%E9%AA%8C%E6%8A%A5%E5%91%8A)
+ [leonardodalinky](https://github.com/leonardodalinky/DailySche/blob/master/myRcore/%E5%AE%9E%E9%AA%8C%E9%A2%98%E6%B1%87%E6%80%BB.md#lab-6)

