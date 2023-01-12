# 基于 WebAssembly 的贪吃蛇小游戏

## 环境

- Rust
- Node.js
- wasm-pack

## 项目结构

Rust 项目和一个 vite 项目同时置于一个目录中。`src` 目录为 Rust 代码，`www` 被设为 vite 项目的根目录。

程序启动通过 npm 脚本，这里使用了 `concurrently` 同时运行两个命令，并使用 `nodemon` 监听 Rust 代码是否有修改，如果有修改就重新打包到 `pkg` 目录，由于 vite 项目启用了 pnpm 工作空间，前端可以始终拿到最新的 webassembly 产物，从而有了便捷的开发环境，不再需要改完再手动执行命令了。

## 代码

最关键的代码就是 `src/lib.rs` 和 `www/index.ts` 了。代码整体是很清晰的很容易读懂，关键的计算和逻辑都会给上详细的注释。

## 参考链接

- [从零开始创建一个 WebAssembly 游戏](https://www.bilibili.com/video/BV19a41127Dq)

感谢 up 主[原子之音](https://space.bilibili.com/437860379)的视频教程，但实际上不推荐去看原视频。视频本来可以做的十分精简，但却过于啰嗦了（不想评价过多，可以去试试看）。整体代码难度不大，自己过一遍代码即可。
