//! Tests auto-converted from "sass-spec/spec/core_functions/color/change_color"
#[allow(unused)]
use super::rsass;

mod error;

// From "sass-spec/spec/core_functions/color/change_color/hsl.hrx"
mod hsl {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
        rsass(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #98695d;\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_arg() {
        assert_eq!(
        rsass(
            "a {b: change-color(black, $hue: 12, $saturation: 24%, $lightness: 48%, $alpha: 0.7)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(152, 105, 93, 0.7);\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_input() {
        assert_eq!(
        rsass(
            "a {b: change-color(rgba(black, 0.7), $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(152, 105, 93, 0.7);\
        \n}\
        \n"
    );
    }
    mod hue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn above_max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $hue: 540)}\
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
        fn fraction() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $hue: 0.5)}\
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
                rsass(
                    "a {b: change-color(red, $hue: 359)}\
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
                rsass(
                    "a {b: change-color(red, $hue: 123)}\
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
                rsass(
                    "a {b: change-color(blue, $hue: 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn negative() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $hue: -60)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: fuchsia;\
        \n}\
        \n"
            );
        }
    }
    mod lightness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn fraction() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 0.5%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #030000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 63%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #ff4242;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 27%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #8a0000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: white;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: change-color($color: black, $hue: 12, $saturation: 24%, $lightness: 48%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #98695d;\
        \n}\
        \n"
    );
    }
    mod saturation {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(plum, $saturation: 76%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #f08df0;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(plum, $saturation: 14%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #c8b5c8;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(plum, $saturation: 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #ff7eff;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(plum, $saturation: 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bfbfbf;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/change_color/hwb.hrx"
mod hwb {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
        rsass(
            "a {b: change-color(blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #339966;\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_arg() {
        assert_eq!(
        rsass(
            "a {b: change-color(blue, $hue: 150, $whiteness: 20%, $blackness: 40%, $alpha: 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(51, 153, 102, 0.3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_input() {
        assert_eq!(
        rsass(
            "a {b: change-color(rgba(blue, 0.7), $hue: 150, $whiteness: 20%, $blackness: 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(51, 153, 102, 0.7);\
        \n}\
        \n"
    );
    }
    mod blackness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#993333, $blackness: 80%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #333333;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#993333, $blackness: 20%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #cc3333;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#993333, $blackness: 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #2b2b2b;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#993333, $blackness: 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #ff3333;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: change-color($color: blue, $hue: 150, $whiteness: 20%, $blackness: 40%)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #339966;\
        \n}\
        \n"
    );
    }
    mod whiteness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#cc6666, $whiteness: 80%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #cccccc;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#cc6666, $whiteness: 20%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #cc3333;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#cc6666, $whiteness: 100%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #d5d5d5;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(#cc6666, $whiteness: 0%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #cc0000;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/change_color/no_rgb_hsl.hrx"
mod no_rgb_hsl {
    #[allow(unused)]
    use super::rsass;
    mod alpha {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(rgba(red, 0.5), $alpha: 0.72)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.72);\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(rgba(red, 0.5), $alpha: 0.36)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0.36);\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(rgba(red, 0.5), $alpha: 1)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(rgba(red, 0.5), $alpha: 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: rgba(255, 0, 0, 0);\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
            rsass(
                "a {b: change-color($color: red)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
    #[test]
    fn positional() {
        assert_eq!(
            rsass(
                "a {b: change-color(red)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: red;\
        \n}\
        \n"
        );
    }
}

// From "sass-spec/spec/core_functions/color/change_color/rgb.hrx"
mod rgb {
    #[allow(unused)]
    use super::rsass;
    #[test]
    fn all() {
        assert_eq!(
            rsass(
                "a {b: change-color(black, $red: 12, $green: 24, $blue: 48)}\
            \n"
            )
            .unwrap(),
            "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
        );
    }
    #[test]
    fn alpha_arg() {
        assert_eq!(
        rsass(
            "a {b: change-color(black, $red: 12, $green: 24, $blue: 48, $alpha: 0.3)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.3);\
        \n}\
        \n"
    );
    }
    #[test]
    fn alpha_input() {
        assert_eq!(
        rsass(
            "a {b: change-color(rgba(black, 0.3), $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: rgba(12, 24, 48, 0.3);\
        \n}\
        \n"
    );
    }
    mod blue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $blue: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #0000c8;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(blue, $blue: 100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #000064;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $blue: 255)}\
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
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(blue, $blue: 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    mod green {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $green: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #00c800;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(lime, $green: 100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: darkgreen;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $green: 255)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: lime;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(lime, $green: 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
    #[test]
    fn named() {
        assert_eq!(
        rsass(
            "a {b: change-color($color: black, $red: 12, $green: 24, $blue: 48)}\
            \n"
        )
        .unwrap(),
        "a {\
        \n  b: #0c1830;\
        \n}\
        \n"
    );
    }
    mod red {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn high() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $red: 200)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #c80000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn low() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $red: 100)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #640000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn max() {
            assert_eq!(
                rsass(
                    "a {b: change-color(black, $red: 255)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: red;\
        \n}\
        \n"
            );
        }
        #[test]
        fn min() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $red: 0)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: black;\
        \n}\
        \n"
            );
        }
    }
}

// From "sass-spec/spec/core_functions/color/change_color/units.hrx"
mod units {
    #[allow(unused)]
    use super::rsass;
    mod hue {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn angle() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $hue: 60rad)}\
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
                rsass(
                    "a {b: change-color(red, $hue: 60deg)}\
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
                rsass(
                    "a {b: change-color(red, $hue: 60)}\
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
                rsass(
                    "a {b: change-color(red, $hue: 60in)}\
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
    mod lightness {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn percent() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 30%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #990000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 30)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #990000;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $lightness: 30in)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #990000;\
        \n}\
        \n"
            );
        }
    }
    mod saturation {
        #[allow(unused)]
        use super::rsass;
        #[test]
        fn percent() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $saturation: 50%)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unitless() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $saturation: 50)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
            );
        }
        #[test]
        fn unknown() {
            assert_eq!(
                rsass(
                    "a {b: change-color(red, $saturation: 50in)}\
            \n"
                )
                .unwrap(),
                "a {\
        \n  b: #bf4040;\
        \n}\
        \n"
            );
        }
    }
}
