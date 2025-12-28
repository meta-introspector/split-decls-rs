macro_rules! deps {
    () => {
        WS!();
        FilterSection!();
        Expr!();
        FunctionCall!();
        Node!();
        ExprVal!();
    };
}

macro_rules! parse_filter_section_preserves_ws {
    () => {
        deps!();
        # [test] fn parse_filter_section_preserves_ws () { let ast = parse ("{% filter upper %}  {{a}}  B  {% endfilter %}") . unwrap () ; assert_eq ! (ast [0] , Node :: FilterSection (WS :: default () , FilterSection { filter : FunctionCall { name : "upper" . to_string () , args : HashMap :: new () } , body : vec ! [Node :: Text ("  " . to_string ()) , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Ident ("a" . to_string ()))) , Node :: Text ("  B  " . to_string ())] } , WS :: default () ,)) ; }
    };
}

parse_filter_section_preserves_ws!()