## packages、crates、modules

通过 package、creates、modules 来管理大规模应用，确保模块共享和基础库共享使用统一的规范

## packages 和 crates

* 通过 `Cargo create package_name` 创建的就是 `package`，默认存放的入口文件是 `src/main.rs`
    * package 可以通过 `cargo.toml` 内的 [`dependencies`](https://doc.rust-lang.org/cargo/reference/manifest.html) 指定依赖
* 通过 `cargo new --lib crates_name` 创建的就是 `crate`，默认存放的入口文件是 `src/lib.rs`
    * 可以使用 `cargo` 上现有的 `crate`，也可以通过 `dependencies` 指定 `crate` 内的 `path`，指向本地的 `crate`


## module 

* 使用文件夹的路径作为获取 `module` 将会默认获取目录下的 `mod.rs` 文件


### scope 

`rust` 的 `module` 可以以嵌套的方式存在，支持在同一个 mod 文件中编写嵌套的 mod，也可以通过文件的方式拆分维护 module

> 同一个文件中存放 module

```rust
// restaurant/src/lib.rs
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

> 将 module 拆分成多个文件维护

```rust
// restaurant/src/lib.rs
pub mod front_of_house;

// restaurant/src/front_of_house.mod.rs
pub mod hosting;
pub mod serving;

// restaurant/src/front_of_house.hosting.rs
pub fn add_to_waitlist() {
    print!("hello world mod add_to_waitlist")
}

pub fn seat_at_table() {
    print!("hello world mod seat_at_table")
}


// restaurant/src/front_of_house.serving.rs
pub fn take_order() {
    print!("hello world mod take_order")
}

pub fn serve_order() {
    print!("hello world mod serve_order")
}

pub fn take_payment() {}

```
