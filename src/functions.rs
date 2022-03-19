use std::collections::HashMap;
use syn::{FnArg, Pat};
use itertools::Itertools;
use crate::parser::OptArgs;


///converts a single FnArg object into a string representing its ident
pub fn fn_arg_to_name(arg: &FnArg) -> String {
    match arg {
        FnArg::Typed(pat_type) => {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                return pat_ident.ident.to_string();
            }
            panic!("If you can read this, please open a PR")
        }
        _ => panic!("If you can read this, please open a PR")
    }
}


pub enum CombinationType {
    Unordered,
    Ordered,
}


/// returns the vector of all combinations for optional arguments
pub fn combinations(args: Vec<String>, combination_type: CombinationType) -> Vec<Vec<String>> {
    let mut result = vec![];
    for i in 0..=args.len() {
        match combination_type {
            CombinationType::Unordered => {
                args
                    .iter()
                    .permutations(i)
                    .for_each(
                        |combination| result.push(
                            combination
                                .iter()
                                .map(ToString::to_string)
                                .collect()
                        )
                    )
            }
            CombinationType::Ordered => {
                args
                    .iter()
                    .combinations(i)
                    .for_each(
                        |combination| result.push(
                            combination
                                .iter()
                                .map(ToString::to_string)
                                .collect()
                        )
                    )
            }
        }
    }
    result
}


/// creates couples (pattern, branch) where:
/// pattern is a pattern of the final macro
/// branch is the code that will substitute the macro call
pub fn macro_branches(
    name: &str,
    args: Vec<String>,
    combinations: Vec<Vec<String>>,
    opt_args: OptArgs,
    is_function: bool,
) -> Vec<(String, String)> {
    let required_args_formatter = if is_function {
        |a: &String| format!("${}", a)
    } else {
        |a: &String| format!("{}: ${}", a, a)
    };
    let opt_args_formatter = if is_function {
        |a: &String, v: &String, c: &Vec<String>| if c.contains(a) {
            format!("${}", a)
        } else {
            v.to_string()
        }
    } else {
        |a: &String, v: &String, c: &Vec<String>| if c.contains(a) {
            format!("{}: ${}", a, a)
        } else {
            format!("{}: {}", a, v)
        }
    };
    let required_args_num = args
        .len()
        .checked_sub(
            combinations[combinations.len() - 1].len()
        )
        .unwrap_or_else(|| panic!("Too many optionals"));
    if required_args_num == args.len() {
        panic!("Provide at least one optional parameter");
    }
    let args_indexes: HashMap<String, usize> = args
        .iter()
        .enumerate()
        .map(|(i, a)| (a.clone(), i))
        .collect();
    opt_args
        .iter()
        .for_each(
            |a| if args_indexes[&a.name] < required_args_num {
                panic!("All the arguments after the first optional must be optional")
            }
        );


    let required_args: Vec<String> = args
        .into_iter()
        .take(required_args_num)
        .collect();
    let opt_arg_to_value: Vec<(String, String)> = opt_args
        .iter()
        .map(|a| (a.name.clone(), a.value.clone()))
        .sorted_by_key(|(a, _)| args_indexes[a])
        .collect();
    let required_args_pattern = required_args
        .iter()
        .map(|a| format!("${}:expr", a))
        .join(",");
    let mut opt_args_pattern: String;
    let required_args_branch = required_args
        .iter()
        .map(required_args_formatter)
        .join(",");
    let mut opt_args_branch: String;
    let mut result: Vec<(String, String)> = vec![];


    for combination in combinations {
        opt_args_pattern = combination
            .iter()
            .map(|a| format!("{} = ${}:expr", a, a))
            .join(",");
        let pattern = vec![&required_args_pattern, &opt_args_pattern]
            .iter()
            .filter(|e| !e.is_empty())
            .join(",");
        opt_args_branch = opt_arg_to_value
            .iter()
            .map(|(a, v)| opt_args_formatter(a, v, &combination))
            .join(",");
        let branch = vec![&required_args_branch, &opt_args_branch]
            .iter()
            .filter(|e| !e.is_empty())
            .join(",");
        result.push((
            pattern,
            if is_function {
                format!("{}({})", name, branch)
            } else {
                format!("{}{{{}}}", name, branch)
            }
        ));
    }
    result
}
