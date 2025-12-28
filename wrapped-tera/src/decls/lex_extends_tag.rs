macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_extends_tag {
    () => {
        deps!();
        # [test] fn lex_extends_tag () { assert ! (TeraParser :: parse (Rule :: extends_tag , "{% extends \"index.html\" %}") . is_ok ()) ; }
    };
}

lex_extends_tag!()