macro_rules! deps {
    () => {
        Node!();
        WS!();
        ExprVal!();
        Expr!();
        Forloop!();
    };
}

macro_rules! parse_value_forloop_empty {
    () => {
        deps!();
        # [test] fn parse_value_forloop_empty () { let ast = parse ("{% for item in [1,2,] %}A{% else %}B{%- endfor %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Forloop (start_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Int (2)) ,])) , body : vec ! [Node :: Text ("A" . to_string ())] , empty_body : Some (vec ! [Node :: Text ("B" . to_string ())]) , } , end_ws ,)) ; }
    };
}

parse_value_forloop_empty!()