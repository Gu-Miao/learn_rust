# cargo 相关

- [文档注释，模块、包注释，pub use](crate1/src/lib.rs)
- [opt-level](crate1/Cargo.toml)
- [workspace](Cargo.toml)

使用 token 登录，token 保存在 `~/.cargo/credentials`

```
cargo login token
```

发布

```
cargo publish
```

撤回版本

```
cargo yank --vers 1.0.0 # 撤回 1.0.0 版本
cargo yank --vers 1.0.0 --undo #取消 1.0.0 版本的撤回
```

在工作空间中运行指定 binary crate (可以运行 `cargo run -p crate1` 试试)

```
cargo run -p crate_name
```

整个工作空间中只会有一个 Cargo.lock，可以保证工作空间中所有 crate 使用的依赖的版本都相同

`cargo install` 可以安装二进制文件到 `$HOME/.cargo/bin` 目录下，需要确保目录在环境变量中

如果 $PATH 中存在一个二进制文件是 cargo-something，那么你可以像运行 cargo 的子命令一样运行它：

```
cargo something
```

这意味着我们可以安装各种各样的拓展，然后像内置工具一样运行

可以使用 `cargo --list` 列出所有的自定义命令
