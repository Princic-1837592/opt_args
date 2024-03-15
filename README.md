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
    fn function(a: i32, b: &str = "default", c: (u8,)?) -> (i32, &str, (u8,)) {
        (a, b, c)
    }
}

opt_args! {
    #[derive(Debug, PartialEq, Eq)]
    struct Struct {
        x: i32,
        y: i32 = 1,
        z: i32?,
        other: Option<Box<Self>>?,
    }
}
```

To auto-generate macros that can be used like this

```rust
fn main() {
    assert_eq!(
        function!(1, b = "not the default"),
        (1, "not the default", (0,))
    );
    assert_eq!(
        Struct!(4, z = 5),
        Struct {
            x: 4,
            y: 1,
            z: 5,
            other: None
        }
    );
}
```

Full documentation [here](https://docs.rs/opt_args/latest/opt_args/)
