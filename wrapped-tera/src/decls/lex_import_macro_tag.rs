macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_import_macro_tag {
    () => {
        deps!();
        # [test] fn lex_import_macro_tag () { assert ! (TeraParser :: parse (Rule :: import_macro_tag , "{% import \"macros.html\" as macros %}" ,) . is_ok ()) ; }
    };
}

lex_import_macro_tag!()