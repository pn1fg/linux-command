# linux-command

## 用 Rust 打造 Linux 命令行工具

得益于各种零开销抽象、深入到底层的优化潜力、优质的标准库和第三方库实现，Rust 具备非常优秀的性能，和 C、C++ 是 一个级别。同时 Rust 有一个极大的优点：只要按照正确的方式使用 Rust，无需性能优化，就能有非常优秀的表现，不可谓不惊艳。

命令行工具不失为学习 Rust 的一个好方法，可以使我们能够快速的上手 Rust 而又不会感到枯燥无味。

# 前置

## 1.1 安装

第一步是安装 Rust。我们可以通过 rustup 下载 Rust，这是一个管理 Rust 版本和相关工具的命令行工具
Linux 和 MacOS 上可以输入下列命令：

```shell
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

此命令下载一个脚本并开始安装 rustup 工具，这会下载最新稳定版的 rustup。如果安装成功，将会出现如下内容：

```shell
Rust is installed now. Great!
```

Windows 上安装 rustup 稍微有点麻烦
在 windows 上，前往 https://www.rust-lang.org/install.html 并按照说明安装 Rust。

## 1.2 更新与卸载

通过 rustup 安装的 Rust 更新版本很简单，只需输入如下命令即可：

```shell
$ rustup update
```

若要卸载 rustup 与 Rust，请在命令行中输入如下命令即可：

```shell
$ rustup self uninstall
```

## 1.3 编译与运行

当你编写了一个 `main.rs` 的源码以后，我们该如何运行这个 Rust 程序呢？
`rsutc` 这个 Rust 中编译单个程序的工具，格式如下：

```shell
$ rustc main.rs
```

编译完以后它会像 `gcc` 与 `clang` 类似，输出一个二进制可执行文件，我们直接运行即可

## 1.4 Cargo

Cargo 是 Rust 的构建系统和包管理器。它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库。
查看 Cargo 版本号：

```shell
$ cargo --version
```

使用 Cargo 创建一个新项目：

```shell
$ cargo new hello_cargo
$ cd hello_cargo
```

第一行作用是创建了一个名为 `hello_cargo` 的项目和目录。进入 `hello_cargo` 后并列出目录，可以看到当前目录下拥有一个 `src` 目录与一个 `Cargo.toml` 文件，在 `src` 中则有一个 `main.rs` 的源程序文件，我们的程序源码写在 `main.rs` 中即可，`Cargo.toml` 中放的是你的程序中所用到的所有的依赖库

使用 Cargo 构建并运行一个项目：

```shell
$ cargo build
```

在 `hello_cargo` 目录下运行上述的命令后，会构建一个 `target` 目录，我们的二进制可执行文件则在 `target/debug` 目录下，我们只需进入该目录中直接运行即可

上述方法需要两步操作才能运行一个 Rust 程序，更快的方法如下：

```shell
$ cargo run
```

在 `hello_cargo` 目录下执行完上述命令后即可直接在命令行中输出运行结果

# 主体

## 解析命令参数

### 1.`cat`

NAME

```shell
cat - concatenate files and print on the standard output
```

SYNOPSIS

```shell
cat [OPTION]... [FILE]...
```
