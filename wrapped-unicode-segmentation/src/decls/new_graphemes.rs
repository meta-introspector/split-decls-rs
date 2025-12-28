macro_rules! deps {
    () => {
        GraphemeCursor!();
        Graphemes!();
    };
}

macro_rules! new_graphemes {
    () => {
        deps!();
        # [inline] pub fn new_graphemes (s : & str , is_extended : bool) -> Graphemes < '_ > { let len = s . len () ; Graphemes { string : s , cursor : GraphemeCursor :: new (0 , len , is_extended) , cursor_back : GraphemeCursor :: new (len , len , is_extended) , } }
    };
}

new_graphemes!();