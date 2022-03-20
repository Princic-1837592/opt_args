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
//! Please read the description of each macro before using them, especially [`macro@opt_args`]!
mod parser;
mod functions;


use proc_macro::TokenStream;
use itertools::Itertools;
use quote::quote;
use syn::Item;
use crate::parser::{OptArgs, OptArgsItem};
use crate::functions::{combinations, fn_arg_to_name, macro_branches, CombinationType};


/// Apply to a function or struct to generate a macro that can be called with named optional arguments.
///
/// Please note that in order to obtain the result, [`macro@opt_args`] creates a macro with a number
/// of branches that is in `O((n+1)!)`, where `n` is the number of optional arguments,
/// which means that compiling a function with 5 optional arguments will result
/// in a macro with ~400 branches. Although this is a very rare case, consider not giving
/// your functions too many optional arguments or consider using another macro,
/// like [`macro@opt_args_ord`].
///
/// You can provide default values for optional arguments or leave them blank:
/// if left blank, Default::default() will be used (if possible, otherwise the macro won't work).
/// For example, blank default values are usually not possible for references.
/// In that case you must specify the default value for the argument.
///
/// Examples below are about functions but the same goes for structs:
/// same rules, same syntax, same limitations.
///
/// Optional arguments must cover all the final positions from the first optional argument to the
/// last function argument. For example:
/// ```
/// #[opt_args(c = 5)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// }
/// ```
/// is not a valid macro call since arguments `d` and `e` are not optional,
/// but they come after `c` which is optional.
/// Since all the arguments after the first optional must be optional,
/// one correct macro call (w.r.t. `c` being the first optional) would be the following:
/// ```
/// #[opt_args(c = 5, d, e)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// }
/// ```
/// where all arguments after the first optional (`c`) are optional.
/// In other words: if you mark `n` arguments as optional, they **must be the last `n` arguments**
/// of the function.
///
/// Optional arguments can be passed to the macro in any order,
/// in both the [`macro@opt_args`] and the generated macro calls.
/// For example:
/// ```
/// #[opt_args(d, c = 5, e)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) {
///     println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
/// }
/// ```
/// would be a correct macro call, and the code generated would be the same as before.
///
/// To use the function macro, simply use the name of the function as a macro and pass first the
/// positional required arguments, then the named optional arguments (in any order),
/// like in the following:
/// ```
/// f!(1, 2, e = 6, c = 3);
/// ```
/// In this case we would have `d = 0`,
/// since no custom default value was provided for the optional `d`.
///
/// Once an argument is marked as optional, it cannot be used as positional in the function macro:
/// ```
/// f!(1, 2, 3, d = 5);
/// ```
/// is not a valid call since `c` is used as positional (with value `3`).
/// The correct call would be the following:
/// ```
/// f!(1, 2, c = 3, d = 5);
/// ```
///
/// The macro is generated in such a way that it is possible to use the function macro inside the
/// original function:
/// ```
/// #[opt_args(d, c = 5, e)]
/// fn f(a: i32, b: i32, c: i32, d: i32, e: i32) {
///     if a == 3 {
///         f!(1, b, c = c, d = d, e = e);
///     } else {
///         println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
///     }
/// }
/// ```
/// which would result in a recursive call.
#[proc_macro_attribute]
pub fn opt_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    internal(attr, item, CombinationType::Unordered)
}


/// Same as [`macro@opt_args`], but the generated macro
/// must be called with arguments in the right order.
///
/// You can still pass arguments to [`macro@opt_args_ord`] in any order:
/// ```
/// #[opt_args_ord(c, b)]
/// fn f(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
///     (a, b, c)
/// }
///
/// fn main() {
///     f!(1, b = 2, c = 3); //RIGHT
///     f!(1, c = 3, b = 2); //WRONG: arguments not in the same order of the original function
/// }
/// ```
///
/// This macro was added to make the compilation faster in the case of functions with more
/// than 5 optional arguments.
#[proc_macro_attribute]
pub fn opt_args_ord(attr: TokenStream, item: TokenStream) -> TokenStream {
    internal(attr, item, CombinationType::Ordered)
}


fn internal(attr: TokenStream, item: TokenStream, combination_type: CombinationType) -> TokenStream {
    let opt_args = syn::parse_macro_input!(attr as OptArgs);
    let input = syn::parse_macro_input!(item as OptArgsItem);
    let combinations = combinations(
        opt_args
            .attrs
            .iter()
            .map(ToString::to_string)
            .collect(),
        combination_type,
    );
    let (name, args, is_function) = match &input {
        OptArgsItem::Function(input) => {
            let name: String = input.sig.ident.to_string();
            let args: Vec<String> = input
                .sig
                .inputs
                .iter()
                .map(fn_arg_to_name)
                .collect();
            (name, args, true)
        }
        OptArgsItem::Struct(input) => {
            let name: String = input.ident.to_string();
            let args: Vec<String> = input
                .fields
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect();
            (name, args, false)
        }
    };
    let macro_branches = macro_branches(
        &name,
        args,
        combinations,
        opt_args,
        is_function,
    );
    let macro_body = macro_branches
        .iter()
        .map(|(p, b)| format!("({}) => {{{}}}", p, b))
        .join(";");
    let macro_token: Item = syn::parse_str(
        &format!(
            "#[macro_export]macro_rules! {} {{{}}}",
            name,
            macro_body
        )
    ).unwrap();
    let result = quote! {
        #macro_token

        #input
    };
    result.into()
}
