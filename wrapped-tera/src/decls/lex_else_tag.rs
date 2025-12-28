macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_else_tag {
    () => {
        deps!();
        # [test] fn lex_else_tag () { assert ! (TeraParser :: parse (Rule :: else_tag , "{% else %}") . is_ok ()) ; }
    };
}

lex_else_tag!()