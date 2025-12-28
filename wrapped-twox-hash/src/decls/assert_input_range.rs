macro_rules! assert_input_range {
    () => {
        macro_rules ! assert_input_range { ($ min : literal .., $ len : expr) => { assert ! ($ min <= $ len) ; } ; ($ min : literal ..=$ max : literal , $ len : expr) => { assert ! ($ min <= $ len) ; assert ! ($ len <= $ max) ; } ; }
    };
}

assert_input_range!();