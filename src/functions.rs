use itertools::Itertools;
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::Expr;

use crate::parser::GenericOptArg;

pub(crate) fn compute_combinations(opt_args: &[GenericOptArg], shuffle: bool) -> Vec<Vec<&Ident>> {
    let mut result = vec![];
    for i in 0..=opt_args.len() {
        result.extend(if shuffle {
            opt_args
                .iter()
                .permutations(i)
                .map(|permutation| permutation.iter().map(|a| &a.ident).collect())
                .collect::<Vec<_>>()
        } else {
            opt_args
                .iter()
                .combinations(i)
                .map(|combination| combination.iter().map(|a| &a.ident).collect())
                .collect::<Vec<_>>()
        })
    }
    result
}

pub(crate) fn macro_branches(
    name: &Ident,
    combinations: Vec<Vec<&Ident>>,
    opt_args: &[GenericOptArg],
    required_args: &[GenericOptArg],
    is_function: bool,
) -> Vec<TokenStream> {
    let required_args_formatter = if is_function {
        |GenericOptArg { ident, .. }: &GenericOptArg| quote!($#ident)
    } else {
        |GenericOptArg { ident, .. }: &GenericOptArg| quote!(#ident: $#ident)
    };
    let opt_args_formatter = if is_function {
        |a: &Ident, v: &Expr, c: &Vec<&Ident>| {
            if c.contains(&a) {
                quote!($#a)
            } else {
                quote!(#v)
            }
        }
    } else {
        |a: &Ident, v: &Expr, c: &Vec<&Ident>| {
            if c.contains(&a) {
                quote!(#a: $#a)
            } else {
                quote!(#a: #v)
            }
        }
    };

    let tmp = required_args
        .iter()
        .map(|GenericOptArg { ident, .. }| quote!($#ident:expr));
    let required_args_pattern = quote!(#(#tmp),*);

    let tmp = required_args.iter().map(required_args_formatter);
    let required_args_branch = quote!(#(#tmp),*);
    let mut result: Vec<TokenStream> = vec![];

    for combination in combinations {
        let tmp = combination.iter().map(|a| quote!(#a = $#a:expr));
        let opt_args_pattern = quote!(#(#tmp),*);
        let tmp = [&required_args_pattern, &opt_args_pattern];
        let tmp = tmp.iter().filter(|e| !e.is_empty());
        let pattern = quote!(#(#tmp),*);
        let tmp = opt_args.iter().map(|GenericOptArg { ident, value, .. }| {
            opt_args_formatter(ident, value.as_ref().unwrap(), &combination)
        });
        let opt_args_branch = quote!(#(#tmp),*);
        let tmp = [&required_args_branch, &opt_args_branch];
        let tmp = tmp.iter().filter(|e| !e.is_empty());
        let branch = quote!(#(#tmp),*);
        let body = if is_function {
            quote!(#name (#branch))
        } else {
            quote!(#name { #branch })
        };
        result.push(quote!((#pattern) => {#body}));
    }

    // fallback branch for wrong order or wrong names
    result.push(quote!(
        ($($tt:tt)*) => {
            panic!(
                "Unrecognized order or name for arguments: `{}`.\
                If you want to pass named parameters in any order, use the attribute #[shuffle]",
                stringify!($($tt)*)
            )
        }
    ));
    result
}
