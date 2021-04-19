//! Tests auto-converted from "sass-spec/spec/core_functions/string/index.hrx"

#[test]
fn beginning() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index(\"cde\", \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn both_empty() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index(\"\", \"\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn combining_character() {
    assert_eq!(
        crate::rsass(
            "// Sass does *not* treat strings as sequences of glyphs, so this string which\
            \n// contains \"c\" followed by a combining umlaut should be considered two separate\
            \n// characters even though it\'s rendered as only one.\
            \na {b: str-index(\"c\\0308 a\", \"a\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
#[test]
fn double_width_character() {
    assert_eq!(
        crate::rsass(
            "// Sass treats strings as sequences of Unicode codepoint; it doesn\'t care if a\
            \n// character is represented as two UTF-16 code units.\
            \na {b: str-index(\"👭a\", \"a\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn empty_substring() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index(\"cde\", \"\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn end() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index(\"cde\", \"e\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 3;\
        \n}\
        \n"
    );
}
mod error {
    #[test]
    fn too_few_args() {
        assert_eq!(
            crate::rsass(
                "a {b: str-index(\"c\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Missing argument $substring.\
         \n  ,--> input.scss\
         \n1 | a {b: str-index(\"c\")}\
         \n  |       ^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function index($string, $substring) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    #[test]
    fn too_many_args() {
        assert_eq!(
            crate::rsass(
                "a {b: str-index(\"c\", \"d\", \"e\")}\
             \n"
            )
            .unwrap_err(),
            "Error: Only 2 arguments allowed, but 3 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: str-index(\"c\", \"d\", \"e\")}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:string\
         \n1 | @function index($string, $substring) {\
         \n  |           ========================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
        );
    }
    mod test_type {
        #[test]
        fn string() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-index(1, \"c\")}\
             \n"
                )
                .unwrap_err(),
                "Error: $string: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-index(1, \"c\")}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
        #[test]
        fn substring() {
            assert_eq!(
                crate::rsass(
                    "a {b: str-index(\"c\", 1)}\
             \n"
                )
                .unwrap_err(),
                "Error: $substring: 1 is not a string.\
         \n  ,\
         \n1 | a {b: str-index(\"c\", 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet\
         \n",
            );
        }
    }
}
#[test]
fn middle() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index(\"cde\", \"d\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 2;\
        \n}\
        \n"
    );
}
#[test]
fn named() {
    assert_eq!(
        crate::rsass(
            "a {b: str-index($string: \"cde\", $substring: \"c\")}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: 1;\
        \n}\
        \n"
    );
}
#[test]
fn not_found() {
    assert_eq!(
        crate::rsass(
            "a {b: inspect(str-index(\"cde\", \"f\"))}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: null;\
        \n}\
        \n"
    );
}
