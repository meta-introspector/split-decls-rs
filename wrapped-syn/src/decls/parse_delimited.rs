macro_rules! deps {
    () => {
        Result!();
        ParseBuffer!();
    };
}

macro_rules! parse_delimited {
    () => {
        deps!();
        fn parse_delimited < 'a > (input : & ParseBuffer < 'a > , delimiter : Delimiter ,) -> Result < (DelimSpan , ParseBuffer < 'a >) > { input . step (| cursor | { if let Some ((content , span , rest)) = cursor . group (delimiter) { let scope = span . close () ; let nested = crate :: parse :: advance_step_cursor (cursor , content) ; let unexpected = crate :: parse :: get_unexpected (input) ; let content = crate :: parse :: new_parse_buffer (scope , nested , unexpected) ; Ok (((span , content) , rest)) } else { let message = match delimiter { Delimiter :: Parenthesis => "expected parentheses" , Delimiter :: Brace => "expected curly braces" , Delimiter :: Bracket => "expected square brackets" , Delimiter :: None => "expected invisible group" , } ; Err (cursor . error (message)) } }) }
    };
}

parse_delimited!()