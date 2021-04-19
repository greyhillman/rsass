//! Tests auto-converted from "sass-spec/spec/core_functions/color/hsla/error/three_args.hrx"

mod hue {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(\"foo\", 100%, 50%);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $hue: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(\"foo\", 100%, 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
}
mod lightness {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0, 100%, \"foo\");\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $lightness: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0, 100%, \"foo\");\
         \n  |      ^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
}
mod saturation {
    #[test]
    fn test_type() {
        assert_eq!(
            crate::rsass(
                "a {\
             \n  b: hsla(0, \"foo\", 50%);\
             \n}\
             \n"
            )
            .unwrap_err(),
            "Error: $saturation: \"foo\" is not a number.\
         \n  ,\
         \n2 |   b: hsla(0, \"foo\", 50%);\
         \n  |      ^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 2:6  root stylesheet\
         \n",
        );
    }
}
