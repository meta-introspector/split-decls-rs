macro_rules! delim_to_str {
    () => {
        fn delim_to_str (d : tt :: DelimiterKind , closing : bool) -> Option < & 'static str > { let texts = match d { tt :: DelimiterKind :: Parenthesis => "()" , tt :: DelimiterKind :: Brace => "{}" , tt :: DelimiterKind :: Bracket => "[]" , tt :: DelimiterKind :: Invisible => return None , } ; let idx = closing as usize ; Some (& texts [idx .. texts . len () - (1 - idx)]) }
    };
}

delim_to_str!()