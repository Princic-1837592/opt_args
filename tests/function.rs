use opt_args::{opt_args, opt_args_ord};

#[test]
fn one_opt_arg() {
    #[opt_args(b)]
    fn one_opt_arg_internal(a: i32, b: u8) -> (i32, u8) {
        (a, b)
    }

    assert_eq!(one_opt_arg_internal!(1), (1, 0));
    assert_eq!(one_opt_arg_internal!(1, b = 42), (1, 42));
}

#[test]
fn all_opt_arg() {
    #[opt_args(a, b)]
    fn all_opt_arg_internal(a: i32, b: u8) -> (i32, u8) {
        (a, b)
    }

    assert_eq!(all_opt_arg_internal!(), (0, 0));
    assert_eq!(all_opt_arg_internal!(a = 1), (1, 0));
    assert_eq!(all_opt_arg_internal!(b = 1), (0, 1));
    //call with arguments in different order
    assert_eq!(all_opt_arg_internal!(b = 1, a = 1), (1, 1));
}

#[test]
fn recursive() {
    #[opt_args(n = 5)]
    fn factorial(n: u64) -> u64 {
        if n <= 1 {
            1
        } else {
            factorial!(n = n - 1) * n
        }
    }

    assert_eq!(factorial!(), factorial(5));
}

#[test]
fn many_types() {
    #[opt_args_ord(a, b = "default", c, d, e)]
    fn many_types_internal<'a, 'b>(
        a: i32,
        b: &'a str,
        c: (u128, f32),
        d: Option<[String; 4]>,
        e: &'b str,
    ) -> (i32, &'a str, (u128, f32), Option<[String; 4]>, &'b str) {
        (a, b, c, d, e)
    }

    assert_eq!(many_types_internal!(), (0, "default", (0, 0.0), None, ""));
    assert_eq!(
        many_types_internal!(e = "e"),
        (0, "default", (0, 0.0), None, "e")
    );
}

#[test]
fn generics() {
    #[opt_args(a, b)]
    fn generics_internal<A, B>(a: A, b: B) -> (A, B) {
        (a, b)
    }

    //macros can infer the type to return
    let result: (i32, f64) = generics_internal!();
    assert_eq!(result, (0, 0.0));

    #[derive(Default, PartialEq, Debug)]
    struct X<'a> {
        a: i32,
        b: &'a str,
        c: String,
    }

    let result: (X, &str) = generics_internal!();
    assert_eq!(
        result,
        (
            X {
                a: 0,
                b: "",
                c: String::new()
            },
            ""
        )
    )
}

#[test]
fn ordered() {
    #[opt_args_ord(c, b)]
    fn ordered_internal(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
        (a, b, c)
    }

    let mut result = ordered_internal!(1);
    assert_eq!(result, (1, 0, 0));

    result = ordered_internal!(1, b = 10);
    assert_eq!(result, (1, 10, 0));

    result = ordered_internal!(1, c = 1);
    assert_eq!(result, (1, 0, 1));

    // result = ordered_internal!(1, b = 1, c = 1);
    // assert_eq!(result, (1, 1, 1));
}
