//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/103_test_nested_extender_chooses_first_subseq.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a .b .c .d .foo {a: b}\
             \n.c .d .a .b .bar {@extend .foo}\n"),
        ".a .b .c .d .foo, .a .b .c .d .a .b .bar {\
         \n  a: b;\
         \n}\n"
    );
}
