//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/236_extend_with_universal_selector_empty_namespace.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok("%-a |*.foo {a: b}\
             \na {@extend .foo}\
             \n-a {@extend %-a}\n"),
        "-a |*.foo {\
         \n  a: b;\
         \n}\n"
    );
}
