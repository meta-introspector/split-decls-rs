macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        # [doc = " Displays a TOML parse error"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " TOML parse error at line 1, column 10"] # [doc = "   |"] # [doc = " 1 | 00:32:00.a999999"] # [doc = "   |          ^"] # [doc = " Unexpected `a`"] # [doc = " Expected `digit`"] # [doc = " While parsing a Time"] # [doc = " While parsing a Date-Time"] impl core :: fmt :: Display for Error { fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { let mut context = false ; if let (Some (input) , Some (span)) = (& self . input , self . span ()) { context = true ; let (line , column) = translate_position (input . as_bytes () , span . start) ; let line_num = line + 1 ; let col_num = column + 1 ; let gutter = line_num . to_string () . len () ; let content = input . split ('\n') . nth (line) . expect ("valid line number") ; let highlight_len = span . end - span . start ; let highlight_len = highlight_len . min (content . len () . saturating_sub (column)) ; writeln ! (f , "TOML parse error at line {line_num}, column {col_num}") ? ; for _ in 0 ..= gutter { write ! (f , " ") ? ; } writeln ! (f , "|") ? ; write ! (f , "{line_num} | ") ? ; writeln ! (f , "{content}") ? ; for _ in 0 ..= gutter { write ! (f , " ") ? ; } write ! (f , "|") ? ; for _ in 0 ..= column { write ! (f , " ") ? ; } write ! (f , "^") ? ; for _ in 1 .. highlight_len { write ! (f , "^") ? ; } writeln ! (f) ? ; } writeln ! (f , "{}" , self . message) ? ; if ! context && ! self . keys . is_empty () { writeln ! (f , "in `{}`" , self . keys . join (".")) ? ; } Ok (()) } }
    };
}

impl_166!()