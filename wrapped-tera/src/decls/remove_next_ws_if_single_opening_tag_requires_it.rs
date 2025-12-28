macro_rules! deps {
    () => {
        Node!();
        WS!();
    };
}

macro_rules! remove_next_ws_if_single_opening_tag_requires_it {
    () => {
        deps!();
        # [test] fn remove_next_ws_if_single_opening_tag_requires_it () { let ws = WS { left : true , right : true } ; let ast = vec ! [Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) , Node :: Text ("  hey" . to_string ()) ,] ; assert_eq ! (remove_whitespace (ast , None) , vec ! [Node :: ImportMacro (ws , "hey " . to_string () , "ho" . to_string ()) , Node :: Text ("hey" . to_string ()) ,]) ; }
    };
}

remove_next_ws_if_single_opening_tag_requires_it!()