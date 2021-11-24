# Using Rust

## Reference

- Learning
  - [The Book](https://doc.rust-lang.org/book/)
  - [Example](https://doc.rust-lang.org/rust-by-example/)
- Ecosystem
  - [StdLib](https://doc.rust-lang.org/std/index.html)
  - [Crates](https://docs.rs/)


## Environment

```shell
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
source ${HOME}/.cargo/env
```

```toml
# ~/.cargo/config
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```


## Install

```shell
# [installation](https://www.rust-lang.org/learn/get-started)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs >rustup-init.sh
chmod +x ./rustup-init.sh
./rustup-init.sh
```
