macro_rules! deps {
    () => {
        Forloop!();
        Node!();
        ExprVal!();
        Expr!();
        WS!();
    };
}

macro_rules! parse_break {
    () => {
        deps!();
        # [test] fn parse_break () { let ast = parse ("{% for item in items %}{% break -%}{% endfor %}") . unwrap () ; let for_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Forloop (for_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Ident ("items" . to_string ())) , body : vec ! [Node :: Break (WS { left : false , right : true }) ,] , empty_body : None , } , for_ws ,)) ; }
    };
}

parse_break!();