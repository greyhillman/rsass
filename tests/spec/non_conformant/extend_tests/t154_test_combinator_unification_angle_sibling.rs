//! Tests auto-converted from "sass-spec/spec/non_conformant/extend-tests/154_test_combinator_unification_angle_sibling.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner()
}

#[test]
#[ignore] // wrong result
fn test() {
    assert_eq!(
        runner().ok(".a ~ x {a: b}\
             \n.b > y {@extend x}\n"),
        ".a ~ x, .b > .a ~ y {\
         \n  a: b;\
         \n}\n"
    );
}
