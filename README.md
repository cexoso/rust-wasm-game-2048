# Rust webassembly 2048 game

这是一个 demo，用来学习 rust wasm 怎么与前端交互的。

# 安装依赖

项目中使用了 rust 来写 wasm 的代码，所以需要你了解 rust 的一些知识，比如说 cargo.

## rust

安装 wasm-pack

`cargo install wasm-pack`

生成 wasm npm 库
在要目录下运行 `bash bin/build-wasm.sh`

> 这条命令会将 `packages/core` 的代码编译成 npm 包形式的 wasm 代码，并且生成在 `packages/core-wasm`, 前端项目会直接引用 `packages/core-wasm` 包代码.

## 前端项目

项目使用 pnpm 管理，项目结构使用了 Monorepo。

在根目录下使用 `pnpm i` 安装依赖，在此之前，需要确保 `packages/core-wasm` 已经生成，pnpm 会使用链接的方式将 `packages/core-wasm` 安装到 `packages/game2048` 目录下。

`packages/game2048` 目录就是前端的开发目录了，这个项目没有使用前端框架。
