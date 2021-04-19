//! Tests auto-converted from "sass-spec/spec/core_functions/color/darken.hrx"

#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(rgba(red, 0.2), 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(0, 0, 0, 0.2);\
        \n}\
        \n"
    );
}
mod error {
    mod bounds {
        #[test]
        fn too_high() {
            assert_eq!(
                crate::rsass(
                    "a {b: darken(red, 100.001)}\
             \n"
                )
                .unwrap_err(),
                "Error: $amount: Expected 100.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: darken(red, 100.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn too_low() {
            assert_eq!(
                crate::rsass(
                    "a {b: darken(red, -0.001)}\
             \n"
                )
                .unwrap_err(),
                "Error: $amount: Expected -0.001 to be within 0 and 100.\
         \n  ,\
         \n1 | a {b: darken(red, -0.001)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: darken(red)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $amount.\
         \n  ,--> input.scss\
         \n1 | a {b: darken(red)}\
         \n  |       ^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function darken($color, $amount) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: darken(red, 1%, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: darken(red, 1%, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function darken($color, $amount) {\
         \n  |           ======================= declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn color() {
            assert_eq!(
                crate::rsass(
                    "a {b: darken(1, 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: darken(1, 2)}\
         \n  |       ^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn lightness() {
            assert_eq!(
                crate::rsass(
                    "a {b: darken(red, blue)}\
             \n"
                )
                .unwrap_err(),
                "Error: $amount: blue is not a number.\
         \n  ,\
         \n1 | a {b: darken(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn fraction() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 0.5%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #fc0000;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 100%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
#[test]
fn max_remaining() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 50%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: black;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b80000;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: darken(red, 0%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: red;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: darken($color: red, $amount: 14%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #b80000;\
        \n}\
        \n"
    );
}
