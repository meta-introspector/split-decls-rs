macro_rules! deps {
    () => {
        Forloop!();
        ExprVal!();
        Expr!();
        WS!();
        Node!();
    };
}

macro_rules! parse_continue {
    () => {
        deps!();
        # [test] fn parse_continue () { let ast = parse ("{% for item in items %}{% continue -%}{% endfor %}") . unwrap () ; let for_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Forloop (for_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Ident ("items" . to_string ())) , body : vec ! [Node :: Continue (WS { left : false , right : true }) ,] , empty_body : None , } , for_ws ,)) ; }
    };
}

parse_continue!()