macro_rules! kw {
    () => {
        mod kw { syn :: custom_keyword ! (Keywords) ; syn :: custom_keyword ! (Symbols) ; }
    };
}

kw!()