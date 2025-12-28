macro_rules! deps {
    () => {
        Node!();
        WS!();
    };
}

macro_rules! parse_comments_before_extends {
    () => {
        deps!();
        # [test] fn parse_comments_before_extends () { let ast = parse ("{# A comment #}{% extends \"index.html\" -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: Extends (WS { left : false , right : true } , "index.html" . to_string () ,) ,) ; }
    };
}

parse_comments_before_extends!();