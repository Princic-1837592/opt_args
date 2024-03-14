use opt_args::opt_args;

#[test]
fn opt_struct() {
    opt_args! {
        #[shuffle]
        #[non_export]
        #[derive(Default, Debug, PartialEq)]
        struct Opt<'a, 'b, T: 'b> {
            a: i32,
            b: &'a str = "b",
            c: &'b str = "c",
            d: T?,
        }
    }

    let result: Opt<'_, '_, Vec<u8>> = Opt! {4};
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "b",
            c: "c",
            d: vec![],
        }
    );

    let result = Opt! {
        4,
        c = "not default",
        d = vec!["type inference Vec<&str>"]
    };
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "b",
            c: "not default",
            d: vec!["type inference Vec<&str>"],
        }
    );

    let result = Opt! {
        4,
        b = "c",
        d = 1
    };
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "c",
            c: "c",
            d: 1,
        }
    );
}
