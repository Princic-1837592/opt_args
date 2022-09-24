use opt_args::opt_args;

#[test]
fn opt_struct() {
    #[derive(Default, Debug, PartialEq)]
    #[opt_args(b = "b", c = "c", d)]
    struct Opt<'a> {
        a: i32,
        b: &'a str,
        c: &'a str,
        d: &'a str,
    }

    let result = Opt!(4);
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "b",
            c: "c",
            d: ""
        }
    );

    let result = Opt!(4, c = "not default");
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "b",
            c: "not default",
            d: ""
        }
    );

    let result = Opt!(4, b = "c", d = "not default");
    assert_eq!(
        result,
        Opt {
            a: 4,
            b: "c",
            c: "c",
            d: "not default"
        }
    );
}
