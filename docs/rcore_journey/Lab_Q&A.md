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



## Lab 1

### 补充说明

1. “进入中断处理流程”一节中，添加了use riscv::register::scause::Scause;，否则handle_interrupt函数的定义会报找不到Scause类型。

2. os/src/interrupt/handler.rs 中引入缺少一部分，因此会编译错误，加上以下部分即可。最重要的还是要仔细阅读编译器的报错信息，定位问题并解决。

   ~~~Rust
   use riscv::register::{
       scause::{Exception, Interrupt, Scause, Trap},
       sie, stvec,
   };
   ~~~
   
3. “进入中断处理流程”一节中，在给main函数加入`interrupt::init()`时，如果直接整段复制有可能会把`#[no_mangle]`覆盖掉，进而导致链接错误。

4. "时钟中断"一节中，在最后给handler.rs添加时钟中断的处理时，要记得从`super`里引入timer，即：

   ~~~rust
   use super::timer;
   ~~~

5. "时钟中断"一节中，`main.rs`最后要更改将rust_main函数返回类型修改一下。

   ~~~rust
   // 原本
   pub extern "C" fn rust_main() -> ! {
   	// 初始化各种模块
       interrupt::init();
       
       unsafe {
           llvm_asm!("ebreak"::::"volatile");
       };
   
       unreachable!();
   }
   
   
   // 修改后
   pub extern "C" fn rust_main() {
   	// 初始化各种模块
       interrupt::init();
       
       unsafe {
           llvm_asm!("ebreak"::::"volatile");
       };
   
       // unreachable!();
   }
   ~~~



## Lab 2

### 补充说明

1. 从这一章开始，编译错误、导入错误的提示变得更少了，需要自己debug，或者参考完成后的版本。

2. "动态内存分配"一节中，并没有提示要创建`memory/mod.rs`，需要创建并在`main.rs`中引入。

3. 在 `os/main.rs` 中添加对 Rust 新特性 `alloc_error_handler` 的引用。

   ~~~rust
   //!
   //! - `#![feature(alloc_error_handler)]`
   //!   我们使用了一个全局动态内存分配器，以实现原本标准库中的堆内存分配。
   //!   而语言要求我们同时实现一个错误回调，这里我们直接 panic
   #![feature(alloc_error_handler)]
   ~~~

4. 在`os/main.rs`中添加：

   ~~~rust
   extern crate alloc;
   ~~~

5. 在`memory/heap.rs`的开头添加：

   ~~~rust
   use super::config::KERNEL_HEAP_SIZE;
   use buddy_system_allocator::LockedHeap;
   ~~~

   在`Cargo.toml`中的dependency中添加：

   ~~~toml
   [dependencies]
   buddy_system_allocator = "0.3.9"
   ~~~

6. 注意`memory/mod.rs`里面的引入需要使用`pub`关键字来修饰。

   ~~~rust
   pub mod address;
   pub mod config;
   pub mod frame;
   pub mod heap;
   pub mod range;
   ~~~

7. 

