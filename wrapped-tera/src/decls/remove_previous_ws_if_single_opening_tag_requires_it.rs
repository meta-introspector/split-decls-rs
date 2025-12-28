macro_rules! deps {
    () => {
        WS!();
        Node!();
    };
}

macro_rules! remove_previous_ws_if_single_opening_tag_requires_it {
    () => {
        deps!();
        # [test] fn remove_previous_ws_if_single_opening_tag_requires_it () { let ws = WS { left : true , right : false } ; let ast = vec ! [Node :: Text ("hey " . to_string ()) , Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: Text ("hey" . to_string ()) , Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) ,]) ; }
    };
}

remove_previous_ws_if_single_opening_tag_requires_it!();