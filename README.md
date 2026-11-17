# raiment <!-- omit in toc -->

Application-level helpers influenced by C-family scope techniques, for Rust.

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/raiment.svg)](https://github.com/synesissoftware/raiment/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/raiment)](https://github.com/synesissoftware/raiment/commits/master)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![docs.rs](https://docs.rs/raiment/badge.svg)](https://docs.rs/raiment)


## Table of Contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Developer Experience](#developer-experience)
- [Installation](#installation)
- [Components](#components)
- [Examples](#examples)
- [Project Information](#project-information)
  - [Where to get help](#where-to-get-help)
  - [Contribution guidelines](#contribution-guidelines)
  - [Minimum Supported Rust Version (MSRV)](#minimum-supported-rust-version-msrv)
  - [Dependencies](#dependencies)
    - [Efferent (fan-out)](#efferent-fan-out)
      - [Runtime Dependencies](#runtime-dependencies)
      - [Build Dependencies](#build-dependencies)
      - [Development Dependencies](#development-dependencies)
    - [Afferent (fan-in)](#afferent-fan-in)
  - [Related projects](#related-projects)
  - [License](#license)


## Introduction

**raiment** is reserved for application-level helpers influenced by C-family scope techniques (such as RAII guards). This repository is that Rust crate.

No helpers are landed. The public surface is intentionally empty.



## Developer Experience

This crate is a scaffold. There is no sample call, because there is no public API.

Rust **1.74** or newer is required. From the repository root:

```sh
cargo test
```

`cargo test` builds the empty library and runs no tests. Further reading: [**CHANGES.md**](./CHANGES.md) and [**TODO.md**](./TODO.md).



## Installation

Reference in **Cargo.toml** in the usual way:

```toml
raiment = { version = "0.0.0" }
```



## Components

No public constants, types, functions, traits, macros, or modules are defined.



## Examples

No examples apply. There is no supported public API.



## Project Information



### Where to get help

Use the [raiment issue tracker](https://github.com/synesissoftware/raiment/issues) for questions about the scaffold.



### Contribution guidelines

Contributions should stay limited to the scaffold, documentation, and packaging until a later task adds a supported public API.



### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **raiment** is **1.74**.

This MSRV applies to the library crate itself. The scaffold declares no runtime or build dependencies.



### Dependencies



#### Efferent (fan-out)

No dependencies are required.



##### Runtime Dependencies

None.



##### Build Dependencies

None.



##### Development Dependencies

None.



#### Afferent (fan-in)

No downstream consumers are recorded.



### Related projects

No related Rust implementation is recorded. The crate is a placeholder for later scope helpers.



### License

**raiment** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
