---
description: 在跟随第三版tutorial的过程中，可能会碰到一些问题。本文档总结了tutoria中现存的一些问题，并附有往届同学对该问题的一些见解。既可以将本文档当作tutorial的订正，也可以认为是个讨论区。
---

# Lab搭建Q&A

## 环境部署

### 补充说明

1. 





## Lab 0

### 补充说明

1. "使用QEMU运行"小节----makefile直接复制的话会报错，将各命令前的空格替换为TAB就好了。这里建议大家自学下Makefile的知识。
2. 



## Lab 1

### 补充说明

1. “进入中断处理流程”一节中，添加了use riscv::register::scause::Scause;，否则handle_interrupt函数的定义会报找不到Scause类型。

2. os/src/interrupt/handler.rs 中引入缺少一部分，因此会编译错误，加上以下部分即可。最重要的还是要仔细阅读编译器的报错信息，定位问题并解决。

   ~~~Rust
   use super::timer;
   use riscv::register::{
       scause::{Exception, Interrupt, Scause, Trap},
       sie, stvec,
   };
   ~~~

3. 
