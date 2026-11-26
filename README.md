# raiment <!-- omit in toc -->

Application-level helpers influenced by C-family scope techniques, for Rust.

![Language](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
[![License](https://img.shields.io/badge/License-BSD_3--Clause-blue.svg)](https://opensource.org/licenses/BSD-3-Clause)
[![GitHub release](https://img.shields.io/github/v/release/synesissoftware/raiment.svg)](https://github.com/synesissoftware/raiment/releases/latest)
[![Last Commit](https://img.shields.io/github/last-commit/synesissoftware/raiment)](https://github.com/synesissoftware/raiment/commits/master)
![MSRV](https://img.shields.io/badge/MSRV-1.74-lightgrey)
[![CI](https://github.com/synesissoftware/raiment/actions/workflows/ci.yml/badge.svg)](https://github.com/synesissoftware/raiment/actions/workflows/ci.yml)
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

**raiment** is application-level helpers influenced by C-family scope techniques (such as RAII guards), for Rust. The library is `no_std` and depends only on `core`.

The first helper is a block-scoped cleanup guard. `on_drop!` runs a closure when its binding leaves the enclosing block, including on unwind. The design follows **STLSoft**'s `scoped_lambda`, and is related to `scoped_handle<>`.



## Developer Experience

Rust **1.74** or newer is required. From the repository root:

```sh
cargo test
```

The simplest use is `on_drop!`: a block, a guard, a later statement, then the proof that the closure has run.

```rust
let ran = std::cell::Cell::new(false);

{
    raiment::on_drop! {
        ran.set(true);
    }

    assert!(!ran.get());
}

assert!(ran.get());
```

Further reading: [**EXAMPLES.md**](./EXAMPLES.md), [**CHANGES.md**](./CHANGES.md), and [**TODO.md**](./TODO.md).



## Installation

Reference in **Cargo.toml** in the usual way:

```toml
raiment = { version = "0.0.1" }
```



## Components

* `on_drop` — builds a named guard;
* `on_drop!` — binds a guard for the enclosing block;
* `OnDropGuard` — the guard type;
* `OnDropGuard::defuse` — drops a named guard without running the closure;



## Examples

See [**EXAMPLES.md**](./EXAMPLES.md). The **on_drop** program is [examples/on_drop.rs](./examples/on_drop.rs).



## Project Information



### Where to get help

Use the [raiment issue tracker](https://github.com/synesissoftware/raiment/issues).



### Contribution guidelines

Contributions should follow the documented public API (`on_drop`, `on_drop!`, and `OnDropGuard::defuse`) and the crate's packaging constraints.



### Minimum Supported Rust Version (MSRV)

The declared Minimum Supported Rust Version (MSRV) for **raiment** is **1.74**.

This MSRV applies to the library crate itself. The library is `no_std` and declares no runtime or build dependencies.



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

The design follows **STLSoft**'s `scoped_lambda`, and is related to `scoped_handle<>`. No related Rust implementation is recorded.



### License

**raiment** is released under the 3-clause BSD license. See [LICENSE](./LICENSE) for details.


<!-- ########################### end of file ########################### -->
