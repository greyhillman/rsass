//! Tests auto-converted from "sass-spec/spec/core_functions/list/join/error.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // missing error
fn named() {
    assert_eq!(
        runner().err(
            "a {b: join(c, d, $invalid: true)}\n"
        ),
        "Error: No argument named $invalid.\
         \n  ,--> input.scss\
         \n1 | a {b: join(c, d, $invalid: true)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
#[ignore] // wrong error
fn positional_and_named() {
    assert_eq!(
        runner().err(
            "a {b: join(c, d, comma, true, false, $invalid: true)}\n"
        ),
        "Error: Only 4 positional arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: join(c, d, comma, true, false, $invalid: true)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_few_args() {
    assert_eq!(
        runner().err(
            "a {b: join(c)}\n"
        ),
        "Error: Missing argument $list2.\
         \n  ,--> input.scss\
         \n1 | a {b: join(c)}\
         \n  |       ^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
#[test]
fn too_many_args() {
    assert_eq!(
        runner().err(
            "a {b: join(c, d, comma, true, false)}\n"
        ),
        "Error: Only 4 arguments allowed, but 5 were passed.\
         \n  ,--> input.scss\
         \n1 | a {b: join(c, d, comma, true, false)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invocation\
         \n  \'\
         \n  ,--> sass:list\
         \n1 | @function join($list1, $list2, $separator: auto, $bracketed: auto) {\
         \n  |           ======================================================== declaration\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
mod test_type {
    #[allow(unused)]
    use super::runner;

    #[test]
    fn separator() {
        assert_eq!(
            runner().err("a {b: join(c, d, $separator: 1)}\n"),
            "Error: $separator: 1 is not a string.\
         \n  ,\
         \n1 | a {b: join(c, d, $separator: 1)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
        );
    }
}
#[test]
#[ignore] // wrong error
fn unknown_separator() {
    assert_eq!(
        runner().err(
            "a {b: join(c, d, $separator: e)}\n"
        ),
        "Error: $separator: Must be \"space\", \"comma\", \"slash\", or \"auto\".\
         \n  ,\
         \n1 | a {b: join(c, d, $separator: e)}\
         \n  |       ^^^^^^^^^^^^^^^^^^^^^^^^^\
         \n  \'\
         \n  input.scss 1:7  root stylesheet",
    );
}
