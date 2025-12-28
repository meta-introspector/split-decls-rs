macro_rules! parse_empty_template {
    () => {
        # [test] fn parse_empty_template () { let ast = parse ("") . unwrap () ; assert_eq ! (ast . len () , 0) ; }
    };
}

parse_empty_template!()