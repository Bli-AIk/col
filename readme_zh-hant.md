# Configurable Open Language

[![license](https://img.shields.io/github/license/Bli-AIk/col
)](LICENSE)
<img src="https://img.shields.io/github/repo-size/Bli-AIk/col.svg"/>
<img src="https://img.shields.io/github/last-commit/Bli-AIk/col.svg"/>
[![codecov](https://codecov.io/gh/Bli-AIk/col/graph/badge.svg?token=98QA8G15H1)](https://codecov.io/gh/Bli-AIk/col)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **状态**：🚧 初始迭代中（功能与结构可能频繁变动）

**Configurable Open Language (COL)** 是一个受 **Gamemaker Language (GML)** 启发的开源脚本语言。

| 英语                     | 简体中文 |
|------------------------|------|
| [English](./readme.md) | 简体中文 |

## 简介

COL 旨在成为 GML（仅语言层面）的开源替代品，让 GML 的使用者在开源环境下也能感受到“宾至如归”。

**其目标用户面向普罗大众**：

- 如果你是引擎/框架作者，COL 可用于集成 GML 脚本体验；
- 如果你是普通开发者，也可以直接使用 COL 并参与社区贡献。

## 设计目标
COL 的设计初衷就是为了实现这些目标——**兼容 GML、可扩展和可配置**。

无论未来如何演进，这些目标都将始终指导着 COL 的开发方向和理念。
1. **兼容 GML**：适配 GML 各版本，并尽量跟随大版本更新，保持基本语法和用法一致。
2. **可扩展**：对于与引擎深度绑定的方法或语法糖，提供接口以接入其他引擎/框架。
3. **可配置**：允许自定义接口名称、方法或语法糖，以适配特定 GML 游戏框架。  
   同时，可能提供并允许配置一些 GML 本身不具备的特性，例如静态类型、类/继承/多态等（待定）。

## 核心理念

### Configurable

COL 允许高度配置接口和方法，使其可以适配不同引擎和框架，也为 GML 使用者提供熟悉的开发体验。

### Open

COL 完全开源，基于 **LGPL v3** 许可证。欢迎所有游戏开发爱好者、开源软件爱好者参与：提交 Issue 或 Pull
Request，提供功能建议，或在社区交流经验。

### Language

COL 是一门脚本语言，其使命是让 GML 开发者在开源世界中有归属感，同时提供高度可配置的语言特性。


## 许可证
COL 采用双重许可模式：

### 开源许可证（LGPL-3.0）
- 只要您不修改 COL 的源代码，就可以在闭源项目中使用 COL。
- 如果您修改了 COL（例如添加 / 更改核心模块），您必须在相同的许可证（LGPL-3.0）下发布这些修改。
- 您自己的游戏 / 应用程序代码可以保持闭源。

### 商业许可
如果您希望修改 COL 并保持这些修改闭源，或在不接受 LGPL 的项目中包含 COL，您可以联系我以获取商业许可。