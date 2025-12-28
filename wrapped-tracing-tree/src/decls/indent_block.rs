macro_rules! deps {
    () => {
        SpanMode!();
    };
}

macro_rules! indent_block {
    () => {
        deps!();
        fn indent_block (block : & str , buf : & mut String , mut indent : usize , indent_amount : usize , indent_lines : bool , prefix : & str , style : SpanMode ,) { let lines : Vec < & str > = block . lines () . collect () ; let indent_spaces = indent * indent_amount ; buf . reserve (block . len () + (lines . len () * indent_spaces)) ; match style { SpanMode :: PreOpen | SpanMode :: PostClose => { indent += 1 ; } _ => () , } if indent_lines { indent_block_with_lines (& lines , buf , indent , indent_amount , prefix , style) ; } else { let mut indent_str = " " . repeat (indent_spaces) ; let mut first_line = true ; for line in lines { buf . push_str (prefix) ; buf . push (' ') ; buf . push_str (& indent_str) ; if first_line { first_line = false ; indent_str . push_str ("  ") ; } buf . push_str (line) ; buf . push ('\n') ; } } }
    };
}

indent_block!()