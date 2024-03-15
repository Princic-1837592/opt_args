//! # opt_args
//!
//! This crate lets you easily derive macros to call functions and instantiate structs
//! without having to specify all of their arguments.
//! Wrap your function or struct inside an [`macro@opt_args`] body
//! to generate a macro that can be called with named optional arguments.
//! ```
//! use opt_args::opt_args;
//!
//! opt_args! {
//!     fn function(a: i32, b: &str = "default", c: (u8,)?) -> (i32, &str, (u8,)) {
//!         (a, b, c)
//!     }
//! }
//!
//! opt_args! {
//!     #[derive(Debug, PartialEq, Eq)]
//!     struct Struct {
//!         x: i32,
//!         y: i32 = 1,
//!         z: i32?,
//!         other: Option<Box<Self>>?,
//!     }
//! }
//!
//! assert_eq!(
//!     function!(1, b = "not the default"),
//!     (1, "not the default", (0,))
//! );
//! assert_eq!(
//!     Struct!(4, z = 5),
//!     Struct {
//!         x: 4,
//!         y: 1,
//!         z: 5,
//!         other: None
//!     }
//! );
//! ```
//!
//! # Using the macro
//! To use the macro, just wrap the target item (function or struct) inside the macro body.
//! Here, you can use a special syntax that lets you easily mark arguments as optional and,
//! additionally, indicate their default value. To mark an argument as optional, put a `?` after the type.
//! To indicate the default value of an optional argument, use the syntax ` = value` after the type.
//! Here is an example:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!         a + b + c
//!     }
//! }
//!
//! let result = f!(1);
//! assert_eq!(result, 1 + 5 + 0);
//! ```
//! In the example above, `b` and `c` are marked as optional. For `b` a default value is given (`5`),
//! while no default value is given for `c`. This means that the value assigned to `c` is the default
//! value for the type `u8`.
//!
//! When listing optional arguments in the item, it's important that all optionals come after the non-optionals.
//! ```compile_fail
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn f(a: u8, b: u8 = 5, c: u8) -> u8 {
//!         a + b + c
//!     }
//! }
//! ```
//! The example above is not valid since argument `c` is not optional,
//! but it comes after `b` which is optional. In this case the macro will result in a compile error.
//!
//! # Calling the function
//! To call the function, simply use the name of the function as a macro and pass first the
//! positional required arguments, then the named optional arguments, like in the following:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!         a + b + c
//!     }
//! }
//!
//! let result = f!(1, c = 3);
//! assert_eq!(result, 1 + 5 + 3);
//! ```
//!
//! # Options
//! ## Order of optionals
//! By default, named arguments must be passed in the same order as they are declared in the item.
//! The following example fails because `a = 1` is passed after `c = 3`,
//! but in the original function `a` comes before `c`:
//! ```compile_fail
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!         a + b + c
//!     }
//! }
//!
//! let result = f!(1, c = 3, b = 0);
//! assert_eq!(result, 1 + 0 + 3);
//! ```
//! This behavior can be changed with the `shuffle` attribute. This attribute allows to call the
//! function with arbitrary order of named arguments:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     #[opt_args(shuffle)]
//!     fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!         a + b + c
//!     }
//! }
//!
//! let result = f!(1, c = 3, b = 1);
//! assert_eq!(result, 1 + 1 + 3);
//! ```
//! <span style="color:red">**IMPORTANT**</span>: this doesn't come without disadvantage:
//! to obtain this result, [`macro@opt_args`] creates a macro that matches any possible
//! permutation of the given optional arguments. When applying the `shuffle` attribute,
//! the number of possible permutations scales in the order of `n!`, where `n` is the number of
//! optional arguments.
//! While macro expansion has no impact on runtime, it may impact compile time
//! with a great number of optionals.
//!
//! ## Export the macro
//! By default, the generated macro is annotated with `#[macro_export]` to make it possible to
//! use it from outside. To change this behavior, use the `non_export` attribute:
//! ```compile_fail
//! mod macros {
//!     # use opt_args::*;
//!     #
//!     opt_args! {
//!         #[opt_args(shuffle, non_export)]
//!         pub fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!             a + b + c
//!         }
//!      }
//! }
//! use macros::f;
//! f!(1);
//! ```
//! ```
//! mod macros {
//!     # use opt_args::*;
//!     #
//!     opt_args! {
//!         #[opt_args(shuffle)]
//!         pub fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!             a + b + c
//!         }
//!      }
//! }
//! use macros::f;
//! f!(1);
//! ```
//! Of course for the macro to work outside the original module, it's needed that the original item
//! is available in the same scope where the macro is used:
//! ```compile_fail
//! mod macros {
//!     # use opt_args::*;
//!     #
//!     opt_args! {
//!         #[opt_args(shuffle)]
//!         fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!             a + b + c
//!         }
//!      }
//! }
//! use macros::f;
//! f!(1);
//! ```
//! In the above example the function macro `macros::f` is reachable, but the function `macros::f`
//! is not.
//!
//! ## Rename the macro
//! It's also possible to give the generated macro a different name than the original item:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     #[opt_args(rename = f_macro)]
//!     fn f(a: u8, b: u8 = 5, c: u8?) -> u8 {
//!         a + b + c
//!     }
//! }
//!
//! let result = f_macro!(1);
//! assert_eq!(result, f(1, 5, 0));
//! ```
//!
//! # Recursion
//! It's also possible to use the generated macro inside the original function:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn f(a: u8 = 10, b: u8 = 1, c: u8 = 2) -> u8 {
//!         if a == 0 {
//!             b + c
//!         } else {
//!             f!(a = a - 1, c = c + 1)
//!         }
//!     }
//! }
//!
//! let result = f!();
//! assert_eq!(result, 13);
//! ```
//!
//! # Generics and lifetimes
//! The macro supports any kind of generic types, lifetimes and type inference:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     fn generics<'a, 'b, 'c, T: 'c>(
//!         a: i32?,
//!         b: &'a str = "default",
//!         c: (u128, f32)?,
//!         d: Option<[String; 4]>?,
//!         e: &'b str?,
//!         f: Vec<T>?,
//!     ) -> (i32, &'a str, (u128, f32), Option<[String; 4]>, &'b str, Vec<T>) {
//!         (a, b, c, d, e, f)
//!     }
//! }
//!
//! assert_eq!(
//!     generics!(b = "!default"),
//!     (0, "!default", (0, 0.0), None, "", Vec::<u8>::new())
//! );
//! assert_eq!(
//!     generics!(e = "e", f = vec![9]),
//!     (0, "default", (0, 0.0), None, "e", vec![9])
//! );
//! ```
//!
//! # Types that don't implement Default
//! It's possible to use the macro to mark as optional even a type that doesn't implement `Default`.
//! ```
//! # use opt_args::*;
//! #
//! struct X {
//!     x: usize
//! }
//!
//! opt_args! {
//!     fn f(a: X = X { x: 0 }, b: X?) -> usize {
//!         a.x + b.x
//!     }
//! }
//! ```
//! In this case it's impossible to call the generated macro `f!` without passing `b` as a named argument,
//! because it would result in an attempt to use `Default::default()` as value for `X`.
//! ```compile_fail
//! # use opt_args::*;
//! #
//! # // doesn't implement `Default`
//! # struct X {
//! #     x: usize
//! # }
//! #
//! # opt_args! {
//! #     fn f(a: X = X { x: 0 }, b: X?) -> usize {
//! #         a.x + b.x
//! #     }
//! # }
//! f!();
//! ```
//! This would result in a call to `f(X { x: 0 }, Default::default())` which would trigger the compile error:
//! ```the trait `Default` is not implemented for `X` ```.
//!
//! This may be useful to force the caller to pass the argument `b` as a named argument.
//!
//! # Structs
//! The syntax and usage of the macro for structs is the same as it is for functions:
//! ```
//! # use opt_args::*;
//! #
//! opt_args! {
//!     #[opt_args(shuffle)]
//!     #[derive(Debug, PartialEq, Eq)]
//!     struct Opt<'a, 'b, T: 'b> {
//!         a: i32,
//!         b: &'a str = "b",
//!         c: &'b str = "c",
//!         d: T?,
//!     }
//! }
//!
//! let result: Opt<'_, '_, Vec<u8>> = Opt! {4, c = "b", b = "c"};
//! assert_eq!(
//!     result,
//!     Opt {
//!         a: 4,
//!         b: "c",
//!         c: "b",
//!         d: vec![],
//!     }
//! );
//! ```

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Error};

use crate::{
    functions::{compute_combinations, macro_branches},
    parser::{GenericOptArg, OptArgsAttributes, OptArgsItem, OptArgsItemType},
};

mod functions;
mod parser;
mod tokens;

/// Wrap the item (function or struct) inside the macro to declare optional arguments
/// ```
/// use opt_args::opt_args;
///
/// opt_args! {
///     fn function(a: i32, b: &str = "default", c: (u8,)?) -> (i32, &str, (u8,)) {
///         (a, b, c)
///     }
/// }
///
/// opt_args! {
///     #[derive(Debug, PartialEq, Eq)]
///     struct Struct {
///         x: i32,
///         y: i32 = 1,
///         z: i32?,
///         other: Option<Box<Self>>?,
///     }
/// }
///
/// assert_eq!(
///     function!(1, b = "not the default"),
///     (1, "not the default", (0,))
/// );
/// assert_eq!(
///     Struct!(4, z = 5),
///     Struct {
///         x: 4,
///         y: 1,
///         z: 5,
///         other: None
///     }
/// );
/// ```
#[proc_macro]
pub fn opt_args(item: TokenStream1) -> TokenStream1 {
    let item = parse_macro_input!(item as OptArgsItem);
    internal(item)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn internal(mut opt_args_item: OptArgsItem) -> syn::Result<TokenStream> {
    let OptArgsItem {
        ref mut attrs,
        item,
        ..
    } = &mut opt_args_item;
    let ident = item.ident().clone();
    let parsed_attrs: OptArgsAttributes = deluxe::extract_attributes(attrs)?;
    let shuffle = parsed_attrs.shuffle.is_some();
    let macro_export = (parsed_attrs.non_export.is_none()).then_some(quote!(#[macro_export]));
    let macro_ident = if let Some(ident) = parsed_attrs.rename {
        ident
    } else {
        item.ident().clone()
    };

    // convert the list of attributes in a list of generic required/optional arguments
    let mut args: Vec<_> = match item {
        OptArgsItemType::ItemFn(item_fn) => item_fn
            .inputs
            .clone()
            .into_iter()
            .map(GenericOptArg::from)
            .collect(),
        OptArgsItemType::ItemStruct(item_struct) => item_struct
            .fields
            .clone()
            .into_iter()
            .map(GenericOptArg::from)
            .collect(),
    };
    let mut opt_args = vec![];
    let mut first_optional = args.len();
    for (a, mut arg) in args.clone().into_iter().enumerate() {
        // check that all optional arguments are declared after the last non-optional argument
        if !arg.is_optional() {
            if !opt_args.is_empty() {
                return Err(Error::new(
                    arg.ident.span().join(arg.ty.span()).unwrap(),
                    "Non-default arguments should come before default arguments",
                ));
            }
        } else {
            // if the argument doesn't have an explicit default value, use `Default::default()`
            // (this is not a constraint on the actual type to be implement `Default`,
            // but will only be used in the case of a macro invocation without an explicit value)
            if arg.default {
                arg.value =
                    Some(syn::parse(quote!(::std::default::Default::default()).into()).unwrap());
            }
            opt_args.push(arg);
            if first_optional == args.len() {
                first_optional = a;
            }
        }
    }
    // removes all optional arguments from the original array
    args.truncate(first_optional);

    let combinations = compute_combinations(&opt_args, shuffle);
    let macro_branches = macro_branches(
        &ident,
        combinations,
        &opt_args,
        &args,
        matches!(item, OptArgsItemType::ItemFn(_)),
    );

    Ok(quote!(
        #[allow(non_snake_case, unused)]
        #macro_export
        macro_rules! #macro_ident {
            #(#macro_branches);*
        }

        #opt_args_item
    ))
}
