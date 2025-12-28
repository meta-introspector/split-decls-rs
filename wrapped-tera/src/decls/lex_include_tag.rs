macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_include_tag {
    () => {
        deps!();
        # [test] fn lex_include_tag () { assert ! (TeraParser :: parse (Rule :: include_tag , "{% include \"index.html\" %}") . is_ok ()) ; assert ! (TeraParser :: parse (Rule :: include_tag , "{% include [\"index.html\"] %}") . is_ok ()) ; assert ! (TeraParser :: parse (Rule :: include_tag , "{% include [\"index.html\"] ignore missing %}") . is_ok ()) ; }
    };
}

lex_include_tag!();