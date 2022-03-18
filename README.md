# opt_args: Optional arguments for functions and structs in Rust

This crate allows you to auto-generate macros to call functions and instantiate structs with default named arguments

Import the macro and use it on a function or struct definition like this
```rust
use opt_args::opt_args;


#[opt_args(c = "default", d, e)]
fn f(a: i32, b: u64, c: &str, d: (i32, ), e: Option<[f64; 42]>) {
    println!("a = {}, b = {}, c = {}, d = {:?}, e = {:?}", a, b, c, d, e);
}


#[derive(Debug)]
#[opt_args(y = 1, z, next)]
struct S {
    pub x: i32,
    y: i32,
    z: i32,
    next: Option<Box<S>>,
}
```
To auto-generate macros that can be used like this
```rust
fn main() {
    f!(1, 2, d = (1,));
    println!("{:#?}", S!(4, z = 5));
}
```
