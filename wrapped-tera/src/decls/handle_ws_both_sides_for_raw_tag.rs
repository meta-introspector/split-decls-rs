macro_rules! deps {
    () => {
        Node!();
        WS!();
    };
}

macro_rules! handle_ws_both_sides_for_raw_tag {
    () => {
        deps!();
        # [test] fn handle_ws_both_sides_for_raw_tag () { let start_ws = WS { left : true , right : false } ; let end_ws = WS { left : true , right : true } ; let ast = vec ! [Node :: Raw (start_ws , "  hey " . to_string () , end_ws) , Node :: Text ("  hey" . to_string ())] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Raw (start_ws , "  hey" . to_string () , end_ws) , Node :: Text ("hey" . to_string ()) ,]) ; }
    };
}

handle_ws_both_sides_for_raw_tag!();