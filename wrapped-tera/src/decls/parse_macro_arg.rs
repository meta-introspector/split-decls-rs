macro_rules! deps {
    () => {
        Error!();
        ExprVal!();
    };
}

macro_rules! parse_macro_arg {
    () => {
        deps!();
        fn parse_macro_arg (p : Pair < Rule >) -> TeraResult < ExprVal > { let val = match p . as_rule () { Rule :: int => Some (ExprVal :: Int (p . as_str () . parse () . map_err (| _ | Error :: msg (format ! ("Integer out of bounds: `{}`" , p . as_str ()))) ? ,)) , Rule :: float => Some (ExprVal :: Float (p . as_str () . parse () . map_err (| _ | Error :: msg (format ! ("Float out of bounds: `{}`" , p . as_str ()))) ? ,)) , Rule :: boolean => match p . as_str () { "true" => Some (ExprVal :: Bool (true)) , "True" => Some (ExprVal :: Bool (true)) , "false" => Some (ExprVal :: Bool (false)) , "False" => Some (ExprVal :: Bool (false)) , _ => unreachable ! () , } , Rule :: string => Some (ExprVal :: String (replace_string_markers (p . as_str ()))) , _ => unreachable ! ("Got {:?} in parse_macro_arg: {}" , p . as_rule () , p . as_str ()) , } ; Ok (val . unwrap ()) }
    };
}

parse_macro_arg!()