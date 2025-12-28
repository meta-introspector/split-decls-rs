macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! parse_include_tag {
    () => {
        deps!();
        # [test] fn parse_include_tag () { let ast = parse ("{% include \"index.html\" -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: Include (WS { left : false , right : true } , vec ! ["index.html" . to_string ()] , false ,) ,) ; let ast = parse ("{% include [\"custom/index.html\", \"index.html\"] ignore missing %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Include (WS { left : false , right : false } , vec ! ["custom/index.html" . to_string () , "index.html" . to_string ()] , true ,) ,) ; }
    };
}

parse_include_tag!();