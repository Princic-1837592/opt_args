use opt_args::opt_args;

#[test]
fn one_opt_arg() {
    opt_args! {
        #[opt_args(non_export)]
        fn one_opt_arg(a: i32, b: u8?) -> (i32, u8) {
            (a, b)
        }
    }

    assert_eq!(one_opt_arg!(1), (1, 0));
    assert_eq!(one_opt_arg!(1, b = 42), (1, 42));
}

#[test]
fn all_opt_arg() {
    opt_args! {
        #[opt_args(shuffle, non_export)]
        fn all_opt_arg_internal(a: i32?, b: u8?) -> (i32, u8) {
            (a, b)
        }
    }

    assert_eq!(all_opt_arg_internal!(), (0, 0));
    assert_eq!(all_opt_arg_internal!(a = 1), (1, 0));
    assert_eq!(all_opt_arg_internal!(b = 1), (0, 1));
    // call with arguments in different order
    assert_eq!(all_opt_arg_internal!(b = 1, a = 1), (1, 1));
}

#[test]
fn recursive() {
    opt_args! {
        #[opt_args(non_export)]
        fn factorial(n: u64 = 5) -> u64 {
            if n <= 1 {
                1
            } else {
                factorial!(n = n - 1) * n
            }
        }
    }

    assert_eq!(factorial!(), factorial(5));
}

#[test]
#[allow(clippy::type_complexity)]
fn complex_types() {
    opt_args! {
        #[opt_args(non_export)]
        fn complex_types<'a, 'b, 'c, T: 'c>(
            a: i32?,
            b: &'a str = "default",
            c: (u128, f32)?,
            d: Option<[String; 4]>?,
            e: &'b str?,
            f: Vec<T>?,
        ) -> (i32, &'a str, (u128, f32), Option<[String; 4]>, &'b str, Vec<T>) {
            (a, b, c, d, e, f)
        }
    }

    assert_eq!(
        complex_types!(),
        (0, "default", (0, 0.0), None, "", Vec::<u8>::new())
    );
    assert_eq!(
        complex_types!(e = "e", f = vec![9]),
        (0, "default", (0, 0.0), None, "e", vec![9])
    );
}

#[test]
fn generics_and_type_inference() {
    opt_args! {
        #[opt_args(shuffle, non_export)]
        fn type_inference<A, B>(a: A?, b: B?) -> (A, B) {
            (a, b)
        }
    }

    let result: (i32, f64) = type_inference!();
    assert_eq!(result, (0, 0.0));

    #[derive(Default, PartialEq, Debug)]
    struct X<'a> {
        a: i32,
        b: &'a str,
        c: String,
    }

    let result: (X, &str) = type_inference!();
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
    opt_args! {
        #[opt_args(non_export)]
        fn ordered_internal(a: i32, b: i32?, c: i32?) -> (i32, i32, i32) {
            (a, b, c)
        }
    }

    let mut result = ordered_internal!(1);
    assert_eq!(result, (1, 0, 0));

    result = ordered_internal!(1, b = 10);
    assert_eq!(result, (1, 10, 0));

    result = ordered_internal!(1, c = 1);
    assert_eq!(result, (1, 0, 1));

    result = ordered_internal!(1, b = 1, c = 1);
    assert_eq!(result, (1, 1, 1));
}
