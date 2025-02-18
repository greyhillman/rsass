//! Tests auto-converted from "sass-spec/spec/values/numbers/units/multiple/division_cancels_both.hrx"

#[allow(unused)]
fn runner() -> crate::TestRunner {
    super::runner().with_cwd("division_cancels_both")
}

#[test]
fn test() {
    assert_eq!(
        runner().ok("$number: 1px * 1rad / 1ms / 1Hz;\
             \na {\
             \n  b: inspect($number / (1px / 1ms));\
             \n}\n"),
        "a {\
         \n  b: 1rad/Hz;\
         \n}\n"
    );
}
