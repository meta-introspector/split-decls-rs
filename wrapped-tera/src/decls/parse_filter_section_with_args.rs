macro_rules! deps {
    () => {
        Node!();
        FilterSection!();
        ExprVal!();
        FunctionCall!();
        WS!();
        Expr!();
    };
}

macro_rules! parse_filter_section_with_args {
    () => {
        deps!();
        # [test] fn parse_filter_section_with_args () { let ast = parse ("{% filter upper(attr=1) -%}A{%- endfilter %}") . unwrap () ; let start_ws = WS { right : true , .. Default :: default () } ; let end_ws = WS { left : true , .. Default :: default () } ; let mut args = HashMap :: new () ; args . insert ("attr" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: FilterSection (start_ws , FilterSection { filter : FunctionCall { name : "upper" . to_string () , args } , body : vec ! [Node :: Text ("A" . to_string ())] , } , end_ws ,)) ; }
    };
}

parse_filter_section_with_args!()