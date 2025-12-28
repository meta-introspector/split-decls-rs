macro_rules! invalid_width {
    () => {
        # [test] fn invalid_width () { musterr ("{:18446744073709551616}") ; }
    };
}

invalid_width!();