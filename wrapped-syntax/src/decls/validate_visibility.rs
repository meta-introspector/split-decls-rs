macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_visibility {
    () => {
        deps!();
        fn validate_visibility (vis : ast :: Visibility , errors : & mut Vec < SyntaxError >) { let path_without_in_token = vis . in_token () . is_none () && vis . path () . and_then (| p | p . as_single_name_ref ()) . and_then (| n | n . ident_token ()) . is_some () ; if path_without_in_token { errors . push (SyntaxError :: new ("incorrect visibility restriction" , vis . syntax . text_range ())) ; } let parent = match vis . syntax () . parent () { Some (it) => it , None => return , } ; match parent . kind () { FN | CONST | TYPE_ALIAS => () , _ => return , } let impl_def = match parent . parent () . and_then (| it | it . parent ()) . and_then (ast :: Impl :: cast) { Some (it) => it , None => return , } ; if impl_def . trait_ () . is_some () && impl_def . attrs () . next () . is_none () { errors . push (SyntaxError :: new ("Unnecessary visibility qualifier" , vis . syntax . text_range ())) ; } }
    };
}

validate_visibility!();