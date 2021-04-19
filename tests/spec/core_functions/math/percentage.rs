//! Tests auto-converted from "sass-spec/spec/core_functions/math/percentage.hrx"

mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: percentage()}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $number.\
         \n  ,--> input.scss\
         \n1 | a {b: percentage()}\
         \n  |       ^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function percentage($number) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: percentage(1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 1 argument allowed, but 2 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: percentage(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:math\
         \n1 | @function percentage($number) {\
         \n  |           =================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {b: percentage(c)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: c is not a number.\
         \n  ,\
         \n1 | a {b: percentage(c)}\
         \n  |       ^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn unit() {
        assert_eq!(
            crate::rsass(
                "a {b: percentage(1%)}\
             \n"
            )
            .unwrap_err(),
            "Error: $number: Expected 1% to have no units.\
         \n  ,\
         \n1 | a {b: percentage(1%)}\
         \n  |       ^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
}
#[test]
fn integer() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(42)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 4200%;\
        \n}\
        \n"
    );
}
#[test]
fn large() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(123.456)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 12345.6%;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage($number: 1)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 100%;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(-0.4)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: -40%;\
        \n}\
        \n"
    );
}
#[test]
fn small() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(0.246)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 24.6%;\
        \n}\
        \n"
    );
}
#[test]
fn zero() {
    assert_eq!(
        crate::rsass(
            "a {b: percentage(0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 0%;\
        \n}\
        \n"
    );
}
