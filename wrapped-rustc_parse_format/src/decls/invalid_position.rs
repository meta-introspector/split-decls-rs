macro_rules! invalid_position {
    () => {
        # [test] fn invalid_position () { musterr ("{18446744073709551616}") ; }
    };
}

invalid_position!()