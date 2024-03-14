//! # opt_args
//!
//! This crate lets you easily derive macros to call functions and instantiate structs
//! without having to specify all of their arguments.
//!
//! Macros generated by this crate are annotated with `#[macro_export]`, meaning that you can use
//! them outside the module in which the function is defined. Anyway, the actual possibility to use
//! the macro depends on the possibility to use the item: if you don't expose the item
//! on the outside, the macro will be visible but the code won't compile.
//!
//! Please read the description of each macro before using them!

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Attribute, Error, Meta};

use crate::{
    functions::{compute_combinations, macro_branches},
    parser::{GenericOptArg, OptArgsItem, OptArgsItemType},
};

mod functions;
mod parser;
mod tokens;

/// Apply to a function or struct to generate a macro that can be called with named optional arguments.
///
/// <span style="color:red">**IMPORTANT**</span>:
/// Please note that in order to obtain the result,
/// [`macro@opt_args`] creates a macro that matches any possible combination of given arguments,
/// with a number of branches that is in `O(n!)`, where `n` is the number of optional arguments.
/// This means that compiling a function with 5 optional arguments will result
/// in a macro with `~400` branches.
///
/// Although this has no impact on runtime, it impacts on compile time,
/// so consider not giving your functions too many optional arguments,
/// or consider using another macro (like [`macro@opt_args_ord`]).
///
/// # Usage
/// Annotate your function or struct with the macro
/// and list the arguments that you want to make optional.
/// You can provide default values for optional arguments or leave them blank:
/// if left blank, `Default::default()` will be used (if possible, otherwise the macro won't work).
///
/// # Examples
/// Examples below are about functions but the same goes for structs:
/// same rules, same syntax, same limitations.
///
/// ## Annotating
/// Optional arguments must cover all the final positions from the first optional argument to the
/// last function argument. For example:
/// ```compile_fail
/// # use opt_args::*;
/// #
/// #[opt_args(c = 5)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// }
/// ```
/// is not a valid annotation since arguments `d` and `e` are not optional,
/// but they come after `c` which is optional.
/// Since all the arguments after the first optional must be optional,
/// one correct annotation (with `c` being the first optional) would be the following:
/// ```
/// # use opt_args::*;
/// #
/// #[opt_args(c = 5, d, e)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
///     a + b + c + d + e
/// }
/// ```
/// where all arguments after the first optional (`c`) are optional.
/// In other words: if you mark a total of `n` arguments as optional, they **must be the last `n` arguments**
/// of the function.
///
/// Optional arguments can be passed to the macro in any order,
/// in both the [`macro@opt_args`] and the generated macro calls.
/// For example:
/// ```
/// # use opt_args::*;
/// #
/// #[opt_args(d, c = 5, e)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
///     a + b + c + d + e
/// }
/// ```
/// would be a correct annotation, and the code generated would be the same as before.
///
/// ## Calling
/// To use the function macro, simply use the name of the function as a macro and pass first the
/// positional required arguments, then the named optional arguments (in any order),
/// like in the following:
/// ```
/// # use opt_args::*;
/// #
/// # #[opt_args(d, c = 5, e)]
/// # fn f(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
/// #     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// #     a + b + c + d + e
/// # }
/// let result = f!(1, 2, e = 6, c = 3);
/// assert_eq!(result, 1 + 2 + 3 + 0 + 6);
/// ```
/// In this case we would have `d = 0`, since no custom default value was provided for `d`.
///
/// Once an argument is marked as optional, it cannot be used as positional in the function macro:
/// ```compile_fail
/// # use opt_args::*;
/// #
/// # #[opt_args(d, c = 5, e)]
/// # fn f(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
/// #     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// #     a + b + c + d + e
/// # }
/// f!(1, 2, 3, e = 6);
/// ```
/// is not a valid call since `c` is used as positional (with value `3`).
///
/// ## Recursion
/// It's also possible to use the generated macro inside the original function:
/// ```
/// # use opt_args::*;
/// #
/// #[opt_args(d = 1, c = 1, e = 1)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
///     if a == 3 {
///         f!(1, b, c = c, d = d, e = e)
///     } else {
///         a + b + c + d + e
///     }
/// }
///
/// let result = f!(3, 1);
/// assert_eq!(result, 5);
/// ```
/// which would result in a recursive call.
///
/// ## Default::default
/// If you don't specify a default value for an argument,
/// the macro will assume that you want to use `Default::default()`.
/// This may lead to errors if the type of your argument does not implement `Default`,
/// but only when you call the function as a macro, not in the annotation:
/// ```
/// # use opt_args::*;
/// #
/// #[opt_args(a)]
/// fn f(a: &Vec<String>) -> Vec<String> {
///     a.clone()
/// }
/// ```
/// The above would compile perfectly, but when we try to use the generated macro:
/// ```compile_fail
/// # use opt_args::*;
/// #
/// # #[opt_args(a)]
/// # fn f(a: &Vec<String>) -> Vec<String> {
/// #     a.clone()
/// # }
/// assert_eq!(f!(), Vec::new());
/// ```
/// it will raise a compile time error: the trait `Default` is not implemented for `&Vec<String>`
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
    // take ownership of ident
    let ident = item.ident().clone();
    // check if #[shuffle] is present
    let shuffle = if let Some(i) = attrs.iter().position(|Attribute { meta, .. }| match meta {
        Meta::Path(syn::Path { segments, .. }) => syn::parse::<Ident>(quote!(#segments).into())
            .map(|ident| ident == "shuffle")
            .unwrap_or_default(),
        _ => false,
    }) {
        // If present, remove it from the attributes
        attrs.remove(i);
        true
    } else {
        false
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
        #opt_args_item
        #[macro_export]
        #[allow(non_snake_case, unused)]
        macro_rules! #ident {
            #(#macro_branches);*
        }
    ))
}
