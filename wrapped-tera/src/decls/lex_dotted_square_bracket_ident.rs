macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_dotted_square_bracket_ident {
    () => {
        deps!();
        # [test] fn lex_dotted_square_bracket_ident () { let inputs = vec ! ["hey.ho.hu" , "hey.0" , "h.u.x.0" , "hey['ho'][\"hu\"]" , "hey[0]" , "h['u'].x[0]" , "hey[a[0]]" ,] ; for i in inputs { assert_lex_rule ! (Rule :: dotted_square_bracket_ident , i) ; } let invalid_inputs = vec ! ["." , "9.w"] ; for i in invalid_inputs { assert ! (TeraParser :: parse (Rule :: dotted_square_bracket_ident , i) . is_err ()) ; } }
    };
}

lex_dotted_square_bracket_ident!()