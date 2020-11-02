# ex2 : Rust包管理 - Cargo

像ex1中的方式写少量的代码还可以，面对大型项目时就无能为力了。更何况现在编程不是一个人的事情，我们要学会站在巨人的肩膀上。因此包管理的需要和概念应需而生：
+ 可以使用其他人开发的模块，并且具有版本控制，可复现
+ 自己创作的模块可以供别人方便地调用

Rust的包管理体系做得很干净简洁，名为Cargo。(从命名上就可以看出设计者希望Rust具有良好的拓展性了)
由于是静态语言，所以只要管理好源代码，下载到本地后再编译就好了。只要rustc没有问题，编译的参数正确，那么必然可以得到相同的可执行文件。

在包管理这件事上，python经历了pip、venv、再到pipenv的过程（然而还是很不方便），而且需要成熟管理虚拟环境的开销；nodejs社区较为混乱，包质量良莠不齐、更新不及时，且不同平台兼容性经常出问题...相比这些语言，rust从设计之初就遵循了比较好的规范，而且静态语言的优势也得以完全体现。(目前可能只有golang做得差不多，不过golang的go.mod也是不久前才有的)

从命令行进入工作目录，使用cargo来创建一个新的Rust项目：
```bash
# 默认创建二进制应用，如果是创建库则加上 --lib
cargo new ex2

# 进入目录
cd ex2

# 查看项目结构
tree
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```

可以看到代码被管理在src文件夹下，由于我们创建的是应用，所以入口文件是`main.rs`。另一个文件`Cargo.toml`则是配置文件，同时也描述了包的依赖。目前并不需要详细了解，在后面涉及到依赖第三方包的时候再做说明。更方便的是，cargo在发现当前路径不在被git管理时还会自动创建git仓库，并且添加了`.gitignore`文件。

此时查看`main.rs`的内容，会发现就是ex1里面的hello world程序。这是cargo创建项目时默认的模版。学习新技术的第一步当然就是运行了，让我们快点使用cargo来运行项目吧：
```bash
cargo run

# 输出内容
   Compiling ex2 v0.1.0 (/Users/markjiyuan/Desktop/ex2)
    Finished dev [unoptimized + debuginfo] target(s) in 0.83s
     Running `target/debug/ex2`
Hello, world!
```

一串提示信息后应当可以看到输出到控制台的Hello World，这说明你的rust工具链一切正常。仔细观察输出的提示信息，没有一句多余的话。
+ `Compiling`意味着开始编译。写明编译的对象，版本，文件位置。
+ `Finished dev`表示编译通过，已经生成了可执行文件。`dev`表明这个构建是开发阶段的，而不是最终的`release`版本。通常来说`dev`编译更迅速，输出更多调试信息（有时也跟日志等级有关），`release`则与之相对。日常开发就用`dev`即可，真正要发布了再进行`release`构建。
+ `Running`表示即将运行刚刚生成的可执行文件。

此时再次观察项目结构: (`target/debug`文件夹未展开)
```
├── Cargo.lock
├── Cargo.toml
├── src
│   └── main.rs
└── target
    ├── CACHEDIR.TAG
    └── debug

3 directories, 4 files
```

其中`Cargo.lock`是Cargo根据`Cargo.toml`在编译时自动生成、维护的，为了确定下载的到底是某个包的哪一个版本，Cargo自动确定，不要手动进行更改。可以看到除此之外还多出来了一个`target`文件夹，里面还有一个`debug`文件夹。毫无疑问，我们刚才生成的可执行文件就在`debug`文件夹中了。

当然我们也可以只是构建而不运行：
```bash
# dev
cargo build

# release
cargo build --release
```

## 命令清单

```bash
# 
```


## 补充说明

`Cargo`作为一个成熟的包管理工具，功能和命令都相当地完善。作为新手入门教程，我们不希望初学者被大量不重要的东西吓到，而关注点没有放到真正重要的Rust学习上。固然设计思想值得了解，但Cargo毕竟只是工具。所以本章涉及的命令较少，在后续的章节中使用到时会再做说明。

有问题的话，推荐去看Cargo的[官方文档](https://doc.rust-lang.org/cargo/index.html)。
