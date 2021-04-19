//! Tests auto-converted from "sass-spec/spec/core_functions/color/adjust_hue.hrx"

#[test]
fn above_max() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 540)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
#[test]
fn alpha() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(rgba(red, 0.1), 359)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(255, 0, 4, 0.1);\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red)}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $degrees.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red)}\
         \n  |       ^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 1, 2)}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: adjust-hue(red, 1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:color\
         \n1 | @function adjust-hue($color, $degrees) {\
         \n  |           ============================ declaration\
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
                    "a {b: adjust-hue(1, 2)}\
             \n"
                )
                .unwrap_err(),
                "Error: $color: 1 is not a color.\
         \n  ,\
         \n1 | a {b: adjust-hue(1, 2)}\
         \n  |       ^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn hue() {
            assert_eq!(
                crate::rsass(
                    "a {b: adjust-hue(red, blue)}\
             \n"
                )
                .unwrap_err(),
                "Error: $degrees: blue is not a number.\
         \n  ,\
         \n1 | a {b: adjust-hue(red, blue)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^\
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
            "a {b: adjust-hue(red, 0.5)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ff0200;\
        \n}\
        \n"
    );
}
#[test]
fn max() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 359)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #ff0004;\
        \n}\
        \n"
    );
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, 123)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #00ff0d;\
        \n}\
        \n"
    );
}
#[test]
fn min() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(blue, 0)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: blue;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue($color: red, $degrees: 123)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #00ff0d;\
        \n}\
        \n"
    );
}
#[test]
fn negative() {
    assert_eq!(
        crate::rsass(
            "a {b: adjust-hue(red, -180)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: aqua;\
        \n}\
        \n"
    );
}
mod units {
    #[test]
    fn angle() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 60rad)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
    #[test]
    fn deg() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 60deg)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unitless() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 60)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
    #[test]
    fn unknown() {
        assert_eq!(
            crate::rsass(
                "a {b: adjust-hue(red, 60in)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: yellow;\
        \n}\
        \n"
        );
    }
}
