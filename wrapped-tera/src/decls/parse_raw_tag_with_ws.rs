macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_raw_tag_with_ws {
    () => {
        deps!();
        # [test] fn parse_raw_tag_with_ws () { let ast = parse ("{% raw %}    yaml_test:     {% endraw %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Raw (start_ws , "    yaml_test:     " . to_string () , end_ws)) ; }
    };
}

parse_raw_tag_with_ws!();