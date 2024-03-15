# `opt_args` CHANGELOG

## 2.0.0

Completely renewed the macro. New features:

- **new** special syntax for optional arguments, with ant without default value
- **new** option `shuffle` instead of the double macro `opt_args` and `opt_args_ord`
- **new** option `non_export` to disable `#[macro_export]`
- **new** option `rename` to rename the macro instead of using the same name as the original function/struct

## 0.1.0

Added new macro `opt_args_ord` to make the compilation faster. Added content to `README` and tests.

## 0.0.1

The first uploaded version for testing functionalities of [crates.io](https://crates.io/).
It's fully functional but is missing a `README` and the `tests` directory.
