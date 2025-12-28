macro_rules! kw {
    () => {
        mod kw { syn :: custom_keyword ! (fields) ; syn :: custom_keyword ! (skip) ; syn :: custom_keyword ! (skip_all) ; syn :: custom_keyword ! (level) ; syn :: custom_keyword ! (target) ; syn :: custom_keyword ! (parent) ; syn :: custom_keyword ! (follows_from) ; syn :: custom_keyword ! (name) ; syn :: custom_keyword ! (err) ; syn :: custom_keyword ! (ret) ; }
    };
}

kw!();