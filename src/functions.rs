use itertools::Itertools;
use syn::{FnArg, Pat};

///converts a single FnArg object into a string representing its ident
pub fn fn_arg_to_name(arg: &FnArg) -> String {
    match arg {
        FnArg::Typed(pat_type) => {
            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                return pat_ident.ident.to_string();
            }
            panic!("If you can read this, please open a PR");
        }
        _ => panic!("If you can read this, please open a PR"),
    }
}

pub enum CombinationType {
    Unordered,
    Ordered,
}

/// returns the vector of all combinations for optional arguments
pub fn get_combinations(args: Vec<String>, combination_type: CombinationType) -> Vec<Vec<String>> {
    let mut result = vec![];
    for i in 0..=args.len() {
        result.extend(match combination_type {
            CombinationType::Unordered => args
                .iter()
                .permutations(i)
                .map(|permutation| permutation.iter().map(ToString::to_string).collect())
                .collect::<Vec<_>>(),
            CombinationType::Ordered => args
                .iter()
                .combinations(i)
                .map(|combination| combination.iter().map(ToString::to_string).collect())
                .collect::<Vec<_>>(),
        })
    }
    result
}

/// creates couples (pattern, branch) where:
/// pattern is a pattern of the final macro
/// branch is the code that will substitute the macro call
pub fn macro_branches(
    name: &str,
    combinations: Vec<Vec<String>>,
    opt_args: Vec<(String, String)>,
    required_args: Vec<String>,
    is_function: bool,
) -> Vec<(String, String)> {
    // build formatters
    let required_args_formatter = if is_function {
        |a: &String| format!("${}", a)
    } else {
        |a: &String| format!("{}: ${}", a, a)
    };
    let opt_args_formatter = if is_function {
        |a: &String, v: &String, c: &Vec<String>| {
            if c.contains(a) {
                format!("${}", a)
            } else {
                v.to_string()
            }
        }
    } else {
        |a: &String, v: &String, c: &Vec<String>| {
            if c.contains(a) {
                format!("{}: ${}", a, a)
            } else {
                format!("{}: {}", a, v)
            }
        }
    };

    let required_args_pattern = required_args
        .iter()
        .map(|a| format!("${}:expr", a))
        .join(",");
    let mut opt_args_pattern: String;
    let required_args_branch = required_args.iter().map(required_args_formatter).join(",");
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
        opt_args_branch = opt_args
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
            },
        ));
    }
    result
}
