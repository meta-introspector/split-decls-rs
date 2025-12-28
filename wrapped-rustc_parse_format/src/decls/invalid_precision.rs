macro_rules! invalid_precision {
    () => {
        # [test] fn invalid_precision () { musterr ("{:.18446744073709551616}") ; }
    };
}

invalid_precision!()