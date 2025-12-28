macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_continue_tag {
    () => {
        deps!();
        # [test] fn lex_continue_tag () { assert ! (TeraParser :: parse (Rule :: continue_tag , "{% continue %}") . is_ok ()) ; }
    };
}

lex_continue_tag!();