macro_rules! deps {
    () => {
        MacroDefinition!();
        Node!();
        WS!();
    };
}

macro_rules! handle_ws_both_sides_for_macro_definitions {
    () => {
        deps!();
        # [test] fn handle_ws_both_sides_for_macro_definitions () { let start_ws = WS { left : true , right : true } ; let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: MacroDefinition (start_ws , MacroDefinition { name : "something" . to_string () , args : HashMap :: new () , body : vec ! [Node :: Text ("\n  " . to_string ()) , Node :: Text ("hey" . to_string ()) , Node :: Text ("  " . to_string ()) ,] , } , end_ws ,)] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: MacroDefinition (start_ws , MacroDefinition { name : "something" . to_string () , args : HashMap :: new () , body : vec ! [Node :: Text ("hey" . to_string ())] , } , end_ws ,) ,]) ; }
    };
}

handle_ws_both_sides_for_macro_definitions!()