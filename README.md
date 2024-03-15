<div align="center">
<h1><code>opt_args</code>: Optional arguments for functions and structs in Rust</h1>
<p>
    <a href="https://crates.io/crates/opt_args"><img alt="Crates.io" src="https://img.shields.io/crates/v/opt_args?logo=rust"></a>
    <a href="https://crates.io/crates/opt_args"><img alt="Crates.io" src="https://img.shields.io/crates/d/opt_args?logo=rust"></a>
    <a href="https://docs.rs/opt_args"><img alt="docs.rs" src="https://img.shields.io/docsrs/opt_args?logo=rust"></a>
</p>
</div>

This crate allows you to auto-generate macros to call functions and instantiate structs with default named arguments

Import the macro and use it on a function or struct like this

```rust
use opt_args::opt_args;

opt_args! {
    fn f(a: i32, b: u64, c: &str = "default", d: (u8,)?, e: Option<[f64; 42]>?) {
        println!("a = {}, b = {}, c = {}, d = {:?}, e = {:?}", a, b, c, d, e);
    }
}

opt_args! {
    #[derive(Debug)]
    struct S {
        x: i32,
        y: i32 = 1,
        z: i32?,
        next: Option<Box<S>>?,
    }
}
```

To auto-generate macros that can be used like this

```rust
fn main() {
    f!(1, 2, d = (1,));             // prints `a = 1, b = 2, c = default, d = (1,), e = None`
    println!("{:?}", S!(4, z = 5)); // prints `S { x: 4, y: 1, z: 5, next: None }`
}
```

Full documentation [here](https://docs.rs/opt_args/latest/opt_args/)
