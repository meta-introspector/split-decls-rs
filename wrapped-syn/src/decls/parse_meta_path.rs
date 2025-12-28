macro_rules! deps {
    () => {
        Punctuated!();
        ParseStream!();
        Result!();
    };
}

macro_rules! parse_meta_path {
    () => {
        deps!();
        fn parse_meta_path (input : ParseStream) -> Result < Path > { Ok (Path { leading_colon : input . parse () ? , segments : { let mut segments = Punctuated :: new () ; if input . peek (Ident :: peek_any) { let ident = Ident :: parse_any (input) ? ; segments . push_value (PathSegment :: from (ident)) ; } else if input . is_empty () { return Err (input . error ("expected nested attribute")) ; } else if input . peek (Lit) { return Err (input . error ("unexpected literal in nested attribute, expected ident")) ; } else { return Err (input . error ("unexpected token in nested attribute, expected ident")) ; } while input . peek (Token ! [::]) { let punct = input . parse () ? ; segments . push_punct (punct) ; let ident = Ident :: parse_any (input) ? ; segments . push_value (PathSegment :: from (ident)) ; } segments } , }) }
    };
}

parse_meta_path!();