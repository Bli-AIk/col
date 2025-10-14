# Configurable Open Language

[![license](https://img.shields.io/github/license/Bli-AIk/col
)](LICENSE)
<img src="https://img.shields.io/github/repo-size/Bli-AIk/col.svg"/>
<img src="https://img.shields.io/github/last-commit/Bli-AIk/col.svg"/>
[![codecov](https://codecov.io/gh/Bli-AIk/col/graph/badge.svg?token=98QA8G15H1)](https://codecov.io/gh/Bli-AIk/col)

<img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" />

> **Status**: 🚧 Initial iteration (features and structure may change frequently)

**Configurable Open Language (COL)** is an open-source scripting language inspired by **Gamemaker Language (GML)**.

| English         | Chinese                     |
|-----------------|-----------------------------|
| English Version | [简体中文](./readme_zh-hant.md) |

## Introduction

COL aims to be an open-source alternative to GML (language level only), allowing GML users to feel “at home” in an open-source environment.

**Its target audience is the general public**:

* If you are an engine/framework developer, COL can be integrated to provide a GML scripting experience;
* If you are a regular developer, you can use COL directly and contribute to the community.

## Design Goals

COL was designed with these objectives in mind — **GML compatibility, extensibility, and configurability**.

No matter how it evolves, these goals will always guide the development and philosophy of COL.

1. **GML Compatibility**: Adapt to various GML versions and follow major updates as closely as possible, maintaining consistent syntax and usage.
2. **Extensibility**: Provide interfaces for engine-bound methods or syntactic sugar to integrate with other engines/frameworks.
3. **Configurability**: Allow customization of interface names, methods, or syntactic sugar to fit specific GML game frameworks.
   Additionally, some features not present in GML (e.g., static typing, classes/inheritance/polymorphism) may be added and configurable (TBD).

## Core Philosophy

### Configurable

COL allows highly configurable interfaces and methods, making it adaptable to different engines and frameworks while providing GML users with a familiar development experience.

### Open

COL is fully open-source under the **LGPL v3** license. All game developers and open-source enthusiasts are welcome to participate: submit issues or pull requests, suggest features, or share experiences in the community.

### Language

COL is a scripting language designed to give GML developers a sense of belonging in the open-source world while offering highly configurable language features.

## License

COL uses a dual licensing model:

### Open-Source License (LGPL-3.0)

* You can use COL in closed-source projects as long as you do not modify its source code.
* If you modify COL (e.g., add/change core modules), you must release those modifications under the same license (LGPL-3.0).
* Your own game/application code can remain closed-source.

### Commercial License

If you want to modify COL and keep your modifications closed-source, or include COL in projects that do not accept LGPL, you can contact me for a commercial license.