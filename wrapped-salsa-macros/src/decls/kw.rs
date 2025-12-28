macro_rules! kw {
    () => {
        mod kw { syn :: custom_keyword ! (with) ; syn :: custom_keyword ! (maybe_update) ; }
    };
}

kw!();