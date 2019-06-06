//! Tests auto-converted from "sass-spec/spec/mixin"
//! version 98496644, 2019-06-04 12:57:39 +0100.
//! See <https://github.com/sass/sass-spec> for source material.\n
use rsass::{compile_scss, OutputStyle};

mod content;

// From "sass-spec/spec/mixin/environment_locality.hrx"
#[test]
fn environment_locality() {
    assert_eq!(
        rsass(
            "// The \"$var\" variable should only be set locally, despite being in the same\n// mixin each time.\n@mixin with-local-variable($recurse) {\n  $var: before;\n\n  @if ($recurse) {\n    @include with-local-variable($recurse: false);\n  }\n\n  var: $var;\n  $var: after;\n}\n\n.environment-locality {\n  @include with-local-variable($recurse: true);\n}\n"
        )
        .unwrap(),
        ".environment-locality {\n  var: before;\n  var: before;\n}\n"
    );
}

mod error;

fn rsass(input: &str) -> Result<String, String> {
    compile_scss(input.as_bytes(), OutputStyle::Expanded)
        .map_err(|e| format!("rsass failed: {}", e))
        .and_then(|s| {
            String::from_utf8(s)
                .map(|s| s.replace("\n\n", "\n"))
                .map_err(|e| format!("{:?}", e))
        })
}
