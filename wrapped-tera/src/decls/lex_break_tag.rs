macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_break_tag {
    () => {
        deps!();
        # [test] fn lex_break_tag () { assert ! (TeraParser :: parse (Rule :: break_tag , "{% break %}") . is_ok ()) ; }
    };
}

lex_break_tag!()