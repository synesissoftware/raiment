# raiment - Changes <!-- omit in toc -->


## 0.0.1 - 26th November 2026

* Added the `on_drop` guard, which runs its closure when the binding leaves the enclosing block, including on unwind;
* Added the `on_drop!` macro, which binds that guard for the rest of the block;
* Added `OnDropGuard::defuse`, which drops a named guard without running the closure;
* Declared the library `no_std`; it depends only on `core`;
* Added GitHub Actions CI in **.github/workflows/ci.yml**, on Linux, macOS, and Windows;


## 0.0.0 - 17th November 2026

* Created the **raiment** scaffold with an empty public API;


<!-- ########################### end of file ########################### -->
